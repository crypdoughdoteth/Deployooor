use crate::{errors::Errors, keys::Keys};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deployment {
    pub sc_name: String,
    pub deployer_address: String,
    pub deploy_date: String,
    pub sc_address: String,
    pub network: String,
    pub fee: String,
    pub verified: bool,
}

impl Default for Database {
    fn default() -> Database {
        Database::init("./deployer.db").unwrap()
    }
}

impl Database {
    pub fn init(path: &str) -> Result<Database, Errors> {
        let conn = Connection::open(path).unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS deployments
            (
                sc_name TEXT NOT NULL,
                deployer_address varchar(42) NOT NULL,
                deploy_date  TEXT NOT NULL,
                sc_address varchar(42) NOT NULL,
                network TEXT NOT NULL,
                fee TEXT NOT NULL,
                verified BOOL NOT NULL
            );

            CREATE TABLE IF NOT EXISTS keys 
            (
                name TEXT NOT NULL PRIMARY KEY,
                path TEXT UNIQUE NOT NULL
            );",
            (),
        )?;

        Ok(Database { conn })
    }

    pub fn record_deployment(&self, deployment_data: Deployment) -> Result<(), Errors> {
        let name = PathBuf::from(&deployment_data.sc_name)
            .file_name()
            .ok_or_else(|| Errors::FsError)?
            .to_string_lossy()
            .to_string();

        let mut statement = self
            .conn
            .prepare("INSERT INTO deployments VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")?;
        statement.execute([
            name,
            deployment_data.deployer_address,
            deployment_data.deploy_date,
            deployment_data.sc_address,
            deployment_data.network,
            deployment_data.fee,
            deployment_data.verified.to_string(),
        ])?;

        Ok(())
    }

    pub fn get_deployments(&self, offset: usize) -> Result<Vec<Deployment>, Errors> {
        let mut statement = self
            .conn
            .prepare("SELECT * FROM deployments LIMIT 20 OFFSET ?1 ORDER BY rowid DESC")?;
        let deployments: Vec<Deployment> = statement
            .query_map([offset], |row| {
                Ok(Deployment {
                    sc_name: row.get_unwrap(0),
                    deployer_address: row.get(1)?,
                    deploy_date: row.get(2)?,
                    sc_address: row.get(3)?,
                    network: row.get(4)?,
                    fee: row.get(5)?,
                    verified: row.get(6)?,
                })
            })?
            .collect::<Result<Vec<Deployment>>>()?;
        Ok(deployments)
    }

    pub fn record_key_metadata(&self, path: &Path, name: &str) -> Result<(), Errors> {
        self.conn.execute(
            "INSERT INTO keys (name, path) VALUES (?1, ?2)",
            (path.to_str().ok_or_else(|| Errors::FsError)?, name),
        )?;
        Ok(())
    }

    pub fn get_key_metadata(&self, offset: usize) -> Result<Vec<Keys>, Errors> {
        let mut statement = self
            .conn
            .prepare("SELECT name, path from Keys LIMIT 20 OFFSET ?1")?;
        let query_map = statement
            .query_map([offset], |e| {
                Ok(Keys {
                    name: e.get(0)?,
                    path: e.get(1)?,
                })
            })?
            .collect::<Result<Vec<Keys>>>()?;
        Ok(query_map)
    }

    pub fn get_all_keys_metadata(&self) -> Result<Vec<Keys>, Errors> {
        let mut statement = self.conn.prepare("SELECT name, path from Keys")?;
        let query_map = statement
            .query_map([], |e| {
                Ok(Keys {
                    name: e.get(0)?,
                    path: e.get(1)?,
                })
            })?
            .collect::<Result<Vec<Keys>>>()?;
        Ok(query_map)
    }
}
