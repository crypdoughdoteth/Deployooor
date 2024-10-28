use crate::errors::Errors;
use alloy::{
    dyn_abi::DynSolType,
    network::{Ethereum, EthereumWallet, TransactionBuilder},
    primitives::Address,
    providers::{
        fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller},
        Identity, Provider, ProviderBuilder, RootProvider,
    },
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    transports::http::{Client, Http},
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::path::Path;
use vyper_rs::{venv::Venv, vyper::Evm};

#[derive(Debug, Clone)]
pub struct Initcode(pub(crate) Vec<u8>);

impl Initcode {
    #[inline]
    pub fn extend_from_slice(&mut self, bytes: &[u8]) {
        self.0.extend_from_slice(bytes);
    }
    #[inline]
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

#[derive(Debug)]
pub struct VyperSettings<'a> {
    vyper_version: &'a str,
    evm_version: Evm,
}

pub enum CompilerSettings<'a> {
    Vyper(VyperSettings<'a>),
    //    Solc(SolcSettings),
}

#[derive(Debug)]
pub struct CompilerOutput {
    pub abi: String,
    pub bytecode: Initcode,
}

#[derive(Debug)]
pub enum Contract<'a> {
    Solidity(&'a Path),
    Vyper(&'a Path, Option<VyperSettings<'a>>),
    //    Stylus(&'a Path),
}

impl<'a> Contract<'a> {
    pub fn from_str(s: &'a str) -> Result<Self, Errors> {
        match Path::new(s)
            .extension()
            .ok_or_else(|| Errors::MissingExtension)?
            .to_str()
            .ok_or_else(|| Errors::ExtensionConversionError)?
        {
            "sol" => Ok(Contract::Solidity(Path::new(s))),
            "vy" => Ok(Contract::Vyper(Path::new(s), None)),
            //            ("vy", Some(_)) => Err(Errors::SettingsMismatch),
            //            ("rs", None) | ("c", None) | ("cpp", None) | ("zig", None) => {
            //                Ok(Contract::Stylus(Path::new(path)))
            //           }
            _ => Err(Errors::NotSupportedYet),
        }
    }

    pub fn with_settings(
        path: &'a str,
        compiler_settings: CompilerSettings<'a>,
    ) -> Result<Self, Errors> {
        match (
            Path::new(path)
                .extension()
                .ok_or_else(|| Errors::MissingExtension)?
                .to_str()
                .ok_or_else(|| Errors::ExtensionConversionError)?,
            compiler_settings,
        ) {
            ("sol", _) => Ok(Contract::Solidity(Path::new(path))),
            ("vy", CompilerSettings::Vyper(s)) => Ok(Contract::Vyper(Path::new(path), Some(s))),
            //            ("vy", Some(_)) => Err(Errors::SettingsMismatch),
            //            ("rs", None) | ("c", None) | ("cpp", None) | ("zig", None) => {
            //                Ok(Contract::Stylus(Path::new(path)))
            //           }
            _ => Err(Errors::NotSupportedYet),
        }
    }

    #[inline]
    pub fn inner(&self) -> &'a Path {
        match self {
            Contract::Solidity(p) => p,
            Contract::Vyper(p, _) => p,
            //            Contract::Stylus(p) => p,
        }
    }

    #[inline]
    fn compile_vyper(&self) -> Result<CompilerOutput, Errors> {
        let settings = &VyperSettings {
            vyper_version: "0.4.0",
            evm_version: Evm::Cancun,
        };
        let mut contract = Venv::default()
            .init()?
            .ivyper_venv(Some(settings.vyper_version))?
            .vyper(self.inner());
        contract.compile_ver(&settings.evm_version)?;
        let abi = contract.get_abi()?.to_string();
        Ok(CompilerOutput {
            abi,
            bytecode: Initcode(hex::decode(
                contract.bytecode.unwrap().strip_prefix("0x").unwrap(),
            )?),
        })
    }

    #[inline]
    fn compile_vyper_settings(&self, settings: &VyperSettings) -> Result<CompilerOutput, Errors> {
        let mut contract = Venv::default()
            .init()?
            .ivyper_venv(Some(settings.vyper_version))?
            .vyper(self.inner());
        contract.compile_ver(&settings.evm_version)?;
        let abi = contract.get_abi()?.to_string();
        Ok(CompilerOutput {
            abi,
            bytecode: Initcode(hex::decode(
                contract.bytecode.unwrap().strip_prefix("0x").unwrap(),
            )?),
        })
    }

    async fn compile(&self) -> Result<CompilerOutput, Errors> {
        match self {
            Contract::Solidity(_) => Ok(self.compile_solidity().await?),
            Contract::Vyper(_, None) => Ok(self.compile_vyper()?),
            Contract::Vyper(_, Some(s)) => Ok(self.compile_vyper_settings(s)?),
            //            Contract::Stylus(_) => todo!(),
        }
    }

    // create a version of the above with EVM version to target
}

pub type SignerProvider = FillProvider<
    JoinFill<
        JoinFill<JoinFill<JoinFill<Identity, GasFiller>, NonceFiller>, ChainIdFiller>,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
>;

#[derive(Debug)]
pub struct Deploy<'a> {
    provider: SignerProvider,
    contract: Contract<'a>,
    args: Option<ConstructorArguments>,
}

// the the abi-encoded arguments are appended to the initcode
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstructorArguments {
    pub sol_type: String,
    pub values: String,
}

impl ConstructorArguments {
    pub fn new(types: &[String], values: &[String]) -> Self {
        let (sol_types, values) = Itertools::intersperse(
            types.iter().zip(values),
            (&",".to_string(), &",".to_string()),
        )
        .fold(
            (String::new(), String::new()),
            |(mut val, mut ty), (t, v)| {
                ty.push_str(t);
                val.push_str(v);
                (ty, val)
            },
        );

        ConstructorArguments {
            sol_type: format!("({})", sol_types),
            values: format!("({})", values),
        }
    }
}

impl<'a> Deploy<'a> {
    pub fn init(
        provider: &'a str,
        key: &'a str,
        contract_path: &'a str,
        args: Option<ConstructorArguments>,
    ) -> Result<Self, Errors> {
        let signer: PrivateKeySigner = key.parse()?;
        let wallet = EthereumWallet::from(signer);
        let rpc_url = provider.parse()?;
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);

        let contract = Contract::from_str(contract_path)?;

        Ok(Deploy {
            provider,
            contract,
            args,
        })
    }

    #[inline]
    async fn get_deployment_bytecode_and_abi(&self) -> Result<CompilerOutput, Errors> {
        let mut bin = self.contract.compile().await?;
        if let Some(ConstructorArguments { sol_type, values }) = &self.args {
            let ty = DynSolType::parse(sol_type)?;
            let sol_values = ty.coerce_str(values)?;
            let bytes = sol_values.abi_encode_params();
            bin.bytecode.extend_from_slice(&bytes);
        }
        Ok(bin)
    }

    #[inline]
    pub async fn deploy(&self) -> Result<Address, Errors> {
        let deployment_info = self.get_deployment_bytecode_and_abi().await?;
        let tx =
            TransactionRequest::default().with_deploy_code(deployment_info.bytecode.into_inner());
        // Deploy the contract.
        let receipt = self
            .provider
            .send_transaction(tx)
            .await?
            .get_receipt()
            .await?;

        println!("{receipt:?}");

        Ok(receipt
            .contract_address
            .ok_or_else(|| Errors::MissingContractAddress)?)
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use alloy::{
        network::EthereumWallet, node_bindings::Anvil, providers::ProviderBuilder,
        signers::local::PrivateKeySigner,
    };

    #[tokio::test]
    async fn deploy_solidity() {
        let anvil = Anvil::new().try_spawn().unwrap();

        // Set up signer from the first default Anvil account (Alice).
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer);

        let rpc_url = anvil.endpoint().parse().unwrap();
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);

        let contract =
            Contract::from_str("/Users/crypdoughdoteth/dev/active/VyperDeployooor/deployooor_gui/src-tauri/src/soliditylayout/contracts/storage.sol").unwrap();

        let deployer = Deploy {
            provider,
            contract,
            args: None,
        };

        let res = deployer.deploy().await.unwrap();
        println!("{}", res);
    }

    #[tokio::test]
    async fn deploy_vyper() {
        let anvil = Anvil::new().try_spawn().unwrap();

        // Set up signer from the first default Anvil account (Alice).
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer);

        let rpc_url = anvil.endpoint().parse().unwrap();
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);

        let contract = Contract::from_str("./multisig.vy").unwrap();
        let types = "(address[])".to_string();
        let c_args = "([0x940ACd9375b46EC2FA7C0E8aAd9D7241fb01e205])".to_string();
        let args = ConstructorArguments {
            sol_type: types,
            values: c_args,
        };
        let deployer = Deploy {
            provider,
            contract,
            args: Some(args),
        };

        let res = deployer.deploy().await.unwrap();
        println!("{}", res);
    }

    #[tokio::test]
    async fn deploy_vyper_ver() {
        let anvil = Anvil::new().try_spawn().unwrap();
        // Set up signer from the first default Anvil account (Alice).
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer);
        let rpc_url = anvil.endpoint().parse().unwrap();
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);
        let settings = VyperSettings {
            vyper_version: "0.4.0",
            evm_version: Evm::Cancun,
        };
        let contract =
            Contract::with_settings("./multisig.vy", CompilerSettings::Vyper(settings)).unwrap();
        let types = "(address[])";
        let c_args = "([0x940ACd9375b46EC2FA7C0E8aAd9D7241fb01e205])";
        let args = ConstructorArguments {
            sol_type: types.to_string(),
            values: c_args.to_string(),
        };
        let deployer = Deploy {
            provider,
            contract,
            args: Some(args),
        };

        let res = deployer.deploy().await.unwrap();
        println!("{}", res);
    }
}
