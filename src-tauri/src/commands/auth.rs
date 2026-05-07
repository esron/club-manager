use crate::security::password::{hash_password, verify_password, derive_encryption_key, encrypt_master_key, decrypt_master_key};
use crate::security::config::{AppConfig, save_config, load_config};
use crate::db::connection::open_encrypted_db;
use crate::db::schema::initialize_schema;
use std::path::PathBuf;
use chrono::Utc;

/// Check if this is the first launch (config file doesn't exist)
#[tauri::command]
pub fn check_first_launch() -> Result<bool, String> {
    let config_path = get_config_path();
    Ok(!config_path.exists())
}

/// Check if config needs migration to master key format
#[tauri::command]
pub fn needs_migration() -> Result<bool, String> {
    let config_path = get_config_path();

    if !config_path.exists() {
        return Ok(false); // First launch, no migration needed
    }

    let config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    Ok(config.master_key_encrypted.is_none())
}

/// Migrate existing database to master key encryption
#[tauri::command]
pub fn migrate_to_master_key(password: String) -> Result<(), String> {
    let config_path = get_config_path();
    let mut config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    // Check if already migrated
    if config.master_key_encrypted.is_some() {
        return Err("Already migrated".to_string());
    }

    // Verify password
    let is_valid = verify_password(&password, &config.password_hash)
        .map_err(|e| format!("Failed to verify password: {}", e))?;

    if !is_valid {
        return Err("Incorrect password".to_string());
    }

    // Derive current encryption key from password
    let current_key = derive_encryption_key(&password, &config.salt)
        .map_err(|e| format!("Failed to derive key: {}", e))?;
    let current_key_hex = hex::encode(&current_key);

    // Open database with current key
    let db_path = get_db_path();
    let conn = open_encrypted_db(&db_path, &current_key_hex)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // Generate new random master key
    let master_key: [u8; 32] = rand::random();
    let master_key_hex = hex::encode(&master_key);

    // Re-encrypt database with master key
    conn.execute(&format!("PRAGMA rekey = \"x'{}'\";", master_key_hex), [])
        .map_err(|e| format!("Failed to re-encrypt database: {}", e))?;

    // Encrypt master key with password-derived key
    let encrypted_master_key = encrypt_master_key(&master_key, &password, &config.salt)
        .map_err(|e| format!("Failed to encrypt master key: {}", e))?;

    // Update config
    config.master_key_encrypted = Some(encrypted_master_key);
    save_config(&config, &config_path)
        .map_err(|e| format!("Failed to save config: {}", e))?;

    Ok(())
}

/// Setup initial password (first launch only)
#[tauri::command]
pub fn setup_password(password: String) -> Result<(), String> {
    let config_path = get_config_path();

    // Verify this is first launch
    if config_path.exists() {
        return Err("Password already configured".to_string());
    }

    // Hash password with bcrypt
    let password_hash = hash_password(&password)
        .map_err(|e| format!("Failed to hash password: {}", e))?;

    // Generate random salt for encryption
    let salt: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect();

    // Generate random master key
    let master_key: [u8; 32] = rand::random();
    let master_key_hex = hex::encode(&master_key);

    // Encrypt master key with password
    let encrypted_master_key = encrypt_master_key(&master_key, &password, &salt)
        .map_err(|e| format!("Failed to encrypt master key: {}", e))?;

    // Create config
    let config = AppConfig {
        password_hash,
        salt: salt.clone(),
        master_key_encrypted: Some(encrypted_master_key),
        minimum_fee_brl: "15.00".to_string(),
        created_at: Utc::now().to_rfc3339(),
    };

    // Save config
    save_config(&config, &config_path)
        .map_err(|e| format!("Failed to save config: {}", e))?;

    // Create encrypted database with master key
    let db_path = get_db_path();
    let conn = open_encrypted_db(&db_path, &master_key_hex)
        .map_err(|e| format!("Failed to create database: {}", e))?;

    // Initialize schema
    initialize_schema(&conn)
        .map_err(|e| format!("Failed to initialize schema: {}", e))?;

    Ok(())
}

/// Verify password and return success
#[tauri::command]
pub fn verify_password_cmd(password: String) -> Result<bool, String> {
    let config_path = get_config_path();

    // Load config
    let config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    // Verify password
    let is_valid = verify_password(&password, &config.password_hash)
        .map_err(|e| format!("Failed to verify password: {}", e))?;

    if !is_valid {
        return Ok(false);
    }

    // Decrypt master key
    let encrypted_master_key = config.master_key_encrypted
        .ok_or("Master key not found in config".to_string())?;

    let master_key = decrypt_master_key(&encrypted_master_key, &password, &config.salt)
        .map_err(|e| format!("Failed to decrypt master key: {}", e))?;

    let master_key_hex = hex::encode(&master_key);

    // Test database connection with master key
    let db_path = get_db_path();
    let _conn = open_encrypted_db(&db_path, &master_key_hex)
        .map_err(|_| "Failed to open database with password".to_string())?;

    Ok(true)
}

/// Get config file path
fn get_config_path() -> PathBuf {
    let mut path = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("GestorDoClube");
    std::fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}

/// Get database file path
fn get_database_path() -> PathBuf {
    let mut path = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("GestorDoClube");
    std::fs::create_dir_all(&path).ok();
    path.push("club.db");
    path
}

fn get_db_path() -> PathBuf {
    get_database_path()
}
