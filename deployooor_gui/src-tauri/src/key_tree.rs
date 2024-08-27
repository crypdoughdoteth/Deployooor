use deployooor_core::{database::{Database, KeyMetadata}, keys::Keys};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::RwLock,
};
use tauri::State;

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
pub fn create_key(
    mut path: String,
    nickname: &str,
    password: &str,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // create keystore

    Keys::new(Path::new(&path), &nickname, &password).map_err(|e| e.to_string())?;

    path.push_str(format!("/{}", &nickname).as_str());
    Database::default()
        .record_key_metadata(Path::new(&path), &nickname)
        .map_err(|e| e.to_string())?;

    state
        .map
        .write()
        .unwrap()
        .insert(nickname.to_string(), PathBuf::from(path));
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
        let pk = Keys::decrypt_to_string(&state, password).map_err(|e| e.to_string())?;
        Ok(Account {
            name: name.to_string(),
            path: state.to_owned(),
            pk,
        })
    } else {
        Err("Failed to retrieve state")?
    }
}

pub fn load_keys_to_state() -> Result<HashMap<String, PathBuf>, String> {
    Ok(Database::default()
        .get_all_keys_metadata()
        .map_err(|e| e.to_string())?
        .into_iter()
        .fold(HashMap::new(), |mut acc: HashMap<String, PathBuf>, x: KeyMetadata| {
            acc.insert(x.name, PathBuf::from(x.path));
            acc
        }))
}

// Remove key from tree and database

#[cfg(test)]
pub mod tests {
    use std::path::Path;

    use deployooor_core::keys::Keys;

    #[test]
    pub fn key_gen() {
        Keys::new(Path::new("./"), "testing", "123").unwrap()
    }
}
