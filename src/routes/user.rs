use std::sync::{Arc, RwLock};
use anyhow::Error;
use axum::body::Body;
use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{middleware, Router};
use tracing::log::info;
use crate::app_ctx::AppCtx;
use crate::database::user::User;
use crate::routes::root::RequestContext;
use crate::utils::enc_string::EncString;

pub struct UserRoutes {}

impl UserRoutes {
    pub fn create() -> Result<Router, Error> {
        let router = Router::new()
            .route("/", get(handler_user))
            .layer(middleware::from_fn(middleware_get_display_user));

        Ok(router)
    }
}
async fn handler_user(Path(display_user): Path<String>, request: Request) -> impl IntoResponse {

    let ctx = request.extensions().get::<Arc<RequestContext>>();

    ctx.
    
    info!("Found user : {} / {:?}", display_user, ctx);
    (StatusCode::FOUND, format!("Found user : {}", display_user))
}


async fn middleware_get_display_user(Path(user_id): Path<String>, request: Request, next: Next) -> Result<Response, StatusCode> {


    let ctx = request.extensions().get::<Arc<RequestContext>>();

    let response = next.run(request).await;

    Ok(response)
}