use gestor_do_clube_lib::models::payment::{Payment, create_payment, get_payments, get_payment_by_member_month};
use gestor_do_clube_lib::models::member::create_member;
use gestor_do_clube_lib::db::schema::initialize_schema;
use rusqlite::Connection;

#[test]
fn test_create_and_get_payment() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let member_id = create_member(&conn, "Test Member", "2026-01-01").unwrap();

    let payment_id = create_payment(&conn, member_id, 5, 2026, 15.0, "2026-05-10").unwrap();
    let payment = get_payment_by_member_month(&conn, member_id, 5, 2026).unwrap();

    assert_eq!(payment.member_id, member_id);
    assert_eq!(payment.month, 5);
    assert_eq!(payment.year, 2026);
    assert_eq!(payment.amount_brl, 15.0);
}

#[test]
fn test_get_all_payments() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let member1 = create_member(&conn, "Member 1", "2026-01-01").unwrap();
    let member2 = create_member(&conn, "Member 2", "2026-01-01").unwrap();

    create_payment(&conn, member1, 5, 2026, 15.0, "2026-05-10").unwrap();
    create_payment(&conn, member2, 5, 2026, 20.0, "2026-05-12").unwrap();

    let payments = get_payments(&conn).unwrap();
    assert_eq!(payments.len(), 2);
}

#[test]
fn test_duplicate_payment_prevented() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let member_id = create_member(&conn, "Test Member", "2026-01-01").unwrap();

    create_payment(&conn, member_id, 5, 2026, 15.0, "2026-05-10").unwrap();
    let result = create_payment(&conn, member_id, 5, 2026, 15.0, "2026-05-11");

    assert!(result.is_err()); // Should fail due to UNIQUE constraint
}
