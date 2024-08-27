use crate::{
    deploy::{CompilerOutput, Contract, Initcode},
    errors::Errors,
};
use std::process::Command;

impl<'a> Contract<'a> {
    #[inline]
    pub(super) async fn compile_solidity(&self) -> Result<CompilerOutput, Errors> {
        let file_heap = self.inner().to_string_lossy().to_string();
        let file_copy = file_heap.clone();
        let binary: tokio::task::JoinHandle<Result<String, Errors>> = tokio::spawn(async move {
            let output = Command::new("solc").args(["--bin", &file_copy]).output()?;
            if !output.status.success() {
                let error_message = format!(
                    "Command executed with failing error code: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                Err(Errors::SolcError(error_message))?
            }

            let out = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(out.split("Binary:\n").last().unwrap().trim_end().to_string())
        });

        let abi: tokio::task::JoinHandle<Result<String, Errors>> = tokio::spawn(async move {
            let abi_output = Command::new("solc").args(["--abi", &file_heap]).output()?;

            if !abi_output.status.success() {
                let error_message = format!(
                    "Command executed with failing error code: {}",
                    String::from_utf8_lossy(&abi_output.stderr)
                );
                Err(Errors::SolcError(error_message))?
            }

            let res = String::from_utf8_lossy(&abi_output.stdout).to_string();
            Ok(res.split("ABI").last().unwrap().to_string())
        });

        let (bin, abi) = tokio::join!(binary, abi);

        Ok(CompilerOutput {
            abi: abi??,
            bytecode: Initcode(hex::decode(bin??)?),
        })
    }
}

#[cfg(test)]
mod tests {

    use std::path::Path;

    use super::*;

    #[tokio::test]
    async fn test_solc() {
        let contract =
            Contract::Solidity(Path::new("/Users/crypdoughdoteth/dev/active/VyperDeployooor/deployooor_gui/src-tauri/src/soliditylayout/contracts/storage.sol"));
        let res = contract.compile_solidity().await.unwrap();
        println!("\nBytecode: {:?}", res.bytecode);
        println!("\nABI: {:?}", res.abi);
    }
}
