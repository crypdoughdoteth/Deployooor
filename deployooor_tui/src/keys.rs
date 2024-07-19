use crate::{database::DB_POOL, errors::Errors, App};
use alloy::{
    network::EthereumWallet,
    signers::{
        k256::{elliptic_curve::SecretKey, Secp256k1},
        local::LocalSigner,
    },
};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
};

#[derive(Serialize, Deserialize)]
pub struct Account {
    name: String,
    path: PathBuf,
    pk: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keys {
    name: String,
    path: String,
}

impl<'a> App<'a> {
    pub async fn create_key(
        &mut self,
        mut path: PathBuf,
        nickname: &str,
        password: &str,
    ) -> Result<(), Errors> {
        let mut rng = thread_rng();
        let pk: SecretKey<Secp256k1> = SecretKey::random(&mut rng);
        let mut rng2 = thread_rng();
        LocalSigner::encrypt_keystore(&path, &mut rng2, pk.to_bytes(), password, Some(&nickname))?;
        path.push(&nickname);
        let p_str = path.to_string_lossy().to_string();
        sqlx::query!(
            "INSERT INTO keys (name, path) VALUES ($1, $2)",
            nickname,
            p_str,
        )
        .execute(DB_POOL.get().unwrap())
        .await?;
//        self.keys
//            .insert(nickname.to_string(), PathBuf::from(path.clone()));
        Ok(())
    }

    pub async fn decrypt(&self, name: &str, password: &str) -> Result<EthereumWallet, Errors> {
//        let path = self.keys.get(name).ok_or_else(|| Errors::KeyNotFound)?;
//        Ok(EthereumWallet::from(LocalSigner::decrypt_keystore(
//            path, password,
//        )?))
          todo!()
    }

    pub async fn load_keys_to_state() -> Result<HashMap<String, PathBuf>, Errors> {
        Ok(sqlx::query_as!(Keys, "SELECT name, path FROM Keys")
            .fetch_all(DB_POOL.get().unwrap())
            .await?
            .into_iter()
            .fold(HashMap::new(), |mut acc, x| {
                acc.insert(x.name, PathBuf::from(x.path));
                acc
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
