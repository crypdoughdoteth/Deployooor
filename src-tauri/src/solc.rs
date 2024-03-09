use std::process::Command;
use std::{io::BufReader, fs::File};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct SolcOutput {
    contracts: Contracts,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Contracts {
    #[serde(rename = "src/soliditylayout/contracts/storage.sol:Owner")]
    owner: Contract,
}

#[derive(Serialize, Deserialize, Debug)]
struct Contract {
    abi: Value, // Using serde_json::Value to handle the complexity of ABI items
    bin: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompileOutput {
    abi: String,
    bytecode: String,
}

#[tauri::command]
pub fn compile_solidity(file_path: &str, output_path: &str) -> Result<CompileOutput, String> {
    let solc_path = "/opt/homebrew/bin/solc";

    let output = Command::new(solc_path)
        .args([
            "--combined-json",
            "abi,bin,metadata",
            "--overwrite",
            file_path,
            "-o",
            output_path,
        ])
        .output()
        .map_err(|e| e.to_string())?; // Convert IO errors to String

    if !output.status.success() {
        let error_message = format!(
            "Command executed with failing error code: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(error_message); // Directly return the error message as a String
    }

    let json_file_path = format!("{}/combined.json", output_path);
    let file = File::open(&json_file_path).map_err(|e| e.to_string())?; // Convert IO errors to String
    let reader = BufReader::new(file);

    let solc_output: SolcOutput = serde_json::from_reader(reader).map_err(|e| e.to_string())?; // Convert serde_json errors to String
    let contract = solc_output.contracts.owner;

    let abi = serde_json::to_string_pretty(&contract.abi).map_err(|e| e.to_string())?; // Convert serde_json errors to String
    let bytecode = contract.bin;

    Ok(CompileOutput { abi, bytecode })

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test] // Changed to synchronous test for simplicity
    fn test_solc() {
        let file_path = "src/soliditylayout/contracts/storage.sol";
        let output_path = "src/soliditylayout/contracts";
        //let file_path = "/Users/protocolw/Public/Rustcodes/Protocoldenver/VyperDeployooor/src-tauri/src/soliditylayout/contracts/storage.sol"; // Update this path
        match compile_solidity(file_path, output_path) {
            Ok(resp) => println!("{:?}", resp),
            Err(e) => eprintln!("Compilation failed: {}", e),
        }
    }

}
