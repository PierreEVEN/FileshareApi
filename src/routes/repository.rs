use crate::app_ctx::AppCtx;
use crate::routes::root::RequestContext;
use anyhow::Error;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub struct RepositoryRoutes {}

impl RepositoryRoutes {
    pub fn create(_: &Arc<AppCtx>) -> Result<Router, Error> {
        let router = Router::new()
            .route("/", get(handle_repos));

        Ok(router)
    }
}

async fn handle_repos(request: Request) -> impl IntoResponse {

    let ctx = request.extensions().get::<Arc<RequestContext>>().unwrap();

    (StatusCode::FOUND, format!("Display repository : {}", ctx.display_repository().await.as_ref().unwrap().display_name))
}