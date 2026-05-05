use crate::security::password::{hash_password, verify_password, derive_encryption_key};
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

/// Setup initial password (first launch only)
#[tauri::command]
pub fn setup_password(password: String) -> Result<(), String> {
    eprintln!("[AUTH] setup_password called");
    let config_path = get_config_path();
    eprintln!("[AUTH] Config path: {:?}", config_path);

    // Verify this is first launch
    if config_path.exists() {
        eprintln!("[AUTH] ERROR: Config already exists");
        return Err("Password already configured".to_string());
    }

    // Hash password with bcrypt
    eprintln!("[AUTH] Hashing password...");
    let password_hash = hash_password(&password)
        .map_err(|e| {
            eprintln!("[AUTH] ERROR hashing password: {}", e);
            format!("Failed to hash password: {}", e)
        })?;
    eprintln!("[AUTH] Password hashed successfully");

    // Generate random salt for encryption
    eprintln!("[AUTH] Generating random salt...");
    let salt: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect();
    eprintln!("[AUTH] Salt generated: {} bytes", salt.len());

    // Derive encryption key from password
    eprintln!("[AUTH] Deriving encryption key...");
    let key_bytes = derive_encryption_key(&password, &salt)
        .map_err(|e| {
            eprintln!("[AUTH] ERROR deriving key: {}", e);
            format!("Failed to derive key: {}", e)
        })?;
    let key_hex = hex::encode(&key_bytes);
    eprintln!("[AUTH] Key derived successfully, hex length: {}", key_hex.len());

    // Create config
    eprintln!("[AUTH] Creating config...");
    let config = AppConfig {
        password_hash,
        salt: salt.clone(),
        minimum_fee_brl: "15.00".to_string(),
        created_at: Utc::now().to_rfc3339(),
    };

    // Save config
    eprintln!("[AUTH] Saving config to {:?}...", config_path);
    save_config(&config, &config_path)
        .map_err(|e| {
            eprintln!("[AUTH] ERROR saving config: {}", e);
            format!("Failed to save config: {}", e)
        })?;
    eprintln!("[AUTH] Config saved successfully");

    // Create encrypted database
    let db_path = get_db_path();
    eprintln!("[AUTH] Opening encrypted database at {:?}...", db_path);
    let conn = open_encrypted_db(&db_path, &key_hex)
        .map_err(|e| {
            eprintln!("[AUTH] ERROR opening database: {}", e);
            format!("Failed to create database: {}", e)
        })?;
    eprintln!("[AUTH] Database opened successfully");

    // Initialize schema
    eprintln!("[AUTH] Initializing database schema...");
    initialize_schema(&conn)
        .map_err(|e| {
            eprintln!("[AUTH] ERROR initializing schema: {}", e);
            format!("Failed to initialize schema: {}", e)
        })?;
    eprintln!("[AUTH] Schema initialized successfully");

    eprintln!("[AUTH] setup_password completed successfully");
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

    // Derive key and test database connection
    let key_bytes = derive_encryption_key(&password, &config.salt)
        .map_err(|e| format!("Failed to derive key: {}", e))?;
    let key_hex = hex::encode(&key_bytes);

    let db_path = get_db_path();
    let _conn = open_encrypted_db(&db_path, &key_hex)
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
