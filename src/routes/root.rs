use std::sync::Arc;
use anyhow::Error;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{middleware, Router};
use tracing::{warn};
use crate::app_ctx::AppCtx;
use crate::database::user::User;
use crate::routes::user::UserRoutes;

pub struct RootRoutes {}


impl RootRoutes {
    pub fn create(ctx: Arc<AppCtx>) -> Result<Router, Error> {
        let router = axum::Router::new()
            .nest("/:user", UserRoutes::create(ctx)?)
            .layer(middleware::from_fn(middleware_get_connected_user))
            .fallback(handler_404);

        Ok(router)
    }
}

async fn handler_404(request: Request<Body>) -> impl IntoResponse {

    let uri = request.uri().clone();
    warn!("[NOT FOUND] -> {}", uri);

    (StatusCode::NOT_FOUND, "Not found !")
}

#[derive(Default)]
struct RequestContext {
    connected_user: Option<User>,
    display_user: Option<User>,
    display_repository: Option<User>,
}

async fn middleware_get_connected_user(mut request: Request<Body>, next: Next) -> Result<Response, StatusCode> {

    let mut context = Arc::new(RequestContext::default());
    match request.headers().get("authtoken") {
        None => {

        }
        Some(authentication_token) => {
            //context.connected_user
        }
    }


    request.extensions_mut().insert(context);
    let uri = request.uri().clone();
    warn!("{}", uri);
    let mut response = next.run(request).await;

    Ok(response)
}