use crate::models::settings::{get_setting, update_setting};
use crate::security::config::load_config;
use crate::security::password::derive_encryption_key;
use crate::db::connection::open_encrypted_db;
use std::path::PathBuf;

const VALID_SETTING_KEYS: &[&str] = &["minimum_fee_brl"];
const MAX_FEE_BRL: f64 = 9999.99;

#[tauri::command]
pub fn get_setting_cmd(password: String, key: String) -> Result<String, String> {
    let conn = get_authenticated_connection(&password)?;
    get_setting(&conn, &key)
        .map_err(|e| format!("Failed to get setting: {}", e))
}

#[tauri::command]
pub fn update_setting_cmd(password: String, key: String, value: String) -> Result<(), String> {
    if !VALID_SETTING_KEYS.contains(&key.as_str()) {
        return Err(format!("Configuração desconhecida: {}", key));
    }

    // Validate based on key
    let validated_value = if key == "minimum_fee_brl" {
        validate_minimum_fee(&value)?
    } else {
        value
    };

    let conn = get_authenticated_connection(&password)?;
    update_setting(&conn, &key, &validated_value)
        .map_err(|e| format!("Failed to update setting: {}", e))
}

fn validate_minimum_fee(value: &str) -> Result<String, String> {
    let amount: f64 = value.parse()
        .map_err(|_| "Valor inválido".to_string())?;

    if !amount.is_finite() || amount <= 0.0 {
        return Err("Valor deve ser maior que zero".to_string());
    }

    if amount > MAX_FEE_BRL {
        return Err(format!("Valor máximo: R$ {:.2}", MAX_FEE_BRL));
    }

    // Check decimal places on original input
    let parts: Vec<&str> = value.split('.').collect();
    if parts.len() == 2 && parts[1].len() > 2 {
        return Err("Máximo 2 casas decimais".to_string());
    }

    // Return normalized value
    Ok(format!("{:.2}", amount))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_minimum_fee_valid_cases() {
        assert!(validate_minimum_fee("15.00").is_ok());
        assert!(validate_minimum_fee("0.01").is_ok());
        assert!(validate_minimum_fee("9999.99").is_ok());
        assert!(validate_minimum_fee("1").is_ok());
        assert!(validate_minimum_fee("100").is_ok());
    }

    #[test]
    fn test_validate_minimum_fee_rejects_negative() {
        let result = validate_minimum_fee("-10");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Valor deve ser maior que zero");
    }

    #[test]
    fn test_validate_minimum_fee_rejects_zero() {
        let result = validate_minimum_fee("0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Valor deve ser maior que zero");
    }

    #[test]
    fn test_validate_minimum_fee_rejects_too_large() {
        let result = validate_minimum_fee("10000");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("máximo"));
    }

    #[test]
    fn test_validate_minimum_fee_rejects_too_many_decimals() {
        let result = validate_minimum_fee("15.999");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Máximo 2 casas decimais");
    }

    #[test]
    fn test_validate_minimum_fee_rejects_invalid_format() {
        let result = validate_minimum_fee("abc");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Valor inválido");
    }
}
