use ethers::{
    abi::{Address, Int, Token, Uint},
    contract::ContractFactory,
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::Bytes,
    utils::hex::FromHex,
};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};

#[derive(Serialize, Deserialize, Clone)]
pub struct ContractDeployment<'a> {
    provider: &'a str,
    args: Box<[ConstructorArg]>,
    abi: &'a str,
    initcode: &'a str,
    private_key: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstructorArg {
    ty: EthType,
    value: Box<[String]>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum EthType {
    Address,
    String,
    Uint,
    Int,
    Array(Box<EthType>),
    FixedArray(Box<EthType>),
    Bytes,
    FixedBytes,
    Struct(Box<[EthType]>),
    Bool,
}

// This method will work for any regular contract that needs to be deployed to an EVM environment
// from vyper, solidity etc
#[tauri::command]
pub async fn deploy_contract(contract: ContractDeployment<'_>) -> Result<(), String> {
    let provider = Provider::<Http>::try_from(contract.provider)
        .map_err(|e| e.to_string())?
        .interval(Duration::from_millis(10u64));
    let chain_id = &provider.get_chainid().await.map_err(|e| e.to_string())?;
    let local_wallet: LocalWallet = contract
        .private_key
        .parse::<LocalWallet>()
        .map_err(|e| e.to_string())?
        .with_chain_id(chain_id.as_u64());

    let client = Arc::new(SignerMiddleware::new(provider, local_wallet));
    let abi = ethers::abi::Contract::load(contract.abi.as_bytes()).map_err(|e| e.to_string())?;
    let initcode = Bytes::from_hex(contract.initcode).map_err(|e| e.to_string())?;
    let factory = ContractFactory::new(abi, initcode, client);

    let _res = match &contract.args.len() {
        0 => factory
            .deploy(())
            .map_err(|e| e.to_string())?
            .send_with_receipt()
            .await
            .map_err(|e| e.to_string())?,
        _ => factory
            .deploy_tokens(contract.parse_constructor_args()?)
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
            .map(|e| ConstructorArg::parse_constructor_arg(&e.ty, &e.value))
            .collect::<Result<Vec<Token>, String>>()?)
    }
}

impl ConstructorArg {
    // Arrays and Structs require an inner type since these types are collections of sorts.
    // Structs are heterogeneous types that are encoded as tuples in the Ethereum ABI.
    // For each field of the struct, include the corresponding EthType
    // Arrays will have their type specified as an inner field, values should be comma seperated strings
    fn parse_constructor_arg(abi_type: &EthType, value: &[String]) -> Result<Token, String> {
        match abi_type {
            EthType::Address => Ok(Token::Address(
                value
                    .first()
                    .ok_or_else(|| "Error: missing argument".to_owned())?
                    .parse::<Address>()
                    .map_err(|e| e.to_string())?,
            )),
            EthType::String => Ok(Token::String(
                value
                    .first()
                    .ok_or_else(|| "Error: missing argument".to_owned())?
                    .to_owned(),
            )),
            EthType::Uint => Ok(Token::Uint(
                value
                    .first()
                    .ok_or_else(|| "Error: missing argument".to_owned())?
                    .parse::<Uint>()
                    .map_err(|e| e.to_string())?,
            )),
            EthType::Int => Ok(Token::Int(
                value
                    .first()
                    .ok_or_else(|| "Error: missing argument".to_owned())?
                    .parse::<Int>()
                    .map_err(|e| e.to_string())?,
            )),
            EthType::Bool => Ok(Token::Bool(
                value
                    .first()
                    .ok_or_else(|| "Error: missing argument".to_owned())?
                    .parse::<bool>()
                    .map_err(|e| e.to_string())?,
            )),
            EthType::FixedArray(ty) => Ok(Token::FixedArray(
                value
                    .into_iter()
                    .map(|e| -> Result<Token, String> {
                        Self::parse_constructor_arg(&*ty, &[e.to_owned()])
                    })
                    .collect::<Result<Vec<Token>, String>>()?,
            )),
            EthType::Array(ty) => Ok(Token::Array(
                value
                    .into_iter()
                    .map(|e| -> Result<Token, String> {
                        Self::parse_constructor_arg(&*ty, &[e.to_owned()])
                    })
                    .collect::<Result<Vec<Token>, String>>()?,
            )),
            EthType::Bytes => Ok(Token::Bytes(
                value
                    .first()
                    .ok_or_else(|| "Error: missing argument".to_owned())?
                    .as_bytes()
                    .to_vec(),
            )),
            EthType::FixedBytes => Ok(Token::FixedBytes(
                value
                    .first()
                    .ok_or_else(|| "Error: missing argument".to_owned())?
                    .as_bytes()
                    .to_vec(),
            )),
            EthType::Struct(types) => Ok(Token::Tuple(
                types
                    .into_iter()
                    .zip(value)
                    .map(|e| -> Result<Token, String> {
                        Self::parse_constructor_arg(e.0, &[e.1.to_owned()])
                    })
                    .collect::<Result<Vec<Token>, String>>()?,
            )),
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
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
            serde_json::to_string(&EthType::Struct(Box::new([
                EthType::Address,
                EthType::Uint,
                EthType::String
            ])))
            .unwrap()
        );
        println!(
            "{:?}",
            serde_json::to_string(&ConstructorArg {
                ty: EthType::Uint,
                value: Box::new(["1".to_owned()]),
            })
            .unwrap()
        );
        println!(
            "{:?}",
            serde_json::to_string(&ConstructorArg {
                ty: EthType::Array(Box::new(EthType::Address)),
                value: Box::new(["0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326".to_owned()]),
            })
            .unwrap()
        );
    }
}
