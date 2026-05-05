use crate::security::config::load_config;
use crate::security::password::derive_encryption_key;
use crate::db::connection::open_encrypted_db;
use crate::db::schema::initialize_schema;
use std::path::PathBuf;

#[tauri::command]
pub fn check_database_initialized(password: String) -> Result<bool, String> {
    let config_path = get_config_path();
    let config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    let key_bytes = derive_encryption_key(&password, &config.salt)
        .map_err(|e| format!("Failed to derive key: {}", e))?;
    let key_hex = hex::encode(&key_bytes);

    let db_path = get_db_path();
    let conn = open_encrypted_db(&db_path, &key_hex)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // Check if members table exists
    let table_exists: Result<i64, rusqlite::Error> = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='members'",
        [],
        |row| row.get(0)
    );

    match table_exists {
        Ok(count) => Ok(count > 0),
        Err(e) => Err(format!("Failed to check database: {}", e))
    }
}

#[tauri::command]
pub fn initialize_database(password: String) -> Result<(), String> {
    let config_path = get_config_path();
    let config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    let key_bytes = derive_encryption_key(&password, &config.salt)
        .map_err(|e| format!("Failed to derive key: {}", e))?;
    let key_hex = hex::encode(&key_bytes);

    let db_path = get_db_path();
    let conn = open_encrypted_db(&db_path, &key_hex)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    initialize_schema(&conn)
        .map_err(|e| format!("Failed to initialize schema: {}", e))?;

    Ok(())
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
