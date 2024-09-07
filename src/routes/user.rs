use std::sync::Arc;
use anyhow::Error;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::get;
use uuid::Uuid;
use crate::app_ctx::AppCtx;
use crate::routes::root::RequestContext;


pub struct UserRoutes {
}

impl UserRoutes {
    pub fn create() -> Result<Router, Error> {
        
        let router = Router::new()
            .route("/:user_id", get(user_info));

        Ok(router)
    }
}
async fn user_info(Path(user_id): Path<Uuid>) {
    // ...
}
async fn middleware_get_display_user(Path(user_id): Path<Uuid>, mut request: Request<Body>)  {
    let ctx = request.extensions().get::<RequestContext>().unwrap();



}
async fn handler_user() -> impl IntoResponse {





    (StatusCode::FOUND, "User :) !")
}
