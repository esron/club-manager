use rusqlite::{Connection, Result};

pub fn initialize_schema(conn: &Connection) -> Result<()> {
    // Create members table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS members (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            start_date TEXT NOT NULL,
            created_at TEXT NOT NULL,
            active BOOLEAN DEFAULT 1
        )",
        [],
    )?;

    // Create payments table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS payments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            member_id INTEGER NOT NULL,
            month INTEGER NOT NULL,
            year INTEGER NOT NULL,
            amount_brl REAL NOT NULL,
            payment_date TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (member_id) REFERENCES members(id),
            UNIQUE(member_id, month, year)
        )",
        [],
    )?;

    // Create settings table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    // Create indexes
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_payments_member ON payments(member_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_payments_date ON payments(year, month)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_members_active ON members(active)",
        [],
    )?;

    // Insert default settings if not exists
    conn.execute(
        "INSERT OR IGNORE INTO settings (key, value) VALUES ('minimum_fee_brl', '15.00')",
        [],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creation() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        // Test that we can query the tables
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM members", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }
}
