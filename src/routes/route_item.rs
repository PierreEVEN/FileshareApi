use crate::app_ctx::AppCtx;
use crate::database::item::{DirectoryData, Item, ItemId, Trash};
use crate::utils::server_error::ServerError;
use anyhow::Error;
use axum::extract::{FromRequest, Request, State};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use std::sync::Arc;
use axum::http::StatusCode;
use serde::Deserialize;
use regex::Regex;
use crate::database::repository::RepositoryId;
use crate::require_connected_user;
use crate::utils::enc_string::EncString;
use crate::utils::permissions::Permissions;

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