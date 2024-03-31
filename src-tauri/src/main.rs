// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod config;
pub mod db;
pub mod deploy;
pub mod key_tree;
pub mod solc;
pub mod stylus;
pub mod verification;
pub mod vyper;
use std::sync::RwLock;

use config::{get_config, set_config};
use db::{db_read, db_write, Database, DB_POOL, DB_URL};
use deploy::deploy_contract;
use key_tree::{create_key, get_key_by_name, list_keys, load_keys_to_state, AppState};
use solc::compile_solidity;
use sqlx::SqlitePool;
use stylus::{stylus_deploy_contract, stylus_estimate_gas};
use vyper::{compile_version, fetch_data};
// use verification::etherscan_verification;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Database::init().await?;
    let pool = SqlitePool::connect(DB_URL).await?;
    sqlx::migrate!("../migrations").run(&pool).await?;
    DB_POOL.set(pool).unwrap();
    let key_state = RwLock::new(load_keys_to_state().await.unwrap());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            fetch_data,
            set_config,
            get_config,
            get_key_by_name,
            compile_version,
            db_read,
            db_write,
            list_keys,
            create_key,
            stylus_deploy_contract,
            stylus_estimate_gas,
            compile_solidity,
            deploy_contract,
        ])
        .manage(AppState { map: key_state })
        .run(tauri::generate_context!())?;
    Ok(())
}
