use gestor_do_clube_lib::models::reports::{generate_debt_status_report, generate_payment_history_report};
use gestor_do_clube_lib::models::member::create_member;
use gestor_do_clube_lib::models::payment::create_payment;
use gestor_do_clube_lib::models::settings::update_setting;
use gestor_do_clube_lib::db::schema::initialize_schema;
use rusqlite::Connection;

#[test]
fn test_debt_status_report_empty() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();
    update_setting(&conn, "minimum_fee_brl", "15.00").unwrap();

    let report = generate_debt_status_report(&conn, false).unwrap();
    assert_eq!(report.members.len(), 0);
}

#[test]
fn test_debt_status_report_with_debt() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();
    update_setting(&conn, "minimum_fee_brl", "15.00").unwrap();

    let _member_id = create_member(&conn, "Test Member", "2026-01-01").unwrap();
    // No payments - will have debt

    let report = generate_debt_status_report(&conn, false).unwrap();
    assert_eq!(report.members.len(), 1);
    assert_eq!(report.members[0].member_name, "Test Member");
    assert!(report.members[0].total_debt > 0.0);
    assert!(report.members[0].unpaid_month_count > 0);
}

#[test]
fn test_debt_status_excludes_inactive() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();
    update_setting(&conn, "minimum_fee_brl", "15.00").unwrap();

    let member_id = create_member(&conn, "Active Member", "2026-01-01").unwrap();
    let inactive_id = create_member(&conn, "Inactive Member", "2026-01-01").unwrap();

    // Deactivate second member
    conn.execute("UPDATE members SET active = 0 WHERE id = ?", [inactive_id]).unwrap();

    let report = generate_debt_status_report(&conn, false).unwrap();
    assert_eq!(report.members.len(), 1);
    assert_eq!(report.members[0].member_id, member_id);
}

#[test]
fn test_debt_status_includes_inactive() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();
    update_setting(&conn, "minimum_fee_brl", "15.00").unwrap();

    create_member(&conn, "Active Member", "2026-01-01").unwrap();
    let inactive_id = create_member(&conn, "Inactive Member", "2026-01-01").unwrap();

    conn.execute("UPDATE members SET active = 0 WHERE id = ?", [inactive_id]).unwrap();

    let report = generate_debt_status_report(&conn, true).unwrap();
    assert_eq!(report.members.len(), 2);
}

#[test]
fn test_payment_history_report_single_month() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let member_id = create_member(&conn, "Test", "2026-01-01").unwrap();
    create_payment(&conn, member_id, 1, 2026, 15.0, "2026-01-05").unwrap();

    let report = generate_payment_history_report(&conn, "2026-01-01", "2026-01-31").unwrap();

    assert_eq!(report.month_columns.len(), 1);
    assert_eq!(report.month_columns[0].display, "Jan/2026");
    assert_eq!(report.members.len(), 1);
    assert_eq!(report.members[0].payments.get("2026-01").unwrap(), "R$ 15,00");
}

#[test]
fn test_payment_history_multi_month() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let member_id = create_member(&conn, "Test", "2026-01-01").unwrap();
    create_payment(&conn, member_id, 1, 2026, 15.0, "2026-01-05").unwrap();

    let report = generate_payment_history_report(&conn, "2026-01-01", "2026-03-31").unwrap();

    assert_eq!(report.month_columns.len(), 3);
    assert_eq!(report.month_columns[0].display, "Jan/2026");
    assert_eq!(report.month_columns[1].display, "Fev/2026");
    assert_eq!(report.month_columns[2].display, "Mar/2026");

    assert_eq!(report.members[0].payments.get("2026-01").unwrap(), "R$ 15,00");
    assert_eq!(report.members[0].payments.get("2026-02").unwrap(), "-");
    assert_eq!(report.members[0].payments.get("2026-03").unwrap(), "-");
}

#[test]
fn test_payment_history_member_not_started() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let _member_id = create_member(&conn, "Test", "2026-03-01").unwrap();

    let report = generate_payment_history_report(&conn, "2026-01-01", "2026-03-31").unwrap();

    // Jan and Feb should be blank (member not started)
    assert_eq!(report.members[0].payments.get("2026-01").unwrap(), "");
    assert_eq!(report.members[0].payments.get("2026-02").unwrap(), "");
    assert_eq!(report.members[0].payments.get("2026-03").unwrap(), "-");
}
