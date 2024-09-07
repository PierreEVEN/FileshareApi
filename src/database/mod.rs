use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use anyhow::{Error};
use tokio::net::TcpStream;
use tokio_postgres::{Client, Config, Connection};
use tokio_postgres::tls::NoTlsStream;
use tracing::info;
use crate::config::PostgresConfig;

pub mod item;
pub mod object;
pub mod repository;
pub mod user;

pub struct Database {
    db: Client,
    pub schema_name: String,
    pub file_storage_path: PathBuf,
    pub thumbnail_storage_path: PathBuf,
}

async fn connect_raw(s: &str) -> Result<(Client, Connection<TcpStream, NoTlsStream>), Error> {
    let socket = TcpStream::connect("127.0.0.1:5432").await?;
    let config = s.parse::<Config>()?;
    Ok(config.connect_raw(socket, tokio_postgres::NoTls).await?)
}

async fn connect(s: &str) -> Result<Client, Error> {
    let (client, connection) = connect_raw(s).await?;
    //let connection = connection.map(|r| r.unwrap());
    tokio::spawn(connection);
    Ok(client)
}

pub type DatabaseId = i64;
trait DatabaseIdTrait {
    fn is_valid(&self) -> bool;
}
impl DatabaseIdTrait for DatabaseId {
    fn is_valid(&self) -> bool { *self != 0 }
}

impl Database {
    pub async fn new(config: &PostgresConfig) -> Result<Self, Error> {
        let db = connect(format!("host={} port={} user={} password={} dbname={} sslmode={}", config.url, config.port, config.username, config.secret, config.database, if config.ssl_mode { "enable" } else { "disable" }).as_str()).await?;
        let database = Self { db, schema_name: config.scheme_name.to_string(), file_storage_path: config.file_storage_path.clone(), thumbnail_storage_path: config.thumbnail_storage_path.clone() };
        database.migrate(PathBuf::from("./migrations"), "fileshare_v3").await?;
        Ok(database)
    }


    pub async fn migrate(&self, migrations_dir: PathBuf, schema_name: &str) -> Result<(), Error> {
        let mut entries = vec![];
        for entry in fs::read_dir(migrations_dir)? { entries.push(entry?); }

        entries.sort_by(|a, b| {
            let a = a.file_name();
            let b = b.file_name();
            let mut a_split = a.to_str().unwrap().split("_");
            let mut b_split = b.to_str().unwrap().split("_");

            if let Some(a) = a_split.next() {
                if let Some(b) = b_split.next() {
                    if let Ok(a) = i32::from_str(a) {
                        if let Ok(b) = i32::from_str(b) {
                            return a.cmp(&b);
                        }
                    }
                }
            }
            a.cmp(&b)
        });

        for entry in entries {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(std::ffi::OsStr::to_str) == Some("sql") {
                let sql = fs::read_to_string(path)?.replace("SCHEMA_NAME", schema_name);

                match self.db
                    .simple_query(&sql,
                    ).await {
                    Ok(_) => {
                        info!("Successfully executed migrations {}", entry.file_name().to_str().unwrap());
                    }
                    Err(error) => {
                        return Err(Error::msg(format!("Failed run migration migrate {} : {}", entry.file_name().to_str().unwrap(), error)));
                    }
                };
            }
        }

        Ok(())
    }

    pub fn db(&self) -> &Client {
        &self.db
    }
}

#[macro_export]
macro_rules! make_wrapped_db_type {
    ($T:ident, $Inside:ty $(,$traits:ty)*) => {
        #[derive($($traits,)*)]
        pub struct $T($Inside);
        impl postgres_types::ToSql for $T {
            fn to_sql(&self, ty: &postgres_types::Type, out: &mut postgres_types::private::BytesMut) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> { self.0.to_sql(ty, out) }
            fn accepts(ty: &postgres_types::Type) -> bool { <$Inside>::accepts(ty) }
            postgres_types::to_sql_checked!();
        }
        impl<'a> postgres_types::FromSql<'a> for $T {
            fn from_sql(ty: &postgres_types::Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> { Ok(Self(<$Inside>::from_sql(ty, raw)?)) }
            fn accepts(ty: &postgres_types::Type) -> bool { <$Inside>::accepts(ty) }
        }
    };
}

#[macro_export]
macro_rules! make_database_id {
    ($T:ident) => {
        crate::make_wrapped_db_type!($T, crate::database::DatabaseId, serde::Serialize, serde::Deserialize, Default, std::fmt::Debug, Clone);
        impl std::ops::Deref for $T {
            type Target = crate::database::DatabaseId;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! query_fmt {
    ($db:expr, $query:expr) => {{
        $db.db.query(&$query.replace("SCHEMA_NAME", &$db.schema_name), &[]).await?
    }};

    ($db:expr, $query:expr, $( $bound_values:expr),*) => {{
        let params: &[&(dyn postgres_types::ToSql + Sync)] = &[$(&$bound_values,)*];
        $db.db.query(&$query.replace("SCHEMA_NAME", &$db.schema_name), params).await?
    }};
}

#[macro_export]
macro_rules! query_objects {
    ($db:expr, $StructType:ty, $query:expr) => {{
        let query = $db.db.query(&$query.replace("SCHEMA_NAME", &$db.schema_name), &[]).await?;
        let mut rows = Vec::with_capacity(query.len());
        for row in query {
            rows.push(<$StructType>::try_from_row(&row)?);
        }
        rows
    }};
    ($db:expr, $StructType:ty, $query:expr, $( $bound_values:expr),*) => {{
        let params: &[&(dyn postgres_types::ToSql + Sync)] = &[$(&$bound_values,)*];
        let query = $db.db.query(&$query.replace("SCHEMA_NAME", &$db.schema_name), params).await?;
        let mut rows = Vec::with_capacity(query.len());
        for row in query {
            rows.push(<$StructType>::try_from_row(&row)?);
        }
        rows
    }}
}

#[macro_export]
macro_rules! query_object {
    ($db:expr, $StructType:ty, $query:expr) => {{
        let mut query = $db.db.query(&$query.replace("SCHEMA_NAME", &$db.schema_name), &[]).await?;
        if query.len() > 1 {
            return Err(Error::msg("Received more than one expected item"))
        }
        match query.pop() {
            Some(item) => { Some(<$StructType>::try_from_row(&item)?) }
            None => { None }
        }
    }};
    ($db:expr, $StructType:ty, $query:expr, $( $bound_values:expr),*) => {{
        let params: &[&(dyn postgres_types::ToSql + Sync)] = &[$(&$bound_values,)*];
        let mut query = $db.db.query(&$query.replace("SCHEMA_NAME", &$db.schema_name), params).await?;
        if query.len() > 1 {
            return Err(Error::msg("Received more than one expected item"))
        }
        match query.pop() {
            Some(item) => { Some(<$StructType>::try_from_row(&item)?) }
            None => { None }
        }
    }}
}
