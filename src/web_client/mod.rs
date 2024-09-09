mod static_file_server;

use std::{env, fs};
use std::path::PathBuf;
use std::process::{Stdio};
use std::sync::Arc;
use anyhow::Error;
use axum::body::Body;
use axum::extract::Request;
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse, Redirect};
use axum::Router;
use axum::routing::{get, post};
use tokio::fs::File;
use tokio::process::{Child, Command};
use tracing::{info};
use which::which;
use crate::app_ctx::AppCtx;
use crate::config::WebClientConfig;
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

    pub fn router(&self) -> Result<Router, Error> {
        Ok(Router::new()
            .route("/", get(get_index))
            .route("/:display_user/", get(get_index))
            .route("/:display_user/:display_repository/", get(get_index))
            .route("/:display_user/:display_repository/*path", get(get_index))
            .route("/favicon.ico", get(StaticFileServer::serve_file_from_path).with_state(PathBuf::from("web_client/public/images/icons/favicon.ico")))
            .nest("/public/", StaticFileServer::router(PathBuf::from("web_client/public")))
        )
    }
}

async fn get_index() -> Result<impl IntoResponse, ServerError> {
    Ok(Html(fs::read_to_string("web_client/public/index.html")?))
}