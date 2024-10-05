use std::{
    fmt::{self, Display, Formatter},
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;
use url::Url;

use crate::errors::Errors;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub networks: Vec<NetworkSettings>,
    path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            networks: vec![],
            path: PathBuf::from("./"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NetworkSettings {
    name: String,
    provider: Url,
    etherscan_api: Option<Url>,
}

impl Display for NetworkSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Network: {}", self.name,)
    }
}

impl NetworkSettings {
    pub fn from_str(
        name: &str,
        provider: &str,
        etherscan_api: Option<&str>,
    ) -> Result<NetworkSettings, Errors> {
        match etherscan_api {
            Some(k) => Ok(NetworkSettings {
                name: name.to_string(),
                provider: provider.parse()?,
                etherscan_api: Some(k.parse()?),
            }),
            None => Ok(NetworkSettings {
                name: name.to_string(),
                provider: provider.parse()?,
                etherscan_api: None,
            }),
        }
    }
}

impl Config {
    pub fn new(networks: Vec<NetworkSettings>, path: PathBuf) -> Config {
        Config { networks, path }
    }

    pub fn push(&mut self, network: NetworkSettings) {
        self.networks.push(network);
    }

    pub fn from_default_file() -> Config {
        if let Ok(config) = File::open("./deployooor-config.json") {
            let mut reader = BufReader::new(config);
            let mut buf = String::new();
            reader.read_to_string(&mut buf).unwrap();
            let networks = serde_json::from_str::<Vec<NetworkSettings>>(&buf).unwrap();
            let config = Config::new(networks, PathBuf::from("./deployooor-config.json"));
            config
        } else {
            Config::default()
        }
    }

    pub fn set_config(&self) -> Result<(), Errors> {
        let config_path: PathBuf = PathBuf::from("./vyper_deployer_config.json");
        let file: File = File::create(config_path)?;
        to_writer_pretty(file, &self)?;
        Ok(())
    }

    pub fn from_file(path: &Path) -> Result<Config, Errors> {
        let file: File = if Path::new(path).exists() {
            File::open(path)?
        } else {
            File::create_new(path)?;
            File::open(path)?
        };

        let reader: BufReader<File> = BufReader::new(file);
        let conf: Config = serde_json::from_reader(reader)?;
        Ok(conf)
    }
}
