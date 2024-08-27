// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod config;
pub mod db;
pub mod deploy;
pub mod key_tree;
pub mod verification;
use std::sync::RwLock;

use config::{get_config, set_config};
use deploy::deploy_contract;
use key_tree::{create_key, get_key_by_name, list_keys, load_keys_to_state, AppState};
// use verification::etherscan_verification;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key_state = RwLock::new(load_keys_to_state().unwrap());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_config,
            get_config,
            get_key_by_name,
            list_keys,
            create_key,
            deploy_contract,
        ])
        .manage(AppState { map: key_state })
        .run(tauri::generate_context!())?;
    Ok(())
}
