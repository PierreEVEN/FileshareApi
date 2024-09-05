pub mod routes;
pub mod config;

use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use crate::routes::root::RootRoutes;
use axum::http::StatusCode;
use axum::Json;
use axum_server::tls_rustls::RustlsConfig;
use serde::{Deserialize, Serialize};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::config::Config;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry().init();

    // Open Config
    let config = Config::from_file(env::current_exe().expect("Failed to find executable path").join("config.json")).unwrap();

    // Instantiate router
    let router = RootRoutes::create().unwrap();

    // Create server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    if config.use_tls {
        tracing::debug!("[secured] listening on {}", addr);
        let tls_config = RustlsConfig::from_pem_file(config.tls_config.certificate.clone(), config.tls_config.private_key.clone()).await.unwrap();
        axum_server::bind_rustls(addr, tls_config).serve(router.into_make_service()).await.unwrap();
    } else {
        tracing::debug!("listening on {}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    }
}