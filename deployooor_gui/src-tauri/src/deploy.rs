use deployooor_core::deploy::{ConstructorArguments, Deploy};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ContractDeployment<'a> {
    provider: &'a str,
    args: Option<(Vec<String>, Vec<String>)>,
    path: &'a str,
    private_key: &'a str,
}

#[tauri::command]
pub async fn deploy_contract(contract: ContractDeployment<'_>) -> Result<String, String> {
    Ok(match contract.args {
        Some(a) => Deploy::init(
            contract.provider,
            contract.private_key,
            contract.path,
            Some(ConstructorArguments::new(&a.0, &a.1)),
        )
        .map_err(|e| e.to_string())?,
        None => Deploy::init(contract.provider, contract.private_key, contract.path, None)
            .map_err(|e| e.to_string())?,
    }
    .deploy()
    .await
    .map_err(|e| e.to_string())?
    .to_string())
}
