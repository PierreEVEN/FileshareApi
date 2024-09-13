use crate::app_ctx::AppCtx;
use crate::database::item::{Item, ItemId};
use crate::utils::server_error::ServerError;
use anyhow::Error;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use std::sync::Arc;

pub struct ItemRoutes {}

impl ItemRoutes {
    pub fn create(ctx: &Arc<AppCtx>) -> Result<Router, Error> {
        Ok(Router::new()
            .route("/find/", post(find_items).with_state(ctx.clone()))
            .route("/directory-content/", post(directory_content).with_state(ctx.clone()))
        )
    }
}

async fn find_items(State(ctx): State<Arc<AppCtx>>, Json(json): Json<Vec<ItemId>>) -> Result<impl IntoResponse, ServerError> {
    let mut repositories = vec![];
    for item_id in &json {
        repositories.push(Item::from_id(&ctx.database, item_id).await?)
    }
    Ok(Json(repositories))
}

async fn directory_content(State(ctx): State<Arc<AppCtx>>, Json(json): Json<Vec<ItemId>>) -> Result<impl IntoResponse, ServerError> {
    let mut items = vec![];
    for directory in &json {
        items.push(Item::from_parent(&ctx.database, &directory).await?);
    }
    Ok(Json(items))
}
