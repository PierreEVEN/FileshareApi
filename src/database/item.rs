use crate::database::repository::RepositoryId;
use crate::database::user::UserId;
use crate::database::{Database};
use crate::{make_database_id, query_object, query_objects};
use anyhow::Error;
use postgres_from_row::FromRow;

make_database_id!(ItemId);

#[derive(Debug, FromRow)]
pub struct Item {

}

impl Item {
    pub async fn from_id(db: &Database, id: &ItemId) -> Result<Self, Error> {
        Ok(query_object!(db, Item, "SELECT * FROM SCHEMA_NAME.items WHERE id = $1", id).unwrap())
    }

    pub async fn from_repos(db: &Database, id: &RepositoryId) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Item, "SELECT * FROM SCHEMA_NAME.items WHERE repos = $1", id))
    }

    pub async fn from_user(db: &Database, id: &UserId) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Item, "SELECT * FROM SCHEMA_NAME.items WHERE id = $1", id))
    }

    pub async fn delete(&mut self, db: &Database) -> Result<(), Error> {

        //query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.items WHERE id = $1;"#, *self.id)?;
        todo!();
        Ok(())
    }
}