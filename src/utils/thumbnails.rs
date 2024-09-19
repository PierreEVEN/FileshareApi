use std::ffi::{OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use anyhow::Error;
use crate::database::object::ObjectId;
use crate::utils::enc_string::EncString;

pub struct Thumbnail {}

impl Thumbnail {
    pub fn create(file: PathBuf, thumbnail_path: &Path, mimetype: &EncString, size: u32) -> Result<PathBuf, Error> {
        
        if !mimetype.plain()?.starts_with("image/") {
            return Err(Error::msg("Unsupported mimetype"));
        }
        fs::create_dir_all(thumbnail_path)?;
        let mime_plain = mimetype.plain()?;
        let mime_plain = match mime_plain.as_str() {
            "image/vnd.microsoft.icon" => {
                "image/ico"
            }
            plain => {plain}
        };
        let mut mime = mime_plain.split("/");
        mime.next();
        let mut path_str = OsString::from(mime.next().ok_or(Error::msg(format!("invalid mimetype : {}", mime_plain)))?);
        path_str.push(":");
        path_str.push(file.as_os_str());

        println!("path : {:?}", path_str);
        let cmd = Command::new("mogrify")
            .arg("-format")
            .arg("webp")
            .arg("-interlace")
            .arg("plane")
            .arg("-quality")
            .arg("70%")
            .arg("-path")
            .arg(thumbnail_path)
            .arg("-thumbnail")
            .arg(format!("{size}x{size}"))
            .arg("-auto-orient")
            .arg(&path_str)
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()?;
        cmd.wait_with_output()?;
        Ok(thumbnail_path.join(Self::thumbnail_filename(file.as_path())))
    }
    pub fn mimetype<'a>() -> &'a str {
        "image/jpeg"
    }
    pub fn path(id: &ObjectId, thumbnail_path: &Path) -> PathBuf {
        thumbnail_path.join(format!("{id}"))
    }
    pub fn thumbnail_filename(file: &Path) -> OsString {
        let mut file_name = OsString::from(file.file_name().unwrap());
        file_name.push(".webp");
        file_name
    }
    pub fn find_or_create(file: &Path, thumbnail_path: &Path, mimetype: &EncString, size: u32) -> Result<PathBuf, Error> {
        let path = thumbnail_path.join(Self::thumbnail_filename(file));
        if path.exists() {
            Ok(path.to_path_buf())
        } else {
            Self::create(PathBuf::from(file), thumbnail_path, mimetype, size)
        }
    }
}