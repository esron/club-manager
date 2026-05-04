use rusqlite::{Connection, Result, Row};
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: i64,
    pub member_id: i64,
    pub month: i32,
    pub year: i32,
    pub amount_brl: f64,
    pub payment_date: String,
    pub created_at: String,
}

impl Payment {
    fn from_row(row: &Row) -> Result<Self> {
        Ok(Payment {
            id: row.get(0)?,
            member_id: row.get(1)?,
            month: row.get(2)?,
            year: row.get(3)?,
            amount_brl: row.get(4)?,
            payment_date: row.get(5)?,
            created_at: row.get(6)?,
        })
    }
}

/// Create a new payment
pub fn create_payment(
    conn: &Connection,
    member_id: i64,
    month: i32,
    year: i32,
    amount_brl: f64,
    payment_date: &str,
) -> Result<i64> {
    let created_at = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO payments (member_id, month, year, amount_brl, payment_date, created_at)
         VALUES (?, ?, ?, ?, ?, ?)",
        [
            &member_id.to_string(),
            &month.to_string(),
            &year.to_string(),
            &amount_brl.to_string(),
            payment_date,
            &created_at,
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

/// Get all payments ordered by year/month descending
pub fn get_payments(conn: &Connection) -> Result<Vec<Payment>> {
    let mut stmt = conn.prepare(
        "SELECT id, member_id, month, year, amount_brl, payment_date, created_at
         FROM payments
         ORDER BY year DESC, month DESC"
    )?;

    let payments = stmt.query_map([], Payment::from_row)?
        .collect::<Result<Vec<_>>>()?;

    Ok(payments)
}

/// Get payment for specific member and month
pub fn get_payment_by_member_month(
    conn: &Connection,
    member_id: i64,
    month: i32,
    year: i32,
) -> Result<Payment> {
    conn.query_row(
        "SELECT id, member_id, month, year, amount_brl, payment_date, created_at
         FROM payments
         WHERE member_id = ? AND month = ? AND year = ?",
        [member_id, month as i64, year as i64],
        Payment::from_row,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::initialize_schema;
    use crate::models::member::create_member;

    #[test]
    fn test_payment_crud() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        let member_id = create_member(&conn, "Test", "2026-01-01").unwrap();
        let payment_id = create_payment(&conn, member_id, 5, 2026, 15.0, "2026-05-10").unwrap();
        let payment = get_payment_by_member_month(&conn, member_id, 5, 2026).unwrap();

        assert_eq!(payment.id, payment_id);
        assert_eq!(payment.amount_brl, 15.0);
    }
}
