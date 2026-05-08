use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result as SqlResult};
use chrono::{Datelike, Local, NaiveDate};

use super::debt::calculate_member_debt;

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthData {
    pub month_key: String,       // "2026-01"
    pub month_display: String,   // "Jan/26"
    pub total_payments: f64,     // Sum of payments in this month
    pub total_debt: f64,         // Total club debt as of end of month
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartData {
    pub months: Vec<MonthData>,
}

const MONTH_ABBREV_PT: [&str; 12] = [
    "Jan", "Fev", "Mar", "Abr", "Mai", "Jun",
    "Jul", "Ago", "Set", "Out", "Nov", "Dez"
];

pub fn generate_chart_data(conn: &Connection) -> SqlResult<ChartData> {
    let now = Local::now();
    let mut months = Vec::new();

    // Generate last 6 months (current month + 5 previous)
    for i in (0..6).rev() {
        let target_month = if now.month() as i32 - i > 0 {
            now.month() as i32 - i
        } else {
            12 + (now.month() as i32 - i)
        };

        let target_year = if now.month() as i32 - i > 0 {
            now.year()
        } else {
            now.year() - 1
        };

        let month_key = format!("{}-{:02}", target_year, target_month);
        let month_abbrev = MONTH_ABBREV_PT[(target_month as usize) - 1];
        let month_display = format!("{}/{}", month_abbrev, target_year % 100);

        // Calculate total payments for this month
        let total_payments: f64 = conn.query_row(
            "SELECT COALESCE(SUM(amount_brl), 0.0) FROM payments
             WHERE strftime('%Y-%m', payment_date) = ?",
            [&month_key],
            |row| row.get(0)
        ).unwrap_or(0.0);

        // Calculate total debt as of end of month
        let last_day = get_last_day_of_month(target_year, target_month as u32);
        let end_of_month = format!("{}-{:02}-{:02}", target_year, target_month, last_day);

        let total_debt = calculate_total_debt(conn, &end_of_month)?;

        months.push(MonthData {
            month_key,
            month_display,
            total_payments,
            total_debt,
        });
    }

    Ok(ChartData { months })
}

fn calculate_total_debt(conn: &Connection, as_of_date: &str) -> SqlResult<f64> {
    // Get all active members
    let mut stmt = conn.prepare("SELECT id FROM members WHERE active = 1")?;
    let member_ids = stmt.query_map([], |row| row.get::<_, i64>(0))?;

    let mut total = 0.0;
    for member_id in member_ids {
        let id = member_id?;
        let debt = calculate_member_debt(conn, id, as_of_date)?;
        total += debt;
    }

    Ok(total)
}

fn get_last_day_of_month(year: i32, month: u32) -> u32 {
    if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day()
    }
}
