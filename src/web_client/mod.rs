mod static_file_server;

use std::{env, fs};
use std::path::PathBuf;
use std::process::{Stdio};
use std::sync::Arc;
use anyhow::Error;
use axum::body::Body;
use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Response};
use axum::{middleware, Router};
use axum::routing::{get};
use serde::{Deserialize, Serialize};
use tokio::process::{Child, Command};
use tracing::{info};
use which::which;
use crate::app_ctx::AppCtx;
use crate::config::WebClientConfig;
use crate::database::repository::Repository;
use crate::database::user::User;
use crate::{get_connected_user, get_display_repository, get_display_user};
use crate::routes::RequestContext;
use crate::utils::enc_string::EncString;
use crate::utils::server_error::ServerError;
use crate::web_client::static_file_server::StaticFileServer;

pub struct WebClient {
    _subcommand: Child,
}

impl WebClient {
    pub async fn new(config: &WebClientConfig) -> Result<Self, Error> {
        let base_directory = env::current_dir()?;
        let client = Self::try_create_client(config).await;
        env::set_current_dir(base_directory)?;
        client
    }

    async fn try_create_client(config: &WebClientConfig) -> Result<Self, Error> {
        env::set_current_dir(&config.client_path)?;

        let result = which("node").or(Err(Error::msg("Failed to find node path. Please ensure nodejs is correctly installed")))?;
        let npm_cli_path = result.parent().unwrap().join("node_modules").join("npm").join("bin").join("npm-cli.js");

        info!("Installing webclient dependencies...");
        let mut install_cmd = Command::new("node")
            .arg(npm_cli_path.to_str().unwrap())
            .arg("install")
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()?;
        install_cmd.wait().await?;
        info!("Installed webclient dependencies !");

        let command = if config.debug
        {
            Command::new("node")
                .arg(npm_cli_path.to_str().unwrap())
                .arg("run")
                .arg("dev")
                .stderr(Stdio::inherit())
                .stdout(Stdio::inherit())
                .spawn()?
        } else {
            Command::new("node")
                .arg(npm_cli_path.to_str().unwrap())
                .arg("run")
                .arg("prod")
                .stderr(Stdio::inherit())
                .stdout(Stdio::inherit())
                .spawn()?
        };

        Ok(Self { _subcommand: command })
    }

    pub fn router(ctx: &Arc<AppCtx>) -> Result<Router, Error> {
        Ok(Router::new()
            .route("/", get(get_index).with_state(ctx.clone()))
            .route("/:display_user/", get(get_index).with_state(ctx.clone()))
            .route("/:display_user/:display_repository/", get(get_index).with_state(ctx.clone()))
            .route("/:display_user/:display_repository/*path", get(get_index).with_state(ctx.clone()))
            .route("/favicon.ico", get(StaticFileServer::serve_file_from_path).with_state(PathBuf::from("web_client/public/images/icons/favicon.ico")))
            .nest("/public/", StaticFileServer::router(PathBuf::from("web_client/public")))
            .layer(middleware::from_fn_with_state(ctx.clone(), middleware_get_path_context))
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct PathData {
    display_user: Option<String>,
    display_repository: Option<String>,
}

pub async fn middleware_get_path_context(State(ctx): State<Arc<AppCtx>>, Path(PathData { display_user, display_repository }): Path<PathData>, request: axum::http::Request<Body>, next: Next) -> Result<Response, ServerError> {
    let context = request.extensions().get::<Arc<RequestContext>>().unwrap();
    if let Some(display_user) = display_user {
        if let Ok(display_user) = User::from_url_name(&ctx.database, &EncString::from_url_path(display_user.clone())?).await {
            *context.display_user.write().await = Some(display_user);
        } else {
            return Err(ServerError::msg(StatusCode::NOT_FOUND, format!("Unknown user '{}'", display_user)));
        }
    }

    if let Some(display_repository) = display_repository {
        if let Ok(display_repository) = Repository::from_url_name(&ctx.database, &EncString::from_url_path(display_repository.clone())?).await {
            *context.display_repository.write().await = Some(display_repository);
        } else {
            return Err(ServerError::msg(StatusCode::NOT_FOUND, format!("Unknown repository '{}'", display_repository)));
        }
    }

    Ok(next.run(request).await)
}

#[derive(Serialize, Default)]
struct ClientAppConfig {
    pub origin: String,
    pub connected_user: Option<User>,
    pub display_user: Option<User>,
    pub display_repository: Option<Repository>,
}

async fn get_index(State(ctx): State<Arc<AppCtx>>, request: Request) -> Result<impl IntoResponse, ServerError> {
    let mut client_config = ClientAppConfig {
        origin: format!("{}://{}", if ctx.config.use_tls { "https" } else { "http" }, request.headers().get("host").ok_or(ServerError::msg(StatusCode::INTERNAL_SERVER_ERROR, "invalid host in request headers"))?.to_str()?),
        ..Default::default()
    };

    get_connected_user!(request, user, {
        client_config.connected_user = Some(user.clone());
    });
    get_display_user!(request, user, {
        client_config.display_user = Some(user.clone());
    });
    get_display_repository!(request, repository, {
        client_config.display_repository = Some(repository.clone());
    });

    let index_data = fs::read_to_string("web_client/public/index.html")?;
    let index_data = index_data.replace(r#"data-app_config='{}'"#, format!(r##"data-app_config='{}'"##, serde_json::to_string(&client_config)?).as_str());
    Ok(Html(index_data))
}