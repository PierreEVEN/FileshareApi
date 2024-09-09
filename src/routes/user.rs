use crate::app_ctx::AppCtx;
use crate::routes::repository::RepositoryRoutes;
use crate::routes::root::RequestContext;
use anyhow::Error;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub struct UserRoutes {}

impl UserRoutes {
    pub fn create(ctx: &Arc<AppCtx>) -> Result<Router, Error> {
        let router = Router::new()
            .route("/", get(handle_user))
            .nest("/:display_repository/", RepositoryRoutes::create(ctx)?);

        Ok(router)
    }
}

async fn handle_user(request: Request) -> impl IntoResponse {

    let ctx = request.extensions().get::<Arc<RequestContext>>().unwrap();

    (StatusCode::FOUND, format!("Display user : {}", ctx.display_user().await.as_ref().unwrap().name))
}