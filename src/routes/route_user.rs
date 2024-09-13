use crate::app_ctx::AppCtx;
use crate::database::user::{AuthToken, PasswordHash, User, UserId, UserRole};
use crate::utils::enc_string::EncString;
use crate::utils::server_error::ServerError;
use crate::require_connected_user;
use anyhow::Error;
use axum::body::Body;
use axum::extract::{FromRequest, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, Router};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

pub struct UserRoutes {}

impl UserRoutes {
    pub fn router(ctx: &Arc<AppCtx>) -> Result<Router, Error> {
        let router = Router::new()
            .route("/find/", post(find_users).with_state(ctx.clone()))
            .route("/login/", post(login).with_state(ctx.clone()))
            .route("/delete/", post(delete_user).with_state(ctx.clone()))
            .route("/logout/", post(logout).with_state(ctx.clone()))
            .route("/tokens/", post(auth_tokens).with_state(ctx.clone()))
            .route("/create/", post(create_user).with_state(ctx.clone()));

        Ok(router)
    }
}

async fn find_users(State(ctx): State<Arc<AppCtx>>, Json(json): Json<Vec<UserId>>) -> Result<impl IntoResponse, ServerError> {
    let mut repositories = vec![];
    for repository in &json {
        repositories.push(User::from_id(&ctx.database, repository).await?)
    }
    Ok(Json(repositories))
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

    Ok(Json(LoginResult {
        user,
        token: auth_token,
    }))
}

#[axum::debug_handler]
async fn auth_tokens(State(_ctx): State<Arc<AppCtx>>, request: axum::http::Request<Body>) -> Result<Json<Vec<AuthToken>>, ServerError> {
    let connected_user = require_connected_user!(request);
    Ok(Json(AuthToken::from_user(&_ctx.database, connected_user.id()).await?))
}

async fn logout(jar: CookieJar, State(ctx): State<Arc<AppCtx>>, request: axum::http::Request<Body>) -> Result<impl IntoResponse, ServerError> {
    let token = match jar.get("authtoken") {
        None => { request.headers().get("content-authtoken").map(EncString::try_from) }
        Some(token) => { Some(EncString::from_url_path(token.value().to_string())) }
    };

    match token {
        None => { Err(Error::msg("No token provided".to_string()))? }
        Some(authentication_token) => {
            let token = AuthToken::find(&ctx.database, &authentication_token?).await?;
            token.delete(&ctx.database).await?;
            Ok((StatusCode::ACCEPTED, "Successfully disconnected user".to_string()))
        }
    }
}

async fn delete_user(State(ctx): State<Arc<AppCtx>>, request: axum::http::Request<Body>) -> Result<impl IntoResponse, ServerError> {
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
