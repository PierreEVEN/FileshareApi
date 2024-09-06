use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use anyhow::{Context, Error};
use rand::{random};
use rand::distributions::{Distribution, Standard};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use crate::config::PostgresConfig;

pub mod item;
pub mod object;
pub mod repository;
pub mod user;

pub struct Database {
    db: Pool<Postgres>,
    pub schema_name: String,
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
        let db = PgPoolOptions::new()
            .max_connections(20)
            .connect(&config.database_url)
            .await
            .context("failed to connect to DATABASE_URL")?;

        let database = Self { db, schema_name: config.scheme_name.to_string() };

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
                match sqlx::query(&sql).execute(&self.db).await {
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

    pub fn db(&self) -> &Pool<Postgres> {
        &self.db
    }
}

#[macro_export]
macro_rules! query_fmt {
    ($db:expr, $query:expr, $( $bound_values:expr),*) => {{
        let str = $query.replace("SCHEMA_NAME", $db.schema_name.as_str());
        let mut query = sqlx::query(str.as_str());
        $(
                query = query.bind($bound_values);
        )*
        query.execute($db.db()).await
    }};
}

#[macro_export]
macro_rules! query_objects {
    ($db:expr, $obj:ty, $query:expr) => {{
        let str = $query.replace("SCHEMA_NAME", $db.schema_name.as_str());
        let mut query = sqlx::query_as(str.as_str());
        let objects : Result<Vec<$obj>, sqlx::Error> = query.fetch_all($db.db()).await;
        objects
    }};
    
    ($db:expr, $obj:ty, $query:expr, $( $bound_values:expr),*) => {{
        let str = $query.replace("SCHEMA_NAME", $db.schema_name.as_str());
        let mut query = sqlx::query_as(str.as_str());
        $(
                query = query.bind($bound_values);
        )*
        let objects : Result<Vec<$obj>, sqlx::Error> = query.fetch_all($db.db()).await;
        objects
    }}
}

#[macro_export]
macro_rules! query_object {
    ($db:expr, $obj:ty, $query:expr, $( $bound_values:expr),*) => {{
        match crate::query_objects!($db, $obj, $query, $($bound_values)*) {
            Ok(mut object) => {
                if object.len() <= 1 { Ok(object.pop().unwrap()) }
                else { Err(sqlx::Error::ColumnNotFound(format!("Expected 1 object, but retrieved {} items", object.len()))) }
            }
            Err(err) => { Err(err) }
        }
    }}
}
