use std::ops::Deref;
use crate::database::DatabaseId;

pub struct RepositoryId(DatabaseId);

impl Deref for RepositoryId {
    type Target = DatabaseId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
