use std::pin::Pin;
use std::task::{Context, Poll};
use anyhow::Error;
use axum::body::Bytes;
use tokio_stream::Stream;
use tokio_util::bytes::BufMut;
use crate::database::item::Item;

enum CompressionMethod {
    DEFLATE,
}

struct LocalFileHeader {
    signature: [u8; 4],
    versions_required: u16,
    bit_flag: u16,
    compression_method: CompressionMethod,
    last_modification_time: u16,
    last_modification_date: u16,
    crc32: u32,
    compressed_size: u32,
    uncompressed_size: u32,
    name_length: u16,
    num_extra_fields: u16,
    file_name: String,
}

struct CentralFileHeader {
    signature: [u8; 4],
    versions_required: u16,
    bit_flag: u16,
    compression_method: CompressionMethod,
    last_modification_time: u16,
    last_modification_date: u16,
    crc32: u32,
    compressed_size: u32,
    uncompressed_size: u32,
    name_length: u16,
    num_extra_fields: u16,
    file_name: String,
}

#[derive(Default)]
pub struct AsyncDirectoryZip {

}

impl AsyncDirectoryZip {
    fn local_header(&item: &Item) -> Result<Vec<u8>, Error>{

        let file_base_name = item.name.plain()?;

        // local file header
        let signature = 0x04034b50u32.to_le_bytes();
        let version = 0x2Du16.to_le_bytes();
        let flags = 0x0u16.to_le_bytes();
        let compression_method = 0x0u16.to_le_bytes();
        let last_modification_time = 0x0u16.to_le_bytes();
        let last_modification_date = 0x0u16.to_le_bytes();
        let crc32 = 0x0u32.to_le_bytes();
        let compressed_size = 0x0u32.to_le_bytes();
        let uncompressed_size = 0x0u32.to_le_bytes();
        let file_name_length = (file_base_name.len() as u16).to_le_bytes();
        let extra_field_length = 1u16.to_le_bytes();
        let file_name = file_base_name.as_str().as_bytes();
        let extra_fields = &[0];
        
        let mut local_file_header = vec![];
        local_file_header.put_slice(&signature);
        local_file_header.put_slice(&version);
        local_file_header.put_slice(&flags);
        local_file_header.put_slice(&compression_method);
        local_file_header.put_slice(&last_modification_time);
        local_file_header.put_slice(&last_modification_date);
        local_file_header.put_slice(&crc32);
        local_file_header.put_slice(&compressed_size);
        local_file_header.put_slice(&uncompressed_size);
        local_file_header.put_slice(&file_name_length);
        local_file_header.put_slice(&extra_field_length);
        local_file_header.put_slice(&file_name);
        Ok(local_file_header)
    }

    fn make_central_directory(item: &Item) -> Result<Vec<u8>, Error> {
        let signature = 0x02014b50u32.to_le_bytes();
        let version = 0x3F00u16.to_le_bytes();
        let version_required = 0x0A00u16.to_le_bytes();
        let flags = 0x0u16.to_le_bytes();
        let compression_method = 0x0u16.to_le_bytes();
        let last_modification_time = 0x0u16.to_le_bytes(); // @TODO : calculer
        let last_modification_date = 0x0u16.to_le_bytes(); // @TODO : calculer
        let crc32 = 0x0u32.to_le_bytes();
        let compressed_size = 0x0u32.to_le_bytes(); // @TODO file size (0 pour un dossier)
        let uncompressed_size = 0x0u32.to_le_bytes(); // @TODO file size (0 pour un dossier)
        let file_name_length = (item.name.plain()?.len() as u16).to_le_bytes();
        let extra_field_length = 1u16.to_le_bytes();
        let file_comment_length = 0u16.to_le_bytes();
        let disk_number = 0u16.to_le_bytes();
        let internal_file_attributes = 0u16.to_le_bytes();
        let external_file_attributes = 0u32.to_le_bytes();
        let relative_offset = 0u32.to_le_bytes(); // Should stay at 0 for ZIP64
        let file_name = item.name.plain()?.as_bytes();
        let extra_fields = &[0];

        let mut directory = vec![];
        Ok(directory)
    }
    
    fn end_of_directory(&self) -> Vec<u8> {
        let signature = 0x06054b50u32.to_le_bytes();
        let disk = 0u16.to_le_bytes();
        let central_directory_start_disk = 0u16.to_le_bytes(); // TODO ; compter début du premier central dir
        let central_directory_record_count_on_disk = 0u16.to_le_bytes(); // TODO nombre d'items
        let central_directory_record_count = 0u16.to_le_bytes(); // TODO nombre d'items
        let central_directory_size = 0u32.to_le_bytes();
        let central_directory_start = 0u32.to_le_bytes();
        let comment_length = 0u16.to_le_bytes();
        
        let mut directory = vec![];
        directory.put_slice(&signature);
        directory.put_slice(&disk);
        directory.put_slice(&central_directory_start_disk);
        directory.put_slice(&central_directory_record_count_on_disk);
        directory.put_slice(&central_directory_record_count);
        directory.put_slice(&central_directory_size);
        directory.put_slice(&central_directory_start);
        directory.put_slice(&comment_length);
        directory
    }
    
    pub fn size(&self) -> usize {
        0
    }
}

impl Stream for AsyncDirectoryZip {
    type Item = std::io::Result<Bytes>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {


        todo!()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!()
    }
}