use crate::models::reports::{
    generate_debt_status_report, generate_payment_history_report,
    DebtStatusReport, PaymentHistoryReport,
};
use crate::security::config::load_config;
use crate::security::password::derive_encryption_key;
use crate::db::connection::open_encrypted_db;
use std::path::PathBuf;

#[tauri::command]
pub fn get_debt_status_report_cmd(
    password: String,
    include_inactive: bool,
) -> Result<DebtStatusReport, String> {
    let conn = get_authenticated_connection(&password)?;

    generate_debt_status_report(&conn, include_inactive)
        .map_err(|e| format!("Failed to generate debt status report: {}", e))
}

#[tauri::command]
pub fn get_payment_history_report_cmd(
    password: String,
    start_date: String,
    end_date: String,
) -> Result<PaymentHistoryReport, String> {
    let conn = get_authenticated_connection(&password)?;

    generate_payment_history_report(&conn, &start_date, &end_date)
        .map_err(|e| format!("Failed to generate payment history report: {}", e))
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
