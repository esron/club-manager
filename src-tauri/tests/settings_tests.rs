use gestor_do_clube_lib::models::settings::{get_setting, update_setting};
use gestor_do_clube_lib::db::schema::initialize_schema;
use rusqlite::Connection;

#[test]
fn test_get_setting() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    // Schema initializes with default minimum_fee_brl = '15.00'
    let value = get_setting(&conn, "minimum_fee_brl").unwrap();
    assert_eq!(value, "15.00");
}

#[test]
fn test_update_setting() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    update_setting(&conn, "minimum_fee_brl", "20.00").unwrap();
    let value = get_setting(&conn, "minimum_fee_brl").unwrap();
    assert_eq!(value, "20.00");
}

#[test]
fn test_nonexistent_setting() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let result = get_setting(&conn, "nonexistent_key");
    assert!(result.is_err());
}
