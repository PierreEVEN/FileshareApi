use std::{env, fs};
use std::path::PathBuf;
use anyhow::Error;

static META_DIR_NAME: &str = ".fileshare";
static CONNECTION_CONFIG_FILE: &str = "connection.json";

#[derive(Default)]
pub struct MetaDir {
    root_dir: PathBuf
}

impl MetaDir {
    pub fn new_here() -> Result<Self, Error> {
        Ok(Self {
            root_dir: env::current_dir()?,
        })
    }

    pub fn search_here() -> Result<Self, Error> {
        Ok(Self {
            root_dir: env::current_dir()?,
        })
    }
    
    fn create_dirs(&self) -> Result<(), Error> {
        if !self.root_dir.join(META_DIR_NAME).exists() {
            fs::create_dir_all(self.root_dir.join(META_DIR_NAME))?;
        }
        Ok(())
    }
    
    pub fn connection_config_path(&self) -> Result<PathBuf, Error> {
        self.create_dirs()?;
        Ok(env::current_dir()?.join(META_DIR_NAME).join(CONNECTION_CONFIG_FILE))
    }
}