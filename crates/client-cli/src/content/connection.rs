use std::{env, fs};
use std::io::{stdin, stdout, Write};
use std::str::FromStr;
use anyhow::Error;
use gethostname::gethostname;
use paris::{success, warn};
use reqwest::Response;
use rpassword::read_password;
use serde_derive::{Deserialize, Serialize};
use types::database_ids::RepositoryId;
use types::enc_string::EncString;
use crate::content::meta_dir::MetaDir;



#[derive(Serialize, Deserialize, Default)]
pub struct Url {
    remote_id: Option<RepositoryId>,
    origin: String
}


#[derive(Serialize, Deserialize, Default)]
pub struct Connection {
    authentication_token: String,
    url: Option<Url>,
    #[serde(skip_deserializing, skip_serializing)]
    meta_dir: MetaDir,
}

impl Connection {
    pub fn new(meta_dir: MetaDir) -> Result<Self, Error> {
        let mut config: Self = serde_json::from_str(fs::read_to_string(meta_dir.connection_config_path()?)?.as_str())?;
        config.meta_dir = meta_dir;
        Ok(config)
    }

    pub async fn set_public_url(&mut self, public_url: &str) -> Result<(), Error> {

        let response = self.get(format!("{}api-link/", self.repos_path()?)).await?.send().await?;
        #[derive(Deserialize)]
        struct RemoteResponse {
            id: RepositoryId,
        }
        let response: RemoteResponse = self.parse_result(response).await?.error_for_status()?.json().await?;
        self.remote_id = Some(response.id);
        Ok()
    }

    pub async fn get(&mut self, path: String) -> Result<reqwest::RequestBuilder, Error> {
        let auth_token = match &self.auth_token {
            None => {
                self.authenticate(None).await?
            }
            Some(auth_token) => { auth_token.token.clone() }
        };

        Ok(reqwest::Client::new()
            .get(self.get_origin()?.join(path.as_str())?)
            .header("content-authtoken", auth_token))
    }


    pub async fn authenticate(&mut self, username: Option<String>) -> Result<String, Error> {
        #[derive(Serialize, Debug, Default)]
        struct AuthenticationBody {
            username: EncString,
            password: String,
            device: EncString,
        }

        let mut body = AuthenticationBody {
            device: EncString::from(format!("Fileshare client - {} ({}) : {}", gethostname().to_str().unwrap(), env::consts::OS, env::current_dir()?.to_str().unwrap()).as_str()),
            ..Default::default()
        };
        body.username = match username {
            None => {
                print!("Username or email : ");
                stdout().flush()?;
                let mut buffer = String::new();
                stdin().read_line(&mut buffer)?;
                if let Some('\n') = buffer.chars().next_back() {
                    buffer.pop();
                }
                if let Some('\r') = buffer.chars().next_back() {
                    buffer.pop();
                }
                EncString::from(buffer.as_str())
            }
            Some(username) => { EncString::from(username.as_str()) }
        };

        for _ in 0..3 {
            print!("Password : ");
            stdout().flush()?;
            body.password = match read_password() {
                Ok(pswd) => { pswd }
                Err(err) => {
                    return Err(Error::msg(err));
                }
            };
            let client = reqwest::Client::new()
                .post(self.get_origin()?.join("api/create-authtoken")?)
                .json(&body)
                .send().await?;

            if client.status().as_u16() == 401 {
                warn!("Wrong credentials. Please try again...");
                continue;
            }

            self.auth_token = Some(client.error_for_status()?.json::<crate::repository::AuthToken>().await?);
            return match &self.auth_token {
                None => { Err(Error::msg("Invalid authentication token")) }
                Some(auth_token) => {
                    Ok(auth_token.token.clone())
                }
            };
        }
        Err(Error::msg("Authentication failed"))
    }

    pub async fn logout(&mut self) -> Result<(), Error> {
        let auth_token = match &self.auth_token {
            None => {
                warn!("Already disconnected");
                return Ok(());
            }
            Some(auth_token) => {
                auth_token.clone()
            }
        };

        self.post(format!("api/delete-authtoken/{}/", auth_token.token)).await?
            .send().await?.error_for_status()?;

        self.auth_token = None;
        success!("Successfully disconnected");

        Ok(())
    }

    pub async fn post(&mut self, path: String) -> Result<reqwest::RequestBuilder, Error> {
        let auth_token = match &self.auth_token {
            None => {
                self.authenticate(None).await?
            }
            Some(auth_token) => { auth_token.token.clone() }
        };

        Ok(reqwest::Client::new()
            .post(self.get_origin()?.join(path.as_str())?)
            .header("content-authtoken", auth_token))
    }

    pub async fn parse_result(&mut self, response: Response) -> Result<Response, Error> {
        if response.status().as_u16() == 401 {
            self.authenticate(None).await?;
        }
        Ok(response)
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        match serde_json::to_string(self) {
            Ok(config_string) => {
                match fs::write(self.meta_dir.connection_config_path().unwrap(), config_string.as_str()) {
                    Ok(_) => {}
                    Err(_err) => { panic!("Failed to serialize local database : {_err}") }
                };
            }
            Err(_err) => { panic!("Failed to serialize local database : {_err}") }
        }
    }
}