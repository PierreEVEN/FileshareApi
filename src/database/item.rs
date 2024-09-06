use anyhow::Error;
use crate::database::{Database, DatabaseId};
use crate::{query_object};

#[derive(sqlx::FromRow, Debug)]
pub struct Item {}

impl Item {
    pub async fn from_db(db: &Database, id: DatabaseId) -> Result<Self, Error> {
        Ok(query_object!(db, Item, "SELECT * FROM SCHEMA_NAME WHERE id = $1", id)?)
    }

    pub async fn from_repos(db: &Database, id: DatabaseId) -> Result<Vec<Self>, Error> {
        Ok(query_object!(db, Item, "SELECT * FROM SCHEMA_NAME WHERE id = $1", id)?)
    }
}