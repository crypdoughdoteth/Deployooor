use alloy::{
    signers::{k256::ecdsa, local::LocalSignerError},
    transports::{RpcError, TransportErrorKind},
};
use thiserror::Error;
use tokio::task::JoinError;
use url::ParseError;
use vyper_rs::vyper_errors::VyperErrors;

#[derive(Error, Debug)]
pub enum Errors {
    #[error(transparent)]
    ProviderParseError(#[from] ParseError),
    #[error("No contract address")]
    MissingContractAddress,
    #[error(transparent)]
    LocalSignerError(#[from] LocalSignerError),
    #[error(transparent)]
    EcdsaError(#[from] ecdsa::Error),
    #[error("Could not locate key in state")]
    KeyNotFound,
    #[error("Incorrect type of settings for this compiler")]
    SettingsMismatch,
    #[error("This contract type is not supported yet. Please open an issue or send the devs some money :3.")]
    NotSupportedYet,
    #[error("Failed to cast OsStr to str")]
    ExtensionConversionError,
    #[error("Missing extension for file")]
    MissingExtension,
    #[error(transparent)]
    HexError(#[from] hex::FromHexError),
    #[error("no arguments found for this contract")]
    NoArguments,
    #[error(transparent)]
    AbiParsingError(#[from] alloy::dyn_abi::Error),
    #[error(transparent)]
    VyperError(#[from] VyperErrors),
    #[error(transparent)]
    RpcTransportError(#[from] RpcError<TransportErrorKind>),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseJsonError(#[from] serde_json::Error),
    #[error("Solc Error: {0}")]
    SolcError(String),
    #[error("Path does not exist")]
    FsError,
    #[error(transparent)]
    JoinError(#[from] JoinError),
    #[error(transparent)]
    RusqliteError(#[from] rusqlite::Error),
}
