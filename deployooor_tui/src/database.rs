use crate::errors::Errors;
use sqlx::{sqlite::SqliteConnectOptions, FromRow, Pool, Sqlite, SqlitePool};
use std::path::PathBuf;
use std::sync::OnceLock;
use tabled::{settings::Style, Table, Tabled};
pub const DB_URL: &str = "deployer.db";
pub static DB_POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

pub struct Database;

#[derive(Clone, FromRow, Debug, Tabled)]
pub struct Deployment {
    pub sc_name: String,
    pub deployer_address: String,
    pub deploy_date: String,
    pub sc_address: String,
    pub network: String,
    pub fee: String,
    pub verified: bool,
}

impl Database {
    pub async fn init() -> Result<Database, Errors> {
        let options = SqliteConnectOptions::new()
            .filename(DB_URL)
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        DB_POOL.set(pool).unwrap();
        println!("Database initialized!");
        Ok(Database)
    }

    pub async fn record_deployment(&self, deployment_data: Deployment) -> Result<Database, Errors> {
        let db: &sqlx::Pool<sqlx::Sqlite> = DB_POOL.get().unwrap();
        let name = PathBuf::from(&deployment_data.sc_name)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let query_result = sqlx::query_as!(
            Deployment,
            "INSERT INTO deployments VALUES ($1, $2, $3, $4, $5, $6, $7)",
            name,
            deployment_data.deployer_address,
            deployment_data.deploy_date,
            deployment_data.sc_address,
            deployment_data.network,
            deployment_data.fee,
            deployment_data.verified,
        )
        .execute(db)
        .await?;
        println!("{query_result:?}");
        Ok(Database)
    }

    pub async fn get_deployments(&self) -> Result<Vec<Deployment>, Errors> {
        let db: &sqlx::Pool<sqlx::Sqlite> = DB_POOL.get().unwrap();
        let query: Vec<Deployment> =
            sqlx::query_as!(Deployment, "SELECT * FROM deployments ORDER BY rowid DESC")
                .fetch_all(db)
                .await?;
        let mut table = Table::new(&query);
        table.with(Style::psql());
        println!("\n{table}");
        Ok(query)
    }
}
