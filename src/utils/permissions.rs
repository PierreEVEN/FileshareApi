use crate::database::item::{Item, ItemId};
use crate::database::repository::{Repository, RepositoryId, RepositoryStatus};
use crate::database::subscription::{Subscription, SubscriptionAccessType};
use crate::database::Database;
use crate::get_connected_user;
use anyhow::Error;
use axum::extract::Request;

pub struct Permissions;

impl Permissions {
    pub async fn view_repository(db: &Database, request: &Request, repository_id: &RepositoryId) -> Result<bool, Error> {
        let repository = Repository::from_id(db, repository_id).await?;
        match repository.status {
            RepositoryStatus::Public | RepositoryStatus::Hidden => {
                return Ok(true);
            }
            _ => {}
        }

        Ok(get_connected_user!(request, user, {
            if repository.owner == *user.id() {
                true
            }
            else if Subscription::find(db, user.id(), repository_id).await.is_ok() {
                true
            }
            else {
                false
            }
        }, {
            false
        }))
    }

    pub async fn edit_repository(db: &Database, request: &Request, repository_id: &RepositoryId) -> Result<bool, Error> {
        let repository = Repository::from_id(db, repository_id).await?;
        Ok(get_connected_user!(request, user, {
            if repository.owner == *user.id() {
                true
            }
            else if let Ok(subscription) = Subscription::find(db, user.id(), repository_id).await {
                match subscription.access_type{
                    SubscriptionAccessType::Moderator => { true }
                    _ => { false }
                }
            }
            else {
                false
            }
        }, {
            false
        }))
    }

    pub async fn upload_to_repository(db: &Database, request: &Request, repository_id: &RepositoryId) -> Result<bool, Error> {
        let repository = Repository::from_id(db, repository_id).await?;
        Ok(get_connected_user!(request, user, {
            if repository.owner == *user.id() || repository.allow_visitor_upload {
                true
            }
            else if let Ok(subscription) = Subscription::find(db, user.id(), repository_id).await {
                match subscription.access_type{
                    SubscriptionAccessType::Contributor |
                    SubscriptionAccessType::Moderator => { true }
                    _ => { false }
                }
            }
            else {
                false
            }
        }, {
            false
        }))
    }

    pub async fn view_item(db: &Database, request: &Request, item_id: &ItemId) -> Result<bool, Error> {
        let item = Item::from_id(db, item_id).await?;
        Ok(if Self::view_repository(db, request, &item.repository_id).await? {
            true
        } else {
            false
        })
    }

    pub async fn edit_item(db: &Database, request: &Request, item_id: &ItemId) -> Result<bool, Error> {
        let item = Item::from_id(db, item_id).await?;
        Ok(if Self::edit_repository(db, request, &item.repository_id).await? {
            true
        } else {
            get_connected_user!(request, user, {
                if item.owner == *user.id() {
                    true
                }
                else {
                    false
                }
            }, {
                false
            })
        })
    }

    pub async fn upload_to_directory(db: &Database, request: &Request, item_id: &ItemId) -> Result<bool, Error> {
        let item = Item::from_id(db, item_id).await?;
        Ok(if Self::upload_to_repository(db, request, &item.repository_id).await? {
            true
        } else {
            get_connected_user!(request, user, {
                if item.owner == *user.id() {
                    true
                }
                else if let Some(directory_data) = item.directory {
                    directory_data.open_upload
                }
                else {
                    false
                }
            }, {
                false
            })
        })
    }
}