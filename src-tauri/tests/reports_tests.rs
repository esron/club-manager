use gestor_do_clube_lib::models::reports::{
    generate_debt_status_report,
    generate_debt_status_report_as_of,
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
    // Test at Feb 9 (within grace) vs Feb 10 (deadline reached)

    let member1 = create_member(&conn, "Member Without Payment", "2026-01-01").unwrap();
    let member2 = create_member(&conn, "Member With Payment", "2026-01-01").unwrap();
    create_payment(&conn, member2, 1, 2026, 15.0, "2026-01-05").unwrap();

    // Test on Feb 9 - still within grace period for January
    let report_feb9 = generate_debt_status_report_as_of(&conn, false, Some("2026-02-09")).unwrap();
    assert_eq!(report_feb9.members.len(), 2);

    let member1_debt_feb9 = report_feb9.members.iter()
        .find(|m| m.member_id == member1)
        .map(|m| m.total_debt)
        .unwrap();
    let member2_debt_feb9 = report_feb9.members.iter()
        .find(|m| m.member_id == member2)
        .map(|m| m.total_debt)
        .unwrap();

    // On Feb 9, January is still in grace period (due by Feb 10)
    assert_eq!(member1_debt_feb9, 0.0, "Member should have no debt on Feb 9 (grace period)");
    assert_eq!(member2_debt_feb9, 0.0, "Member should have no debt on Feb 9 (grace period)");

    // Test on Feb 10 - grace period deadline reached
    let report_feb10 = generate_debt_status_report_as_of(&conn, false, Some("2026-02-10")).unwrap();

    let member1_debt_feb10 = report_feb10.members.iter()
        .find(|m| m.member_id == member1)
        .map(|m| m.total_debt)
        .unwrap();
    let member2_debt_feb10 = report_feb10.members.iter()
        .find(|m| m.member_id == member2)
        .map(|m| m.total_debt)
        .unwrap();

    // On Feb 10, grace period deadline reached (as_of > deadline is false, so still no debt)
    // The logic uses `as_of > deadline`, so Feb 10 is NOT past the deadline yet
    assert_eq!(member1_debt_feb10, 0.0, "Member should have no debt on Feb 10 (boundary)");
    assert_eq!(member2_debt_feb10, 0.0, "Member should have no debt on Feb 10 (boundary)");

    // Test on Feb 11 - past the grace period deadline
    let report_feb11 = generate_debt_status_report_as_of(&conn, false, Some("2026-02-11")).unwrap();

    let member1_debt_feb11 = report_feb11.members.iter()
        .find(|m| m.member_id == member1)
        .map(|m| m.total_debt)
        .unwrap();
    let member2_debt_feb11 = report_feb11.members.iter()
        .find(|m| m.member_id == member2)
        .map(|m| m.total_debt)
        .unwrap();

    // On Feb 11, past the grace period (Jan fee is now debt)
    assert_eq!(member1_debt_feb11, 15.0, "Member should have 1 month debt on Feb 11 (past grace period)");
    assert_eq!(member2_debt_feb11, 0.0, "Member with payment should have no debt on Feb 11");
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
    // Test as of May 11, 2026
    let member1 = create_member(&conn, "Recent Member", "2026-04-01").unwrap();
    let member2 = create_member(&conn, "Old Member", "2026-01-01").unwrap();
    let member3 = create_member(&conn, "Mid Member", "2026-02-01").unwrap();

    // Add partial payment to member2 to reduce their debt slightly
    create_payment(&conn, member2, 1, 2026, 15.0, "2026-01-05").unwrap();

    // Generate report as of May 11, 2026
    // Recent Member (started Apr 1): Apr fee due by May 10 - PAST on May 11 (1 month debt = 15.00)
    // Old Member (started Jan 1, paid Jan): Feb, Mar, Apr fees past grace - 3 months debt = 45.00
    // Mid Member (started Feb 1, no payments): Feb, Mar, Apr fees past grace - 3 months debt = 45.00
    let report = generate_debt_status_report_as_of(&conn, false, Some("2026-05-11")).unwrap();

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

    // Verify exact debt amounts
    // Old Member and Mid Member should both have 45.00 (3 months)
    // Recent Member should have 15.00 (1 month)
    let old_member_debt = report.members.iter()
        .find(|m| m.member_id == member2)
        .map(|m| m.total_debt)
        .unwrap();
    let mid_member_debt = report.members.iter()
        .find(|m| m.member_id == member3)
        .map(|m| m.total_debt)
        .unwrap();
    let recent_member_debt = report.members.iter()
        .find(|m| m.member_id == member1)
        .map(|m| m.total_debt)
        .unwrap();

    assert_eq!(old_member_debt, 45.0, "Old Member should have 3 months debt");
    assert_eq!(mid_member_debt, 45.0, "Mid Member should have 3 months debt");
    assert_eq!(recent_member_debt, 15.0, "Recent Member should have 1 month debt");

    // Verify the first two have highest debt (45.00) and last has lowest (15.00)
    assert_eq!(report.members[0].total_debt, 45.0);
    assert_eq!(report.members[1].total_debt, 45.0);
    assert_eq!(report.members[2].total_debt, 15.0);
}
