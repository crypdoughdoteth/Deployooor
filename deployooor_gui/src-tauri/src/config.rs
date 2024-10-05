use deployooor_core::config::{Config, NetworkSettings};

#[tauri::command]
pub async fn set_config(network_settings: Vec<NetworkSettings>) -> Result<Config, String> {
    let config = Config::new(network_settings);
    config
        .set_config()
        .map_err(|e| e.to_string())
        .map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
pub async fn add_to_config(network_settings: NetworkSettings) -> Result<Config, String> {
    let mut config = Config::from_default_file().map_err(|e| e.to_string())?;
    config.push(network_settings);
    config
        .set_config()
        .map_err(|e| e.to_string())
        .map_err(|e| e.to_string())?;
    Ok(config)
}

#[tauri::command]
pub async fn get_config() -> Result<Config, String> {
    Ok(Config::from_default_file().map_err(|e| e.to_string())?)
}
