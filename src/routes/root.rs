use anyhow::Error;
use axum::body::Body;
use axum::http::{HeaderValue, Request, StatusCode, Uri};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{middleware, Router};
use tracing::{info, warn};
use crate::routes::user::UserRoutes;

pub struct RootRoutes {}


impl RootRoutes {
    pub fn create() -> Result<Router, Error> {
        let router = axum::Router::new()
            .nest("/:user", UserRoutes::create()?)
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


#[derive(Default, Clone)]
pub struct Repository {

}

#[derive(Default, Clone)]
pub struct User {

}

#[derive(Default, Clone)]
struct RequestContext {
    connected_user: Option<User>,
    display_user: Option<User>,
    display_repository: Option<Repository>,
}

async fn middleware_get_connected_user(mut request: Request<Body>, next: Next) -> Result<Response, StatusCode> {

    let mut context = RequestContext::default();
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