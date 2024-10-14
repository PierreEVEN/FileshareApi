use std::{env, fs};
use std::io::{stdin, stdout, Write};
use anyhow::Error;
use gethostname::gethostname;
use paris::{success, warn};
use reqwest::Response;
use rpassword::read_password;
use serde_derive::{Deserialize, Serialize};
use types::database_ids::RepositoryId;
use types::enc_string::EncString;
use types::repository::Repository;
use types::user::{AuthToken, LoginInfos};
use crate::content::meta_dir::MetaDir;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Url {
    remote_id: RepositoryId,
    origin: String,
}


#[derive(Serialize, Deserialize, Default)]
pub struct Connection {
    authentication_token: Option<AuthToken>,
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

    pub async fn ping_repository(&mut self) -> Result<Repository, Error> {
        let response = self.post("/repository/find/".to_string()).await?
            .json(&vec![self.remote_id()?])
            .send().await?;
        let repositories: Vec<Repository> = self.parse_result(response).await?.error_for_status()?.json().await?;
        if repositories.is_empty() {
            Err(Error::msg("Repository does not exists"))
        } else {
            Ok(repositories[0].clone())
        }
    }

    pub async fn set_public_url(&mut self, public_url: &str) -> Result<(), Error> {
        let mut method = "";
        let repository_url = if public_url.starts_with("https") {
            method = "https://";
            &public_url[8..]
        } else if public_url.starts_with("http") {
            method = "http://";
            &public_url[7..]
        } else {
            public_url
        };

        let mut split = repository_url.split("/");

        let domain = split.next().ok_or(Error::msg("Invalid domain"))?;
        let user = split.next().ok_or(Error::msg("Invalid user"))?;
        let repository = split.next().ok_or(Error::msg("Invalid repository"))?;
        let response = reqwest::Client::new()
            .get(format!("{}{}/{}/{}/api-link/", method, domain, user, repository))
            .send().await?;
        #[derive(Deserialize)]
        struct RemoteResponse {
            id: RepositoryId,
        }
        let response: RemoteResponse = self.parse_result(response).await?.error_for_status()?.json().await?;
        self.url = Some(Url {
            remote_id: response.id,
            origin: format!("{}{}", method, domain),
        });
        Ok(())
    }

    pub async fn authenticate(&mut self, username: Option<String>) -> Result<EncString, Error> {
        let mut body = LoginInfos {
            device: Some(EncString::from(format!("Cli client - {} ({}) : {}", gethostname().to_str().unwrap(), env::consts::OS, env::current_dir()?.to_str().unwrap()).as_str())),
            ..Default::default()
        };
        body.login = match username {
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

        let url = if let Some(url) = &self.url {
            url
        } else { return Err(Error::msg("Remote url is not set !")) };

        for _ in 0..3 {
            print!("Password : ");
            stdout().flush()?;
            body.password = match read_password() {
                Ok(password) => { EncString::from(password) }
                Err(err) => {
                    return Err(Error::msg(err));
                }
            };
            let client = reqwest::Client::new()
                .post(format!("{}/user/login/", url.origin))
                .json(&body)
                .send().await?;

            if client.status().as_u16() == 401 {
                warn!("Wrong credentials. Please try again...");
                continue;
            }

            self.authentication_token = Some(client.error_for_status()?.json::<AuthToken>().await?);
            return match &self.authentication_token {
                None => { Err(Error::msg("Invalid authentication token")) }
                Some(new_token) => {
                    Ok(new_token.token.clone())
                }
            };
        }
        Err(Error::msg("Authentication failed"))
    }

    pub async fn logout(&mut self) -> Result<(), Error> {
        self.post("/logout/".to_string()).await?
            .send().await?.error_for_status()?;
        self.authentication_token = None;
        success!("Successfully disconnected");
        Ok(())
    }

    pub async fn get(&mut self, mut path: String) -> Result<reqwest::RequestBuilder, Error> {
        let url = if let Some(url) = self.url.clone() { url } else { return Err(Error::msg("Remote url is not set !")) };
        
        if !path.starts_with('/') {
            path = String::from("/") + path.as_str()
        };
        let mut request = reqwest::Client::new()
            .get(format!("{}/api{}", url.origin, path));
        if let Some(authentication_token) =  &self.authentication_token {
            request = request.header("content-authtoken", authentication_token.token.encoded());
        };
        Ok(request)
    }

    pub async fn post(&mut self, path: String) -> Result<reqwest::RequestBuilder, Error> {
        let url = if let Some(url) = self.url.clone() { url } else { return Err(Error::msg("Remote url is not set !")) };
        let mut request = reqwest::Client::new()
            .post(format!("{}/api{}", url.origin, path));
        if let Some(authentication_token) =  &self.authentication_token {
            request = request.header("content-authtoken", authentication_token.token.encoded());
        };
        Ok(request)
    }

    pub fn remote_id(&self) -> Result<RepositoryId, Error> {
        if let Some(url) = &self.url {
            Ok(url.remote_id.clone())
        } else {
            Err(Error::msg("Remote url is not set !"))
        }
    }

    pub async fn parse_result(&mut self, response: Response) -> Result<Response, Error> {
        if response.status().as_u16() == 401 {
            self.authenticate(None).await?;
        }
        Ok(response)
    }

    pub fn metadata_directory(&self) -> &MetaDir {
        &self.meta_dir
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