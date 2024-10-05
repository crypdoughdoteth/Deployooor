use crate::{database::KeyMetadata, errors::Errors};
use alloy::{primitives::{Address, U256}, providers::{Provider, ProviderBuilder},signers::{
    k256::{ecdsa, elliptic_curve::SecretKey, Secp256k1},
    local::LocalSigner
}};
use rand::thread_rng;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use url::Url;
use std::{
    collections::HashMap,
    path::{Path, PathBuf}, str::FromStr,
};
#[derive(Serialize, Deserialize)]
pub struct Account {
    name: String,
    path: PathBuf,
    pk: String,
}

#[derive(Debug, Clone)]
pub struct Keys;

impl Keys {
    pub fn new(path: &Path, nickname: &str, password: &str) -> Result<(), Errors> {
        let mut rng = thread_rng();
        let pk: SecretKey<Secp256k1> = SecretKey::random(&mut rng);
        let mut rng2 = thread_rng();
        LocalSigner::encrypt_keystore(&path, &mut rng2, pk.to_bytes(), password, Some(&nickname))?;
        Ok(())
    }

    pub fn decrypt(path: &Path, password: &str) -> Result<LocalSigner<ecdsa::SigningKey>, Errors> {
        Ok(LocalSigner::decrypt_keystore(path, password)?)
    }

    pub fn decrypt_to_string(path: &Path, password: &str) -> Result<String, Errors> {
        Ok(hex::encode(
            LocalSigner::decrypt_keystore(path, password)?.to_bytes(),
        ))
    }
    pub async fn get_balance(http_endpoint: Url, key: &str) -> Result<U256, Errors> {
        let provider = ProviderBuilder::new().with_recommended_fillers().on_http(http_endpoint);
        let signer = LocalSigner::from_str(key)?;
        let bal = provider.get_balance(signer.address()).await?;
        Ok(bal)
    }

    /// Password must be the same for each Key
    /// Best used on initialization, not runtime
    /// Panics on failure to decrypt a keystore
    pub fn batch_decrypt(info: Vec<KeyMetadata>, password: &str) -> HashMap<String, String> {
        HashMap::from_par_iter(info.into_par_iter().map(|KeyMetadata { name, path }| {
            let mut path = PathBuf::from(&path);
            path.push(&name);
            (
                name,
                Keys::decrypt(&path, password)
                    .expect("Wrong password, failed to initialize keys")
                    .to_bytes()
                    .to_string(),
            )
        }))
    }
}

// Remove key from tree and database

// #[cfg(test)]
// pub mod tests {
// //
// //     #[test]
// //     pub fn key_gen() {
// //         Wallet::new_keystore("./", &mut thread_rng(), "123", Some("testing_keystore")).unwrap();
// //     }
// // }
