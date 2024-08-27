use anyhow::Result;
use deployooor_core::database::{Database, Deployment};

#[tauri::command]
pub async fn write_deployment(deployment_data: Deployment) -> Result<(), String> {
    Database::default()
        .record_deployment(deployment_data)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_deployments(offset: usize) -> Result<Vec<Deployment>, String> {
    Ok(Database::default()
        .get_deployments(offset)
        .map_err(|e| e.to_string())?)
}
