use std::{sync::Arc, time::Duration};

use ethers::{
    abi::Token,
    contract::ContractFactory,
    providers::{Http, Provider},
    types::Bytes,
    utils::hex::FromHex,
};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct ContractDeployment<'a> {
    provider: &'a str,
    args: Box<[Token]>,
    abi: &'a str,
    initcode: &'a str,
}

#[tauri::command]
pub async fn deploy_contract(contract: ContractDeployment<'_>) -> Result<(), String> {
    let provider = Arc::new(
        Provider::<Http>::try_from(contract.provider)
            .map_err(|e| e.to_string())?
            .interval(Duration::from_millis(10u64)),
    );

    let factory = ContractFactory::new(
        ethers::abi::Contract::load(contract.abi.as_bytes()).map_err(|e| e.to_string())?,
        Bytes::from_hex(contract.initcode).map_err(|e| e.to_string())?,
        provider,
    );
    let _res = factory
        .deploy(&*contract.args)
        .map_err(|e| e.to_string())?
        .send_with_receipt()
        .await
        .map_err(|e| e.to_string())?;

    // let address = res.1.contract_address;
    // let fee = res.1.effective_gas_price.unwrap().checked_mul(res.1.gas_used.unwrap()).unwrap();

    Ok(())
}
