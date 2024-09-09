use crate::database::user::UserId;
use crate::database::Database;
use crate::utils::enc_string::EncString;
use crate::{make_wrapped_db_type, query_object};
use anyhow::Error;
use postgres_from_row::FromRow;
use postgres_types::private::BytesMut;
use postgres_types::{to_sql_checked, IsNull, Type};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

make_wrapped_db_type!(RepositoryId, crate::database::DatabaseId, serde::Serialize, serde::Deserialize, Default, std::fmt::Debug, Clone);

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

#[derive(Serialize, Debug, Default, FromRow)]
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

impl Repository {
    pub async fn from_id(db: &Database, id: &RepositoryId) -> Result<Self, Error> {
        match query_object!(db, Self, "SELECT * FROM SCHEMA_NAME.repository WHERE id = $1", id) {
            None => { Err(Error::msg("Repository not found")) }
            Some(user) => { Ok(user) }
        }
    }
    pub async fn from_url_name(db: &Database, name: &EncString) -> Result<Self, Error> {
        match query_object!(db, Self, "SELECT * FROM SCHEMA_NAME.repository WHERE url_name = lower($1)", name) {
            None => { Err(Error::msg("Repository not found")) }
            Some(repository) => { Ok(repository) }
        }
    }
}