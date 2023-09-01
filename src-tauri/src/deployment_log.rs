use std::fs::File;
use serde::{Deserialize, Serialize};
use anyhow::Result; 

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub store: Vec<Deployment>
}
#[derive(Serialize, Deserialize)]
pub struct Deployment {
    pub contract_name: String,
    pub deployer_address: String,
    pub date: String,
    pub contract_address: String,
    pub network: String,
}

impl Database {

    pub fn init() -> Result<Database> {        
       let db: Vec<Deployment> = match File::open("./deployments_db.json") {
            Ok(f) => serde_json::from_reader(f)?,
            Err(_) => {
                File::create("./deployments_db.json")?;
                Vec::new()
            },
        };
        Ok(Database {store: db})
    }    

    pub fn dump(&self) -> Result<()> {
        let file: File = File::create("./deployments_db.json")?;
        serde_json::to_writer_pretty(file, &self.store)?;
        Ok(())
    }

}