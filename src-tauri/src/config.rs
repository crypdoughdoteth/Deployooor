use std::{fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;

#[derive(Serialize, Deserialize)]
pub struct Config {
    provider: String,
    etherscan_api: String,
}

#[tauri::command]
pub async fn set_config(provider: String, etherscan_api: String) -> Result<Config, String> {
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
pub async fn get_config() -> Result<Config, String> {
    let file: File = File::open("./vyper_deployer_config.json").map_err(|e| e.to_string())?;
    let reader: BufReader<File> = BufReader::new(file);
    let conf: Config = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(conf)
}
