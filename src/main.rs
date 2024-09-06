pub mod routes;
pub mod config;
pub mod database;
mod app_ctx;

use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::Error;
use crate::routes::root::RootRoutes;
use axum::http::StatusCode;
use axum::Json;
use axum_server::tls_rustls::RustlsConfig;
use serde::{Deserialize, Serialize};
use tracing::error;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::app_ctx::AppCtx;
use crate::config::Config;
use crate::database::Database;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // Open Config
    let config = match Config::from_file(env::current_exe().expect("Failed to find executable path").parent().unwrap().join("config.json")) {
        Ok(config) => { config }
        Err(error) => {
            error!("{}", error);
            return;
        }
    };

    let database = match Database::new(&config.postgres_db_config).await {
        Ok(database) => { database }
        Err(error) => {
            error!("{}", error);
            return;
        }
    };

    let ctx = Arc::new(AppCtx::new(
        config.clone(),
        database));

    // Instantiate router
    let router = RootRoutes::create(ctx).unwrap();

    // Create server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    if config.use_tls {
        if !config.tls_config.certificate.exists() || config.tls_config.private_key.exists() {
            error!("Invalid tls certificate paths : cert:{} / key:{}", config.tls_config.certificate.display(), config.tls_config.private_key.display());
            return;
        }

        tracing::info!("[secured] listening on {}", addr);
        let tls_config = RustlsConfig::from_pem_file(config.tls_config.certificate.clone(), config.tls_config.private_key.clone()).await.unwrap();
        axum_server::bind_rustls(addr, tls_config).serve(router.into_make_service()).await.unwrap();
    } else {
        tracing::info!("listening on {}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    }
    tracing::info!("Server closed !");
}