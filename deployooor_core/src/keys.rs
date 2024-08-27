use std::path::{Path, PathBuf};

use crate::errors::Errors;
use alloy::signers::{
    k256::{ecdsa, elliptic_curve::SecretKey, Secp256k1},
    local::LocalSigner,
};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
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
