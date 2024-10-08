use crate::object::{Object, ObjectId};
use crate::repository::RepositoryId;
use crate::user::UserId;
use crate::{Database, DatabaseIdTrait};
use utils::enc_path::EncPath;
use utils::enc_string::EncString;
use crate::{make_database_id, query_fmt, query_object, query_objects};
use anyhow::Error;
use postgres_from_row::FromRow;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use tokio_postgres::Row;

make_database_id!(ItemId);

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct FileData {
    pub size: i64,
    pub mimetype: EncString,
    pub timestamp: i64,
    pub object: ObjectId,
}

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct DirectoryData {
    pub open_upload: bool,
    pub num_items: i64,
    pub content_size: i64,
}

#[derive(Debug, Default, Clone)]
pub struct Item {
    id: ItemId,
    pub repository: RepositoryId,
    pub owner: UserId,
    pub name: EncString,
    pub description: Option<EncString>,
    pub parent_item: Option<ItemId>,
    pub absolute_path: EncPath,
    pub in_trash: bool,
    pub directory: Option<DirectoryData>,
    pub file: Option<FileData>,
}

impl FromRow for Item {
    fn from_row(row: &Row) -> Self {
        let mut item = Self {
            id: row.get::<&str, ItemId>("id"),
            repository: row.get::<&str, RepositoryId>("repository"),
            owner: row.get::<&str, UserId>("owner"),
            name: row.get::<&str, EncString>("name"),
            description: if let Ok(description) = row.try_get::<&str, EncString>("description") { Some(description) } else { None },
            parent_item: if let Ok(parent_item) = row.try_get::<&str, ItemId>("parent_item") { Some(parent_item) } else { None },
            absolute_path: row.get::<&str, EncPath>("absolute_path"),
            in_trash: row.get::<&str, bool>("in_trash"),
            directory: None,
            file: None,
        };
        if let Ok(size) = row.try_get::<&str, i64>("size") {
            item.file = Some(FileData {
                size,
                mimetype: row.get::<&str, EncString>("mimetype"),
                timestamp: row.get::<&str, i64>("timestamp"),
                object: row.get::<&str, ObjectId>("object"),
            })
        } else if let Ok(open_upload) = row.try_get::<&str, bool>("open_upload") {
            item.directory = Some(DirectoryData {
                open_upload,
                num_items: row.get::<&str, i64>("num_items"),
                content_size: row.get::<&str, i64>("content_size"),
            });
        } else {
            panic!("Parsed item is neither a file or a directory : missing data");
        }
        item
    }

    fn try_from_row(row: &Row) -> Result<Self, tokio_postgres::Error> {
        let mut item = Self {
            id: row.try_get::<&str, ItemId>("id")?,
            repository: row.try_get::<&str, RepositoryId>("repository")?,
            owner: row.try_get::<&str, UserId>("owner")?,
            name: row.try_get::<&str, EncString>("name")?,
            description: if let Ok(description) = row.try_get::<&str, EncString>("description") { Some(description) } else { None },
            parent_item: if let Ok(parent_item) = row.try_get::<&str, ItemId>("parent_item") { Some(parent_item) } else { None },
            absolute_path: row.try_get::<&str, EncPath>("absolute_path")?,
            in_trash: row.try_get::<&str, bool>("in_trash")?,
            directory: None,
            file: None,
        };

        if let Ok(size) = row.try_get::<&str, i64>("size") {
            item.file = Some(FileData {
                size,
                mimetype: row.get::<&str, EncString>("mimetype"),
                timestamp: row.get::<&str, i64>("timestamp"),
                object: row.get::<&str, ObjectId>("object"),
            })
        } else if let Ok(open_upload) = row.try_get::<&str, bool>("open_upload") {
            item.directory = Some(DirectoryData {
                open_upload,
                num_items: row.try_get::<&str, i64>("num_items")?,
                content_size: row.try_get::<&str, i64>("content_size")?,
            });
        }
        Ok(item)
    }
}

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Item", 3)?;

        state.serialize_field("id", &self.id)?;
        state.serialize_field("repository", &self.repository)?;
        state.serialize_field("owner", &self.owner)?;
        state.serialize_field("name", &self.name)?;
        if let Some(description) = &self.description {
            state.serialize_field("description", description)?;
        }
        if let Some(parent_item) = &self.parent_item {
            state.serialize_field("parent_item", parent_item)?;
        }
        state.serialize_field("absolute_path", &self.absolute_path)?;
        state.serialize_field("in_trash", &self.in_trash)?;
        if let Some(directory) = &self.directory {
            state.serialize_field("open_upload", &directory.open_upload)?;
            state.serialize_field("content_size", &directory.content_size)?;
            state.serialize_field("num_items", &directory.num_items)?;
            state.serialize_field("is_regular_file", &false)?;
        } else {
            match &self.file {
                None => {
                    return Err(serde::ser::Error::custom("Missing file data : this item is neither a file or a directory."))
                }
                Some(file) => {
                    state.serialize_field("is_regular_file", &true)?;
                    state.serialize_field("timestamp", &file.timestamp)?;
                    state.serialize_field("mimetype", &file.mimetype)?;
                    state.serialize_field("size", &file.size)?;
                }
            };
        }
        state.end()
    }
}

pub enum Trash {
    Yes,
    No,
    Both,
}

impl Display for Trash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Trash::Yes => { "AND in_trash" }
            Trash::No => { "AND NOT in_trash" }
            Trash::Both => { "" }
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct ItemSearchRepositoryField {
    pub repository: RepositoryId,
    pub root_items: Vec<ItemId>,
}

#[derive(Deserialize, Debug)]
pub struct ItemSearchData {
    pub repositories: Vec<ItemSearchRepositoryField>,
    pub name_filter: Option<EncString>,
    pub before: Option<i64>,
    pub after: Option<i64>,
    pub max_size: Option<i64>,
    pub  min_size: Option<i64>,
    pub mime_type: Option<EncString>,
    pub owners: Option<Vec<UserId>>,
}

impl Item {
    pub async fn from_id(db: &Database, id: &ItemId, filter: Trash) -> Result<Self, Error> {
        query_object!(db, Self, format!("SELECT * FROM SCHEMA_NAME.item_full_view WHERE id = $1 {filter}"), id).ok_or(Error::msg("Failed to find item from id"))
    }

    pub async fn from_path(db: &Database, path: &EncPath, repository: &RepositoryId, filter: Trash) -> Result<Self, Error> {
        query_object!(db, Self, format!("SELECT * FROM SCHEMA_NAME.item_full_view WHERE absolute_path = $1 AND repository = $2 {filter}"), path, repository).ok_or(Error::msg(format!("Failed to find item from path : {path}")))
    }

    pub async fn from_repository(db: &Database, id: &RepositoryId, filter: Trash) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Self, format!("SELECT * FROM SCHEMA_NAME.item_full_view WHERE repository = $1 {filter}"), id))
    }

    pub async fn from_user(db: &Database, id: &UserId, filter: Trash) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Self, format!("SELECT * FROM SCHEMA_NAME.item_full_view WHERE owner = $1 {filter}"), id))
    }

    pub async fn from_object(db: &Database, id: &ObjectId, filter: Trash) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Self, format!("SELECT * FROM SCHEMA_NAME.item_full_view WHERE id IN (SELECT id FROM SCHEMA_NAME.files WHERE object = $1) {filter}"), id))
    }

    pub async fn from_parent(db: &Database, parent_directory: &ItemId, filter: Trash) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Self, format!("SELECT * FROM SCHEMA_NAME.item_full_view WHERE parent_item = $1 {filter}"), parent_directory))
    }

    pub async fn repository_root(db: &Database, repository: &RepositoryId, filter: Trash) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Self, format!("SELECT * FROM SCHEMA_NAME.item_full_view WHERE parent_item IS NULL and repository = $1 {filter}"), repository))
    }

    pub async fn repository_trash_root(db: &Database, repository: &RepositoryId) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Self, "SELECT * FROM SCHEMA_NAME.item_full_view WHERE in_trash AND repository = $1 AND (parent_item IS NULL OR parent_item IN (SELECT id FROM SCHEMA_NAME.item_full_view WHERE repository = $1 AND NOT in_trash))", repository))
    }
    
    pub async fn search(db: &Database, filter: ItemSearchData) -> Result<Vec<Self>, Error> {

        if filter.repositories.is_empty() {
            Err(Error::msg("No repository specified"))?;
        }

        let mut repository_req = String::new();
        for repository in filter.repositories {
            repository_req += format!("repository = {} AND ", repository.repository).as_str();
            let mut item_req = String::new();
            for (i, item) in repository.root_items.iter().enumerate() {
                item_req += format!("STARTS_WITH(absolute_path, SELECT(absolute_path FROM item WHERE id = {item}))").as_str();
                if i != repository.root_items.len() - 1 { item_req += " OR " }
            }
            repository_req += format!("({item_req}) AND").as_str();
        }

        let name = if let Some(name) = filter.name_filter {
            format!("LOWER(name) LIKE '%' || LOWER('{name}') || '%' AND")
        } else { String::new() };

        let before = if let Some(before) = filter.before {
            format!("timestamp <= {before} AND")
        } else { String::new() };

        let after = if let Some(after) = filter.after {
            format!("timestamp >= {after} AND")
        } else { String::new() };

        let max_size = if let Some(max_size) = filter.max_size {
            format!("size <= {max_size} AND")
        } else { String::new() };

        let min_size = if let Some(min_size) = filter.min_size {
            format!("size >= {min_size} AND")
        } else { String::new() };

        let mimetype = if let Some(mimetype) = filter.mime_type {
            format!("LOWER(mimetype) LIKE '%' || LOWER('{}') || '%' AND", mimetype.encoded())
        } else { String::new() };

        let owners = if let Some(owners) = filter.owners {
            let mut owner_req = String::new();
            for owner in owners { owner_req += format!("owner = {owner} AND").as_str() }
            owner_req
        } else { String::new() };

        Ok(query_objects!(&db, Item, format!("SELECT * FROM SCHEMA_NAME.item_full_view WHERE {repository_req} {name} {before} {after} {max_size} {min_size} {mimetype} {owners} is_regular_file")))
    }

    pub async fn delete(&self, db: &Database) -> Result<(), Error> {
        let childs = query_objects!(db, ObjectId, r#"SELECT UNNEST(SCHEMA_NAME.remove_item($1)) AS id GROUP BY id;"#, self.id);
        Object::delete_objects(db, &childs).await?;
        Ok(())
    }

    pub async fn push(&mut self, db: &Database) -> Result<(), Error> {
        if self.directory.is_none() && self.file.is_none() {
            return Err(Error::msg("Cannot push : neither a file or a directory"));
        }

        if self.id.is_valid() {
            query_fmt!(db, "INSERT INTO SCHEMA_NAME.items
                        (id, repository, owner, name, is_regular_file, description, parent_item, absolute_path, in_trash) VALUES
                        ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                        ON CONFLICT(id) DO UPDATE SET
                        id = $1, repository = $2, owner = $3, name = $4, is_regular_file = $5, description = $6, parent_item = $7, absolute_path = $8, in_trash = $9;",
                self.id, self.repository, self.owner, self.name, self.file.is_some(), self.description, self.parent_item, self.absolute_path, self.in_trash);
        } else {
            let res = query_object!(db, ItemId, "INSERT INTO SCHEMA_NAME.items
                        (repository, owner, name, is_regular_file, description, parent_item, absolute_path, in_trash) VALUES
                        ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
                self.repository, self.owner, self.name, self.file.is_some(), self.description, self.parent_item, self.absolute_path, self.in_trash);
            if let Some(res) = res {
                self.id = res;
            }
        }

        if let Some(file) = &self.file {
            query_fmt!(db, "INSERT INTO SCHEMA_NAME.files
                        (id, size, mimetype, timestamp, object) VALUES
                        ($1, $2, $3, $4, $5)
                        ON CONFLICT(id) DO UPDATE SET
                        id = $1, size = $2, mimetype = $3, timestamp = $4, object = $5;",
                self.id, file.size, file.mimetype, file.timestamp, file.object);
        } else if let Some(directory) = &self.directory {
            query_fmt!(db, "INSERT INTO SCHEMA_NAME.directories
                        (id, open_upload) VALUES
                        ($1, $2)
                        ON CONFLICT(id) DO UPDATE SET
                        id = $1, open_upload = $2;",
                self.id, directory.open_upload);
        }
        Ok(())
    }

    pub fn id(&self) -> &ItemId {
        &self.id
    }
}