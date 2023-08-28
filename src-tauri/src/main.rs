// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use ethers::{
    abi::{Address, Token},
    contract::ContractFactory,
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::{Bytes, U256, U128, U64, I256, H128, Uint8},
};
use serde::{Serialize, Deserialize};
use std::{
    convert::TryFrom, error::Error, fs::File, path::Path, str::FromStr, sync::Arc,
};
use vyper_rs::vyper::Vyper;

#[derive(Serialize, Deserialize)]
struct Args {
    ty: Types, 
    value: String,
}

#[derive(Serialize, Deserialize)]
enum Types {
  U256(U256),
  U128(U128), 
  U64(U64),
  U8(Uint8),
  I256(I256),
  Address(Address),
  Bytes, 
  Bool
}

#[tauri::command]
async fn deploy(
    provider_url: String,
    password: String,
    path: String,
    key_path: String,
    args: Vec<Args>
) -> Result<(), String> {
      let cpath: &Path = Path::new(path.leak());
      let new_abi_path = format!("./{:?}_abi.json", cpath.file_name().unwrap());
      let abi: &Path = Path::new(new_abi_path.leak());
      let mut contract = Vyper::new(cpath, abi);
      contract.compile().map_err(|e| return e.to_string())?;
      contract.abi().map_err(|e| return e.to_string())?;
      let provider = Provider::<Http>::try_from(provider_url).map_err(|e| e.to_string())?;
      let wallet =
          Wallet::decrypt_keystore(&Path::new(&key_path), password).map_err(|e| e.to_string())?;
      let client = SignerMiddleware::new(provider, wallet);
      let client = Arc::new(client);
      let factory = ContractFactory::new(
          ethers::abi::Contract::load(File::open(contract.abi).map_err(|e| e.to_string())?)
              .map_err(|e| e.to_string())?,
          Bytes::from_str(&contract.bytecode.unwrap()).map_err(|e| e.to_string())?,
          client,
      );
      // let tokenized_args = args.iter().map(|a| {
      //     let t = a.ty; 
      //     let v = a.value;
        
      // }).collect();   
      //factory.deploy(tokenized_args)?.send().await.map_err(|e| e.to_string())?;
      Ok(())
      // password for decryption => keys
      // contract path
      // abi
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![deploy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
