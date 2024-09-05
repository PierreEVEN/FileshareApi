use anyhow::Error;
use axum::Router;

pub struct UserRoutes {
}

impl UserRoutes {
    pub fn create() -> Result<Router, Error> {
        let router = axum::Router::new();

        Ok(router)
    }



}