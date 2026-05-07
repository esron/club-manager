pub mod security;
pub mod db;
pub mod models;
pub mod commands;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init());

    #[cfg(debug_assertions)]
    {
        builder = builder.invoke_handler(tauri::generate_handler![
            greet,
            commands::auth::check_first_launch,
            commands::auth::setup_password,
            commands::auth::verify_password_cmd,
            commands::debt::get_member_debt_cmd,
            commands::debt::get_all_debts_cmd,
            commands::member::add_member_cmd,
            commands::member::get_members_cmd,
            commands::member::get_all_members_cmd,
            commands::member::get_member_cmd,
            commands::member::update_member_active_cmd,
            commands::member::update_member_name_cmd,
            commands::payment::add_payment_cmd,
            commands::payment::get_payments_cmd,
            commands::payment::delete_payment_cmd,
            commands::reports::get_debt_status_report_cmd,
            commands::reports::get_payment_history_report_cmd,
            commands::reports::export_debt_status_csv_cmd,
            commands::reports::export_payment_history_csv_cmd,
            commands::reports::export_debt_status_xlsx_cmd,
            commands::reports::export_payment_history_xlsx_cmd,
            commands::database::check_database_initialized,
            commands::database::initialize_database,
            commands::settings::get_setting_cmd,
            commands::settings::update_setting_cmd,
            commands::seed::seed_database,
        ]);
    }

    #[cfg(not(debug_assertions))]
    {
        builder = builder.invoke_handler(tauri::generate_handler![
            greet,
            commands::auth::check_first_launch,
            commands::auth::setup_password,
            commands::auth::verify_password_cmd,
            commands::debt::get_member_debt_cmd,
            commands::debt::get_all_debts_cmd,
            commands::member::add_member_cmd,
            commands::member::get_members_cmd,
            commands::member::get_all_members_cmd,
            commands::member::get_member_cmd,
            commands::member::update_member_active_cmd,
            commands::member::update_member_name_cmd,
            commands::payment::add_payment_cmd,
            commands::payment::get_payments_cmd,
            commands::payment::delete_payment_cmd,
            commands::reports::get_debt_status_report_cmd,
            commands::reports::get_payment_history_report_cmd,
            commands::reports::export_debt_status_csv_cmd,
            commands::reports::export_payment_history_csv_cmd,
            commands::reports::export_debt_status_xlsx_cmd,
            commands::reports::export_payment_history_xlsx_cmd,
            commands::database::check_database_initialized,
            commands::database::initialize_database,
            commands::settings::get_setting_cmd,
            commands::settings::update_setting_cmd,
        ]);
    }

    builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
