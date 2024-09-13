use crate::app_ctx::AppCtx;
use crate::database::item::{Item, ItemId};
use crate::utils::server_error::ServerError;
use anyhow::Error;
use axum::extract::{FromRequest, Request, State};
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use std::sync::Arc;
use crate::utils::permissions::Permissions;

pub struct ItemRoutes {}

impl ItemRoutes {
    pub fn create(ctx: &Arc<AppCtx>) -> Result<Router, Error> {
        Ok(Router::new()
            .route("/find/", post(find_items).with_state(ctx.clone()))
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
            items.push(Item::from_id(&ctx.database, &item_id).await?)
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
            items.push(Item::from_parent(&ctx.database, &directory).await?);
        }
    }
    Ok(Json(items))
}
