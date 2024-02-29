use std::process::Command;

#[tauri::command]
fn deploy_contract(path: String, pass_path: String) -> Result<(), String> {
    let output = Command::new("cargo")
        .arg("stylus")
        .arg("deploy")
        .arg("--keystore-path")
        .arg(path)
        .arg("--keystore-password-path")
        .arg(pass_path)
        .output()
        .map_err(|e| e.to_string())?;

    Ok(())
}

// fn estimate_gas() {
//
// }

// pk file path (keystore only)
// set cwd to the root of the project
