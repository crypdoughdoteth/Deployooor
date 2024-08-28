use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;
use url::Url;

use crate::errors::Errors;

#[derive(Serialize, Deserialize)]
pub struct Config {
    networks: Vec<NetworkSettings>,
}

impl Default for Config {
    fn default() -> Self {
        Self { networks: vec![] }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NetworkSettings {
    name: String,
    provider: Url,
    etherscan_api: Option<Url>,
}

impl NetworkSettings {
    pub fn from_str(
        name: String,
        provider: String,
        etherscan_api: Option<String>,
    ) -> Result<NetworkSettings, Errors> {
        match etherscan_api {
            Some(k) => Ok(NetworkSettings {
                name,
                provider: provider.parse()?,
                etherscan_api: Some(k.parse()?),
            }),
            None => Ok(NetworkSettings {
                name,
                provider: provider.parse()?,
                etherscan_api: None,
            }),
        }
    }
}

impl Config {
    pub fn new(networks: Vec<NetworkSettings>) -> Config {
        Config { networks }
    }

    pub fn push(&mut self, network: NetworkSettings) {
        self.networks.push(network);
    }

    pub fn from_default_file() -> Result<Config, Errors> {
        let file: File = File::open("./vyper_deployer_config.json")?;
        let reader: BufReader<File> = BufReader::new(file);
        let conf: Config = serde_json::from_reader(reader)?;
        Ok(conf)
    }

    pub fn set_config(&self) -> Result<(), Errors> {
        let config_path: PathBuf = PathBuf::from("./vyper_deployer_config.json");
        let file: File = File::create(config_path)?;
        to_writer_pretty(file, &self)?;
        Ok(())
    }

    pub fn from_file(path: &Path) -> Result<Config, Errors> {
        let file: File = File::open(path)?;
        let reader: BufReader<File> = BufReader::new(file);
        let conf: Config = serde_json::from_reader(reader)?;
        Ok(conf)
    }
}
