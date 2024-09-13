use crate::database::item::{Item, ItemId};
use crate::database::repository::{Repository, RepositoryId, RepositoryStatus};
use crate::database::subscription::{Subscription, SubscriptionAccessType};
use crate::database::Database;
use crate::routes::RequestContext;
use crate::utils::server_error::ServerError;
use axum::extract::Request;
use axum::http::StatusCode;
use std::sync::Arc;

pub struct Permissions {
    request_context: Arc<RequestContext>,
}

unsafe impl Send for Permissions {}
unsafe impl Sync for Permissions {}

pub enum PermissionResult {
    Granted,
    Denied,
}

impl PermissionResult {
    pub fn require(&self) -> Result<(), ServerError> {
        match self {
            PermissionResult::Denied => { Err(ServerError::msg(StatusCode::FORBIDDEN, "Access denied")) }
            _ => { Ok(()) }
        }
    }

    pub fn granted(&self) -> bool {
        match self {
            PermissionResult::Granted => { true }
            _ => { false }
        }
    }
}

impl Permissions {
    pub fn new(request: &Request) -> Result<Self, ServerError> {
        Ok(Self {
            request_context: request.extensions().get::<Arc<RequestContext>>().unwrap().clone(),
        })
    }

    pub async fn view_repository(&self, db: &Database, repository_id: &RepositoryId) -> Result<PermissionResult, ServerError> {
        let repository = Repository::from_id(db, repository_id).await?;
        match repository.status {
            RepositoryStatus::Public | RepositoryStatus::Hidden => {
                return Ok(PermissionResult::Granted);
            }
            _ => {}
        }

        Ok(if let Some(user) = &*self.request_context.connected_user().await {
            if repository.owner == *user.id() {
                PermissionResult::Granted
            } else if Subscription::find(db, user.id(), repository_id).await.is_ok() {
                PermissionResult::Granted
            } else {
                PermissionResult::Denied
            }
        } else {
            PermissionResult::Denied
        })
    }

    pub async fn edit_repository(&self, db: &Database, repository_id: &RepositoryId) -> Result<PermissionResult, ServerError> {
        let repository = Repository::from_id(db, repository_id).await?;
        Ok(if let Some(user) = &*self.request_context.connected_user().await {
            if repository.owner == *user.id() {
                PermissionResult::Granted
            } else if let Ok(subscription) = Subscription::find(db, user.id(), repository_id).await {
                match subscription.access_type {
                    SubscriptionAccessType::Moderator => { PermissionResult::Granted }
                    _ => { PermissionResult::Denied }
                }
            } else {
                PermissionResult::Denied
            }
        } else {
            PermissionResult::Denied
        })
    }

    pub async fn upload_to_repository(&self, db: &Database, repository_id: &RepositoryId) -> Result<PermissionResult, ServerError> {
        let repository = Repository::from_id(db, repository_id).await?;
        Ok(if let Some(user) = &*self.request_context.connected_user().await {
            if repository.owner == *user.id() || repository.allow_visitor_upload {
                PermissionResult::Granted
            } else if let Ok(subscription) = Subscription::find(db, user.id(), repository_id).await {
                match subscription.access_type {
                    SubscriptionAccessType::Contributor |
                    SubscriptionAccessType::Moderator => { PermissionResult::Granted }
                    _ => { PermissionResult::Denied }
                }
            } else {
                PermissionResult::Denied
            }
        } else {
            PermissionResult::Denied
        })
    }

    pub async fn view_item(&self, db: &Database, item_id: &ItemId) -> Result<PermissionResult, ServerError> {
        let item = Item::from_id(db, item_id).await?;
        self.view_repository(db, &item.repository_id).await?.require()?;
        Ok(PermissionResult::Denied)
    }

    pub async fn edit_item(&self, db: &Database, item_id: &ItemId) -> Result<PermissionResult, ServerError> {
        let item = Item::from_id(db, item_id).await?;
        self.edit_repository(db, &item.repository_id).await?.require()?;
        Ok(if let Some(user) = &*self.request_context.connected_user().await {
            if item.owner == *user.id() {
                PermissionResult::Granted
            } else {
                PermissionResult::Denied
            }
        } else {
            PermissionResult::Denied
        })
    }

    pub async fn upload_to_directory(&self, db: &Database, item_id: &ItemId) -> Result<PermissionResult, ServerError> {
        let item = Item::from_id(db, item_id).await?;
        self.upload_to_repository(db, &item.repository_id).await?.require()?;
        Ok(if let Some(user) = &*self.request_context.connected_user().await {
            if item.owner == *user.id() {
                PermissionResult::Granted
            } else if let Some(directory_data) = item.directory {
                if directory_data.open_upload {
                    PermissionResult::Granted
                } else {
                    PermissionResult::Denied
                }
            } else {
                PermissionResult::Denied
            }
        } else {
            PermissionResult::Denied
        })
    }
}