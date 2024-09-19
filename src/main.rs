pub mod routes;
pub mod config;
pub mod database;
mod app_ctx;
pub mod utils;
mod web_client;
mod compatibility_upgrade;

use std::{env, thread, time};
use std::future::IntoFuture;
use std::net::SocketAddr;
use std::sync::Arc;
use anyhow::Error;
use axum::{middleware, Router};
use axum::body::Body;
use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::extract::CookieJar;
use axum_server::tls_rustls::RustlsConfig;
use jobsys::{JobInstance, JobScope, JobSystem};
use tracing::{error, info};
use crate::app_ctx::AppCtx;
use crate::compatibility_upgrade::Upgrade;
use crate::config::Config;
use crate::database::user::User;
use crate::routes::{RequestContext, RootRoutes};
use crate::utils::enc_string::EncString;
use crate::utils::server_error::ServerError;
use crate::web_client::WebClient;

#[tokio::main]
async fn main() {
    // Open Config
    let config = match Config::from_file(env::current_exe().expect("Failed to find executable path").parent().unwrap().join("config.json")) {
        Ok(config) => { config }
        Err(error) => {
            error!("Failed to load config : {}", error);
            return;
        }
    };

    let ctx = Arc::new(match AppCtx::new(config.clone()).await {
        Ok(ctx) => { ctx }
        Err(error) => {
            error!("Failed to load app context : {error}");
            return;
        }
    });


    if env::args().len() > 0 {
        let mut upgrade = false;
        let mut upgrade_schema = None;
        for arg in env::args() {
            if upgrade {
                upgrade_schema = Some(arg);
                upgrade = false;
            } else if arg == "--upgrade" {
                upgrade = true;
            }
        }
        if let Some(upgrade_schema) = upgrade_schema {
            info!("Upgrading from old schema {upgrade_schema}");
            match Upgrade::run(&ctx.database, &upgrade_schema).await {
                Ok(_) => {
                    info!("Successfully upgraded database from {upgrade_schema}");
                    return;
                }
                Err(err) => {
                    error!("Failed to upgrade database : {err}");
                    return;
                }
            };
        }
    }


    match ctx.new_task(|| {
        
        match WebClient::new(&config.web_client_config).await {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to start web client : {err}");
            }
        };
    }) {
        Ok(_) => {}
        Err(err) => {
            error!("Cannot create web client, failed to instantiate the task : {err}");
            return;
        }
    }; 
    
    
    // Start web client

    // Instantiate router
    let mut router = Router::new();
    router = router.nest("/api/", RootRoutes::create(&ctx).unwrap());
    router = router.nest("/", crate::web_client::WebClient::router(&ctx).unwrap());
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

pub async fn middleware_get_request_context(jar: CookieJar, State(ctx): State<Arc<AppCtx>>, mut request: Request<Body>, next: Next) -> Result<Response, ServerError> {
    let mut context = RequestContext::default();

    let token = match jar.get("authtoken") {
        None => { request.headers().get("content-authtoken").map(EncString::try_from) }
        Some(token) => { Some(EncString::from_url_path(token.value().to_string())) }
    };

    if let Some(token) = token {
        context.connected_user = tokio::sync::RwLock::new(match User::from_auth_token(&ctx.database, &token?).await {
            Ok(connected_user) => { Some(connected_user) }
            Err(_) => { None }
        })
    }

    let uri = request.uri().clone();
    let user_string = if let Some(user) = &*context.connected_user().await {
        format!("#{}", user.name)
    } else { String::from("{?}") };
    info!("[{}] {} | {}", request.method(), user_string, uri);
    request.extensions_mut().insert(Arc::new(context));
    Ok(next.run(request).await)
}
