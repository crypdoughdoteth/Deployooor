use std::{fs::File, io::BufReader, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;

use crate::errors::Errors;

#[derive(Serialize, Deserialize)]
pub struct Config {
    networks: Vec<NetworkSettings>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            networks: vec![]
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NetworkSettings {
    name: String,
    provider: String,
    etherscan_api: Option<String>,
}

impl NetworkSettings {
    pub fn new(name: String, provider: String, etherscan_api: Option<String>) -> NetworkSettings {
        NetworkSettings {
            name,
            provider,
            etherscan_api,
        }
    }
}

impl Config {
    pub async fn new(networks: Vec<NetworkSettings>) -> Config {
        Config { networks }
    }

    pub fn push(&mut self, network: NetworkSettings) {
        self.networks.push(network);

    }

    pub async fn from_file() -> Result<Config, Errors> {
        let file: File = File::open("./vyper_deployer_config.json")?;
        let reader: BufReader<File> = BufReader::new(file);
        let conf: Config = serde_json::from_reader(reader)?;
        Ok(conf)
    }

    pub async fn set_config(&self) -> Result<(), Errors> {
        let config_path: PathBuf = PathBuf::from("./vyper_deployer_config.json");
        let file: File = File::create(config_path)?;
        to_writer_pretty(file, &self)?;
        Ok(())
    }
}