use gestor_do_clube_lib::models::member::{create_member, get_members, get_member_by_id};
use gestor_do_clube_lib::db::schema::initialize_schema;
use rusqlite::Connection;

#[test]
fn test_create_and_get_member() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    let member_id = create_member(&conn, "João Silva", "2026-01-15").unwrap();
    let member = get_member_by_id(&conn, member_id).unwrap();

    assert_eq!(member.name, "João Silva");
    assert_eq!(member.start_date, "2026-01-15");
    assert_eq!(member.active, true);
}

#[test]
fn test_get_all_members() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();

    create_member(&conn, "Member 1", "2026-01-01").unwrap();
    create_member(&conn, "Member 2", "2026-02-01").unwrap();

    let members = get_members(&conn).unwrap();
    assert_eq!(members.len(), 2);
}
