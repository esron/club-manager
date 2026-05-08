use crate::models::charts::{generate_chart_data, ChartData};
use crate::security::config::load_config;
use crate::security::password::decrypt_master_key;
use crate::db::connection::open_encrypted_db;
use std::path::PathBuf;
use zeroize::Zeroizing;

#[tauri::command]
pub fn get_dashboard_chart_data_cmd(password: String) -> Result<ChartData, String> {
    let config_path = get_config_path();
    let config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    // Decrypt master key with password
    let encrypted_master_key = config.master_key_encrypted
        .ok_or("Master key not found in config".to_string())?;

    let master_key = decrypt_master_key(&encrypted_master_key, &password, &config.salt)
        .map_err(|e| format!("Failed to decrypt master key: {}", e))?;

    let master_key_hex = Zeroizing::new(hex::encode(&*master_key));

    let db_path = get_db_path();
    let conn = open_encrypted_db(&db_path, &master_key_hex)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    generate_chart_data(&conn)
        .map_err(|e| format!("Failed to generate chart data: {}", e))
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("GestorDoClube");
    std::fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}

fn get_db_path() -> PathBuf {
    let mut path = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("GestorDoClube");
    std::fs::create_dir_all(&path).ok();
    path.push("club.db");
    path
}
