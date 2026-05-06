use gestor_do_clube_lib::models::reports::{
    generate_debt_status_report,
    generate_payment_history_report,
    anonymize_report_debt,
    anonymize_report_payment,
};
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

// Edge case tests below

#[test]
fn test_grace_period_boundary() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();
    update_setting(&conn, "minimum_fee_brl", "15.00").unwrap();

    // Test grace period logic: payments are due by the 10th of the NEXT month
    // January fee is due by February 10th
    // Since we can't mock chrono::Local::now(), we test the logic indirectly

    // Scenario 1: Member started Jan 1, current date is May 6, 2026
    // Jan fee due by Feb 10 - PAST (debt)
    // Feb fee due by Mar 10 - PAST (debt)
    // Mar fee due by Apr 10 - PAST (debt)
    // Apr fee due by May 10 - NOT PAST (no debt yet, we're on May 6)
    // May fee due by Jun 10 - NOT PAST (no debt)

    let member1 = create_member(&conn, "Member Without Payment", "2026-01-01").unwrap();

    // Scenario 2: Member with one payment in January (should reduce debt)
    let member2 = create_member(&conn, "Member With Payment", "2026-01-01").unwrap();
    create_payment(&conn, member2, 1, 2026, 15.0, "2026-01-05").unwrap();

    let report = generate_debt_status_report(&conn, false).unwrap();
    assert_eq!(report.members.len(), 2);

    // Find each member in the report (they may be sorted)
    let member1_debt = report.members.iter()
        .find(|m| m.member_id == member1)
        .map(|m| m.total_debt)
        .unwrap();
    let member2_debt = report.members.iter()
        .find(|m| m.member_id == member2)
        .map(|m| m.total_debt)
        .unwrap();

    // Member 1: Should have debt for Jan, Feb, Mar (3 months @ 15.00 = 45.00)
    // Apr may or may not count depending on exact current date vs May 10
    assert!(member1_debt >= 45.0, "Member without payments should have at least 3 months debt, got: {}", member1_debt);
    assert!(member1_debt <= 60.0, "Member should have at most 4 months debt, got: {}", member1_debt);

    // Member 2: Should have one less month of debt (paid January)
    assert!(member2_debt >= 30.0, "Member with 1 payment should have at least 2 months debt, got: {}", member2_debt);
    assert!(member2_debt <= 45.0, "Member with 1 payment should have at most 3 months debt, got: {}", member2_debt);

    // Member 2 should have 15.00 less debt than Member 1
    assert_eq!(member1_debt - member2_debt, 15.0, "Debt difference should be exactly one month's fee");
}

#[test]
fn test_anonymize_debt_report() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();
    update_setting(&conn, "minimum_fee_brl", "15.00").unwrap();

    create_member(&conn, "Alice Smith", "2026-01-01").unwrap();
    create_member(&conn, "Bob Jones", "2026-01-01").unwrap();
    create_member(&conn, "Carol White", "2026-01-01").unwrap();

    let report = generate_debt_status_report(&conn, false).unwrap();
    let anonymized = anonymize_report_debt(report);

    // Verify all names are anonymized
    assert_eq!(anonymized.members.len(), 3);
    assert_eq!(anonymized.members[0].member_name, "Membro #1");
    assert_eq!(anonymized.members[1].member_name, "Membro #2");
    assert_eq!(anonymized.members[2].member_name, "Membro #3");
}

#[test]
fn test_anonymize_payment_report() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let member1 = create_member(&conn, "Alice Smith", "2026-01-01").unwrap();
    let member2 = create_member(&conn, "Bob Jones", "2026-01-01").unwrap();

    create_payment(&conn, member1, 1, 2026, 15.0, "2026-01-05").unwrap();
    create_payment(&conn, member2, 1, 2026, 20.0, "2026-01-06").unwrap();

    let report = generate_payment_history_report(&conn, "2026-01-01", "2026-01-31").unwrap();
    let anonymized = anonymize_report_payment(report);

    // Verify all names are anonymized
    assert_eq!(anonymized.members.len(), 2);
    assert_eq!(anonymized.members[0].member_name, "Membro #1");
    assert_eq!(anonymized.members[1].member_name, "Membro #2");

    // Verify payment data is preserved
    assert_eq!(anonymized.members[0].payments.get("2026-01").unwrap(), "R$ 15,00");
    assert_eq!(anonymized.members[1].payments.get("2026-01").unwrap(), "R$ 20,00");
}

#[test]
fn test_payment_history_year_transition() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let member_id = create_member(&conn, "Test Member", "2025-11-01").unwrap();

    // Create payments spanning December to January
    create_payment(&conn, member_id, 12, 2025, 15.0, "2025-12-05").unwrap();
    create_payment(&conn, member_id, 1, 2026, 20.0, "2026-01-08").unwrap();

    let report = generate_payment_history_report(&conn, "2025-12-01", "2026-02-28").unwrap();

    // Verify month columns span the year boundary correctly
    assert_eq!(report.month_columns.len(), 3);
    assert_eq!(report.month_columns[0].key, "2025-12");
    assert_eq!(report.month_columns[0].display, "Dez/2025");
    assert_eq!(report.month_columns[1].key, "2026-01");
    assert_eq!(report.month_columns[1].display, "Jan/2026");
    assert_eq!(report.month_columns[2].key, "2026-02");
    assert_eq!(report.month_columns[2].display, "Fev/2026");

    // Verify payments are in the correct months
    assert_eq!(report.members.len(), 1);
    assert_eq!(report.members[0].payments.get("2025-12").unwrap(), "R$ 15,00");
    assert_eq!(report.members[0].payments.get("2026-01").unwrap(), "R$ 20,00");
    assert_eq!(report.members[0].payments.get("2026-02").unwrap(), "-");
}

#[test]
fn test_payment_history_varying_amounts() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let member1 = create_member(&conn, "Member 1", "2026-01-01").unwrap();
    let member2 = create_member(&conn, "Member 2", "2026-01-01").unwrap();

    // Create payments with different amounts
    create_payment(&conn, member1, 1, 2026, 15.0, "2026-01-05").unwrap();
    create_payment(&conn, member1, 2, 2026, 20.0, "2026-02-05").unwrap();
    create_payment(&conn, member1, 3, 2026, 25.50, "2026-03-05").unwrap();

    create_payment(&conn, member2, 1, 2026, 30.0, "2026-01-07").unwrap();
    create_payment(&conn, member2, 2, 2026, 15.0, "2026-02-08").unwrap();
    // Member2 skips March

    let report = generate_payment_history_report(&conn, "2026-01-01", "2026-03-31").unwrap();

    assert_eq!(report.members.len(), 2);

    // Verify Member 1's varying amounts
    assert_eq!(report.members[0].payments.get("2026-01").unwrap(), "R$ 15,00");
    assert_eq!(report.members[0].payments.get("2026-02").unwrap(), "R$ 20,00");
    assert_eq!(report.members[0].payments.get("2026-03").unwrap(), "R$ 25,50");

    // Verify Member 2's amounts
    assert_eq!(report.members[1].payments.get("2026-01").unwrap(), "R$ 30,00");
    assert_eq!(report.members[1].payments.get("2026-02").unwrap(), "R$ 15,00");
    assert_eq!(report.members[1].payments.get("2026-03").unwrap(), "-");
}

#[test]
fn test_debt_report_sorted_by_debt_descending() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();
    update_setting(&conn, "minimum_fee_brl", "15.00").unwrap();

    // Create members with different start dates to generate different debt amounts
    let _member1 = create_member(&conn, "Recent Member", "2026-04-01").unwrap();
    let member2 = create_member(&conn, "Old Member", "2026-01-01").unwrap();
    let _member3 = create_member(&conn, "Mid Member", "2026-02-01").unwrap();

    // Add partial payment to member2 to reduce their debt slightly
    create_payment(&conn, member2, 1, 2026, 15.0, "2026-01-05").unwrap();

    let report = generate_debt_status_report(&conn, false).unwrap();

    // Verify sorting: members should be ordered by debt descending
    assert_eq!(report.members.len(), 3);

    // Verify that debt amounts are in descending order
    for i in 0..report.members.len() - 1 {
        assert!(
            report.members[i].total_debt >= report.members[i + 1].total_debt,
            "Members should be sorted by debt descending: {} (debt: {}) should be >= {} (debt: {})",
            report.members[i].member_name,
            report.members[i].total_debt,
            report.members[i + 1].member_name,
            report.members[i + 1].total_debt
        );
    }

    // Verify highest debt is at index 0 and lowest at the end
    // Mid Member (started Feb, no payments): ~3 months debt
    // Old Member (started Jan, 1 payment): ~2 months debt
    // Recent Member (started Apr, no payments): ~0-1 months debt
    assert!(report.members[0].total_debt > 0.0, "First member should have the highest debt");
    assert!(report.members[0].total_debt >= report.members[1].total_debt);
    assert!(report.members[1].total_debt >= report.members[2].total_debt);
}
