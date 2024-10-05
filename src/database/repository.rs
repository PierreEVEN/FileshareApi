use crate::database::item::Item;
use crate::database::item::Trash::Both;
use crate::database::subscription::Subscription;
use crate::database::user::UserId;
use crate::database::{Database, DatabaseIdTrait};
use crate::utils::enc_string::EncString;
use crate::{make_database_id, query_fmt, query_object, query_objects};
use anyhow::Error;
use postgres_from_row::FromRow;
use postgres_types::private::BytesMut;
use postgres_types::{to_sql_checked, IsNull, Type};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

make_database_id!(RepositoryId);

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum RepositoryStatus {
    #[default]
    Private,
    Hidden,
    Public,
}

impl From<String> for RepositoryStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "private" => { RepositoryStatus::Private }
            "hidden" => { RepositoryStatus::Hidden }
            "public" => { RepositoryStatus::Public }
            _ => { RepositoryStatus::Private }
        }
    }
}

impl<'a> postgres_types::FromSql<'a> for RepositoryStatus {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> { Ok(Self::from(String::from_sql(ty, raw)?)) }
    fn accepts(ty: &Type) -> bool { ty.name() == "repository_status" }
}
impl postgres_types::ToSql for RepositoryStatus {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        match self {
            RepositoryStatus::Private => { "private".to_sql(ty, out) }
            RepositoryStatus::Hidden => { "hidden".to_sql(ty, out) }
            RepositoryStatus::Public => { "public".to_sql(ty, out) }
        }
    }
    fn accepts(ty: &Type) -> bool { ty.name() == "repository_status" }
    to_sql_checked!();
}

#[derive(Serialize, Debug, Default, FromRow, Clone)]
pub struct Repository {
    id: RepositoryId,
    pub url_name: EncString,
    pub owner: UserId,
    pub description: Option<EncString>,
    pub status: RepositoryStatus,
    pub display_name: EncString,
    pub max_file_size: Option<i64>,
    pub visitor_file_lifetime: Option<i64>,
    pub allow_visitor_upload: bool,
}

#[derive(Serialize, Default)]
pub struct RepositoryContributorStats {
    id: UserId,
    count: i64,
}
#[derive(Serialize, Default)]
pub struct RepositoryExtensionStats {
    mimetype: EncString,
    count: i64,
}
#[derive(Serialize, Default)]
pub struct RepositoryStats {
    trash_items: usize,
    trash_directories: usize,
    trash_size: usize,
    items: usize,
    directories: usize,
    size: usize,
    contributors: Vec<RepositoryContributorStats>,
    extensions: Vec<RepositoryExtensionStats>,
}


impl Repository {
    pub async fn from_id(db: &Database, id: &RepositoryId) -> Result<Self, Error> {
        match query_object!(db, Self, "SELECT * FROM SCHEMA_NAME.repository WHERE id = $1", id) {
            None => { Err(Error::msg("Repository not found")) }
            Some(user) => { Ok(user) }
        }
    }
    pub async fn from_user(db: &Database, user: &UserId) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Self, "SELECT * FROM SCHEMA_NAME.repository WHERE owner = $1", user))
    }
    pub async fn shared_with(db: &Database, user: &UserId) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Self, "SELECT * FROM SCHEMA_NAME.repository WHERE id IN (SELECT repository FROM SCHEMA_NAME.subscriptions WHERE owner = $1);", user))
    }
    pub async fn from_url_name(db: &Database, name: &EncString) -> Result<Self, Error> {
        match query_object!(db, Self, "SELECT * FROM SCHEMA_NAME.repository WHERE lower(url_name) = lower($1)", name) {
            None => { Err(Error::msg("Repository not found")) }
            Some(repository) => { Ok(repository) }
        }
    }
    pub async fn public(db: &Database) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, Self, "SELECT * FROM SCHEMA_NAME.repository WHERE status = 'public'"))
    }

    pub async fn push(&mut self, db: &Database) -> Result<(), Error> {
        if self.url_name.is_empty() {
            return Err(Error::msg("Invalid name"));
        }
        if self.id.is_valid() {
            query_fmt!(db, "INSERT INTO SCHEMA_NAME.repository
                        (id, url_name, owner, description, status, display_name, max_file_size, visitor_file_lifetime, allow_visitor_upload) VALUES
                        ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                        ON CONFLICT(id) DO UPDATE SET
                        id = $1, url_name = $2, owner = $3, description = $4, status = $5, display_name = $6, max_file_size = $7, visitor_file_lifetime = $8, allow_visitor_upload = $9;",
                self.id, self.url_name, self.owner, self.description, self.status, self.display_name, self.max_file_size, self.visitor_file_lifetime, self.allow_visitor_upload);
        } else {
            let res = query_object!(db, RepositoryId, "INSERT INTO SCHEMA_NAME.repository
                        (url_name, owner, description, status, display_name, max_file_size, visitor_file_lifetime, allow_visitor_upload) VALUES
                        ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
                self.url_name, self.owner, self.description, self.status, self.display_name, self.max_file_size, self.visitor_file_lifetime, self.allow_visitor_upload);
            if let Some(res) = res {
                self.id = res;
            }
        }
        Ok(())
    }

    pub async fn delete(&self, db: &Database) -> Result<(), Error> {
        for item in Item::from_repository(db, &self.id, Both).await? {
            item.delete(db).await?;
        }
        for subscriptions in Subscription::from_repository(db, &self.id).await? {
            subscriptions.delete(db).await?
        }
        query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.repository WHERE id = $1;"#, self.id);
        Ok(())
    }

    pub fn id(&self) -> &RepositoryId {
        &self.id
    }

    pub async fn stats(&self, db: &Database) -> Result<RepositoryStats, Error> {
        let mut stats = RepositoryStats::default();
        if let Some(files) = query_fmt!(db, "SELECT COUNT(id) AS num, CAST(COALESCE(SUM(size), 0) AS BIGINT) AS size FROM SCHEMA_NAME.files WHERE id IN (SELECT id FROM SCHEMA_NAME.items WHERE repository = $1 AND NOT in_trash)", self.id).pop() {
            stats.items = files.try_get::<&str, i64>("num")? as usize;
            stats.size = files.try_get::<&str, i64>("size")? as usize;
        }
        if let Some(files) = query_fmt!(db, "SELECT COUNT(id) AS num FROM SCHEMA_NAME.items WHERE NOT is_regular_file AND id IN (SELECT id FROM SCHEMA_NAME.items WHERE repository = $1 AND NOT in_trash)", self.id).pop() {
            stats.directories = files.try_get::<&str, i64>("num")? as usize;
        }
        if let Some(files) = query_fmt!(db, "SELECT COUNT(id) AS num, CAST(COALESCE(SUM(size), 0) AS BIGINT) AS size FROM SCHEMA_NAME.files WHERE id IN (SELECT id FROM SCHEMA_NAME.items WHERE repository = $1 AND in_trash)", self.id).pop() {
            stats.trash_items = files.try_get::<&str, i64>("num")? as usize;
            stats.trash_size = files.try_get::<&str, i64>("size")? as usize;
        }
        if let Some(files) = query_fmt!(db, "SELECT COUNT(id) AS num FROM SCHEMA_NAME.items WHERE NOT is_regular_file AND id IN (SELECT id FROM SCHEMA_NAME.items WHERE repository = $1 AND in_trash)", self.id).pop() {
            stats.trash_directories = files.try_get::<&str, i64>("num")? as usize;
        }
        for extension in query_fmt!(db, "SELECT mimetype, COUNT(id) AS num FROM SCHEMA_NAME.files WHERE id IN (SELECT id FROM SCHEMA_NAME.items WHERE repository = $1) GROUP BY mimetype ORDER BY num DESC", self.id) {
            stats.extensions.push(RepositoryExtensionStats {
                mimetype: extension.try_get::<&str, EncString, >("mimetype")?,
                count: extension.try_get::<&str, i64, >("num")?,
            });
        }
        for user in query_fmt!(db, "SELECT owner, COUNT(id) AS num FROM SCHEMA_NAME.items WHERE repository = $1 GROUP BY owner ORDER BY num DESC", self.id) {
            stats.contributors.push(RepositoryContributorStats {
                id: user.try_get::<&str, UserId, >("owner")?,
                count: user.try_get::<&str, i64, >("num")?,
            });
        }
        Ok(stats)
    }
}