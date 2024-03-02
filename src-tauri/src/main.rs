// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod key_tree;
pub mod stylus;
use serde::{Deserialize, Serialize};
use serde_json::{to_writer_pretty, Value};

use sqlx::SqlitePool;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, BufReader, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};
use vyper_rs::vyper::{Evm, Vyper};
pub mod db;
use db::*;
use key_tree::{create_key, get_key_by_name, list_keys, load_keys_to_state, AppState};
use stylus::{stylus_deploy_contract, stylus_estimate_gas};
use tabled::{settings::Style, Table};
use std::process::Command;
use std::error::Error;
use reqwest::Client;
//Structs for solc 

#[derive(Serialize, Deserialize, Debug)]
struct SolcOutput {
    contracts: Contracts,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Contracts {
    #[serde(rename = "src/soliditylayout/contracts/storage.sol:Owner")]
    owner: Contract,
}

#[derive(Serialize, Deserialize, Debug)]
struct Contract {
    abi: Vec<Value>, // Using serde_json::Value to handle the complexity of ABI items
    bin: String,
}
#[derive(Serialize, Deserialize)]
struct ContractWalletData {
    abi: Value,
    initcode: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    provider: String,
    etherscan_api: String,
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
    contract.compile().map_err(|e| e.to_string())?;
    contract.gen_abi().map_err(|e| e.to_string())?;
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
    // remove the "" from the path
    let path = path.replace("\"", "");
    println!("{:?}", ver);
    println!("{:?}", path);
    let cpath: &Path = &Path::new(&path);
    println!("{:?}", cpath);
    let mut contract = Vyper::new(cpath);
    contract.compile_ver(&ver).map_err(|e| e.to_string())?;
    contract.gen_abi().map_err(|e| e.to_string())?;
    let abifile = File::open(&contract.abi).map_err(|e| e.to_string())?;
    let reader = BufReader::new(abifile);
    let abifile_json: Value = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(ContractWalletData::new(
        abifile_json,
        contract.bytecode.unwrap(),
    ))
}

#[tauri::command]
async fn set_config(provider: String, etherscan_api: String) -> Result<Config, String> {
    let config_path: PathBuf = PathBuf::from("./vyper_deployer_config.json");
    let conf: Config = Config {
        provider,
        etherscan_api,
    };
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Database::init().await?;
    let pool = SqlitePool::connect(DB_URL).await?;
    sqlx::migrate!("../migrations").run(&pool).await?;
    DB_POOL.set(pool).unwrap();
    load_keys_to_state().await.unwrap();
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
            compile_solidity
        ])
        .manage(AppState {
            tree: Mutex::new(BTreeMap::new()),
        })
        .run(tauri::generate_context!())?;
    Ok(())
}

#[tauri::command]
fn compile_solidity(file_path: &str, output_path: &str) -> Result<(String, String), String> {
    let solc_path = "/opt/homebrew/bin/solc";

    let output = Command::new(solc_path)
        .args([
            "--combined-json", "abi,bin,metadata",
            "--overwrite",
            file_path,
            "-o", output_path,
        ])
        .output()
        .map_err(|e| e.to_string())?; // Convert IO errors to String

    if !output.status.success() {
        let error_message = format!("Command executed with failing error code: {}", String::from_utf8_lossy(&output.stderr));
        return Err(error_message); // Directly return the error message as a String
    }

    let json_file_path = format!("{}/combined.json", output_path);
    let file = File::open(&json_file_path)
        .map_err(|e| e.to_string())?; // Convert IO errors to String
    let reader = BufReader::new(file);

    let solc_output: SolcOutput = serde_json::from_reader(reader)
        .map_err(|e| e.to_string())?; // Convert serde_json errors to String
    let contract = solc_output.contracts.owner;

    let abi = serde_json::to_string_pretty(&contract.abi)
        .map_err(|e| e.to_string())?; // Convert serde_json errors to String
    let bytecode = contract.bin;

    // Optionally, write the ABI to a file
    let mut file = File::create(format!("{}/output.json", output_path))
        .map_err(|e| e.to_string())?; // Convert IO errors to String
    file.write_all(abi.as_bytes())
        .map_err(|e| e.to_string())?; // Convert IO errors to String

    Ok((abi, bytecode))
}
// Where i can get all this params without asking for it --> Fix this to put default value
async fn etherscan_verification(api_key: &str , contract_address: &str , source_code : &str , contract_name: &str , compiler_version : &str , optimization_used : &str , runs: &str) -> Result<(), Box<dyn std::error::Error>> {
    //Example
    //let compiler_version = "v0.8.0+commit.c7dfd78e"; // Compiler version used
    //let optimization_used = "1"; // Whether optimization was used (0 = No, 1 = Yes)
    //let runs = "200"; // Optimization runs
    // Additional parameters like constructor arguments, library addresses, etc., may be require
    let client = Client::new();
    let res = client.post("https://api.etherscan.io/api")
        .form(&[
            ("apikey", api_key),
            ("module", "contract"),
            ("action", "verifysourcecode"),
            ("contractaddress", contract_address),
            ("sourceCode", source_code),
            ("contractname", contract_name),
            ("compilerversion", compiler_version),
            ("optimizationUsed", optimization_used),
            ("runs", runs),
            // Add other form fields as needed
        ])
        .send()
        .await?;

    let response_text = res.text().await?;
    println!("Response: {}", response_text);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // Changed to synchronous test for simplicity
    fn test_solc() {
        let file_path = "src/soliditylayout/contracts/storage.sol";
        let output_path = "src/soliditylayout/contracts";
        //let file_path = "/Users/protocolw/Public/Rustcodes/Protocoldenver/VyperDeployooor/src-tauri/src/soliditylayout/contracts/storage.sol"; // Update this path
        match compile_solidity(file_path , output_path) {
            Ok(resp) => println!("{:?}", resp),
            Err(e) => eprintln!("Compilation failed: {}", e),
        }
    }
    #[test]
    fn test_etherscan_verification(){
        // send me tokens 0x2faC34866f272f7C7649823FEE98C83E8ddF2000
        let api_key = "D7MEF2GFCVH4WEC69MSAIHTQ64F3U7EXMU";
        let contract_address= "0x91BD3394ce59fe635253E3739D25Af07DD4952f4";
        let source_code = "src/soliditylayout/contracts/storage.sol";
        let contract_name= "Owner";
        let compiler_version="0.8.24+commit.e11b9ed9";
        let optimization_used="1";
        let runs= "200";
        let verification = etherscan_verification(api_key , contract_address , source_code , contract_name , compiler_version , optimization_used , runs); 
        
    }
}

