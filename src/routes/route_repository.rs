use crate::app_ctx::AppCtx;
use crate::database::item::{Item, ItemId};
use crate::database::repository::{Repository, RepositoryId, RepositoryStatus};
use crate::database::user::User;
use crate::routes::route_user::UserCredentials;
use crate::utils::enc_string::EncString;
use crate::utils::server_error::ServerError;
use crate::{require_connected_user, require_display_repository};
use anyhow::Error;
use axum::body::Body;
use axum::extract::{FromRequest, Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct RepositoryRoutes {}

impl RepositoryRoutes {
    pub fn create(ctx: &Arc<AppCtx>) -> Result<Router, Error> {
        let router = Router::new()
            .route("/find/", post(find_repositories).with_state(ctx.clone()))
            .route("/owned/", post(get_owned_repositories).with_state(ctx.clone()))
            .route("/shared/", post(get_shared_repositories).with_state(ctx.clone()))
            .route("/public/", post(get_public_repositories).with_state(ctx.clone()))
            .route("/create/", post(create_repository).with_state(ctx.clone()))
            .route("/delete/", post(delete_repository).with_state(ctx.clone()))
            .route("/root-content/", post(root_content).with_state(ctx.clone()));
        Ok(router)
    }
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
async fn get_owned_repositories(State(ctx): State<Arc<AppCtx>>, request: Request) -> impl IntoResponse {
    let user = require_connected_user!(request);
    #[derive(Serialize)]
    pub struct Response {
        user: User,
        repository: Repository
    }
    let mut repositories = vec![];
    for repository in Repository::from_user(&ctx.database, user.id()).await? {
        repositories.push(Response {
            user: User::from_id(&ctx.database, &repository.owner).await?,
            repository,
        })
    }
    Ok(Json(repositories))
}

async fn get_shared_repositories(State(ctx): State<Arc<AppCtx>>, request: Request) -> impl IntoResponse {
    let user = require_connected_user!(request);
    #[derive(Serialize)]
    pub struct Response {
        user: User,
        repository: Repository
    }
    let mut repositories = vec![];
    for repository in Repository::from_user(&ctx.database, user.id()).await? {
        repositories.push(Response {
            user: User::from_id(&ctx.database, &repository.owner).await?,
            repository,
        })
    }
    Ok(Json(repositories))
}

async fn get_public_repositories(State(ctx): State<Arc<AppCtx>>) -> Result<impl IntoResponse, ServerError> {
    #[derive(Serialize)]
    pub struct Response {
        user: User,
        repository: Repository
    }
    let mut repositories = vec![];
    for repository in Repository::public(&ctx.database).await? {
        repositories.push(Response {
            user: User::from_id(&ctx.database, &repository.owner).await?,
            repository,
        })
    }
    Ok(Json(repositories))
}

async fn find_repositories(State(ctx): State<Arc<AppCtx>>, Json(json): Json<Vec<RepositoryId>>) -> Result<impl IntoResponse, ServerError> {
    let mut repositories = vec![];
    for repository in &json {
        repositories.push(Repository::from_id(&ctx.database, repository).await?)
    }
    Ok(Json(repositories))
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

pub async fn root_content(State(ctx): State<Arc<AppCtx>>, request: axum::http::Request<Body>) -> Result<impl IntoResponse, ServerError> {
    Ok(Json(Item::repository_root(&ctx.database, require_display_repository!(request).id()).await?))
}