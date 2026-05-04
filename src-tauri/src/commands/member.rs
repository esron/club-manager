use crate::models::member::{Member, create_member, get_members, get_member_by_id};
use crate::security::config::load_config;
use crate::security::password::derive_encryption_key;
use crate::db::connection::open_encrypted_db;
use std::path::PathBuf;

#[tauri::command]
pub fn add_member_cmd(password: String, name: String, start_date: String) -> Result<i64, String> {
    let conn = get_authenticated_connection(&password)?;
    create_member(&conn, &name, &start_date)
        .map_err(|e| format!("Failed to create member: {}", e))
}

#[tauri::command]
pub fn get_members_cmd(password: String) -> Result<Vec<Member>, String> {
    let conn = get_authenticated_connection(&password)?;
    get_members(&conn)
        .map_err(|e| format!("Failed to get members: {}", e))
}

#[tauri::command]
pub fn get_member_cmd(password: String, id: i64) -> Result<Member, String> {
    let conn = get_authenticated_connection(&password)?;
    get_member_by_id(&conn, id)
        .map_err(|e| format!("Failed to get member: {}", e))
}

fn get_authenticated_connection(password: &str) -> Result<rusqlite::Connection, String> {
    let config_path = get_config_path();
    let config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    let key_bytes = derive_encryption_key(password, &config.salt)
        .map_err(|e| format!("Failed to derive key: {}", e))?;
    let key_hex = hex::encode(&key_bytes);

    let db_path = get_db_path();
    open_encrypted_db(&db_path, &key_hex)
        .map_err(|e| format!("Failed to open database: {}", e))
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
