use ethers::{core::rand::thread_rng, signers::Wallet};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs::File, path::PathBuf, sync::Mutex};
use tauri::State;

use crate::DB_POOL;

pub struct AppState {
    pub tree: Mutex<BTreeMap<String, PathBuf>>,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    name: String,
    path: PathBuf,
    encrypted_json: String,
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
        .inner()
        .tree
        .lock()
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

#[tauri::command]
pub fn list_keys(state: State<AppState>) -> Result<Vec<Account>, String> {
    state
        .inner()
        .tree
        .lock()
        .unwrap()
        .iter()
        .map(|e| {
            if let Ok(file) = File::open(e.1) {
                if let Ok(json_keystore) = serde_json::from_reader(file) {
                    Ok(Account {
                        name: e.0.to_string(),
                        path: e.1.to_path_buf(),
                        encrypted_json: json_keystore,
                    })
                } else {
                    Err("Failed to get JSON keystore".to_string())
                }
            } else {
                Err("Failed to open file".to_string())
            }
        })
        .collect::<Result<Vec<Account>, String>>()?;
    todo!()
}

#[tauri::command]
pub fn get_key_by_name(state: State<AppState>, name: &str) -> Result<Option<Account>, String> {
    Ok(state.inner().tree.lock().unwrap().get(name).map(|e| {
        if let Ok(file) = File::open(e) {
            if let Ok(keystore) = serde_json::from_reader(&file) {
                Ok(Account {
                    name: name.to_string(),
                    path: e.to_owned(),
                    encrypted_json: keystore,
                })
            } else {
                Err("Failed to get JSON keystore".to_string())
            }
        } else {
            Err("Failed to open file".to_string())
        }
    }).transpose()?)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keys {
    name: String,
    path: String,
}

pub async fn load_keys_to_state() -> Result<BTreeMap<String, PathBuf>, String> {
    Ok(sqlx::query_as!(Keys, "SELECT name, path FROM Keys")
        .fetch_all(DB_POOL.get().unwrap())
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .fold(BTreeMap::new(), |mut acc, x| {
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
