use crate::config::Config;
use crate::database::Database;
use crate::utils::upload::{Upload, UploadState};
use anyhow::Error;
use rand::random;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct AppCtx {
    pub config: Config,
    pub database: Database,
    uploads: RwLock<HashMap<usize, Arc<tokio::sync::RwLock<Upload>>>>
}

impl AppCtx {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let database = Database::new(&config.backend_config).await?;

        Ok(Self {
            config,
            database,
            uploads: Default::default()
        })
    }

    pub fn add_upload(&self, mut upload: Upload) -> UploadState {
        let mut uploads = self.uploads.write().unwrap();

        let mut id;
        loop {
            id = random::<usize>();
            if !uploads.contains_key(&id) {
                break;
            }
        }

        upload.id = id;

        let state = upload.get_state();
        if !state.finished {
            uploads.insert(id, Arc::new(tokio::sync::RwLock::new(upload)));
        }
        state
    }

    pub fn get_upload(&self, id: usize) -> Result<Arc<tokio::sync::RwLock<Upload>>, Error> {
        match self.uploads.read().unwrap().get(&id) {
            None => { Err(Error::msg("Upload not found")) }
            Some(upload) => { Ok(upload.clone()) }
        }
    }

    pub fn finalize_upload(&self, id: usize) {
        self.uploads.write().unwrap().remove(&id);
    }
}