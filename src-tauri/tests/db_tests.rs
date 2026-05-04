use gestor_do_clube_lib::db::schema::initialize_schema;
use gestor_do_clube_lib::db::connection::open_encrypted_db;
use rusqlite::Connection;
use tempfile::NamedTempFile;

#[test]
fn test_schema_initialization() {
    let temp_file = NamedTempFile::new().unwrap();
    let conn = Connection::open(temp_file.path()).unwrap();

    initialize_schema(&conn).expect("Schema initialization failed");

    // Verify tables exist
    let tables: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
        .unwrap()
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    assert!(tables.contains(&"members".to_string()));
    assert!(tables.contains(&"payments".to_string()));
    assert!(tables.contains(&"settings".to_string()));
}

#[test]
fn test_settings_default_data() {
    let temp_file = NamedTempFile::new().unwrap();
    let conn = Connection::open(temp_file.path()).unwrap();

    initialize_schema(&conn).unwrap();

    let min_fee: String = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'minimum_fee_brl'",
            [],
            |row| row.get(0)
        )
        .unwrap();

    assert_eq!(min_fee, "15.00");
}

#[test]
fn test_open_encrypted_database() {
    let temp_file = NamedTempFile::new().unwrap();
    let key = "test_encryption_key_32_bytes!!";

    // Create encrypted database
    let conn = open_encrypted_db(temp_file.path(), key).expect("Failed to open encrypted DB");

    // Verify we can use it
    conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY)", []).unwrap();
    drop(conn);

    // Verify we can reopen with same key
    let conn2 = open_encrypted_db(temp_file.path(), key).expect("Failed to reopen");
    let count: i64 = conn2
        .query_row("SELECT COUNT(*) FROM test", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_wrong_key_fails() {
    let temp_file = NamedTempFile::new().unwrap();
    let key1 = "correct_key_12345678901234567";
    let key2 = "wrong_key_123456789012345678";

    // Create with key1
    let conn = open_encrypted_db(temp_file.path(), key1).unwrap();
    conn.execute("CREATE TABLE test (id INTEGER)", []).unwrap();
    drop(conn);

    // Try to open with key2 - should fail
    let result = open_encrypted_db(temp_file.path(), key2);
    assert!(result.is_err());
}
