use ethers::{
    abi::{Address, Int, Token, Uint},
    contract::ContractFactory,
    providers::{Http, Provider},
    types::Bytes,
    utils::hex::FromHex,
};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};

#[derive(Serialize, Deserialize, Clone)]
pub struct ContractDeployment<'a> {
    provider: &'a str,
    args: Box<[ConstructorArg<'a>]>,
    abi: &'a str,
    initcode: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstructorArg<'a> {
    ty: EthType,
    value: &'a str,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EthType {
    Address,
    String,
    Uint,
    Int,
    Array(Box<EthType>),
    FixedArray(Box<EthType>),
    Bytes,
    FixedBytes,
    Struct(Vec<EthType>),
    Bool,
}

// This method will work for any regular contract that needs to be deployed to an EVM environment
// from vyper, solidity etc
#[tauri::command]
pub async fn deploy_contract(contract: ContractDeployment<'_>) -> Result<(), String> {
    let provider = Arc::new(
        Provider::<Http>::try_from(contract.provider)
            .map_err(|e| e.to_string())?
            .interval(Duration::from_millis(10u64)),
    );

    let abi = ethers::abi::Contract::load(contract.abi.as_bytes()).map_err(|e| e.to_string())?;
    let initcode = Bytes::from_hex(contract.initcode).map_err(|e| e.to_string())?;
    let factory = ContractFactory::new(abi, initcode, provider);

    let _res = match &contract.args.len() {
        0 => factory
            .deploy(())
            .map_err(|e| e.to_string())?
            .send_with_receipt()
            .await
            .map_err(|e| e.to_string())?,
        1 => factory
            .deploy(contract.parse_constructor_args()?.pop().unwrap())
            .map_err(|e| e.to_string())?
            .send_with_receipt()
            .await
            .map_err(|e| e.to_string())?,
        _ => factory
            .deploy(contract.parse_constructor_args()?)
            .map_err(|e| e.to_string())?
            .send_with_receipt()
            .await
            .map_err(|e| e.to_string())?,
    };

    // let address = res.1.contract_address;
    // let fee = res.1.effective_gas_price.unwrap().checked_mul(res.1.gas_used.unwrap()).unwrap();

    Ok(())
}

impl<'a> ContractDeployment<'a> {
    fn parse_constructor_args(&self) -> Result<Vec<Token>, String> {
        Ok(self
            .args
            .into_iter()
            .map(|e| ConstructorArg::parse_constructor_arg(&e.ty, e.value))
            .collect::<Result<Vec<Token>, String>>()?)
    }
}

impl<'a> ConstructorArg<'a> {
    // Arrays and Structs require an inner type since these types are collections of sorts.
    // Structs are heterogeneous types that are encoded as tuples in the Ethereum ABI.
    // For each field of the struct, include the corresponding EthType
    // Arrays will have their type specified as an inner field, values should be comma seperated strings
    fn parse_constructor_arg(abi_type: &EthType, value: &str) -> Result<Token, String> {
        match abi_type {
            EthType::Address => Ok(Token::Address(
                value.parse::<Address>().map_err(|e| e.to_string())?,
            )),
            EthType::String => Ok(Token::String(value.to_owned())),
            EthType::Uint => Ok(Token::Uint(
                value.parse::<Uint>().map_err(|e| e.to_string())?,
            )),
            EthType::Int => Ok(Token::Int(value.parse::<Int>().map_err(|e| e.to_string())?)),
            EthType::Bool => Ok(Token::Bool(
                value.parse::<bool>().map_err(|e| e.to_string())?,
            )),
            EthType::FixedArray(ty) => Ok(Token::FixedArray(
                value
                    .split(",")
                    .map(|e| -> Result<Token, String> { Self::parse_constructor_arg(&*ty, e) })
                    .collect::<Result<Vec<Token>, String>>()?,
            )),
            EthType::Array(ty) => Ok(Token::Array(
                value
                    .split(",")
                    .map(|e| -> Result<Token, String> { Self::parse_constructor_arg(&*ty, e) })
                    .collect::<Result<Vec<Token>, String>>()?,
            )),
            EthType::Bytes => Ok(Token::Bytes(value.as_bytes().to_vec())),
            EthType::FixedBytes => Ok(Token::FixedBytes(value.as_bytes().to_vec())),
            EthType::Struct(types) => Ok(Token::Tuple(
                value
                    .split(",")
                    .zip(types)
                    .map(|e| -> Result<Token, String> { Self::parse_constructor_arg(e.1, e.0) })
                    .collect::<Result<Vec<Token>, String>>()?,
            )),
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use ethers::abi::{self, Address};
    use EthType;

    #[test]
    fn base() {
        println!("{}", serde_json::to_string(&EthType::Address).unwrap());
        println!(
            "{}",
            serde_json::to_string(&EthType::Array(Box::new(EthType::Address))).unwrap()
        );
        println!(
            "{}",
            serde_json::to_string(&EthType::Struct(vec![
                EthType::Address,
                EthType::Uint,
                EthType::String
            ]))
            .unwrap()
        )
    }
}
