use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use sqlx::{FromRow, Sqlite, Pool, migrate::MigrateDatabase, query_as};

pub const DB_URL: &str = "sqlite://deployer.db";
pub static DB_POOL: OnceCell<Pool<Sqlite>> = OnceCell::new();

pub struct Database {
    pub store: Pool<Sqlite>,
}
#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
pub struct Deployment {
    pub sc_name: String,
    pub deployer_address: String,
    pub deploy_date: String,
    pub sc_address: String,
    pub network: String,
}

impl Database {
    pub async fn init() -> Result<()> {        
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            println!("Creating database {}", DB_URL);
            Sqlite::create_database(DB_URL).await?;
        } else {
            println!("Database already exists");
        }
        Ok(())
    }
}

