use crate::database::object::ObjectId;
use crate::database::repository::RepositoryId;
use crate::database::user::UserId;
use crate::database::{Database, DatabaseIdTrait};
use crate::utils::enc_path::EncPath;
use crate::utils::enc_string::EncString;
use crate::{make_database_id, query_fmt, query_object, query_objects};
use anyhow::Error;
use postgres_from_row::FromRow;

make_database_id!(ItemId);


#[derive(Debug, FromRow)]
pub struct FileData {
    id: ItemId,
    pub size: i64,
    pub mimetype: EncString,
    pub timestamp: i64,
    pub object: Option<ObjectId>,
}

impl FileData {
    pub async fn from_item(db: &Database, id: &ItemId) -> Result<Self, Error> {
        query_object!(db, Self, "SELECT * FROM SCHEMA_NAME.files WHERE id = $1", id).ok_or(Error::msg("Failed to find file from id"))
    }
    pub async fn from_object(db: &Database, id: &ObjectId) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Self, "SELECT * FROM SCHEMA_NAME.files WHERE object = $1", id))
    }
    pub async fn push(&mut self, db: &Database) -> Result<(), Error> {
        assert!(self.id.is_valid());
        query_fmt!(db, "INSERT INTO SCHEMA_NAME.files
                        (id, size, mimetype, timestamp, object) VALUES
                        ($1, $2, $3, $4, $5)
                        ON CONFLICT(id) DO UPDATE SET
                        id = $1, size = $2, mimetype = $3, timestamp = $4, object = $5;",
            self.id, self.size, self.mimetype, self.timestamp, self.object);
        Ok(())
    }

    pub async fn delete(&mut self, db: &Database) -> Result<(), Error> {
        query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.files WHERE id = $1;"#, *self.id);
        if let Some(object) = &self.object {
            // If no files use this object : delete the object
            if FileData::from_object(db, object).await?.is_empty() {
                query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.object WHERE id = $1;"#, object);
            }
        }
        Ok(())
    }
}

#[derive(Debug, FromRow)]
pub struct DirectoryData {
    id: ItemId,
    pub open_upload: bool,
}

impl DirectoryData {
    pub async fn from_item(db: &Database, id: &ItemId) -> Result<Self, Error> {
        query_object!(db, Self, "SELECT * FROM SCHEMA_NAME.files WHERE id = $1", id).ok_or(Error::msg("Failed to find file from id"))
    }
    pub async fn push(&mut self, db: &Database) -> Result<(), Error> {
        assert!(self.id.is_valid());
        query_fmt!(db, "INSERT INTO SCHEMA_NAME.files
                        (id, open_upload) VALUES
                        ($1, $2)
                        ON CONFLICT(id) DO UPDATE SET
                        id = $1, open_upload = $2;",
            self.id, self.open_upload);
        Ok(())
    }
}

#[derive(Debug, FromRow)]
pub struct Item {
    id: ItemId,
    pub repository_id: RepositoryId,
    pub owner: UserId,
    pub name: EncString,
    pub is_regular_file: bool,
    pub description: EncString,
    pub parent_item: Option<ItemId>,
    pub absolute_path: EncPath,
    pub in_trash: bool,
}

impl Item {
    pub async fn from_id(db: &Database, id: &ItemId) -> Result<Self, Error> {
        query_object!(db, Self, "SELECT * FROM SCHEMA_NAME.items WHERE id = $1", id).ok_or(Error::msg("Failed to find item from id"))
    }

    pub async fn from_repository(db: &Database, id: &RepositoryId) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Self, "SELECT * FROM SCHEMA_NAME.items WHERE repository = $1", id))
    }

    pub async fn from_user(db: &Database, id: &UserId) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Self, "SELECT * FROM SCHEMA_NAME.items WHERE owner = $1", id))
    }

    pub async fn from_object(db: &Database, id: &ObjectId) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Self, "SELECT * FROM SCHEMA_NAME.items WHERE id = IN (SELECT id FROM SCHEMA_NAME.files WHERE object = $1)", id))
    }

    pub async fn delete(&mut self, db: &Database) -> Result<(), Error> {
        if self.is_regular_file {
            FileData::from_item(db, &self.id).await?.delete(db).await?;
        } else {
            query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.directories WHERE id = $1;"#, *self.id);
        }
        query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.items WHERE id = $1;"#, *self.id);
        Ok(())
    }
}