use crate::app_ctx::AppCtx;
use crate::database::item::Item;
use crate::database::repository::{Repository, RepositoryId, RepositoryStatus};
use crate::database::user::User;
use crate::require_connected_user;
use crate::routes::route_user::UserCredentials;
use crate::utils::enc_string::EncString;
use crate::utils::permissions::Permissions;
use crate::utils::server_error::ServerError;
use anyhow::Error;
use axum::body::Body;
use axum::extract::{FromRequest, Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use std::sync::Arc;

pub struct RepositoryRoutes {}

impl RepositoryRoutes {
    pub fn create(ctx: &Arc<AppCtx>) -> Result<Router, Error> {
        let router = Router::new()
            .route("/find/", post(find_repositories).with_state(ctx.clone()))
            .route("/owned/", get(get_owned_repositories).with_state(ctx.clone()))
            .route("/shared/", get(get_shared_repositories).with_state(ctx.clone()))
            .route("/public/", get(get_public_repositories).with_state(ctx.clone()))
            .route("/create/", post(create_repository).with_state(ctx.clone()))
            .route("/delete/", post(delete_repository).with_state(ctx.clone()))
            .route("/root-content/", post(root_content).with_state(ctx.clone()));
        Ok(router)
    }
}

async fn find_repositories(State(ctx): State<Arc<AppCtx>>, request: Request) -> Result<Json<Vec<Repository>>, ServerError> {
    let permission = Permissions::new(&request)?;
    let json = Json::<Vec<RepositoryId>>::from_request(request, &ctx).await?;
    let mut repositories = vec![];
    for repository in &json.0 {
        if permission.view_repository(&ctx.database, repository).await?.granted() {
            repositories.push(Repository::from_id(&ctx.database, repository).await?)
        }
    }
    Ok(Json(repositories))
}

async fn create_repository(State(ctx): State<Arc<AppCtx>>, request: Request) -> Result<impl IntoResponse, ServerError> {
    let user = require_connected_user!(request);
    
    if !user.can_create_repository() {
        return Err(ServerError::msg(StatusCode::FORBIDDEN, "Missing permissions"));
    }
    
    #[derive(Deserialize)]
    pub struct CreateReposData {
        name: EncString,
        status: String,
    }
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
    Ok(Json(Repository::from_user(&ctx.database, user.id()).await?))
}

async fn get_shared_repositories(State(ctx): State<Arc<AppCtx>>, request: Request) -> impl IntoResponse {
    let user = require_connected_user!(request);
    Ok(Json(Repository::shared_with(&ctx.database, user.id()).await?))
}

async fn get_public_repositories(State(ctx): State<Arc<AppCtx>>) -> Result<impl IntoResponse, ServerError> {
    Ok(Json(Repository::public(&ctx.database).await?))
}

async fn delete_repository(State(ctx): State<Arc<AppCtx>>, request: axum::http::Request<Body>) -> Result<impl IntoResponse, ServerError> {
    let permission = Permissions::new(&request)?;
    let connected_user = require_connected_user!(request);

    #[derive(Deserialize)]
    pub struct RequestParams {
        pub repositories: Vec<RepositoryId>,
        pub credentials: UserCredentials,
    }

    let data = Json::<RequestParams>::from_request(request, &ctx).await?;
    let from_creds = User::from_credentials(&ctx.database, &data.credentials.login, &data.credentials.password).await?;

    let mut deleted_ids = vec![];
    
    for repository in &data.repositories {
        if connected_user.id() != from_creds.id() || !permission.edit_repository(&ctx.database, repository).await?.granted() {
            continue;
        }
        Repository::from_id(&ctx.database, repository).await?.delete(&ctx.database).await?;
        deleted_ids.push(repository.clone());
    }
    Ok(Json(deleted_ids))
}

pub async fn root_content(State(ctx): State<Arc<AppCtx>>, request: axum::http::Request<Body>) -> Result<impl IntoResponse, ServerError> {
    let permission = Permissions::new(&request)?;

    let data = Json::<Vec<RepositoryId>>::from_request(request, &ctx).await?;
    
    let mut result = vec![];
    for repository in data.0 {
        permission.view_repository(&ctx.database, &repository).await?.require()?;
        result.append(&mut Item::repository_root(&ctx.database, &repository).await?);
    }
    Ok(Json(result))
}