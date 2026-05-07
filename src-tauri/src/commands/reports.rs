use crate::models::reports::{
    generate_debt_status_report, generate_payment_history_report,
    DebtStatusReport, PaymentHistoryReport,
    anonymize_report_debt, anonymize_report_payment,
};
use crate::security::config::load_config;
use crate::security::password::derive_encryption_key;
use crate::db::connection::open_encrypted_db;
use std::path::PathBuf;
use csv::Writer;
use rust_xlsxwriter::{Workbook, Format, Color};

#[tauri::command]
pub fn get_debt_status_report_cmd(
    password: String,
    include_inactive: bool,
) -> Result<DebtStatusReport, String> {
    let conn = get_authenticated_connection(&password)?;

    generate_debt_status_report(&conn, include_inactive)
        .map_err(|e| format!("Failed to generate debt status report: {}", e))
}

#[tauri::command]
pub fn get_payment_history_report_cmd(
    password: String,
    start_date: String,
    end_date: String,
) -> Result<PaymentHistoryReport, String> {
    let conn = get_authenticated_connection(&password)?;

    generate_payment_history_report(&conn, &start_date, &end_date)
        .map_err(|e| format!("Failed to generate payment history report: {}", e))
}

fn get_authenticated_connection(password: &str) -> Result<rusqlite::Connection, String> {
    let config_path = get_config_path();
    let config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    let key_bytes = derive_encryption_key(password, &config.salt)
        .map_err(|e| format!("Failed to derive key: {}", e))?;
    let key_hex = hex::encode(&key_bytes);

    let db_path = get_db_path();
    open_encrypted_db(&db_path, &key_hex)
        .map_err(|e| format!("Failed to open database: {}", e))
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("GestorDoClube");
    std::fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}

fn get_db_path() -> PathBuf {
    let mut path = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("GestorDoClube");
    std::fs::create_dir_all(&path).ok();
    path.push("club.db");
    path
}

fn export_debt_status_csv(
    report: &DebtStatusReport,
    file_path: &str,
) -> Result<(), String> {
    let mut wtr = Writer::from_path(file_path)
        .map_err(|e| format!("Failed to create CSV file: {}", e))?;

    // Write UTF-8 BOM for Excel compatibility
    wtr.write_record(&["\u{FEFF}"])
        .map_err(|e| format!("Failed to write BOM: {}", e))?;

    // Write header
    wtr.write_record(&["Nome do Membro", "Dívida Total (R$)", "Meses Não Pagos"])
        .map_err(|e| format!("Failed to write header: {}", e))?;

    // Write rows
    for row in &report.members {
        wtr.write_record(&[
            &row.member_name,
            &format!("R$ {:.2}", row.total_debt).replace('.', ","),
            &row.unpaid_month_count.to_string(),
        ])
        .map_err(|e| format!("Failed to write row: {}", e))?;
    }

    wtr.flush()
        .map_err(|e| format!("Failed to flush CSV: {}", e))?;

    Ok(())
}

fn export_payment_history_csv(
    report: &PaymentHistoryReport,
    file_path: &str,
) -> Result<(), String> {
    let mut wtr = Writer::from_path(file_path)
        .map_err(|e| format!("Failed to create CSV file: {}", e))?;

    // Write UTF-8 BOM for Excel compatibility
    wtr.write_record(&["\u{FEFF}"])
        .map_err(|e| format!("Failed to write BOM: {}", e))?;

    // Build header
    let mut header = vec!["Nome do Membro".to_string(), "Início".to_string()];
    for col in &report.month_columns {
        header.push(col.display.clone());
    }
    wtr.write_record(&header)
        .map_err(|e| format!("Failed to write header: {}", e))?;

    // Write rows
    for row in &report.members {
        let mut record = vec![row.member_name.clone(), row.start_date.clone()];
        for col in &report.month_columns {
            record.push(row.payments.get(&col.key).cloned().unwrap_or_default());
        }
        wtr.write_record(&record)
            .map_err(|e| format!("Failed to write row: {}", e))?;
    }

    wtr.flush()
        .map_err(|e| format!("Failed to flush CSV: {}", e))?;

    Ok(())
}

fn export_debt_status_xlsx(
    report: &DebtStatusReport,
    file_path: &str,
) -> Result<(), String> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create header format
    let header_format = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x404040))
        .set_font_color(Color::White);

    // Create currency format
    let currency_format = Format::new()
        .set_num_format("R$ #,##0.00");

    // Write headers
    worksheet.write_with_format(0, 0, "Nome do Membro", &header_format)
        .map_err(|e| format!("Failed to write header: {}", e))?;
    worksheet.write_with_format(0, 1, "Dívida Total (R$)", &header_format)
        .map_err(|e| format!("Failed to write header: {}", e))?;
    worksheet.write_with_format(0, 2, "Meses Não Pagos", &header_format)
        .map_err(|e| format!("Failed to write header: {}", e))?;

    // Write data
    for (idx, row) in report.members.iter().enumerate() {
        let row_num = (idx + 1) as u32;
        worksheet.write(row_num, 0, &row.member_name)
            .map_err(|e| format!("Failed to write data: {}", e))?;
        worksheet.write_with_format(row_num, 1, row.total_debt, &currency_format)
            .map_err(|e| format!("Failed to write data: {}", e))?;
        worksheet.write(row_num, 2, row.unpaid_month_count)
            .map_err(|e| format!("Failed to write data: {}", e))?;
    }

    workbook.save(file_path)
        .map_err(|e| format!("Failed to save XLSX: {}", e))?;

    Ok(())
}

fn export_payment_history_xlsx(
    report: &PaymentHistoryReport,
    file_path: &str,
) -> Result<(), String> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    let header_format = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x404040))
        .set_font_color(Color::White);

    // Write headers
    worksheet.write_with_format(0, 0, "Nome do Membro", &header_format)
        .map_err(|e| format!("Failed to write header: {}", e))?;
    worksheet.write_with_format(0, 1, "Início", &header_format)
        .map_err(|e| format!("Failed to write header: {}", e))?;

    for (idx, col) in report.month_columns.iter().enumerate() {
        worksheet.write_with_format(0, (idx + 2) as u16, &col.display, &header_format)
            .map_err(|e| format!("Failed to write header: {}", e))?;
    }

    // Write data
    for (row_idx, row) in report.members.iter().enumerate() {
        let row_num = (row_idx + 1) as u32;
        worksheet.write(row_num, 0, &row.member_name)
            .map_err(|e| format!("Failed to write data: {}", e))?;
        worksheet.write(row_num, 1, &row.start_date)
            .map_err(|e| format!("Failed to write data: {}", e))?;

        for (col_idx, col) in report.month_columns.iter().enumerate() {
            let value = row.payments.get(&col.key).cloned().unwrap_or_default();
            worksheet.write(row_num, (col_idx + 2) as u16, value)
                .map_err(|e| format!("Failed to write data: {}", e))?;
        }
    }

    workbook.save(file_path)
        .map_err(|e| format!("Failed to save XLSX: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn export_debt_status_csv_cmd(
    password: String,
    include_inactive: bool,
    anonymize: bool,
    file_path: String,
) -> Result<(), String> {
    let conn = get_authenticated_connection(&password)?;
    let mut report = generate_debt_status_report(&conn, include_inactive)
        .map_err(|e| format!("Failed to generate report: {}", e))?;

    if anonymize {
        report = anonymize_report_debt(report);
    }

    export_debt_status_csv(&report, &file_path)?;
    Ok(())
}

#[tauri::command]
pub fn export_payment_history_csv_cmd(
    password: String,
    start_date: String,
    end_date: String,
    anonymize: bool,
    file_path: String,
) -> Result<(), String> {
    let conn = get_authenticated_connection(&password)?;
    let mut report = generate_payment_history_report(&conn, &start_date, &end_date)
        .map_err(|e| format!("Failed to generate report: {}", e))?;

    if anonymize {
        report = anonymize_report_payment(report);
    }

    export_payment_history_csv(&report, &file_path)?;
    Ok(())
}

#[tauri::command]
pub fn export_debt_status_xlsx_cmd(
    password: String,
    include_inactive: bool,
    anonymize: bool,
    file_path: String,
) -> Result<(), String> {
    let conn = get_authenticated_connection(&password)?;
    let mut report = generate_debt_status_report(&conn, include_inactive)
        .map_err(|e| format!("Failed to generate report: {}", e))?;

    if anonymize {
        report = anonymize_report_debt(report);
    }

    export_debt_status_xlsx(&report, &file_path)?;
    Ok(())
}

#[tauri::command]
pub fn export_payment_history_xlsx_cmd(
    password: String,
    start_date: String,
    end_date: String,
    anonymize: bool,
    file_path: String,
) -> Result<(), String> {
    let conn = get_authenticated_connection(&password)?;
    let mut report = generate_payment_history_report(&conn, &start_date, &end_date)
        .map_err(|e| format!("Failed to generate report: {}", e))?;

    if anonymize {
        report = anonymize_report_payment(report);
    }

    export_payment_history_xlsx(&report, &file_path)?;
    Ok(())
}
