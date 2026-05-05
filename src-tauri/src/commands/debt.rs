// src-tauri/src/commands/debt.rs
use crate::models::debt::calculate_member_debt;
use crate::models::member::get_all_members;
use crate::models::payment::get_payments;
use crate::models::settings::get_setting;
use crate::security::config::load_config;
use crate::security::password::derive_encryption_key;
use crate::db::connection::open_encrypted_db;
use std::path::PathBuf;
use chrono::{Utc, NaiveDate, Datelike};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UnpaidMonth {
    month: i32,
    year: i32,
    amount: f64,
    display: String,
}

#[derive(Serialize, Deserialize)]
pub struct MemberDebtInfo {
    member_id: i64,
    member_name: String,
    total_debt: f64,
    unpaid_months: Vec<UnpaidMonth>,
}

#[tauri::command]
pub fn get_member_debt_cmd(password: String, member_id: i64) -> Result<MemberDebtInfo, String> {
    let conn = get_authenticated_connection(&password)?;

    // Get member details
    let member_name: String = conn.query_row(
        "SELECT name FROM members WHERE id = ?",
        [member_id],
        |row| row.get(0),
    ).map_err(|e| format!("Failed to get member: {}", e))?;

    let member_start: String = conn.query_row(
        "SELECT start_date FROM members WHERE id = ?",
        [member_id],
        |row| row.get(0),
    ).map_err(|e| format!("Failed to get member start date: {}", e))?;

    // Calculate total debt
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let total_debt = calculate_member_debt(&conn, member_id, &today)
        .map_err(|e| format!("Failed to calculate debt: {}", e))?;

    // Get unpaid months
    let unpaid_months = get_unpaid_months(&conn, member_id, &member_start, &today)?;

    Ok(MemberDebtInfo {
        member_id,
        member_name,
        total_debt,
        unpaid_months,
    })
}

#[tauri::command]
pub fn get_all_debts_cmd(password: String) -> Result<Vec<MemberDebtInfo>, String> {
    let conn = get_authenticated_connection(&password)?;

    // Get all active members
    let members = get_all_members(&conn)
        .map_err(|e| format!("Failed to get members: {}", e))?;

    let today = Utc::now().format("%Y-%m-%d").to_string();
    let mut debts = Vec::new();

    for member in members {
        // Only calculate for active members
        if !member.active {
            continue;
        }

        let total_debt = calculate_member_debt(&conn, member.id, &today)
            .map_err(|e| format!("Failed to calculate debt: {}", e))?;

        let unpaid_months = get_unpaid_months(&conn, member.id, &member.start_date, &today)?;

        debts.push(MemberDebtInfo {
            member_id: member.id,
            member_name: member.name,
            total_debt,
            unpaid_months,
        });
    }

    Ok(debts)
}

fn get_unpaid_months(
    conn: &rusqlite::Connection,
    member_id: i64,
    start_date: &str,
    as_of_date: &str,
) -> Result<Vec<UnpaidMonth>, String> {
    let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
        .map_err(|_| "Invalid start date".to_string())?;
    let as_of = NaiveDate::parse_from_str(as_of_date, "%Y-%m-%d")
        .map_err(|_| "Invalid as_of date".to_string())?;

    // Get minimum fee
    let min_fee_str = get_setting(conn, "minimum_fee_brl")
        .map_err(|e| format!("Failed to get minimum fee: {}", e))?;
    let min_fee: f64 = min_fee_str.parse().unwrap_or(15.0);

    // Get all payments for this member
    let payments = get_payments(conn)
        .map_err(|e| format!("Failed to get payments: {}", e))?
        .into_iter()
        .filter(|p| p.member_id == member_id)
        .collect::<Vec<_>>();

    let mut unpaid = Vec::new();
    let mut current = start;

    while current <= as_of {
        let month = current.month() as i32;
        let year = current.year() as i32;

        // Check if payment exists
        let has_payment = payments.iter().any(|p| p.month == month && p.year == year);

        if !has_payment {
            // Check grace period
            let next_month = if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 10)
            } else {
                NaiveDate::from_ymd_opt(year, (month + 1) as u32, 10)
            };

            if let Some(deadline) = next_month {
                if as_of > deadline {
                    unpaid.push(UnpaidMonth {
                        month,
                        year,
                        amount: min_fee,
                        display: format_month_pt(month, year),
                    });
                }
            }
        }

        // Move to next month
        current = if current.month() == 12 {
            NaiveDate::from_ymd_opt(current.year() + 1, 1, 1).unwrap_or(current)
        } else {
            NaiveDate::from_ymd_opt(current.year(), current.month() + 1, 1).unwrap_or(current)
        };
    }

    Ok(unpaid)
}

fn format_month_pt(month: i32, year: i32) -> String {
    let month_name = match month {
        1 => "Janeiro",
        2 => "Fevereiro",
        3 => "Março",
        4 => "Abril",
        5 => "Maio",
        6 => "Junho",
        7 => "Julho",
        8 => "Agosto",
        9 => "Setembro",
        10 => "Outubro",
        11 => "Novembro",
        12 => "Dezembro",
        _ => "Desconhecido",
    };
    format!("{} {}", month_name, year)
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
