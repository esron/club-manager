use crate::models::member::create_member;
use crate::models::payment::create_payment;
use crate::security::config::load_config;
use crate::security::password::{derive_encryption_key, decrypt_master_key};
use crate::db::connection::open_encrypted_db;
use std::path::PathBuf;
use rand::Rng;
use chrono::{Datelike, Local};
use zeroize::Zeroizing;

const FIRST_NAMES: &[&str] = &[
    "João", "Maria", "José", "Ana", "Pedro", "Paula", "Carlos", "Mariana",
    "Fernando", "Juliana", "Ricardo", "Camila", "Rafael", "Beatriz", "Lucas",
    "Larissa", "Gustavo", "Amanda", "Felipe", "Isabela", "Rodrigo", "Carolina",
    "Bruno", "Fernanda", "Daniel", "Patrícia", "Marcelo", "Aline", "André",
    "Gabriela", "Thiago", "Letícia", "Henrique", "Renata", "Vinícius", "Vanessa",
];

const LAST_NAMES: &[&str] = &[
    "Silva", "Santos", "Oliveira", "Souza", "Rodrigues", "Ferreira", "Alves",
    "Pereira", "Lima", "Gomes", "Costa", "Ribeiro", "Martins", "Carvalho",
    "Rocha", "Almeida", "Nascimento", "Araújo", "Fernandes", "Soares",
];

#[tauri::command]
pub fn seed_database(password: String) -> Result<String, String> {
    eprintln!("[SEED] Starting database seed...");

    let member_count = 100;
    let now = Local::now();
    let current_year = now.year();
    let current_month = now.month() as i32;

    let config_path = get_config_path();
    let config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    // Get the correct encryption key (master key if migrated, password-derived if not)
    let key_hex = if let Some(encrypted_master_key) = &config.master_key_encrypted {
        // Post-migration: decrypt master key
        let master_key = decrypt_master_key(encrypted_master_key, &password, &config.salt)
            .map_err(|e| format!("Failed to decrypt master key: {}", e))?;
        Zeroizing::new(hex::encode(&*master_key))
    } else {
        // Pre-migration: derive key from password
        let key_bytes = derive_encryption_key(&password, &config.salt)
            .map_err(|e| format!("Failed to derive key: {}", e))?;
        Zeroizing::new(hex::encode(&*key_bytes))
    };

    let db_path = get_db_path();
    let conn = open_encrypted_db(&db_path, &*key_hex)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    let mut rng = rand::thread_rng();
    let mut members_created = 0;
    let mut payments_created = 0;

    // All members start on Jan 1st of current year
    let start_date = format!("{}-01-01", current_year);

    // Generate members
    for i in 0..member_count {
        let first_name = FIRST_NAMES[rng.gen_range(0..FIRST_NAMES.len())];
        let last_name = LAST_NAMES[rng.gen_range(0..LAST_NAMES.len())];
        let name = format!("{} {} {}", first_name, last_name, i + 1); // Add number to ensure uniqueness

        match create_member(&conn, &name, &start_date) {
            Ok(member_id) => {
                members_created += 1;
                eprintln!("[SEED] Created member: {} (id: {})", name, member_id);

                // Generate payments from January to current month
                // 25% chance of not paying the last month
                let skip_last_month = rng.gen_range(0..100) < 25;
                let last_month_to_pay = if skip_last_month {
                    current_month - 1
                } else {
                    current_month
                };

                for month in 1..=last_month_to_pay {
                    let pay_day = rng.gen_range(1..=28);
                    let payment_date = format!("{:04}-{:02}-{:02}", current_year, month, pay_day);

                    // Random amount between 15.00 and 25.00
                    let amount = rng.gen_range(1500..2500) as f64 / 100.0;

                    match create_payment(&conn, member_id, month, current_year, amount, &payment_date) {
                        Ok(_) => payments_created += 1,
                        Err(e) => {
                            eprintln!("[SEED] Warning: Failed to create payment for {}: {}", name, e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("[SEED] Error creating member {}: {}", name, e);
            }
        }
    }

    let result = format!(
        "Seed complete!\nMembers created: {}\nPayments created: {}\n25% of members have debt on last month only",
        members_created, payments_created
    );

    eprintln!("[SEED] {}", result);
    Ok(result)
}

#[tauri::command]
pub fn clear_database(password: String) -> Result<String, String> {
    eprintln!("[CLEAR] Starting database clear...");

    let config_path = get_config_path();
    let config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    // Get the correct encryption key (master key if migrated, password-derived if not)
    let key_hex = if let Some(encrypted_master_key) = &config.master_key_encrypted {
        // Post-migration: decrypt master key
        let master_key = decrypt_master_key(encrypted_master_key, &password, &config.salt)
            .map_err(|e| format!("Failed to decrypt master key: {}", e))?;
        Zeroizing::new(hex::encode(&*master_key))
    } else {
        // Pre-migration: derive key from password
        let key_bytes = derive_encryption_key(&password, &config.salt)
            .map_err(|e| format!("Failed to derive key: {}", e))?;
        Zeroizing::new(hex::encode(&*key_bytes))
    };

    let db_path = get_db_path();
    let conn = open_encrypted_db(&db_path, &*key_hex)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // Delete all payments first (due to foreign key constraint)
    conn.execute("DELETE FROM payments", [])
        .map_err(|e| format!("Failed to delete payments: {}", e))?;

    // Delete all members
    conn.execute("DELETE FROM members", [])
        .map_err(|e| format!("Failed to delete members: {}", e))?;

    // Reset autoincrement counters
    conn.execute("DELETE FROM sqlite_sequence WHERE name='members'", [])
        .map_err(|e| format!("Failed to reset members sequence: {}", e))?;

    conn.execute("DELETE FROM sqlite_sequence WHERE name='payments'", [])
        .map_err(|e| format!("Failed to reset payments sequence: {}", e))?;

    let result = "Database cleared successfully!\nAll members and payments have been deleted.".to_string();

    eprintln!("[CLEAR] {}", result);
    Ok(result)
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
