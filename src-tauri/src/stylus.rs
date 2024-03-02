use std::{env, fs::File, path::Path, process::Command};

use ethers::core::k256::pkcs8::der::Writer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDeployment {
    pub fee: String,
    pub deployment_address: String,
}

#[tauri::command]
pub fn stylus_deploy_contract(
    root_path: &str,
    keystore_path: &str,
    pass: &str,
) -> Result<ContractDeployment, String> {
    env::set_current_dir(Path::new(root_path)).unwrap();
    let mut pw_file = File::create("./password.txt").map_err(|e| e.to_string())?;
    pw_file.write(pass.as_bytes()).map_err(|e| e.to_string())?;

    let output = Command::new("cargo")
        .arg("stylus")
        .arg("deploy")
        .arg("--keystore-path")
        .arg(keystore_path)
        .arg("--keystore-password-path")
        .arg(pass)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        Err("Failed to calculate gas costs".to_string())?
    }

    let res = String::from_utf8_lossy(&output.stdout)
        .to_string()
        .split_once("Deploying program to address \u{1b}[38;5;48;1m")
        .unwrap()
        .1
        .to_owned();
    let (address, rest) = res.split_once("\u{1b}").unwrap();
    let gas_fee = rest
        .split_once("Transaction fee: \u{1b}[38;5;48;1m")
        .unwrap()
        .1
        .split_once("\u{1b}")
        .unwrap()
        .0;

    Ok(ContractDeployment {
        fee: gas_fee.to_string(),
        deployment_address: address.to_string(),
    })
}

#[tauri::command]
pub fn stylus_estimate_gas(
    root_path: &str,
    keystore_path: &str,
    pass: &str,
) -> Result<u128, String> {
    env::set_current_dir(Path::new(root_path)).unwrap();
    let mut pw_file = File::create("./password.txt").map_err(|e| e.to_string())?;
    pw_file.write(pass.as_bytes()).map_err(|e| e.to_string())?;
    let output = Command::new("cargo")
        .arg("stylus")
        .arg("deploy")
        .arg("--keystore-path")
        .arg(keystore_path)
        .arg("--keystore-password-path")
        .arg("./password.txt")
        .arg("--estimate-gas-only")
        .output()
        .map_err(|e| e.to_string())?;
    std::fs::remove_file("./password.txt").map_err(|e| e.to_string())?;
    if !output.status.success() {
        Err("Failed to calculate gas costs".to_string())?
    }

    let output_str: String = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(output_str
        .split_whitespace()
        .rev()
        .nth(2)
        .ok_or_else(|| "Failed to parse Cargo Stylus CLI output".to_string())?
        //split on the left
        .split_once("m")
        .ok_or_else(|| "Failed to parse Cargo Stylus CLI output".to_string())?
        .1
        //split on the right
        .split_once("\u{1b}")
        .ok_or_else(|| "Failed to parse Cargo Stylus CLI output".to_string())?
        .0
        .parse::<u128>()
        .map_err(|e| e.to_string())?)
}

// pk file path (keystore only)
// set cwd to the root of the project

#[cfg(test)]
pub mod test {
    use super::stylus_estimate_gas;
    use crate::stylus::stylus_deploy_contract;

    #[test]
    fn gas_estimation() {
        let path = "/Users/crypdoughdoteth/dev/testing/first/";
        assert!(stylus_estimate_gas(path).is_ok_and(|x| {
            println!("{}", x);
            x != 0
        }));
    }

    #[test]
    fn deploy() {
        let path = "/Users/crypdoughdoteth/dev/testing/first/";
        let res = stylus_deploy_contract(path).unwrap();

        println!("{:#?}", res);
    }
}
