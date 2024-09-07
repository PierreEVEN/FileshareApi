use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use anyhow::Error;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::{middleware, Router};
use axum::extract::State;
use tracing::{info, warn};
use crate::app_ctx::AppCtx;
use crate::database::user::User;
use crate::routes::user::UserRoutes;
use crate::utils::enc_string::EncString;

#[derive(Default)]
pub struct RequestContext {
    connected_user: RwLock<Option<User>>,
    display_user: RwLock<Option<User>>,
    display_repository: RwLock<Option<User>>,
}

impl RequestContext {
    pub fn connected_user(&self) -> RwLockReadGuard<Option<User>> {
        self.connected_user.read().unwrap()
    }

    pub fn connected_user_mut(&self) -> RwLockWriteGuard<Option<User>> {
        self.connected_user.write().unwrap()
    }
}


pub struct RootRoutes {}

impl RootRoutes {
    pub fn create(ctx: &Arc<AppCtx>) -> Result<Router, Error> {
        let router = Router::new()
            .nest("/:user", UserRoutes::create()?)
            .fallback(handler_404)
            .layer(middleware::from_fn_with_state(ctx.clone(), middleware_get_connected_user));

        Ok(router)
    }
}

async fn handler_404(_: Request<Body>) -> impl IntoResponse {
    warn!("\t\t'-> 404 : NOT FOUND");
    (StatusCode::NOT_FOUND, "Not found !")
}

async fn middleware_get_connected_user(State(ctx): State<Arc<AppCtx>>, mut request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let mut context = RequestContext::default();
    match request.headers().get("authtoken") {
        None => {}
        Some(authentication_token) => {
            context.connected_user = RwLock::new(match User::from_auth_token(&ctx.database, &EncString::from(authentication_token)).await {
                Ok(connected_user) => { Some(connected_user) }
                Err(_) => { None }
            })
        }
    }
    let uri = request.uri().clone();
    let user_string = if let Some(user) = context.connected_user().as_ref() {
        format!("#{}", user.name)
    } else { String::from("{?}") };
    info!("[{}] {} | {}", request.method(), user_string, uri);
    request.extensions_mut().insert(Arc::new(context));
    let response = next.run(request).await;

    Ok(response)
}