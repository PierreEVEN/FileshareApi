pub mod routes;
pub mod config;
pub mod database;
mod app_ctx;
pub mod utils;
mod web_client;

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::{middleware, Router};
use crate::routes::root::{middleware_get_request_context, RootRoutes};
use axum_server::tls_rustls::RustlsConfig;
use tracing::{error, info};
use crate::app_ctx::AppCtx;
use crate::config::Config;
use crate::database::Database;
use crate::web_client::WebClient;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    // Open Config
    let config = match Config::from_file(env::current_exe().expect("Failed to find executable path").parent().unwrap().join("config.json")) {
        Ok(config) => { config }
        Err(error) => {
            error!("Failed to load config : {}", error);
            return;
        }
    };

    // Instantiate router
    let mut router = Router::new();

    // Start server api
    let database = match Database::new(&config.postgres_db_config).await {
        Ok(database) => { database }
        Err(error) => {
            error!("{}", error);
            return;
        }
    };
    let ctx = Arc::new(AppCtx::new(config.clone(), database));
    router = router.nest("/api/", RootRoutes::create(&ctx).unwrap());

    // Start web client
    let web_client = match WebClient::new(&config.web_client_config).await {
        Ok(web_client) => { Some(web_client) }
        Err(error) => {
            error!("Failed to start web client : {}", error);
            None
        }
    };
    if let Some(web_client) = web_client {
        router = router.nest("/", web_client.router(&ctx).unwrap());
    }
    
    let router = router.layer(middleware::from_fn_with_state(ctx.clone(), middleware_get_request_context));
    
    // Create http server
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    if config.use_tls {
        if !config.tls_config.certificate.exists() || config.tls_config.private_key.exists() {
            error!("Invalid tls certificate paths : cert:{} / key:{}", config.tls_config.certificate.display(), config.tls_config.private_key.display());
            return;
        }

        info!("[secured] listening on {}", addr);
        let tls_config = RustlsConfig::from_pem_file(config.tls_config.certificate.clone(), config.tls_config.private_key.clone()).await.unwrap();
        axum_server::bind_rustls(addr, tls_config).serve(router.into_make_service()).await.unwrap();
    } else {
        info!("listening on {}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    }
    info!("Server closed !");
}