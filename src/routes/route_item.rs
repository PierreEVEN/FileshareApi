use crate::app_ctx::AppCtx;
use crate::database::item::{DirectoryData, Item, ItemId, Trash};
use crate::database::repository::RepositoryId;
use crate::database::DatabaseId;
use crate::require_connected_user;
use crate::utils::enc_string::EncString;
use crate::utils::permissions::Permissions;
use crate::utils::server_error::ServerError;
use crate::utils::thumbnails::Thumbnail;
use crate::utils::upload::Upload;
use anyhow::Error;
use axum::body::Body;
use axum::extract::{FromRequest, Path, Request, State};
use axum::http::{header, HeaderName, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Seek, SeekFrom, Write};
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use std::task::{Context, Poll};
use async_zip::base::write::ZipFileWriter;
use futures::StreamExt;
use tokio::io::{duplex, AsyncWrite, AsyncWriteExt};
use tokio_stream::Stream;
use tokio_util::io::ReaderStream;
use zip::CompressionMethod;
use zip::write::FileOptions;
use zipit::archive_size;
use crate::database::object::Object;
use crate::utils::async_zip::AsyncDirectoryZip;

pub struct ItemRoutes {}

impl ItemRoutes {
    pub fn create(ctx: &Arc<AppCtx>) -> Result<Router, Error> {
        Ok(Router::new()
            .route("/find/", post(find_items).with_state(ctx.clone()))
            .route("/move-to-trash/", post(move_to_trash).with_state(ctx.clone()))
            .route("/delete/", post(delete).with_state(ctx.clone()))
            .route("/restore/", post(restore).with_state(ctx.clone()))
            .route("/new-directory/", post(new_directory).with_state(ctx.clone()))
            .route("/directory-content/", post(directory_content).with_state(ctx.clone()))
            .route("/thumbnail/:id/", get(thumbnail).with_state(ctx.clone()))
            .route("/send/", post(send).with_state(ctx.clone()))
            .route("/get/:path/", get(download).with_state(ctx.clone()))
            .route("/preview/:path/", get(download).with_state(ctx.clone()))
        )
    }
}

async fn find_items(State(ctx): State<Arc<AppCtx>>, request: Request) -> Result<impl IntoResponse, ServerError> {
    let permissions = Permissions::new(&request)?;
    let json = Json::<Vec<ItemId>>::from_request(request, &ctx).await?;
    let mut items = vec![];
    for item_id in json.0 {
        if permissions.view_item(&ctx.database, &item_id).await?.granted() {
            items.push(Item::from_id(&ctx.database, &item_id, Trash::Both).await?)
        }
    }
    Ok(Json(items))
}

async fn directory_content(State(ctx): State<Arc<AppCtx>>, request: Request) -> Result<impl IntoResponse, ServerError> {
    let permissions = Permissions::new(&request)?;
    let json = Json::<Vec<ItemId>>::from_request(request, &ctx).await?;
    let mut items = vec![];
    for directory in json.0 {
        if permissions.view_item(&ctx.database, &directory).await?.granted() {
            items.append(&mut Item::from_parent(&ctx.database, &directory, Trash::Both).await?);
        }
    }
    Ok(Json(items))
}

async fn new_directory(State(ctx): State<Arc<AppCtx>>, request: Request) -> Result<impl IntoResponse, ServerError> {
    let permissions = Permissions::new(&request)?;
    let user = require_connected_user!(request);

    #[derive(Deserialize)]
    struct Params {
        name: EncString,
        repository: RepositoryId,
        parent_item: Option<ItemId>,
    }

    let json = Json::<Vec<Params>>::from_request(request, &ctx).await?;
    let mut items = vec![];
    for params in json.0 {
        let mut item = Item::default();

        if let Some(parent_item) = &params.parent_item {
            if !permissions.upload_to_directory(&ctx.database, parent_item).await?.granted() { continue; }
        } else if !permissions.upload_to_repository(&ctx.database, &params.repository).await?.granted() { continue; }

        let re = Regex::new(r#"[<>:"/\\|?*\x00-\x1F]|^(?:aux|con|clock\$|nul|prn|com[1-9]|lpt[1-9])$"#)?;
        if re.is_match(item.name.plain()?.as_str()) {
            return Err(ServerError::msg(StatusCode::NOT_ACCEPTABLE, format!("Invalid directory name '${}'", item.name.plain()?)));
        }

        item.name = params.name;
        item.repository = if let Some(parent) = &params.parent_item { Item::from_id(&ctx.database, parent, Trash::Both).await?.repository } else { params.repository };
        item.parent_item = params.parent_item;
        item.owner = user.id().clone();
        item.directory = Some(DirectoryData {
            open_upload: false,
        });

        item.push(&ctx.database).await?;

        items.push(item);
    }
    Ok(Json(items))
}


async fn move_to_trash(State(ctx): State<Arc<AppCtx>>, request: Request) -> Result<impl IntoResponse, ServerError> {
    let permissions = Permissions::new(&request)?;
    let json = Json::<Vec<ItemId>>::from_request(request, &ctx).await?;
    let mut items = vec![];
    for item in json.0 {
        if permissions.edit_item(&ctx.database, &item).await?.granted() {
            if let Ok(mut item) = Item::from_id(&ctx.database, &item, Trash::No).await
            {
                item.in_trash = true;
                item.push(&ctx.database).await?;
                items.push(item.id().clone());
            }
        }
    }
    Ok(Json(items))
}

async fn restore(State(ctx): State<Arc<AppCtx>>, request: Request) -> Result<impl IntoResponse, ServerError> {
    let permissions = Permissions::new(&request)?;
    let json = Json::<Vec<ItemId>>::from_request(request, &ctx).await?;
    let mut items = vec![];
    for item in json.0 {
        if permissions.edit_item(&ctx.database, &item).await?.granted() {
            if let Ok(mut item) = Item::from_id(&ctx.database, &item, Trash::Yes).await {
                item.in_trash = false;
                item.push(&ctx.database).await?;
                items.push(item.id().clone());
            }
        }
    }
    Ok(Json(items))
}

async fn delete(State(ctx): State<Arc<AppCtx>>, request: Request) -> Result<impl IntoResponse, ServerError> {
    let permissions = Permissions::new(&request)?;
    let json = Json::<Vec<ItemId>>::from_request(request, &ctx).await?;
    let mut items = vec![];
    for item_id in json.0 {
        if permissions.edit_item(&ctx.database, &item_id).await?.granted() {
            Item::from_id(&ctx.database, &item_id, Trash::Both).await?.delete(&ctx.database).await?;
            items.push(item_id);
        }
    }
    Ok(Json(items))
}

async fn thumbnail(State(ctx): State<Arc<AppCtx>>, Path(id): Path<DatabaseId>) -> Result<impl IntoResponse, ServerError> {
    let item = Item::from_id(&ctx.database, &ItemId::from(id), Trash::Both).await?;

    let file = match &item.file {
        None => { return Err(ServerError::msg(StatusCode::NOT_ACCEPTABLE, "Cannot generate thumbnail for a directory")) }
        Some(f) => { f }
    };

    let thumbnail_path = Thumbnail::find_or_create(&ctx.database, &Object::from_id(&ctx.database, &file.object).await?, &file.mimetype, 100)?;

    let stream = ReaderStream::new(tokio::fs::File::open(thumbnail_path).await?);
    let body = Body::from_stream(stream);

    let headers = [
        (header::CONTENT_TYPE, "image/webp".to_string()),
        (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", item.name.encoded()))
    ];
    Ok((headers, body))
}

async fn send(State(ctx): State<Arc<AppCtx>>, request: Request) -> Result<impl IntoResponse, ServerError> {
    let connected_user = require_connected_user!(request);
    let headers = request.headers().clone();
    let id = if let Some(content_id) = headers.get("Content-Id") {
        usize::from_str(content_id.to_str()?)?
    } else {
        let upload = Upload::new(headers, connected_user.id().clone())?;
        ctx.add_upload(upload).await?
    };


    let mut state = {
        let found_upload = ctx.get_upload(id).await?;
        let mut upload = found_upload.write().await;
        upload.push_data(request.into_body()).await?;
        upload.get_state()
    };
    if state.finished {
        state = ctx.finalize_upload(id, &ctx.database).await?;
    }
    Ok(Json(state))
}

async fn download(State(ctx): State<Arc<AppCtx>>, Path(id): Path<DatabaseId>, request: Request) -> Result<impl IntoResponse, ServerError> {
    let item = Item::from_id(&ctx.database, &ItemId::from(id), Trash::Both).await?;

    if let Some(file) = item.file {
        let object = Object::from_id(&ctx.database, &file.object).await?;

        let stream = ReaderStream::new(tokio::fs::File::open(object.data_path(&ctx.database)).await?);
        let body = Body::from_stream(stream);

        let headers = [
            (header::CONTENT_TYPE, file.mimetype.plain()?),
            (header::CONTENT_LENGTH, file.size.to_string()),
            (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", item.name.encoded()))
        ];
        Ok((headers, body))
    } else {

        let mut item_map = HashMap::new();
        let mut items_to_discover = vec![item.id().clone()];

        while let Some(item) = &items_to_discover.pop() {
            let data = Item::from_id(&ctx.database, item, Trash::No).await?;
            item_map.insert(data.id().clone(), data);
            let children = Item::from_parent(&ctx.database, item, Trash::No).await?;
            for child in children {
                items_to_discover.push(child.id().clone());
            }
        }

        
        let mut file_sizes = vec![];
        for item in item_map.values() {
            if let Some(file) = &item.file {
                file_sizes.push((item.absolute_path.plain()?.as_str(), file.size as usize))
            }
            else {
                file_sizes.push((item.absolute_path.plain()?.as_str(), 0))
            }
        }
        let mut zip = AsyncDirectoryZip::default();
        let size = zip.size();
        let body = Body::from_stream(zip);
        let headers = [
            (header::CONTENT_TYPE, String::from("application/zip")),
            (header::CONTENT_LENGTH, size.to_string()),
            (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", item.name.encoded()))
        ];
        Ok((headers, body))
        
            /*
        struct ZipWriter(tokio::io::DuplexStream);
        
        impl Write for ZipWriter {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                println!("write haha : {}", buf.len());
                Ok(buf.len())
                //todo!()
            }

            fn flush(&mut self) -> std::io::Result<()> {
                println!("FLUSH");
                Ok(())
                //todo!()
            }
        }
        
        impl Seek for ZipWriter {
            fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
                println!("SEEK TO  {:?}", pos);
                Ok(0)
                //todo!()
            }
        }
        


        let (w, r) = duplex(4096);
        tokio::spawn(async move {
            println!("start zip");
            let writer = ZipWriter(w);
            let mut zip = zip::ZipWriter::new(writer);
            let options = zip::write::SimpleFileOptions::default()
                .compression_method(CompressionMethod::Stored)
                .unix_permissions(0o755);

            for item in item_map.values() {
                if let Some(file) = &item.file {
                    let object = Object::from_id(&ctx.database, &file.object).await?;
                    zip.start_file(object.data_path(&ctx.database).to_str().unwrap(), options)?;
                } else {
                    zip.add_directory_from_path(item.absolute_path.plain()?, options)?;
                }
            }
            println!("Call finish");
            zip.finish()?;
            println!("Finished");
            Ok::<(), Error>(())
        });
*/

        /*
        let mut file_sizes = vec![];
        let mut streams = vec![];
        for item in item_map.values() {
            if let Some(file) = &item.file {
                file_sizes.push((item.absolute_path.plain()?.as_str(), file.size as usize))
            }
            else {
                file_sizes.push((item.absolute_path.plain()?.as_str(), 0))
            }
        }

        let archive_size = archive_size(file_sizes);

        let (w, r) = duplex(4096);
        tokio::spawn(async move {
            let mut archive = zipit::Archive::new(w);

            for item in item_map.values() {
                if let Some(file) = &item.file {
                    let object = Object::from_id(&ctx.database, &file.object).await.unwrap();
                    let file = tokio::fs::File::open(object.data_path(&ctx.database)).await.unwrap();

                    archive.append(
                        item.absolute_path.plain().unwrap(),
                        zipit::FileDateTime::now(),
                        &mut file,
                    )
                        .await?;
                }
                else {
                    archive.append(
                        item.absolute_path.plain().unwrap(),
                        zipit::FileDateTime::now(),
                    ).await.unwrap();
                }
            }
            archive.finalize().await.unwrap();
        });
*/
    }
}