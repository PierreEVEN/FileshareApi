use crate::database::item::{ItemId};
use crate::database::Database;
use crate::{make_database_id, query_fmt, query_object, query_objects};
use anyhow::Error;
use postgres_from_row::FromRow;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use tracing::{error};

make_database_id!(ObjectId);

#[derive(Debug, FromRow)]
pub struct Object {
    id: ObjectId,
    pub hash: String,
}

impl Object {
    pub async fn from_id(db: &Database, id: &ObjectId) -> Result<Self, Error> {
        Ok(query_object!(db, Object, "SELECT * FROM SCHEMA_NAME.objects WHERE id = $1", id).unwrap())
    }

    pub async fn from_item(db: &Database, id: &ItemId) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Object, "SELECT * FROM SCHEMA_NAME.objects WHERE id = $1", id))
    }

    pub async fn from_hash(db: &Database, hash: &String) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Object, "SELECT * FROM SCHEMA_NAME.objects WHERE hash = $1", hash))
    }

    pub async fn insert(db: &Database, file: &Path, hash: &String) -> Result<Self, Error> {
        let new_object = query_object!(db, Self, "INSERT INTO SCHEMA_NAME.objects (hash) VALUES ($1) RETURNING *", hash).ok_or(Error::msg("Failed to insert object"))?;
        if !new_object.data_path(db).parent().unwrap().exists() {
            fs::create_dir_all(new_object.data_path(db).parent().unwrap())?;
        }
        match fs::rename(file, new_object.data_path(db)) {
            Ok(_) => {}
            Err(err) => {
                query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.objects WHERE id = $1;"#, *new_object.id);
                return Err(Error::msg(format!("Failed to store new object : {err}")));
            }
        };
        Ok(new_object)
    }

    pub async fn delete(&self, db: &Database) -> Result<(), Error> {

        if self.data_path(db).exists() {
            fs::remove_file(self.data_path(db))?;
        }

        if self.thumbnail_path(db).exists() {
            fs::remove_file(self.thumbnail_path(db))?;
        }

        query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.objects WHERE id = $1;"#, *self.id);
        Ok(())
    }

    pub fn data_path(&self, db: &Database) -> PathBuf {
        db.file_storage_path.join(self.id.to_string().as_str())
    }

    pub fn thumbnail_path(&self, db: &Database) -> PathBuf {
        db.thumbnail_storage_path.join(self.id.to_string().as_str())
    }

    pub fn id(&self) -> &ObjectId {
        &self.id
    }


    pub async fn equals_to_file(&self, db: &Database, file: PathBuf) -> Result<bool, Error> {
        if !self.data_path(db).exists() {
            error!("The object {:?} is not pointing to a valid file", self);
            self.delete(db).await?;
            return Ok(false);
        }

        let mut reader1 = BufReader::new(File::open(self.data_path(db)).map_err(|err| Error::msg(format!("Cannot open object data : {err}")))?);
        let mut reader2 = BufReader::new(File::open(file).map_err(|err| Error::msg(format!("Cannot open tested file : {err}")))?);
        let mut buf1 = [0; 10000];
        let mut buf2 = [0; 10000];

        while let Ok(n1) = reader1.read(&mut buf1) {
            if n1 > 0 {
                if let Ok(n2) = reader2.read(&mut buf2) {
                    if n1 == n2 && buf1 == buf2 {
                        continue;
                    }
                    return Ok(false);
                }
            } else {
                break;
            }
        }

        Ok(true)
    }
}
