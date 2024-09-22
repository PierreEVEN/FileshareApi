use crate::database::object::ObjectId;
use crate::utils::enc_string::EncString;
use anyhow::Error;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;

fn video_thumbnail(file: &PathBuf, thumbnail_path: &Path, size: u32) -> Result<(), Error> {
    let get_duration_cmd = match Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(file)
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
        .arg(file)
        .arg("-vf")
        .arg(format!("scale={size}:{size}:force_original_aspect_ratio=decrease"))
        .arg("-vframes")
        .arg("1")
        .arg(thumbnail_path.join(Thumbnail::thumbnail_filename(file)))
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

fn image_thubmnail(file: &Path, thumbnail_path: &Path, mimetype: &EncString, size: u32) -> Result<(), Error> {
    fs::create_dir_all(thumbnail_path)?;
    let mime_plain = mimetype.plain()?;
    let mime_plain = match mime_plain.as_str() {
        "image/vnd.microsoft.icon" => {
            "image/ico"
        }
        "image/svg+xml" => {
            "image/svg"
        }
        plain => { plain }
    };
    let mut mime = mime_plain.split("/");
    mime.next();
    let mut path_str = OsString::from(mime.next().ok_or(Error::msg(format!("invalid mimetype : {}", mime_plain)))?);
    path_str.push(":");
    path_str.push(file.as_os_str());

    let cmd = match Command::new("mogrify")
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
        .spawn() {
        Ok(cmd) => { cmd }
        Err(err) => {
            return Err(Error::msg(format!("This server doesn't support thumbnails because imagemagick is not available : {}", err)))
        }
    };
    cmd.wait_with_output()?;
    Ok(())
}

pub struct Thumbnail {}

impl Thumbnail {
    pub fn create(file: PathBuf, thumbnail_path: &Path, mimetype: &EncString, size: u32) -> Result<PathBuf, Error> {
        let mut mime_start = mimetype.split("/");
        match mime_start.next().ok_or(Error::msg("Invalid mimetype"))? {
            "image" => {
                image_thubmnail(&file, thumbnail_path, mimetype, size)?;
            }
            "video" => {
                video_thumbnail(&file, thumbnail_path, size)?;
            }
            _ => {
                return Err(Error::msg("Unsupported mimetype"));
            }
        }
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