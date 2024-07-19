use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;
use vyper_rs::{
    vyper,
    vyper::{Evm, Vyper},
};

#[derive(Serialize, Deserialize)]
pub struct ContractInfo {
    abi: Value,
    initcode: String,
}

impl ContractInfo {
    fn new(abi: Value, initcode: String) -> ContractInfo {
        Self { abi, initcode }
    }
}

#[tauri::command]
pub async fn fetch_data(path: String) -> Result<ContractInfo, String> {
    let mut contract = vyper!(&path);
    contract.compile().map_err(|e| e.to_string())?;
    let abi = contract.get_abi().map_err(|e| e.to_string())?;
    println!("Back to TS!");
    Ok(ContractInfo::new(abi, contract.bytecode.unwrap()))
}
#[tauri::command]
pub async fn compile_version(path: String, version: String) -> Result<ContractInfo, String> {
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
    println!("{:?}", path);
    let mut contract = vyper!(&path);
    contract.compile_ver(&ver).map_err(|e| e.to_string())?;
    let abi = contract.get_abi().map_err(|e| e.to_string())?;
    Ok(ContractInfo::new(abi, contract.bytecode.unwrap()))
}
