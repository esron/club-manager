use crate::models::payment::{Payment, create_payment, get_payments, get_payment_by_member_month, delete_payment};
use crate::security::config::load_config;
use crate::security::password::{derive_encryption_key, decrypt_master_key};
use crate::db::connection::open_encrypted_db;
use std::path::PathBuf;
use zeroize::Zeroizing;

#[tauri::command]
pub fn add_payment_cmd(
    password: String,
    member_id: i64,
    month: i32,
    year: i32,
    amount_brl: f64,
    payment_date: String,
) -> Result<i64, String> {
    let conn = get_authenticated_connection(&password)?;

    // Check if payment already exists for this member/month/year
    if get_payment_by_member_month(&conn, member_id, month, year).is_ok() {
        return Err("Pagamento já existe para este membro neste mês".to_string());
    }

    create_payment(&conn, member_id, month, year, amount_brl, &payment_date)
        .map_err(|e| format!("Failed to create payment: {}", e))
}

#[tauri::command]
pub fn get_payments_cmd(password: String) -> Result<Vec<Payment>, String> {
    let conn = get_authenticated_connection(&password)?;
    get_payments(&conn)
        .map_err(|e| format!("Failed to get payments: {}", e))
}

#[tauri::command]
pub fn delete_payment_cmd(password: String, id: i64) -> Result<(), String> {
    let conn = get_authenticated_connection(&password)?;
    delete_payment(&conn, id)
        .map_err(|e| format!("Failed to delete payment: {}", e))
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
