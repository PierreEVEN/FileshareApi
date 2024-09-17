use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use anyhow::Error;
use image::error::UnsupportedErrorKind::Format;
use image::imageops::FilterType;
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageFormat, ImageReader, Rgb, Rgba};
use image::DynamicImage::ImageRgb8;
use crate::database::object::ObjectId;
use crate::utils::enc_string::EncString;

pub struct Thumbnails {

}

impl Thumbnails {
    pub fn new(file: &Path, thumbnail_path: &Path, id: &ObjectId, mimetype: &EncString, name: &EncString) -> Result<PathBuf, Error>{
        let img = ImageReader::open(file)?.decode()?;
        let raw_image = rawloader::decode_file(file)?;

        match raw_image.data {
            rawloader::RawImageData::Integer(data) => {

                let format = match raw_image.cpp {
                    1 => {
                        ColorType::L16
                    }
                    2 => {
                        ColorType::La16
                    }
                    3 => {
                        ColorType::Rgb16

                    }
                    4 => {
                        ColorType::Rgba16
                    }
                    _ => {
                        return Err(Error::msg(format!("Invalid component count in raw image : {}", raw_image.cpp)));
                    }
                };

                let image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(100, 100);
                image.put_pixel()
                let mut image = DynamicImage::new(raw_image.width as u32, raw_image.height as u32, format);
                for pixel in data {
                    image.put_pixel(0, 0, Rgba::from(pixel))
                }
                
            },
            rawloader::RawImageData::Float(data) => {

            }
        }

        let thmb = img.thumbnail(100, 100);
        let path_result = thumbnail_path.join(format!("{id}"));
        let mut output = File::create(path_result.clone())?;
        resized.write_to(&mut output, ImageFormat::Jpeg)?;
        let encoder = JpegEncoder::new_with_quality(&mut writer, 95);
        img.write_with_encoder(encoder)?;
        output.flush()?;
        Ok(PathBuf::from(path_result))
    }
    pub fn mimetype<'a>() -> &'a str {
        "image/jpeg"
    }
    pub fn find_or_create()
}