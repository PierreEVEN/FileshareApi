use std::any::type_name;
use std::fmt::Debug;
use crate::database::item::{Item, ItemId};
use crate::database::{Database, DatabaseId, DatabaseIdTrait};
use crate::{query_fmt, query_object, query_objects};
use anyhow::Error;
use std::ops::Deref;
use bcrypt::DEFAULT_COST;
use postgres_from_row::FromRow;
use postgres_types::{to_sql_checked, IsNull, ToSql, Type};
use postgres_types::private::BytesMut;
use rand::random;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use tracing::info;
use crate::utils::enc_string::EncString;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserId(DatabaseId);
impl Deref for UserId {
    type Target = DatabaseId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<DatabaseId> for UserId {
    fn from(value: DatabaseId) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum UserRole {
    #[default]
    Guest,
    Vip,
    Admin,
}

impl From<String> for UserRole {
    fn from(value: String) -> Self {
        match value.as_str() {
            "guest" => { UserRole::Guest }
            "vip" => { UserRole::Vip }
            "admin" => { UserRole::Admin }
            _ => { UserRole::Guest }
        }
    }
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct PasswordHash(String);
impl PasswordHash {
    pub fn new(password_string: &EncString) -> Result<Self, Error> {
        let s = Self(bcrypt::hash(password_string.encoded(), DEFAULT_COST)?);
        Ok(s)
    }
}

impl From<String> for PasswordHash {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl ToSql for PasswordHash {
    fn to_sql(
        &self,
        type_: &Type,
        out: &mut BytesMut
    ) -> Result<IsNull, Box<dyn std::error::Error + 'static + Send + Sync>> {
        if self.type_ != *type_ {
            return Err(format!("expected type {} but saw {}", self.type_, type_).into());
        }

        match self.raw {
            Some(raw) => {
                out.extend_from_slice(raw);
                Ok(IsNull::No)
            }
            None => Ok(IsNull::Yes)
        }
    }

    fn accepts(_: &Type) -> bool { true }

    to_sql_checked!();
}

#[derive(Serialize, Debug, Default, FromRow)]
pub struct User {
    #[from_row(from = "DatabaseId")]
    id: UserId,
    #[from_row(from = "String")]
    pub email: EncString,
    #[from_row(from = "String")]
    pub name: EncString,

    #[serde(skip_serializing)]
    #[from_row(from = "String")]
    password_hash: PasswordHash,

    pub allow_contact: bool,
    #[from_row(from = "String")]
    pub user_role: UserRole,
}

impl User {
    pub async fn from_id(db: &Database, id: &ItemId) -> Result<Self, Error> {
        match query_object!(db, User, "SELECT * FROM SCHEMA_NAME.users WHERE id = $1", **id) {
            None => { Err(Error::msg("User not found")) }
            Some(user) => { Ok(user) }
        }
    }

    pub async fn from_credentials(db: &Database, login: &EncString, password: &EncString) -> Result<Self, Error> {
        info!("login : {}", login.encoded());

        let users = query_objects!(db, User,
            r#"SELECT id, email, name, password_hash, allow_contact, user_role FROM SCHEMA_NAME.users WHERE name = $1 OR email = $1"#, login.encoded());
        info!("res: {:?}", users);
        for user in users {
            if bcrypt::verify(password.encoded(), user.password_hash.0.as_str())? {
                return Ok(user);
            }
        }

        Err(Error::msg("Failed to find user with given credentials"))
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub async fn create_or_reset_password(&mut self, db: &Database, password_hash: &PasswordHash) -> Result<(), Error> {
        if !self.id.is_valid() {
            loop {
                self.id = UserId(random::<DatabaseId>().abs());
                if query_fmt!(db, "SELECT id FROM SCHEMA_NAME.users WHERE id = $1", *self.id).is_empty() {
                    break;
                }
            }
        }
        self.password_hash = password_hash.clone();
        self.push(db).await
    }

    //

    pub async fn push(&mut self, db: &Database) -> Result<(), Error> {
        query_fmt!(db, "INSERT INTO SCHEMA_NAME.users
                        (id, email, password_hash, name, allow_contact, user_role) VALUES
                        ($1, $2, $3, $4, $5, $6)
                        ON CONFLICT(id) DO UPDATE SET
                        id = $1, email = $2, password_hash = $3, name = $4, allow_contact = $5, user_role = $6;",
            self.id, self.email.encoded(), self.password_hash, self.name.encoded(), self.allow_contact);
        Ok(())
    }

    pub async fn delete(&mut self, db: &Database) -> Result<(), Error> {
        todo!();
        for item in Item::from_user(db, &self.id).await? {
            //item.delete(db);
        }

        query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.users WHERE id = $1;"#, *self.id);

        Ok(())
    }
}