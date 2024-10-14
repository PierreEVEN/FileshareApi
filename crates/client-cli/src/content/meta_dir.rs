use std::{env, fs};
use std::path::{Path, PathBuf};
use anyhow::Error;

static META_DIR_NAME: &str = ".fileshare";
static TMP_DOWNLOAD_DIR_NAME: &str = "tmp";
static CONNECTION_CONFIG_FILE: &str = "connection.json";
static DATABASE_CONFIG_FILE: &str = "database.json";
static REPOSITORY_CONFIG_FILE: &str = "repository.json";

#[derive(Default, Clone)]
pub struct MetaDir {
    root_dir: PathBuf,
}

impl MetaDir {
    pub fn new_here() -> Result<Self, Error> {
        if MetaDir::is_a_repository_root(env::current_dir()?.as_path()) {
            return Err(Error::msg("Cannot create repository here : already an existing repository"))
        }
        Ok(Self {
            root_dir: env::current_dir()?,
        })
    }

    pub fn search_here() -> Result<Self, Error> {
        Self::search_here_internal(env::current_dir()?.as_path())
    }

    fn search_here_internal(path: &Path) -> Result<Self, Error> {
        if !Self::is_a_repository_root(path) {
            if let Some(parent) = path.parent() {
                Self::search_here_internal(parent)
            } else {
                Err(Error::msg("Not a fileshare repository"))
            }
        }
        else {
            Ok(Self {
                root_dir: path.to_path_buf(),
            })
        }
    }

    fn create_dirs(&self) -> Result<(), Error> {
        if !self.root_dir.join(META_DIR_NAME).exists() {
            fs::create_dir_all(self.root_dir.join(META_DIR_NAME).join(TMP_DOWNLOAD_DIR_NAME))?;
        }
        Ok(())
    }

    pub fn connection_config_path(&self) -> Result<PathBuf, Error> {
        self.create_dirs()?;
        Ok(self.root_dir.join(META_DIR_NAME).join(CONNECTION_CONFIG_FILE))
    }

    pub fn repository_config_path(&self) -> Result<PathBuf, Error> {
        self.create_dirs()?;
        Ok(self.root_dir.join(META_DIR_NAME).join(REPOSITORY_CONFIG_FILE))
    }

    pub fn local_database_path(&self) -> Result<PathBuf, Error> {
        self.create_dirs()?;
        Ok(self.root_dir.join(META_DIR_NAME).join(DATABASE_CONFIG_FILE))
    }

    pub fn tmp_download_dir(&self) -> Result<PathBuf, Error> {
        self.create_dirs()?;
        Ok(self.root_dir.join(META_DIR_NAME).join(TMP_DOWNLOAD_DIR_NAME))
    }

    pub fn root(&self) -> &PathBuf {
        &self.root_dir
    }


    pub fn is_a_repository_root(path: &Path) -> bool {
        if !path.exists() { return false; }
        let meta_dir = path.join(META_DIR_NAME);
        if !meta_dir.exists() { return false; }
        true
    }
}