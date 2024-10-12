use crate::repository::Repository;
use anyhow::Error;

pub struct ActionLogout {

}

impl ActionLogout {
    pub async fn run() -> Result<Repository, Error> {
        let mut repos = Repository::new()?;
        repos.logout().await?;
        Ok(repos)
    }
}