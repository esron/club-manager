use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result as SqlResult};
use std::collections::HashMap;
use chrono::{NaiveDate, Datelike};

#[derive(Debug, Serialize, Deserialize)]
pub struct DebtStatusRow {
    pub member_id: i64,
    pub member_name: String,
    pub total_debt: f64,
    pub unpaid_month_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebtStatusReport {
    pub members: Vec<DebtStatusRow>,
    pub generated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthColumn {
    pub key: String,        // "2026-01"
    pub display: String,    // "Jan/2026"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentHistoryRow {
    pub member_id: i64,
    pub member_name: String,
    pub start_date: String,
    pub payments: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentHistoryReport {
    pub members: Vec<PaymentHistoryRow>,
    pub month_columns: Vec<MonthColumn>,
    pub generated_at: String,
}

const MONTH_ABBREV_PT: [&str; 12] = [
    "Jan", "Fev", "Mar", "Abr", "Mai", "Jun",
    "Jul", "Ago", "Set", "Out", "Nov", "Dez"
];

/// Calculate debt for a member using pre-fetched payments
/// This avoids N+1 queries by calculating debt from already-fetched payment data
fn calculate_debt_from_payments(
    member_id: i64,
    start_date: &str,
    as_of_date: &str,
    min_fee: f64,
    payments: &[(i64, i32, i32, f64)],
) -> SqlResult<f64> {
    // Parse dates
    let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
        .map_err(|_| rusqlite::Error::InvalidQuery)?;
    let as_of = NaiveDate::parse_from_str(as_of_date, "%Y-%m-%d")
        .map_err(|_| rusqlite::Error::InvalidQuery)?;

    let mut debt = 0.0;
    let mut current = start;

    // Iterate through each month from start_date to as_of_date
    while current <= as_of {
        let month = current.month() as i32;
        let year = current.year() as i32;

        // Check if payment exists for this member in this month
        let has_payment = payments.iter().any(|p| {
            p.0 == member_id && p.1 == month && p.2 == year
        });

        if !has_payment {
            // Calculate grace period deadline (10th of next month)
            let next_month = if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 10)
            } else {
                NaiveDate::from_ymd_opt(year, (month + 1) as u32, 10)
            };

            // If we're past the grace period, add debt
            if let Some(deadline) = next_month {
                if as_of > deadline {
                    debt += min_fee;
                }
            }
        }

        // Move to next month
        current = if current.month() == 12 {
            NaiveDate::from_ymd_opt(current.year() + 1, 1, 1)
                .unwrap_or(current)
        } else {
            NaiveDate::from_ymd_opt(current.year(), current.month() + 1, 1)
                .unwrap_or(current)
        };
    }

    Ok(debt)
}

pub fn generate_debt_status_report(
    conn: &Connection,
    include_inactive: bool,
) -> SqlResult<DebtStatusReport> {
    let query = if include_inactive {
        "SELECT id, name, start_date FROM members ORDER BY id"
    } else {
        "SELECT id, name, start_date FROM members WHERE active = 1 ORDER BY id"
    };

    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    // Fetch minimum fee setting once before the loop
    let min_fee: f64 = conn.query_row(
        "SELECT value FROM settings WHERE key = 'minimum_fee_brl'",
        [],
        |row| row.get(0)
    ).unwrap_or(15.0);

    // Fetch ALL payments once before member loop to avoid N+1 queries
    let all_payments: Vec<(i64, i32, i32, f64)> = {
        let mut stmt = conn.prepare("SELECT member_id, month, year, amount_brl FROM payments")?;
        let payments = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })?;
        payments.collect::<Result<Vec<_>, _>>()?
    };

    let mut members = Vec::new();

    for row in rows {
        let (member_id, member_name, start_date) = row?;
        let total_debt = calculate_debt_from_payments(
            member_id,
            &start_date,
            &today,
            min_fee,
            &all_payments,
        )?;

        // Count unpaid months using the pre-fetched minimum fee
        let unpaid_count = if total_debt > 0.0 {
            (total_debt / min_fee).ceil() as i32
        } else {
            0
        };

        members.push(DebtStatusRow {
            member_id,
            member_name,
            total_debt,
            unpaid_month_count: unpaid_count,
        });
    }

    // Sort by debt descending
    members.sort_by(|a, b| b.total_debt.partial_cmp(&a.total_debt).unwrap_or(std::cmp::Ordering::Equal));

    Ok(DebtStatusReport {
        members,
        generated_at: chrono::Local::now().to_rfc3339(),
    })
}

pub fn generate_payment_history_report(
    conn: &Connection,
    start_date: &str,
    end_date: &str,
) -> SqlResult<PaymentHistoryReport> {
    // Parse dates
    let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
        .map_err(|_| rusqlite::Error::InvalidQuery)?;
    let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
        .map_err(|_| rusqlite::Error::InvalidQuery)?;

    // Generate month columns
    let mut month_columns = Vec::new();
    let mut current = NaiveDate::from_ymd_opt(start.year(), start.month(), 1)
        .ok_or(rusqlite::Error::InvalidQuery)?;
    let end_month = NaiveDate::from_ymd_opt(end.year(), end.month(), 1)
        .ok_or(rusqlite::Error::InvalidQuery)?;

    while current <= end_month {
        let key = format!("{}-{:02}", current.year(), current.month());
        let display = format!(
            "{}/{}",
            MONTH_ABBREV_PT[current.month() as usize - 1],
            current.year()
        );
        month_columns.push(MonthColumn { key, display });

        // Move to next month
        current = if current.month() == 12 {
            NaiveDate::from_ymd_opt(current.year() + 1, 1, 1)
                .ok_or(rusqlite::Error::InvalidQuery)?
        } else {
            NaiveDate::from_ymd_opt(current.year(), current.month() + 1, 1)
                .ok_or(rusqlite::Error::InvalidQuery)?
        };
    }

    // Fetch ALL payments once before member loop to avoid N+1 queries
    let mut all_payments_stmt = conn.prepare(
        "SELECT member_id, month, year, amount_brl FROM payments ORDER BY member_id"
    )?;
    let all_payments_rows = all_payments_stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, i32>(1)?,
            row.get::<_, i32>(2)?,
            row.get::<_, f64>(3)?,
        ))
    })?;

    // Group payments by member_id
    let mut payments_by_member: HashMap<i64, HashMap<String, f64>> = HashMap::new();
    for payment_row in all_payments_rows {
        let (member_id, month, year, amount) = payment_row?;
        let key = format!("{}-{:02}", year, month);
        payments_by_member
            .entry(member_id)
            .or_insert_with(HashMap::new)
            .insert(key, amount);
    }

    // Get all active members
    let mut stmt = conn.prepare(
        "SELECT id, name, start_date FROM members WHERE active = 1 ORDER BY id"
    )?;
    let members_data = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    let mut members = Vec::new();

    for member_data in members_data {
        let (member_id, member_name, start_date_str) = member_data?;
        let date_part = start_date_str.split('T').next()
            .ok_or(rusqlite::Error::InvalidQuery)?;
        let member_start = NaiveDate::parse_from_str(date_part, "%Y-%m-%d")
            .map_err(|_| rusqlite::Error::InvalidQuery)?;

        // Look up payments from pre-fetched HashMap instead of querying
        let payments_map = payments_by_member.get(&member_id).cloned().unwrap_or_default();

        // Build payments hash for all month columns
        let mut payments = HashMap::new();
        for col in &month_columns {
            let month_date = NaiveDate::parse_from_str(&format!("{}-01", col.key), "%Y-%m-%d")
                .map_err(|_| rusqlite::Error::InvalidQuery)?;

            if month_date < member_start {
                // Member not active yet - leave blank
                payments.insert(col.key.clone(), String::new());
            } else if let Some(amount) = payments_map.get(&col.key) {
                // Payment exists
                payments.insert(
                    col.key.clone(),
                    format!("R$ {:.2}", amount).replace('.', ",")
                );
            } else {
                // No payment - show dash
                payments.insert(col.key.clone(), "-".to_string());
            }
        }

        members.push(PaymentHistoryRow {
            member_id,
            member_name,
            start_date: format_date_dd_mm_yyyy(&start_date_str),
            payments,
        });
    }

    Ok(PaymentHistoryReport {
        members,
        month_columns,
        generated_at: chrono::Local::now().to_rfc3339(),
    })
}

fn format_date_dd_mm_yyyy(date_str: &str) -> String {
    if let Some(date_part) = date_str.split('T').next() {
        let parts: Vec<&str> = date_part.split('-').collect();
        if parts.len() == 3 {
            return format!("{}/{}/{}", parts[2], parts[1], parts[0]);
        }
    }
    date_str.to_string()
}

pub fn anonymize_report_debt(mut report: DebtStatusReport) -> DebtStatusReport {
    for (idx, member) in report.members.iter_mut().enumerate() {
        member.member_name = format!("Membro #{}", idx + 1);
    }
    report
}

pub fn anonymize_report_payment(mut report: PaymentHistoryReport) -> PaymentHistoryReport {
    for (idx, member) in report.members.iter_mut().enumerate() {
        member.member_name = format!("Membro #{}", idx + 1);
    }
    report
}
