// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::File, io::BufReader, path::Path};
use vyper_rs::vyper::Vyper;

#[derive(Serialize, Deserialize)]
struct ContractWalletData {
    keystore: Value,
    abi: Value,
    initcode: String,
}
impl ContractWalletData {
    fn new(keystore: Value, abi: Value, initcode: String) -> ContractWalletData {
        Self {
            keystore,
            abi,
            initcode,
        }
    }
}

#[tauri::command]
async fn fetch_data(path: String, key_path: String) -> Result<ContractWalletData, String> {
    let cpath: &Path = Path::new(path.leak());
    let new_abi_path = format!("./{:?}_abi.json", cpath.file_name().unwrap());
    let abi: &Path = Path::new(new_abi_path.leak());
    let mut contract = Vyper::new(cpath, abi);
    contract.compile().map_err(|e| return e.to_string())?;
    contract.abi().map_err(|e| return e.to_string())?;
    let keyfile = File::open(Path::new(&key_path)).map_err(|e| e.to_string())?;
    let reader = BufReader::new(keyfile);
    let keystore_json: Value = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    let abifile = File::open(&abi).map_err(|e| e.to_string())?;
    let reader = BufReader::new(abifile);
    let abifile_json: Value = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(ContractWalletData::new(
        keystore_json,
        abifile_json,
        contract.bytecode.unwrap(),
    ))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
