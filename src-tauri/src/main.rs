// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use ethers::signers::Wallet;
use serde::{Deserialize, Serialize};
use serde_json::{to_writer_pretty, Value};
use sqlx::SqlitePool;
use std::{fs::File, io::BufReader, path::{Path, PathBuf}};
use vyper_rs::vyper::{Evm, Vyper};
pub mod db;
use db::*;
use tabled::{Table, settings::Style};
use ethers::core::rand::thread_rng;

#[derive(Serialize, Deserialize)]
struct ContractWalletData {
    abi: Value,
    initcode: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    provider: String,
    keystore: String,
}

impl ContractWalletData {
    fn new(abi: Value, initcode: String) -> ContractWalletData {
        Self { abi, initcode }
    }
}

#[tauri::command]
async fn fetch_data(path: String) -> Result<ContractWalletData, String> {
    let cpath: &Path = &Path::new(&path);
    let mut contract = Vyper::new(cpath);
    contract.compile().map_err(|e| return e.to_string())?;
    contract.gen_abi().map_err(|e| return e.to_string())?;
    let abifile = File::open(&contract.abi).map_err(|e| e.to_string())?;
    let reader = BufReader::new(abifile);
    let abifile_json: Value = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    //println!("{:?}", contract.bytecode.clone().unwrap());
    println!("Back to TS!");
    Ok(ContractWalletData::new(
        abifile_json,
        contract.bytecode.unwrap(),
    ))
}
#[tauri::command]
async fn compile_version(path: String, version: String) -> Result<ContractWalletData, String> {
    let ver: Evm = match &version.as_str() {
        &"Shanghai" => Evm::Shanghai,
        &"Paris" => Evm::Paris,
        &"Berlin" => Evm::Berlin,
        &"Istanbul" => Evm::Istanbul,
        &"Cancun" => Evm::Cancun,
        _ => Evm::Shanghai,
    };
    let cpath: &Path = &Path::new(&path);
    let mut contract = Vyper::new(cpath);
    contract
        .compile_ver(&ver)
        .map_err(|e| return e.to_string())?;
    contract.gen_abi().map_err(|e| return e.to_string())?;
    let abifile = File::open(&contract.abi).map_err(|e| e.to_string())?;
    let reader = BufReader::new(abifile);
    let abifile_json: Value = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(ContractWalletData::new(
        abifile_json,
        contract.bytecode.unwrap(),
    ))
}

#[tauri::command]
async fn get_keys(key_path: String) -> Result<Value, String> {
    let keyfile = File::open(PathBuf::from(&key_path)).map_err(|e| e.to_string())?;
    let reader = BufReader::new(keyfile);
    let keystore_json: Value = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(keystore_json)
}

#[tauri::command]
async fn set_config(provider: String, keystore: String) -> Result<Config, String> {
    let config_path: PathBuf = PathBuf::from("./vyper_deployer_config.json");
    let conf: Config = Config { provider, keystore };
    let file: File = File::create(config_path).map_err(|e| e.to_string())?;
    to_writer_pretty(file, &conf).map_err(|e| e.to_string())?;
    Ok(conf)
}

#[tauri::command]
async fn get_config() -> Result<Config, String> {
    let file: File = File::open("./vyper_deployer_config.json").map_err(|e| e.to_string())?;
    let reader: BufReader<File> = BufReader::new(file);
    let conf: Config = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(conf)
}

#[tauri::command]
async fn db_write(deployment_data: Deployment) -> Result<(), String> {
    let db: &sqlx::Pool<sqlx::Sqlite> = DB_POOL.get().unwrap();
    let name = PathBuf::from(&deployment_data.sc_name).file_name().unwrap().to_string_lossy().to_string();
    let query_result = sqlx::query_as!(
        Deployment,
        "INSERT INTO deployments VALUES ($1, $2, $3, $4, $5)",
        name,
        deployment_data.deployer_address,
        deployment_data.deploy_date,
        deployment_data.sc_address,
        deployment_data.network
    )
    .execute(db)
    .await
    .map_err(|e| e.to_string())?;
    println!("{query_result:?}");
    Ok(())
}

#[tauri::command]
async fn db_read() -> Result<Vec<Deployment>, String> {
    let db: &sqlx::Pool<sqlx::Sqlite> = DB_POOL.get().unwrap();
    let query: Vec<Deployment> =
        sqlx::query_as!(Deployment, "SELECT * FROM deployments ORDER BY rowid DESC")
            .fetch_all(db)
            .await
            .map_err(|e| e.to_string())?;
        let mut table = Table::new(&query);
        table.with(Style::psql());
        println!("\n{table}");
    Ok(query)
}

#[tauri::command]
fn generate_keystore(path: String, password: String, name: String) -> Result<(), String> {
    Wallet::new_keystore(path, &mut thread_rng(), password, Some(&name)).map_err(|e| e.to_string())?; 
    println!("Success, wallet created!");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Database::init().await?;
    let pool = SqlitePool::connect(DB_URL).await?;
    sqlx::migrate!("../migrations").run(&pool).await?;
    DB_POOL.set(pool).unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            fetch_data,
            set_config,
            get_config,
            get_keys,
            compile_version,
            db_read,
            db_write,
            generate_keystore
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
