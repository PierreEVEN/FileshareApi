use std::ops::Deref;
use anyhow::Error;
use sqlx::{FromRow, Postgres, Row};
use crate::database::{Database, DatabaseId, DatabaseIdTrait};
use crate::{query_fmt, query_object};
use crate::database::item::ItemId;

#[derive(sqlx::FromRow, Debug)]
pub struct UserId(DatabaseId);
impl Deref for UserId {
    type Target = DatabaseId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct PasswordHash(String);

#[derive(Debug)]
pub enum Role {
    Guest,
    Vip,
    Admin,
}

impl<'r, R: Row> FromRow<'r, R> for Role {
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        
        let test = row.try_get(0)?;
        todo!()
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    id: UserId,
    pub email: String,
    pub name: String,
    pub password_hash: PasswordHash,
    pub allow_contact: bool,
    pub role: Role,
}

impl User {


    pub async fn from_id(db: &Database, id: ItemId) -> Result<Self, Error> {
        Ok(query_object!(db, User, "SELECT * FROM SCHEMA_NAME.users WHERE id = $1", *id)?)
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub async fn create();

    pub async fn push(&mut self, db: &Database) -> Result<(), Error> {
        if !self.id.is_valid() { self.id = UserId(DatabaseId::generate_id()); }


        query_fmt!(db, r#"INSERT INTO fileshare.users
                        (id, email, name, allow_contact, role) VALUES
                        ($1, $2, $3, $4, $5)
                        ON CONFLICT(id) DO
                        UPDATE SET id = $1, email = $2, name = $3, allow_contact = $4, role = $5;"#,
                        *self.id, self.email.as_str(), self.name.as_str(), self.allow_contact, self.role)?;
        Ok(())
    }
}