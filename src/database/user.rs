use std::fmt::Debug;
use std::ops::Deref;
use crate::database::item::{Item, ItemId};
use crate::database::{Database, DatabaseId, DatabaseIdTrait};
use crate::{make_database_id, make_wrapped_db_type, query_fmt, query_object, query_objects};
use anyhow::Error;
use bcrypt::DEFAULT_COST;
use postgres_from_row::FromRow;
use postgres_types::{to_sql_checked, IsNull, Type};
use postgres_types::private::BytesMut;
use rand::random;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::utils::enc_string::EncString;

make_database_id!(UserId);

make_wrapped_db_type!(PasswordHash, String);

impl PasswordHash {
    pub fn new(password_string: &EncString) -> Result<Self, Error> {
        let s = Self(bcrypt::hash(password_string.encoded(), DEFAULT_COST)?);
        Ok(s)
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

impl<'a> postgres_types::FromSql<'a> for UserRole {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> { Ok(Self::from(String::from_sql(ty, raw)?)) }

    fn accepts(ty: &Type) -> bool { ty.name() == "user_role" }
}
impl postgres_types::ToSql for UserRole {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        match self {
            UserRole::Guest => { "guest".to_sql(ty, out) }
            UserRole::Vip => { "vip".to_sql(ty, out) }
            UserRole::Admin => { "admin".to_sql(ty, out) }
        }
    }

    fn accepts(ty: &Type) -> bool { ty.name() == "user_role" }

    to_sql_checked!();
}

#[derive(Serialize, Debug, Default, FromRow)]
pub struct User {
    id: UserId,
    #[from_row(from = "String")]
    pub email: EncString,
    #[from_row(from = "String")]
    pub name: EncString,

    #[serde(skip_serializing)]
    #[from_row(from = "String")]
    password_hash: PasswordHash,

    pub allow_contact: bool,
    pub user_role: UserRole,
}

impl User {
    pub async fn from_id(db: &Database, id: &ItemId) -> Result<Self, Error> {
        match query_object!(db, User, "SELECT * FROM SCHEMA_NAME.users WHERE id = $1", id) {
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
            self.id, self.email.encoded(), self.password_hash, self.name.encoded(), self.allow_contact, self.user_role);
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