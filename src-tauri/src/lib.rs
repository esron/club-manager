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
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::auth::check_first_launch,
            commands::auth::setup_password,
            commands::auth::verify_password_cmd,
            commands::member::add_member_cmd,
            commands::member::get_members_cmd,
            commands::member::get_member_cmd,
            commands::payment::add_payment_cmd,
            commands::payment::get_payments_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
