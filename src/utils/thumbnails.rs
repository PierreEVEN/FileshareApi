use crate::database::object::{Object, ObjectId};
use crate::utils::enc_string::EncString;
use anyhow::Error;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use crate::database::Database;

pub struct Thumbnail {}

impl Thumbnail {
    fn video_thumbnail(db: &Database, object: &Object, size: u32) -> Result<(), Error> {
        let get_duration_cmd = match Command::new("ffprobe")
            .arg("-v")
            .arg("error")
            .arg("-show_entries")
            .arg("format=duration")
            .arg("-of")
            .arg("default=noprint_wrappers=1:nokey=1")
            .arg(object.data_path(db))
            .stderr(Stdio::inherit())
            .output() {
            Ok(cmd) => { cmd }
            Err(err) => {
                return Err(Error::msg(format!("This server doesn't support thumbnails because ffmpeg is not available : {}", err)))
            }
        };
        let duration = match f32::from_str(String::from_utf8(get_duration_cmd.stdout)?.as_str()) {
            Ok(duration) => { duration }
            Err(_) => { 0f32 }
        };

        let cmd = match Command::new("ffmpeg")
            .arg("-v")
            .arg("error")
            .arg("-ss")
            .arg((duration / 2f32).to_string())
            .arg("-i")
            .arg(object.data_path(db))
            .arg("-vf")
            .arg(format!("scale={size}:{size}:force_original_aspect_ratio=decrease"))
            .arg("-vframes")
            .arg("1")
            .arg(object.thumbnail_path(db))
            .stderr(Stdio::inherit())
            .spawn() {
            Ok(cmd) => { cmd }
            Err(err) => {
                return Err(Error::msg(format!("This server doesn't support thumbnails because ffmpeg is not available : {}", err)))
            }
        };
        cmd.wait_with_output()?;
        Ok(())
    }

    fn image_thubmnail(db: &Database, object: &Object, mimetype: &EncString, size: u32) -> Result<(), Error> {
        fs::create_dir_all(object.thumbnail_path(db).parent().unwrap())?;
        let mime_plain = mimetype.plain()?;
        let mime_plain = match mime_plain.as_str() {
            "image/x-icon" => {
                "image/ico"
            }
            "image/vnd.microsoft.icon" => {
                "image/ico"
            }
            "image/svg+xml" => {
                "image/svg"
            }
            "image/x-canon-cr2" => {
                "image/cr2"
            }
            plain => { plain }
        };
        let mut mime = mime_plain.split("/");
        mime.next();
        let mut path_str = OsString::from(mime.next().ok_or(Error::msg(format!("invalid mimetype : {}", mime_plain)))?);
        path_str.push(":");
        path_str.push(object.data_path(db).as_os_str());

        let cmd = match Command::new("mogrify")
            .arg("-format")
            .arg("webp")
            .arg("-interlace")
            .arg("plane")
            .arg("-quality")
            .arg("70%")
            .arg("-path")
            .arg(object.thumbnail_path(db).parent().unwrap())
            .arg("-thumbnail")
            .arg(format!("{size}x{size}"))
            .arg("-auto-orient")
            .arg(&path_str)
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn() {
            Ok(cmd) => { cmd }
            Err(err) => {
                return Err(Error::msg(format!("This server doesn't support thumbnails because imagemagick is not available : {}", err)))
            }
        };
        cmd.wait_with_output()?;

        let mut generated_file_name = OsString::from(object.thumbnail_path(db).file_name().unwrap());
        generated_file_name.push(".webp");
        fs::rename(object.thumbnail_path(db).parent().unwrap().join(generated_file_name), object.thumbnail_path(db))?;
        Ok(())
    }

    pub fn create(db: &Database, object: &Object, mimetype: &EncString, size: u32) -> Result<PathBuf, Error> {
        let mime_text = mimetype.plain()?;
        let mut mime_start = mime_text.split("/");
        match mime_start.next().ok_or(Error::msg("Invalid mimetype"))? {
            "image" => {
                Self::image_thubmnail(db, object, mimetype, size)?;
            }
            "video" => {
                Self::video_thumbnail(db, object, size)?;
            }
            _ => {
                return Err(Error::msg(format!("Unsupported mimetype : {mimetype}")));
            }
        }
        Ok(object.thumbnail_path(db))
    }
    pub fn mimetype<'a>() -> &'a str {
        "image/jpeg"
    }
    pub fn path(id: &ObjectId, thumbnail_path: &Path) -> PathBuf {
        thumbnail_path.join(format!("{id}"))
    }
    pub fn find_or_create(db: &Database, object: &Object, mimetype: &EncString, size: u32) -> Result<PathBuf, Error> {
        if object.thumbnail_path(db).exists() {
            Ok(object.thumbnail_path(db))
        } else {
            Self::create(db, object, mimetype, size)
        }
    }
}