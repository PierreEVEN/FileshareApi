use std::fmt::Debug;
use crate::database::item::{Item, ItemId};
use crate::database::{Database, DatabaseId, DatabaseIdTrait};
use crate::{query_fmt, query_object, query_objects};
use anyhow::Error;
use sqlx::{ColumnIndex, Decode, FromRow, Row};
use std::ops::Deref;
use bcrypt::DEFAULT_COST;
use rand::random;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::utils::enc_string::EncString;

#[derive(Serialize, Deserialize, sqlx::Type, Default, Debug)]
#[sqlx(transparent)]
pub struct UserId(DatabaseId);
impl Deref for UserId {
    type Target = DatabaseId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, sqlx::Type, Debug, Default)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    #[default]
    Guest,
    Vip,
    Admin,
}
impl<'a, R: Row> FromRow<'a, R> for UserRole
where
    &'a ::std::primitive::str: ColumnIndex<R>,
    String: ::sqlx::decode::Decode<'a, R::Database>,
    String: ::sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'a R) -> sqlx::Result<Self> {
        Ok(match row.try_get::<'a, String, &str>("role")?.as_str() {
            "guest" => { UserRole::Guest }
            "vip" => { UserRole::Vip }
            "admin" => { UserRole::Admin }
            _ => { UserRole::Guest }
        })
    }
}
impl Deref for UserRole {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            UserRole::Guest => { "guest" }
            UserRole::Vip => { "vip" }
            UserRole::Admin => { "admin" }
        }
    }
}

#[derive(Serialize, sqlx::FromRow, sqlx::Type, Debug, Default, Clone)]
#[sqlx(transparent)]
pub struct PasswordHash(String);
impl PasswordHash {
    pub fn new(password_string: &EncString) -> Result<Self, Error> {
        let s = Self(bcrypt::hash(password_string.encoded(), DEFAULT_COST)?);
        Ok(s)
    }
}

impl Deref for PasswordHash {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, sqlx::FromRow, Decode, Debug, Default)]
pub struct User {
    id: UserId,
    pub email: EncString,
    pub name: EncString,

    #[serde(skip_serializing)]
    password_hash: PasswordHash,

    pub allow_contact: bool,
    pub user_role: UserRole,
}

impl User {
    pub async fn from_id(db: &Database, id: &ItemId) -> Result<Self, Error> {
        Ok(query_object!(db, User, "SELECT * FROM SCHEMA_NAME.users WHERE id = $1", **id)?)
    }

    pub async fn from_credentials(db: &Database, login: &EncString, password: &EncString) -> Result<Self, Error> {
        info!("login : {}", login.encoded());
        let users = query_objects!(db, User, format!("SELECT * FROM SCHEMA_NAME.users WHERE name = $1 OR email = '{}'", login.encoded()), login.encoded())?;
        info!("res: {:?}", users);
        for user in users {
            if bcrypt::verify(password.encoded(), user.password_hash.as_str())? {
                return Ok(user)
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
                if query_fmt!(db, "SELECT id FROM SCHEMA_NAME.users WHERE id = $1", *self.id)?.rows_affected() == 0 {
                    break;
                }
            }
        }
        self.password_hash = password_hash.clone();
        self.push(db).await
    }

    //

    pub async fn push(&mut self, db: &Database) -> Result<(), Error> {
        query_fmt!(db, format!("INSERT INTO SCHEMA_NAME.users
                        (id, email, password_hash, name, allow_contact, user_role) VALUES
                        ($1, $2, $3, $4, $5, '{}')
                        ON CONFLICT(id) DO UPDATE SET
                        id = $1, email = $2, password_hash = $3, name = $4, allow_contact = $5, user_role = '{}';",
            &*self.user_role, &*self.user_role), *self.id, self.email.encoded(), &*self.password_hash, self.name.encoded(), self.allow_contact)?;
        Ok(())
    }

    pub async fn delete(&mut self, db: &Database) -> Result<(), Error> {
        todo!();
        for item in Item::from_user(db, &self.id).await? {
            //item.delete(db);
        }

        query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.users WHERE id = $1;"#, *self.id)?;

        Ok(())
    }
}