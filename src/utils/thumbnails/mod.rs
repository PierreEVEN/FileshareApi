use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use anyhow::Error;
use image::{ColorType, DynamicImage, ExtendedColorType, ImageBuffer, ImageEncoder, ImageReader, Luma, LumaA, Rgb, Rgba};
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use imagepipe::{ImageSource, Pipeline, SRGBImage};
use tracing::info;
use tracing::log::warn;
use crate::database::object::ObjectId;

pub struct Thumbnail {}

impl Thumbnail {
    pub fn create(file: &Path, thumbnail_path: &Path, id: &ObjectId) -> Result<PathBuf, Error> {
        if !thumbnail_path.exists() {
            fs::create_dir_all(thumbnail_path)?;
        }
        info!("A");
        let raw_image = match imagepipe::simple_decode_8bit(file, 500,  500) {
            Ok(res) => { res }
            Err(err) => { return Err(Error::msg(err)) }
        };

        info!("B");
        let final_image = DynamicImage::ImageRgb8(ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(raw_image.width as u32, raw_image.height as u32, raw_image.data).ok_or(Error::msg("failed to convert to image buffer"))?);

        info!("C");
        let final_image = final_image.thumbnail(500, 500);
        info!("D");

        let path_result = Self::path(id, thumbnail_path);
        let output = File::create(path_result.clone())?;
        let encoder = PngEncoder::new(output);
        final_image.write_with_encoder(encoder)?;
        info!("E");
        //encoder.write_image(&raw_image.data, raw_image.width as u32, raw_image.height as u32, ExtendedColorType::from(ColorType::Rgb8))?;
        Ok(path_result)
    }
    pub fn mimetype<'a>() -> &'a str {
        "image/jpeg"
    }
    pub fn path(id: &ObjectId, thumbnail_path: &Path) -> PathBuf {
        thumbnail_path.join(format!("{id}"))
    }
    pub fn find_or_create(file: &Path, thumbnail_path: &Path, id: &ObjectId) -> Result<PathBuf, Error> {
        let path = Self::path(id, thumbnail_path);
        if path.exists() {
            Ok(path.to_path_buf())
        } else {
            Self::create(file, thumbnail_path, id)
        }
    }
}