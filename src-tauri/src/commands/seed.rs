use crate::models::member::create_member;
use crate::models::payment::create_payment;
use crate::security::config::load_config;
use crate::security::password::derive_encryption_key;
use crate::db::connection::open_encrypted_db;
use std::path::PathBuf;
use rand::Rng;

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
pub fn seed_database(password: String, member_count: Option<usize>, payments_per_member: Option<usize>) -> Result<String, String> {
    eprintln!("[SEED] Starting database seed...");

    let member_count = member_count.unwrap_or(100);
    let payments_per_member = payments_per_member.unwrap_or(5);

    let config_path = get_config_path();
    let config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    let key_bytes = derive_encryption_key(&password, &config.salt)
        .map_err(|e| format!("Failed to derive key: {}", e))?;
    let key_hex = hex::encode(&key_bytes);

    let db_path = get_db_path();
    let conn = open_encrypted_db(&db_path, &key_hex)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    let mut rng = rand::thread_rng();
    let mut members_created = 0;
    let mut payments_created = 0;

    // Generate members
    for i in 0..member_count {
        let first_name = FIRST_NAMES[rng.gen_range(0..FIRST_NAMES.len())];
        let last_name = LAST_NAMES[rng.gen_range(0..LAST_NAMES.len())];
        let name = format!("{} {} {}", first_name, last_name, i + 1); // Add number to ensure uniqueness

        // Random start date between Jan 2020 and Dec 2025
        let year = rng.gen_range(2020..=2025);
        let month = rng.gen_range(1..=12);
        let day = rng.gen_range(1..=28); // Use 28 to avoid month-end issues
        let start_date = format!("{:04}-{:02}-{:02}", year, month, day);

        match create_member(&conn, &name, &start_date) {
            Ok(member_id) => {
                members_created += 1;
                eprintln!("[SEED] Created member: {} (id: {})", name, member_id);

                // Generate payments for this member
                for _ in 0..payments_per_member {
                    let pay_year = rng.gen_range(2024..=2026);
                    let pay_month = rng.gen_range(1..=12);
                    let pay_day = rng.gen_range(1..=28);
                    let payment_date = format!("{:04}-{:02}-{:02}", pay_year, pay_month, pay_day);

                    // Random amount between 10.00 and 25.00
                    let amount = rng.gen_range(1000..2500) as f64 / 100.0;

                    match create_payment(&conn, member_id, pay_month, pay_year, amount, &payment_date) {
                        Ok(_) => payments_created += 1,
                        Err(e) => {
                            // Ignore duplicate payment errors (same member/month/year)
                            if !e.to_string().contains("UNIQUE constraint") {
                                eprintln!("[SEED] Warning: Failed to create payment for {}: {}", name, e);
                            }
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
        "Seed complete!\nMembers created: {}\nPayments created: {}",
        members_created, payments_created
    );

    eprintln!("[SEED] {}", result);
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
