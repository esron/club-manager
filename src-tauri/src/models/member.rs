use rusqlite::{Connection, Result, Row};
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: i64,
    pub name: String,
    pub start_date: String,
    pub created_at: String,
    pub active: bool,
}

impl Member {
    fn from_row(row: &Row) -> Result<Self> {
        Ok(Member {
            id: row.get(0)?,
            name: row.get(1)?,
            start_date: row.get(2)?,
            created_at: row.get(3)?,
            active: row.get(4)?,
        })
    }
}

/// Create a new member
pub fn create_member(conn: &Connection, name: &str, start_date: &str) -> Result<i64> {
    let created_at = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO members (name, start_date, created_at, active) VALUES (?, ?, ?, 1)",
        [name, start_date, &created_at],
    )?;

    Ok(conn.last_insert_rowid())
}

/// Get all active members
pub fn get_members(conn: &Connection) -> Result<Vec<Member>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, start_date, created_at, active FROM members WHERE active = 1 ORDER BY name"
    )?;

    let members = stmt.query_map([], Member::from_row)?
        .collect::<Result<Vec<_>>>()?;

    Ok(members)
}

/// Get member by ID
pub fn get_member_by_id(conn: &Connection, id: i64) -> Result<Member> {
    conn.query_row(
        "SELECT id, name, start_date, created_at, active FROM members WHERE id = ?",
        [id],
        Member::from_row,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::initialize_schema;

    #[test]
    fn test_member_crud() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();

        let id = create_member(&conn, "Test Member", "2026-01-01").unwrap();
        let member = get_member_by_id(&conn, id).unwrap();

        assert_eq!(member.name, "Test Member");
        assert!(member.active);
    }
}
