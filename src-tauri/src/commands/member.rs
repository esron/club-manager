use crate::models::member::{Member, create_member, get_members, get_all_members, get_member_by_id, get_member_by_name, update_member_active, update_member_name};
use crate::security::config::load_config;
use crate::security::password::{derive_encryption_key, decrypt_master_key};
use crate::db::connection::open_encrypted_db;
use std::path::PathBuf;
use zeroize::Zeroizing;

#[tauri::command]
pub fn add_member_cmd(password: String, name: String, start_date: String) -> Result<i64, String> {
    let conn = get_authenticated_connection(&password)?;

    // Check if member with this name already exists
    if get_member_by_name(&conn, &name).is_ok() {
        return Err("Já existe um membro com este nome".to_string());
    }

    create_member(&conn, &name, &start_date)
        .map_err(|e| format!("Failed to create member: {}", e))
}

#[tauri::command]
pub fn update_member_active_cmd(password: String, id: i64, active: bool) -> Result<(), String> {
    let conn = get_authenticated_connection(&password)?;
    update_member_active(&conn, id, active)
        .map_err(|e| format!("Failed to update member: {}", e))
}

#[tauri::command]
pub fn update_member_name_cmd(password: String, id: i64, name: String) -> Result<(), String> {
    let conn = get_authenticated_connection(&password)?;

    // Check if another member with this name already exists
    if let Ok(existing) = get_member_by_name(&conn, &name) {
        if existing.id != id {
            return Err("Já existe um membro com este nome".to_string());
        }
    }

    update_member_name(&conn, id, &name)
        .map_err(|e| format!("Failed to update member: {}", e))
}

#[tauri::command]
pub fn get_members_cmd(password: String) -> Result<Vec<Member>, String> {
    let conn = get_authenticated_connection(&password)?;
    get_members(&conn)
        .map_err(|e| format!("Failed to get members: {}", e))
}

#[tauri::command]
pub fn get_all_members_cmd(password: String) -> Result<Vec<Member>, String> {
    let conn = get_authenticated_connection(&password)?;
    get_all_members(&conn)
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

    // Get the correct encryption key (master key if migrated, password-derived if not)
    let key_hex = if let Some(encrypted_master_key) = &config.master_key_encrypted {
        // Post-migration: decrypt master key
        let master_key = decrypt_master_key(encrypted_master_key, password, &config.salt)
            .map_err(|e| format!("Failed to decrypt master key: {}", e))?;
        Zeroizing::new(hex::encode(&*master_key))
    } else {
        // Pre-migration: derive key from password
        let key_bytes = derive_encryption_key(password, &config.salt)
            .map_err(|e| format!("Failed to derive key: {}", e))?;
        Zeroizing::new(hex::encode(&*key_bytes))
    };

    let db_path = get_db_path();
    open_encrypted_db(&db_path, &*key_hex)
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
