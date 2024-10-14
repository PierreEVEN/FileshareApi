use anyhow::Error;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::database_ids::{DatabaseIdTrait, UserId};
use crate::enc_string::EncString;

#[cfg(feature = "tokio-postgres")]
use postgres_from_row::FromRow;
#[cfg(feature = "tokio-postgres")]
use postgres_types::private::BytesMut;
#[cfg(feature = "tokio-postgres")]
use postgres_types::{to_sql_checked, IsNull, Type};

#[cfg(feature = "password")]
use crate::database_ids::PasswordHash;

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

#[cfg(feature = "tokio-postgres")]
impl<'a> postgres_types::FromSql<'a> for UserRole {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> { Ok(Self::from(String::from_sql(ty, raw)?)) }

    fn accepts(ty: &Type) -> bool { ty.name() == "user_role" }
}
#[cfg(feature = "tokio-postgres")]
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

#[cfg_attr(feature = "tokio-postgres", derive(FromRow))]
#[derive(Debug, Default, Clone)]
pub struct User {
    id: UserId,
    pub email: EncString,
    pub name: EncString,
    pub login: EncString,
    #[cfg(feature = "password")]
    password_hash: PasswordHash,
    pub allow_contact: bool,
    pub user_role: UserRole,
}

impl User {
    pub fn can_create_repository(&self) -> bool {
        matches!(self.user_role, UserRole::Vip | UserRole::Admin)
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn set_id(&mut self, id: UserId) -> Result<(), Error> {
        if self.id.is_valid() {
            Err(Error::msg("Cannot override a valid id"))
        } else {
            self.id = id;
            Ok(())
        }
    }

    #[cfg(feature = "password")]
    pub fn update_password(&mut self, password: PasswordHash) {
        self.password_hash = password
    }

    #[cfg(feature = "password")]
    pub fn password(&self) -> &PasswordHash {
        &self.password_hash
    }
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

#[cfg_attr(feature = "tokio-postgres", derive(FromRow))]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AuthToken {
    owner: UserId,
    pub token: EncString,
    pub device: EncString,
    pub expdate: i64,
}

impl AuthToken {
    pub fn owner(&self) -> &UserId {
        &self.owner
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct LoginInfos {
    pub login: EncString,
    pub password: EncString,
    pub device: Option<EncString>,
}