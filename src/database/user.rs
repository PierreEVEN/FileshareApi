use crate::database::item::Item;
use crate::database::{Database, DatabaseId, DatabaseIdTrait};
use crate::utils::enc_string::EncString;
use crate::{make_database_id, make_wrapped_db_type, query_fmt, query_object, query_objects};
use anyhow::Error;
use bcrypt::DEFAULT_COST;
use postgres_from_row::FromRow;
use postgres_types::private::BytesMut;
use postgres_types::{to_sql_checked, IsNull, Type};
use rand::distributions::Alphanumeric;
use rand::{random, Rng};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

make_database_id!(UserId);

make_wrapped_db_type!(PasswordHash, String, Clone, Default, Debug, Serialize, Deserialize);

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
    pub email: EncString,
    pub name: EncString,

    #[serde(skip_serializing)]
    password_hash: PasswordHash,

    pub allow_contact: bool,
    pub user_role: UserRole,
}

#[derive(Serialize, Debug, Default, FromRow)]
pub struct AuthToken {
    owner: UserId,
    pub token: EncString,
    pub device: EncString,
    pub expdate: i64,
}

impl AuthToken {
    pub async fn find(db: &Database, token: &EncString) -> Result<Self, Error> {
        query_object!(db, AuthToken, "SELECT * FROM SCHEMA_NAME.authtoken WHERE token = $1", token).ok_or(Error::msg("Invalid authentication token"))
    }

    pub async fn from_user(db: &Database, id: &UserId) -> Result<Vec<Self>, Error> {
        Ok(query_objects!(db, AuthToken, "SELECT * FROM SCHEMA_NAME.authtoken WHERE owner = $1", id))
    }

    pub async fn delete(&self, db: &Database)  -> Result<(), Error>{
        query_fmt!(db, "DELETE FROM SCHEMA_NAME.authtoken WHERE token = $1", self.token);
        Ok(())
    }
}

impl User {
    pub async fn from_id(db: &Database, id: &UserId) -> Result<Self, Error> {
        match query_object!(db, User, "SELECT * FROM SCHEMA_NAME.users WHERE id = $1", id) {
            None => { Err(Error::msg("User not found")) }
            Some(user) => { Ok(user) }
        }
    }

    pub async fn from_name(db: &Database, name: &String) -> Result<Self, Error> {
        match query_object!(db, User, "SELECT * FROM SCHEMA_NAME.users WHERE name = $1", name) {
            None => { Err(Error::msg("User not found")) }
            Some(user) => { Ok(user) }
        }
    }

    pub async fn exists(db: &Database, login: &EncString) -> Result<bool, Error> {
        Ok(!query_objects!(db, User, r#"SELECT * FROM SCHEMA_NAME.users WHERE name = $1 OR email = $1"#, login.encoded()).is_empty())
    }

    pub async fn from_credentials(db: &Database, login: &EncString, password: &EncString) -> Result<Self, Error> {
        let user = query_object!(db, User, r#"SELECT * FROM SCHEMA_NAME.users WHERE name = $1 OR email = $1"#, login.encoded()).ok_or(Error::msg("User not found"))?;
        if bcrypt::verify(password.encoded(), user.password_hash.0.as_str())? {
            Ok(user)
        } else {
            Err(Error::msg("Failed to find user with given credentials"))
        }
    }

    pub async fn from_auth_token(db: &Database, authtoken: &EncString) -> Result<Self, Error> {
        User::from_id(db, &AuthToken::find(db, authtoken).await?.owner).await
    }

    pub async fn generate_auth_token(&self, db: &Database, device: &EncString) -> Result<EncString, Error> {
        let mut token: String;
        loop {
            token = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect();
            if query_fmt!(db, "SELECT id FROM SCHEMA_NAME.authtoken WHERE token = $1", token).is_empty() {
                break;
            }
        }
        let enc_token = EncString::encode(token.as_str());

        query_fmt!(db, "INSERT INTO SCHEMA_NAME.authtoken (owner, token, device, expdate) VALUES ($1, $2, $3, $4)", self.id, enc_token, device, 0);
        Ok(enc_token)
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

    pub async fn push(&mut self, db: &Database) -> Result<(), Error> {
        query_fmt!(db, "INSERT INTO SCHEMA_NAME.users
                        (id, email, password_hash, name, allow_contact, user_role) VALUES
                        ($1, $2, $3, $4, $5, $6)
                        ON CONFLICT(id) DO UPDATE SET
                        id = $1, email = $2, password_hash = $3, name = $4, allow_contact = $5, user_role = $6;",
            self.id, self.email, self.password_hash, self.name, self.allow_contact, self.user_role);
        Ok(())
    }

    pub async fn delete(&mut self, db: &Database) -> Result<(), Error> {
        for mut item in Item::from_user(db, &self.id).await? {
            item.delete(db).await?;
        }
        query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.users WHERE id = $1;"#, *self.id);
        Ok(())
    }
}