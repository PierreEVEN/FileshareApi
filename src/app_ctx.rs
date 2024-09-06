use crate::config::Config;
use crate::database::Database;

pub struct AppCtx {
    pub config: Config,
    pub database: Database,
}

impl AppCtx {
    pub fn new(config: Config, database: Database) -> Self {
        Self {
            config,
            database,
        }
    }
}