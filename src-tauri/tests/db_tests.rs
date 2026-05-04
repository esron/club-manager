use gestor_do_clube_lib::db::schema::initialize_schema;
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
