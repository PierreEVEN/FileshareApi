use std::env;
use std::path::Path;
use std::process::Command;
use anyhow::Error;
use crate::config::WebClientConfig;

pub struct WebClient {}

impl WebClient {
    pub fn new(config: &WebClientConfig) -> Result<Self, Error> {
        let base_directory = env::current_dir()?;
        {
            env::set_current_dir(&config.client_path)?;

            let command = if config.debug
            {
                Command::new("webpack")
                    .arg("--watch")
                    .arg("--config")
                    .arg("webpack.dev.js")
                    .output()?
            } else {
                Command::new("webpack")
                    .arg("--config")
                    .arg("webpack.prod.js")
                    .output()?
            };
        }
        env::set_current_dir(base_directory)?;

        Ok(Self {})
    }
}