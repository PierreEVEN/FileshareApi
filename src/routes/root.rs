use anyhow::Error;
use axum::Router;
use crate::routes::user::UserRoutes;

pub struct RootRoutes {}

impl RootRoutes {
    pub fn create() -> Result<Router, Error> {
        let router = axum::Router::new()
            .nest("/:user", UserRoutes::create()?);

        Ok(router)
    }
}