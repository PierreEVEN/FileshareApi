use crate::app_ctx::AppCtx;
use crate::database::repository::{Repository, RepositoryStatus};
use crate::routes::repository::RepositoryRoutes;
use crate::routes::root::RequestContext;
use crate::utils::enc_string::EncString;
use crate::utils::server_error::ServerError;
use crate::{get_connected_user, require_connected_user};
use anyhow::Error;
use axum::extract::{FromRequest, Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use std::sync::Arc;

pub struct UserRoutes {}

impl UserRoutes {
    pub fn create(ctx: &Arc<AppCtx>) -> Result<Router, Error> {
        let router = Router::new()
            .route("/repositories/", get(get_repositories).with_state(ctx.clone()))
            .route("/repositories/shared/", get(get_shared_repositories).with_state(ctx.clone()))
            .route("/create-repository/", post(create_repository).with_state(ctx.clone()))
            .nest("/:display_repository/", RepositoryRoutes::create(ctx)?);

        Ok(router)
    }
}

async fn get_repositories(State(ctx): State<Arc<AppCtx>>, request: Request) -> impl IntoResponse {
    let user = require_connected_user!(request);
    let repositories = Repository::from_user(&ctx.database, user.id()).await?;
    Ok(Json(repositories))
}

async fn get_shared_repositories(State(ctx): State<Arc<AppCtx>>, request: Request) -> impl IntoResponse {
    let user = require_connected_user!(request);
    let repositories = Repository::from_user(&ctx.database, user.id()).await?;
    Ok(Json(repositories))
}

#[derive(Deserialize, Debug)]
pub struct CreateReposData {
    name: EncString,
    status: String
}
async fn create_repository(State(ctx): State<Arc<AppCtx>>, request: Request) -> Result<impl IntoResponse, ServerError> {
    let user = require_connected_user!(request);
    let data = Json::<CreateReposData>::from_request(request, &ctx).await?;

    if Repository::from_url_name(&ctx.database, &data.name.url_formated()?).await.is_ok() {
        return Err(ServerError::msg(StatusCode::FORBIDDEN, "A repository with this name already exists"));
    }
    
    let mut repository = Repository::default();
    repository.url_name = data.name.url_formated()?;
    repository.display_name = data.name.clone();
    repository.status = RepositoryStatus::from(data.status.clone());
    repository.owner = user.id().clone();
    repository.push(&ctx.database).await?;

    Ok(Json(repository.url_name))
}