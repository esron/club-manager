use gestor_do_clube_lib::models::debt::calculate_member_debt;
use gestor_do_clube_lib::models::member::create_member;
use gestor_do_clube_lib::models::payment::create_payment;
use gestor_do_clube_lib::db::schema::initialize_schema;
use rusqlite::Connection;

#[test]
fn test_no_debt_when_all_paid() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let member_id = create_member(&conn, "Test", "2026-01-01").unwrap();

    // Pay for January, February, March
    create_payment(&conn, member_id, 1, 2026, 15.0, "2026-01-05").unwrap();
    create_payment(&conn, member_id, 2, 2026, 15.0, "2026-02-05").unwrap();
    create_payment(&conn, member_id, 3, 2026, 15.0, "2026-03-05").unwrap();

    // Check debt as of 2026-04-15 (after grace period for March)
    let debt = calculate_member_debt(&conn, member_id, "2026-04-15").unwrap();
    assert_eq!(debt, 0.0);
}

#[test]
fn test_debt_accumulates() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let member_id = create_member(&conn, "Test", "2026-01-01").unwrap();

    // No payments
    // Check debt as of 2026-04-15
    // Should owe for: Jan (due Feb 10), Feb (due Mar 10), Mar (due Apr 10)
    let debt = calculate_member_debt(&conn, member_id, "2026-04-15").unwrap();
    assert_eq!(debt, 45.0); // 3 months × R$ 15
}

#[test]
fn test_grace_period_respected() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let member_id = create_member(&conn, "Test", "2026-03-01").unwrap();

    // Check on Apr 9 (before grace period ends on Apr 10)
    let debt = calculate_member_debt(&conn, member_id, "2026-04-09").unwrap();
    assert_eq!(debt, 0.0); // Grace period still active

    // Check on Apr 10 (grace period ends, but not "after")
    let debt2 = calculate_member_debt(&conn, member_id, "2026-04-10").unwrap();
    assert_eq!(debt2, 0.0); // Grace period ends at midnight

    // Check on Apr 11 (after grace period)
    let debt3 = calculate_member_debt(&conn, member_id, "2026-04-11").unwrap();
    assert_eq!(debt3, 15.0); // Now owes for March
}
