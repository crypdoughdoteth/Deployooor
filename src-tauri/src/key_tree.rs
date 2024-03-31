use ethers::{core::rand::thread_rng, signers::Wallet, utils::hex::ToHexExt};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::RwLock};
use tauri::State;

use crate::DB_POOL;

pub struct AppState {
    pub map: RwLock<HashMap<String, PathBuf>>,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    name: String,
    path: PathBuf,
    pk: String,
}

#[tauri::command]
pub async fn create_key(
    mut path: String,
    nickname: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // create keystore
    Wallet::new_keystore(&path, &mut thread_rng(), password, Some(&nickname))
        .map_err(|e| e.to_string())?;
    path.push_str(format!("/{}", &nickname).as_str());
    state
        .map
        .write()
        .unwrap()
        .insert(nickname.clone(), PathBuf::from(path.clone()));
    sqlx::query!(
        "INSERT INTO keys (name, path) VALUES ($1, $2)",
        nickname,
        path,
    )
    .execute(DB_POOL.get().unwrap())
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountNames {
    name: String,
}

#[tauri::command]
pub fn list_keys(state: State<AppState>) -> Vec<AccountNames> {
    state
        .map
        .read()
        .unwrap()
        .iter()
        .map(|e| AccountNames {
            name: e.0.to_owned(),
        })
        .collect::<Vec<AccountNames>>()
}

#[tauri::command]
pub fn get_key_by_name(
    state: State<AppState>,
    name: &str,
    password: &str,
) -> Result<Account, String> {
    let state = state.inner().map.read().unwrap();
    if let Some(state) = state.get(name) {
        let wallet = Wallet::decrypt_keystore(&state, password).map_err(|e| e.to_string())?;
        let pk = wallet.signer().to_bytes().encode_hex();
        println!("{pk}");
        Ok(Account {
            name: name.to_string(),
            path: state.to_owned(),
            pk,
        })
    } else {
        Err("Failed to retrieve state")?
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keys {
    name: String,
    path: String,
}

pub async fn load_keys_to_state() -> Result<HashMap<String, PathBuf>, String> {
    Ok(sqlx::query_as!(Keys, "SELECT name, path FROM Keys")
        .fetch_all(DB_POOL.get().unwrap())
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .fold(HashMap::new(), |mut acc, x| {
            acc.insert(x.name, PathBuf::from(x.path));
            acc
        }))
}

// Remove key from tree and database

#[cfg(test)]
pub mod tests {
    use ethers::{core::rand::thread_rng, signers::Wallet};

    #[test]
    pub fn key_gen() {
        Wallet::new_keystore("./", &mut thread_rng(), "123", Some("testing_keystore")).unwrap();
    }
}
