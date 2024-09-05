use std::fs;
use std::path::PathBuf;
use anyhow::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PostgresConfig {
    pub username: String,
    pub password: String,
    pub scheme_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ServiceEmailConfig {
    pub host: String,
    pub smtp_port: String,
    pub email_username: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TlsConfig {
    pub certificate: PathBuf,
    pub private_key: PathBuf,
    pub chain: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub port: u16,
    pub postgres_db_config: PostgresConfig,
    pub server_mail_server: ServiceEmailConfig,
    pub tls_config: TlsConfig,
    pub use_tls: bool
}

impl Config {
    pub fn from_file(path: PathBuf) -> Result<Self, Error> {
        if path.exists() {
            Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
        }
        else {
            fs::write(path, serde_json::to_string(&Config::default())?)?;
            tracing::info!("Created a new config file. Please fill in information first");
            Err(Error::msg("Created a new config file. Please fill in information first"))
        }
    }
}
