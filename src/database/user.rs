use crate::database::repository::Repository;
use crate::database::{Database, DatabaseId, DatabaseIdTrait};
use crate::utils::enc_string::EncString;
use crate::{make_database_id, make_wrapped_db_type, query_fmt, query_object, query_objects};
use anyhow::Error;
use bcrypt::DEFAULT_COST;
use postgres_from_row::FromRow;
use postgres_types::private::BytesMut;
use postgres_types::{to_sql_checked, IsNull, Type};
use rand::distributions::{Alphanumeric, DistString};
use rand::random;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Debug;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::ser::SerializeStruct;
use crate::database::subscription::Subscription;

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

#[derive(Debug, Default, FromRow, Clone)]
pub struct User {
    id: UserId,
    pub email: EncString,
    pub name: EncString,
    pub login: EncString,
    password_hash: PasswordHash,
    pub allow_contact: bool,
    pub user_role: UserRole,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Item", 3)?;

        state.serialize_field("id", &self.id)?;
        if self.allow_contact {
            state.serialize_field("email", &self.email)?;
        }
        state.serialize_field("name", &self.name)?;
        state.serialize_field("login", &self.login)?;
        state.serialize_field("user_role", &self.user_role)?;
        state.end()
    }
}


#[derive(Serialize, Deserialize, Debug, Default, FromRow, Clone)]
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

    pub async fn delete(&self, db: &Database) -> Result<(), Error> {
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

    pub async fn from_url_name(db: &Database, name: &EncString) -> Result<Self, Error> {
        match query_object!(db, User, "SELECT * FROM SCHEMA_NAME.users WHERE LOWER(name) = LOWER($1)", name) {
            None => { Err(Error::msg("User not found")) }
            Some(user) => { Ok(user) }
        }
    }

    pub async fn exists(db: &Database, login: &EncString, email: &EncString) -> Result<bool, Error> {
        Ok(!query_objects!(db, User, r#"SELECT * FROM SCHEMA_NAME.users WHERE login = $1 OR email = $2"#, login, email).is_empty())
    }

    pub async fn has_admin(db: &Database) -> Result<bool, Error> {
        let admins = query_objects!(db, User, "SELECT * FROM SCHEMA_NAME.users WHERE user_role = 'admin'");
        if admins.is_empty() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub async fn from_credentials(db: &Database, login: &EncString, password: &EncString) -> Result<Self, Error> {
        let user = query_object!(db, User, r#"SELECT * FROM SCHEMA_NAME.users WHERE login = $1 OR email = $1"#, login.encoded()).ok_or(Error::msg("User not found"))?;
        if bcrypt::verify(password.encoded(), user.password_hash.0.as_str())? {
            Ok(user)
        } else {
            Err(Error::msg("Failed to find user with given credentials"))
        }
    }

    pub async fn from_auth_token(db: &Database, authtoken: &EncString) -> Result<Self, Error> {
        User::from_id(db, &AuthToken::find(db, authtoken).await?.owner).await
    }

    pub async fn generate_auth_token(&self, db: &Database, device: &EncString) -> Result<AuthToken, Error> {
        let mut token: String;
        loop {
            token = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);
            if query_fmt!(db, "SELECT token FROM SCHEMA_NAME.authtoken WHERE token = $1", token).is_empty() {
                break;
            }
        }
        let enc_token = EncString::encode(token.as_str());

        let exp_date = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

        query_fmt!(db, "INSERT INTO SCHEMA_NAME.authtoken (owner, token, device, expdate) VALUES ($1, $2, $3, $4)", self.id, enc_token, device, exp_date);
        query_object!(db, AuthToken, "SELECT * from SCHEMA_NAME.authtoken WHERE token = $1", enc_token).ok_or(Error::msg("Failed to add authentication token"))
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
        if self.name.is_empty() {
            return Err(Error::msg("Invalid name"));
        }
        query_fmt!(db, "INSERT INTO SCHEMA_NAME.users
                        (id, email, password_hash, name, allow_contact, user_role, login) VALUES
                        ($1, $2, $3, $4, $5, $6, $7)
                        ON CONFLICT(id) DO UPDATE SET
                        id = $1, email = $2, password_hash = $3, name = LOWER($4), allow_contact = $5, user_role = $6, login = $7;",
            self.id, self.email, self.password_hash, self.name, self.allow_contact, self.user_role, self.login);
        Ok(())
    }

    pub async fn delete(&self, db: &Database) -> Result<(), Error> {
        for item in Repository::from_user(db, &self.id).await? {
            item.delete(db).await?;
        }
        for token in AuthToken::from_user(db, self.id()).await? {
            token.delete(db).await?;
        }
        for subscriptions in Subscription::from_user(db, &self.id).await? {
            subscriptions.delete(db).await?
        }
        query_fmt!(db, r#"DELETE FROM SCHEMA_NAME.users WHERE id = $1;"#, self.id);
        Ok(())
    }

    pub fn can_create_repository(&self) -> bool {
        matches!(self.user_role, UserRole::Vip | UserRole::Admin)
    }
}