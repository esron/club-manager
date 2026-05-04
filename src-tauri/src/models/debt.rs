use rusqlite::{Connection, Result};
use chrono::{NaiveDate, Datelike};
use crate::models::payment::get_payments;
use crate::models::settings::get_setting;

/// Calculate total debt for a member as of a specific date
///
/// Business rule: Debt = any month without payment that is past
/// the 10th of the following month
///
/// # Arguments
/// * `conn` - Database connection
/// * `member_id` - Member ID
/// * `as_of_date` - Date to calculate debt (YYYY-MM-DD format)
///
/// # Returns
/// Total debt amount in BRL
pub fn calculate_member_debt(
    conn: &Connection,
    member_id: i64,
    as_of_date: &str,
) -> Result<f64> {
    // Get member start date
    let start_date: String = conn.query_row(
        "SELECT start_date FROM members WHERE id = ?",
        [member_id],
        |row| row.get(0),
    )?;

    // Get minimum fee
    let min_fee_str = get_setting(conn, "minimum_fee_brl")?;
    let min_fee: f64 = min_fee_str.parse().unwrap_or(15.0);

    // Parse dates
    let start = NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
        .map_err(|_| rusqlite::Error::InvalidQuery)?;
    let as_of = NaiveDate::parse_from_str(as_of_date, "%Y-%m-%d")
        .map_err(|_| rusqlite::Error::InvalidQuery)?;

    // Get all payments for this member
    let payments = get_payments(conn)?
        .into_iter()
        .filter(|p| p.member_id == member_id)
        .collect::<Vec<_>>();

    let mut debt = 0.0;
    let mut current = start;

    // Iterate through each month from start_date to as_of_date
    while current <= as_of {
        let month = current.month() as i32;
        let year = current.year() as i32;

        // Check if payment exists for this month
        let has_payment = payments.iter().any(|p| p.month == month && p.year == year);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::initialize_schema;
    use crate::models::member::create_member;

    #[test]
    fn test_debt_calculation_basic() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        let member_id = create_member(&conn, "Test", "2026-01-01").unwrap();

        // No payments, check on Feb 15 (after grace for January)
        let debt = calculate_member_debt(&conn, member_id, "2026-02-15").unwrap();
        assert_eq!(debt, 15.0); // Owes for January
    }
}
