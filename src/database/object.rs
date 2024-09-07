use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Error;
use postgres_from_row::FromRow;
use crate::{make_database_id, query_fmt, query_object, query_objects};
use crate::database::Database;
use crate::database::item::{FileData, ItemId};
use crate::database::repository::RepositoryId;

make_database_id!(ObjectId);


#[derive(Debug, FromRow)]
pub struct Object {
    id: ObjectId,
    pub hash: String
}

impl Object {
    pub async fn from_id(db: &Database, id: &ObjectId) -> Result<Self, Error> {
        Ok(query_object!(db, Object, "SELECT * FROM SCHEMA_NAME.objects WHERE id = $1", id).unwrap())
    }

    pub async fn from_repos(db: &Database, id: &RepositoryId) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Object, "SELECT * FROM SCHEMA_NAME.objects WHERE repos = $1", id))
    }

    pub async fn from_item(db: &Database, id: &ItemId) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Object, "SELECT * FROM SCHEMA_NAME.objects WHERE id = $1", id))
    }

    pub async fn insert(db: &Database, file: &Path) -> Result<Self, Error> {
        let new_object = query_object!(db, Self, "INSERT INTO fileshare.objects () VALUES ()").ok_or(Error::msg("Failed to insert object"))?;
        match fs::rename(file, new_object.data_path(db)) {
            Ok(_) => {}
            Err(err) => {
                query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.objects WHERE id = $1;"#, *new_object.id);
                return Err(Error::msg(format!("Failed to store new object : {err}")))
            }
        };
        Ok(new_object)
    }

    pub async fn delete(&mut self, db: &Database) -> Result<(), Error> {
        // Dereference from files
        for mut file in FileData::from_object(db, &self.id).await? {
            file.object = None;
            file.push(db).await?;
        }
        query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.items WHERE id = $1;"#, *self.id);
        Ok(())
    }

    pub fn data_path(&self, db: &Database) -> PathBuf {
        db.file_storage_path.join(self.id.to_string().as_str())
    }

    pub fn thumbnail_path(&self, db: &Database) -> PathBuf {
        db.thumbnail_storage_path.join(self.id.to_string().as_str())
    }
}