use std::sync::Arc;
use anyhow::Error;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use crate::app_ctx::AppCtx;

pub struct UserRoutes {
}

impl UserRoutes {
    pub fn create(_: Arc<AppCtx>) -> Result<Router, Error> {
        let router = Router::new()
            .route("/", get(handler_user));

        Ok(router)
    }



}
async fn handler_user() -> impl IntoResponse {
    (StatusCode::FOUND, "User :) !")
}
