use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PostgresConfig {
    pub database_url: String,
    pub scheme_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ServiceEmailConfig {
    pub host: String,
    pub smtp_port: String,
    pub email_username: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TlsConfig {
    pub certificate: PathBuf,
    pub private_key: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub postgres_db_config: PostgresConfig,
    pub server_mail_server: ServiceEmailConfig,
    pub tls_config: TlsConfig,
    pub use_tls: bool
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 3000,
            postgres_db_config: PostgresConfig {
                database_url: "postgres://postgres:password@localhost/test".to_string(),
                scheme_name: "fileshare_v3".to_string(),
            },
            server_mail_server: ServiceEmailConfig {
                host: "mail.fileshare.fr".to_string(),
                smtp_port: "465".to_string(),
                email_username: "noreply@fileshare.fr".to_string(),
            },
            tls_config: TlsConfig {
                certificate: PathBuf::from("/Path/To/certificate.pem"),
                private_key: PathBuf::from("/Path/To/private_key.pem"),
            },
            use_tls: true,
        }
    }
}

impl Config {
    pub fn from_file(path: PathBuf) -> Result<Self, Error> {
        if path.exists() {
            Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
        }
        else {
            fs::write(path.clone(), serde_json::to_string_pretty(&Config::default())?)?;
            Err(Error::msg("Created a new config file. Please fill in information first"))
        }
    }
}
