use std::collections::HashSet;
use std::sync::Arc;
use anyhow::Error;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::extract::{FromRequest, Path, State};
use axum::routing::{get, post};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use crate::app_ctx::AppCtx;
use crate::database::repository::Repository;
use crate::database::user::{AuthToken, PasswordHash, User, UserRole};
use crate::{require_connected_user};
use crate::routes::user::{UserRoutes};
use crate::utils::enc_string::EncString;
use crate::utils::server_error::ServerError;

#[derive(Default, Debug)]
pub struct RequestContext {
    connected_user: tokio::sync::RwLock<Option<User>>,
    display_user: tokio::sync::RwLock<Option<User>>,
    display_repository: tokio::sync::RwLock<Option<Repository>>,
}

impl RequestContext {
    pub async fn connected_user(&self) -> tokio::sync::RwLockReadGuard<Option<User>> {
        self.connected_user.read().await
    }

    pub async fn connected_user_mut(&self) -> tokio::sync::RwLockWriteGuard<Option<User>> {
        self.connected_user.write().await
    }
    pub async fn display_user(&self) -> tokio::sync::RwLockReadGuard<Option<User>> {
        self.display_user.read().await
    }

    pub async fn display_user_mut(&self) -> tokio::sync::RwLockWriteGuard<Option<User>> {
        self.display_user.write().await
    }
    pub async fn display_repository(&self) -> tokio::sync::RwLockReadGuard<Option<Repository>> {
        self.display_repository.read().await
    }

    pub async fn display_repository_mut(&self) -> tokio::sync::RwLockWriteGuard<Option<Repository>> {
        self.display_repository.write().await
    }
}

pub struct RootRoutes {}

impl RootRoutes {
    pub fn create(ctx: &Arc<AppCtx>) -> Result<Router<>, Error> {
        let router = Router::new()
            .route("/auth/create-user/", post(create_user).with_state(ctx.clone()))
            .route("/auth/login/", post(login).with_state(ctx.clone()))
            .route("/auth/logout/", post(logout).with_state(ctx.clone()))
            .route("/auth/tokens/", get(auth_tokens).with_state(ctx.clone()))
            .route("/delete-user/", post(delete_user).with_state(ctx.clone()))
            .route("/repositories/", get(get_repositories).with_state(ctx.clone()))
            .route("/repositories/shared/", get(get_shared_repositories).with_state(ctx.clone()))
            .route("/repositories/public/", get(get_public_repositories).with_state(ctx.clone()))
            .nest("/:display_user/", UserRoutes::create(ctx)?)
            .fallback(handler_404)
            ; //.layer(middleware::from_fn_with_state(ctx.clone(), middleware_get_request_context));
        Ok(router)
    }
}

async fn get_repositories(State(ctx): State<Arc<AppCtx>>, request: axum::extract::Request) -> impl IntoResponse {
    let user = require_connected_user!(request);
    let repositories = Repository::from_user(&ctx.database, user.id()).await?;
    Ok(Json(repositories))
}

async fn get_shared_repositories(State(ctx): State<Arc<AppCtx>>, request: axum::extract::Request) -> impl IntoResponse {
    let user = require_connected_user!(request);
    let repositories = Repository::from_user(&ctx.database, user.id()).await?;
    Ok(Json(repositories))
}

async fn get_public_repositories(State(ctx): State<Arc<AppCtx>>, request: axum::extract::Request) -> Result<impl IntoResponse, ServerError> {
    let repositories = Repository::public(&ctx.database).await?;
    Ok(Json(repositories))
}

async fn handler_404(_: Request<Body>) -> impl IntoResponse {
    warn!("\t\t'-> 404 : NOT FOUND");
    (StatusCode::NOT_FOUND, "Not found !")
}

#[derive(Deserialize)]
struct CreateUserInfos {
    pub username: EncString,
    pub email: EncString,
    pub password: EncString,
}
async fn create_user(State(ctx): State<Arc<AppCtx>>, Json(payload): Json<CreateUserInfos>) -> Result<impl IntoResponse, ServerError> {
    let forbidden_usernames: HashSet<&str> = HashSet::from_iter(vec!["auth", "delete-user", "api", "public", "repositories"]);

    if forbidden_usernames.contains(payload.username.plain()?.as_str()) {
        Err(Error::msg("Forbidden username"))?
    }

    let url_name = payload.username.url_formated()?;

    if User::from_url_name(&ctx.database, &url_name).await.is_ok() {
        return Ok((StatusCode::CONFLICT, "User already exists : duplicated url identifier !".to_string()));
    } else if User::exists(&ctx.database, &payload.username, &payload.email).await? {
        return Ok((StatusCode::CONFLICT, "User already exists : duplicated logins !".to_string()));
    } else {
        let mut new_user = User::default();

        new_user.name = url_name;
        new_user.login = payload.username;
        new_user.email = payload.email;
        new_user.user_role = UserRole::Guest;
        match new_user.create_or_reset_password(&ctx.database, &PasswordHash::new(&payload.password)?).await {
            Ok(_) => {}
            Err(err) => {
                return Ok((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create user : {err}")))
            }
        };
    };

    Ok((StatusCode::FOUND, "Created new user".to_string()))
}

#[derive(Deserialize)]
pub struct UserCredentials {
    pub login: EncString,
    pub password: EncString,
}
#[derive(Deserialize)]
struct LoginInfos {
    pub login: EncString,
    pub password: EncString,
    pub device: Option<EncString>,
}
async fn login(State(ctx): State<Arc<AppCtx>>, Json(payload): Json<LoginInfos>) -> Result<impl IntoResponse, ServerError> {
    let user = User::from_credentials(&ctx.database, &payload.login, &payload.password).await?;
    let auth_token = user.generate_auth_token(&ctx.database, &match payload.device {
        None => { EncString::from("Unknown device") }
        Some(device) => { device }
    }).await?;

    #[derive(Serialize)]
    struct LoginResult {
        token: AuthToken,
        user: User,
    }

    Ok(Json(LoginResult{
        user,
        token: auth_token
    }))
}

#[axum::debug_handler]
async fn auth_tokens(State(_ctx): State<Arc<AppCtx>>, request: Request<Body>) -> Result<Json<Vec<AuthToken>>, ServerError> {
    let connected_user = require_connected_user!(request);
    Ok(Json(AuthToken::from_user(&_ctx.database, connected_user.id()).await?))
}

async fn logout(State(ctx): State<Arc<AppCtx>>, request: Request<Body>) -> Result<impl IntoResponse, ServerError> {
    match request.headers().get("authtoken") {
        None => { Err(Error::msg("No token provided in headers".to_string()))? }
        Some(authentication_token) => {
            let token = AuthToken::find(&ctx.database, &EncString::from(authentication_token)).await?;
            token.delete(&ctx.database).await?;
            Ok((StatusCode::ACCEPTED, "Successfully disconnected user".to_string()))
        }
    }
}

async fn delete_user(State(ctx): State<Arc<AppCtx>>, request: Request<Body>) -> Result<impl IntoResponse, ServerError> {
    let connected_user = require_connected_user!(request);
    let data = Json::<UserCredentials>::from_request(request, &ctx).await?;
    let mut from_creds = User::from_credentials(&ctx.database, &data.login, &data.password).await?;

    if connected_user.id() == from_creds.id() {
        from_creds.delete(&ctx.database).await?;
        Ok((StatusCode::OK, "Successfully deleted user"))
    } else {
        Err(ServerError::msg(StatusCode::NOT_FOUND, "Not found"))
    }
}

#[derive(Deserialize, Debug)]
pub struct PathData {
    display_user: Option<String>,
    display_repository: Option<String>,
}
pub async fn middleware_get_request_context(jar: CookieJar, State(ctx): State<Arc<AppCtx>>, Path(PathData { display_user, display_repository }): Path<PathData>, mut request: Request<Body>, next: Next) -> Result<Response, impl IntoResponse> {
    let mut context = RequestContext::default();

    let token = match jar.get("authtoken") {
        None => { request.headers().get("content-authtoken").map(EncString::from) }
        Some(token) => { Some(EncString::from_url_path(token.value().to_string())) }
    };

    if let Some(token) = token {
        context.connected_user = tokio::sync::RwLock::new(match User::from_auth_token(&ctx.database, &token).await {
            Ok(connected_user) => { Some(connected_user) }
            Err(_) => { None }
        })
    }

    if let Some(display_user) = display_user {
        if let Ok(display_user) = User::from_url_name(&ctx.database, &EncString::from_url_path(display_user.clone())).await {
            *context.display_user.write().await = Some(display_user);
        } else {
            return Err((StatusCode::NOT_FOUND, format!("Unknown user '{}'", display_user)));
        }
    }

    if let Some(display_repository) = display_repository {
        if let Ok(display_repository) = Repository::from_url_name(&ctx.database, &EncString::from_url_path(display_repository.clone())).await {
            *context.display_repository.write().await = Some(display_repository);
        } else {
            return Err((StatusCode::NOT_FOUND, format!("Unknown repository '{}'", display_repository)));
        }
    }

    let uri = request.uri().clone();
    let user_string = if let Some(user) = &*context.connected_user().await {
        format!("#{}", user.name)
    } else { String::from("{?}") };
    info!("[{}] {} | {}", request.method(), user_string, uri);
    request.extensions_mut().insert(Arc::new(context));
    let response = next.run(request).await;

    Ok(response)
}
