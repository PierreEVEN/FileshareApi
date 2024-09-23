use std::{env, fs};
use std::io::Write;
use crate::database::item::{FileData, Item, ItemId, Trash};
use crate::database::object::Object;
use crate::database::repository::RepositoryId;
use crate::database::user::UserId;
use crate::database::{Database, DatabaseId};
use crate::utils::enc_string::EncString;
use anyhow::Error;
use axum::body::Body;
use axum::http::HeaderMap;
use futures::{io, TryStreamExt};
use serde::Serialize;
use std::path::PathBuf;
use std::str::FromStr;
use sha2::{Digest, Sha256};
use tokio::io::AsyncReadExt;
use tokio_util::io::StreamReader;

pub struct Upload {
    pub id: usize,
    item: Item,
    file: FileData,
    bytes_read: usize,
    hasher: Sha256,
}

impl Upload {
    pub fn new(headers: HeaderMap, owner: UserId) -> Result<Self, Error> {
        let mut item = Item::default();

        let name = EncString::try_from(headers.get("Content-Name").ok_or(Error::msg("missing Content-Name header"))?)?;

        item.name = name.clone();
        item.description = match headers.get("Content-Description") {
            None => { None }
            Some(header) => { Some(EncString::try_from(header)?) }
        };
        item.in_trash = false;
        item.repository = RepositoryId::from(DatabaseId::from_str(headers.get("Content-Repository").ok_or(Error::msg("missing Content-Repository header"))?.to_str()?)?);
        item.parent_item = match headers.get("Content-Parent") {
            None => { None }
            Some(header) => { Some(ItemId::from(DatabaseId::from_str(header.to_str()?)?)) }
        };
        item.owner = owner.clone();

        Ok(Self {
            id: 0,
            item,
            file: FileData {
                size: i64::from_str(headers.get("Content-Size").ok_or(Error::msg("missing Content-Size header"))?.to_str()?)?,
                mimetype: EncString::from(match mime_guess::from_path(PathBuf::from(name.plain()?.as_str())).first_raw() {
                    None => { "application/octet-stream" }
                    Some(mime_type) => {
                        mime_type
                    }
                }),
                timestamp: i64::from_str(headers.get("Content-Timestamp").ok_or(Error::msg("missing Content-Timestamp header"))?.to_str()?)?,
                object: Default::default(),
            },
            bytes_read: 0,
            hasher: Sha256::new(),
        })
    }

    pub async fn push_data(&mut self, body: Body) -> Result<(), Error> {
        let stream = body.into_data_stream();
        let stream = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let mut read = StreamReader::new(stream);

        let mut buffer = [0u8; 1024];

        if !self.get_file_path().parent().unwrap().exists() {
            fs::create_dir_all(self.get_file_path().parent().unwrap())?;
        };

        let mut file = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(self.get_file_path()).map_err(|err| { Error::msg(format!("Cannot open file sink : {err}")) })?;

        loop {
            let bytes = read.read(&mut buffer).await?;
            if bytes == 0 {
                break;
            }
            let input_bytes = &buffer[..bytes];
            if self.hasher.write(input_bytes)? != bytes {
                return Err(Error::msg("Invalid write amount"));
            }
            if file.write(input_bytes)? != bytes {
                return Err(Error::msg("Invalid write amount"));
            }
            self.bytes_read += bytes;
        }
        file.flush()?;

        Ok(())
    }

    pub fn get_file_path(&self) -> PathBuf {
        env::temp_dir().join("fileshare_upload").join(self.id.to_string())
    }

    pub fn get_state(&self) -> UploadState {
        let finished = self.bytes_read == self.file.size as usize;
        UploadState {
            id: self.id,
            finished,
            item: if finished { Some(self.item.clone()) } else { None },
        }
    }

    pub async fn store(&mut self, db: &Database) -> Result<Item, Error> {
        assert_eq!(self.bytes_read, self.file.size as usize);

        let hash = self.hasher.clone().finalize();
        let hash = format!("{:x}", hash);

        for existing in Object::from_hash(db, &hash).await? {
            if existing.equals_to_file(db, self.get_file_path()).await? {
                fs::remove_file(self.get_file_path())?;
                self.file.object = existing.id().clone();
                self.item.file = Some(self.file.clone());
                self.item.push(db).await?;
                return Ok(self.item.clone());
            }
        }

        let object = Object::insert(db, self.get_file_path().as_path(), &hash).await?;
        self.file.object = object.id().clone();
        self.item.file = Some(self.file.clone());
        self.item.push(db).await?;

        Ok(self.item.clone())
    }
}

#[derive(Serialize, Debug)]
pub struct UploadState {
    pub id: usize,
    pub finished: bool,
    pub item: Option<Item>,
}