use rusqlite::{Connection, Result};

/// Get setting value by key
pub fn get_setting(conn: &Connection, key: &str) -> Result<String> {
    conn.query_row(
        "SELECT value FROM settings WHERE key = ?",
        [key],
        |row| row.get(0),
    )
}

/// Update setting value (INSERT OR REPLACE for upsert behavior)
pub fn update_setting(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)",
        [key, value],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::initialize_schema;

    #[test]
    fn test_setting_operations() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        let initial = get_setting(&conn, "minimum_fee_brl").unwrap();
        assert_eq!(initial, "15.00");

        update_setting(&conn, "minimum_fee_brl", "25.00").unwrap();
        let updated = get_setting(&conn, "minimum_fee_brl").unwrap();
        assert_eq!(updated, "25.00");
    }
}
