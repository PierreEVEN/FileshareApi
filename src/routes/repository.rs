use crate::app_ctx::AppCtx;
use crate::routes::root::{RequestContext, UserCredentials};
use anyhow::Error;
use axum::extract::{FromRequest, Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::sync::Arc;
use axum::body::Body;
use crate::database::user::User;
use crate::{require_connected_user, require_display_repository};
use crate::utils::server_error::ServerError;

pub struct RepositoryRoutes {}

impl RepositoryRoutes {
    pub fn create(ctx: &Arc<AppCtx>) -> Result<Router, Error> {
        let router = Router::new()
            .route("/delete/", post(delete_repository).with_state(ctx.clone()));
        Ok(router)
    }
}

async fn delete_repository(State(ctx): State<Arc<AppCtx>>, request: axum::http::Request<Body>) -> Result<impl IntoResponse, ServerError> {
    let connected_user = require_connected_user!(request);
    let mut display_repository = require_display_repository!(request);
    let data = Json::<UserCredentials>::from_request(request, &ctx).await?;
    let from_creds = User::from_credentials(&ctx.database, &data.login, &data.password).await?;
    if connected_user.id() == from_creds.id() {
        display_repository.delete(&ctx.database).await?;
        Ok((StatusCode::OK, "Successfully deleted repository"))
    } else {
        Err(ServerError::msg(StatusCode::NOT_FOUND, "Not found"))
    }
}