pub mod routes;
pub mod config;
pub mod database;
mod app_ctx;
pub mod utils;

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use crate::routes::root::{RootRoutes};
use axum_server::tls_rustls::RustlsConfig;
use tracing::{error, info};
use crate::app_ctx::AppCtx;
use crate::config::Config;
use crate::database::Database;
use crate::database::user::{PasswordHash, UserRole, User};
use crate::utils::enc_string::EncString;

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

    let database = match Database::new(&config.postgres_db_config).await {
        Ok(database) => { database }
        Err(error) => {
            error!("{}", error);
            return;
        }
    };


    let user = if !User::exists(&database, &EncString::from("Toto")).await.unwrap() {
        let mut new_user = User::default();
        new_user.name = EncString::from("Toto");
        new_user.email = EncString::from("Toto@gmail.com");
        new_user.user_role = UserRole::Admin;

        match new_user.create_or_reset_password(&database, &PasswordHash::new(&EncString::from("TESTUSER")).unwrap()).await {
            Ok(_) => {}
            Err(err) => { error!("err : {}", err); }
        };
        new_user
    }
    else {
        User::from_credentials(&database, &EncString::from("Toto"), &EncString::from("TESTUSER")).await.expect("User not found")
    };

    info!("{:?}", user);


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