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
use std::fs::File;
use std::io::Write as IoWrite;

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
    // Create file and write UTF-8 BOM for Excel compatibility
    let mut file = File::create(file_path)
        .map_err(|e| format!("Failed to create CSV file: {}", e))?;
    file.write_all(&[0xEF, 0xBB, 0xBF])
        .map_err(|e| format!("Failed to write BOM: {}", e))?;

    // Create CSV writer from file
    let mut wtr = Writer::from_writer(file);

    // Write header
    wtr.write_record(&["Nome do Membro", "Dívida Total (R$)", "Meses Não Pagos"])
        .map_err(|e| format!("Failed to write header: {}", e))?;

    // Write rows and calculate total
    let mut total_debt = 0.0;
    for row in &report.members {
        wtr.write_record(&[
            &row.member_name,
            &format!("R$ {:.2}", row.total_debt).replace('.', ","),
            &row.unpaid_month_count.to_string(),
        ])
        .map_err(|e| format!("Failed to write row: {}", e))?;
        total_debt += row.total_debt;
    }

    // Write totals row
    wtr.write_record(&[
        "TOTAL",
        &format!("R$ {:.2}", total_debt).replace('.', ","),
        "",
    ])
    .map_err(|e| format!("Failed to write totals row: {}", e))?;

    wtr.flush()
        .map_err(|e| format!("Failed to flush CSV: {}", e))?;

    Ok(())
}

fn export_payment_history_csv(
    report: &PaymentHistoryReport,
    file_path: &str,
) -> Result<(), String> {
    // Create file and write UTF-8 BOM for Excel compatibility
    let mut file = File::create(file_path)
        .map_err(|e| format!("Failed to create CSV file: {}", e))?;
    file.write_all(&[0xEF, 0xBB, 0xBF])
        .map_err(|e| format!("Failed to write BOM: {}", e))?;

    // Create CSV writer from file
    let mut wtr = Writer::from_writer(file);

    // Build header
    let mut header = vec!["Nome do Membro".to_string(), "Início".to_string()];
    for col in &report.month_columns {
        header.push(col.display.clone());
    }
    wtr.write_record(&header)
        .map_err(|e| format!("Failed to write header: {}", e))?;

    // Initialize totals for each month
    let mut month_totals: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    for col in &report.month_columns {
        month_totals.insert(col.key.clone(), 0.0);
    }

    // Write rows and calculate totals
    for row in &report.members {
        let mut record = vec![row.member_name.clone(), row.start_date.clone()];
        for col in &report.month_columns {
            let payment_str = row.payments.get(&col.key).cloned().unwrap_or_default();
            record.push(payment_str.clone());

            // Extract value and add to total
            if let Some(value) = extract_currency_value(&payment_str) {
                *month_totals.get_mut(&col.key).unwrap() += value;
            }
        }
        wtr.write_record(&record)
            .map_err(|e| format!("Failed to write row: {}", e))?;
    }

    // Write totals row
    let mut totals_record = vec!["TOTAL".to_string(), "".to_string()];
    for col in &report.month_columns {
        let total = month_totals.get(&col.key).unwrap_or(&0.0);
        if *total > 0.0 {
            totals_record.push(format!("R$ {:.2}", total).replace('.', ","));
        } else {
            totals_record.push(String::new());
        }
    }
    wtr.write_record(&totals_record)
        .map_err(|e| format!("Failed to write totals row: {}", e))?;

    wtr.flush()
        .map_err(|e| format!("Failed to flush CSV: {}", e))?;

    Ok(())
}

// Helper function to extract numeric value from currency string "R$ 15,00"
fn extract_currency_value(s: &str) -> Option<f64> {
    if s.is_empty() || s == "-" {
        return None;
    }

    // Remove "R$ " prefix and convert comma to dot
    let cleaned = s.replace("R$ ", "").replace(',', ".");
    cleaned.parse::<f64>().ok()
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

    // Create totals format (bold)
    let totals_format = Format::new()
        .set_bold();

    let totals_currency_format = Format::new()
        .set_num_format("R$ #,##0.00")
        .set_bold();

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

    // Write totals row
    let totals_row = (report.members.len() + 1) as u32;
    worksheet.write_with_format(totals_row, 0, "TOTAL", &totals_format)
        .map_err(|e| format!("Failed to write totals label: {}", e))?;

    // Use SUM formula for total debt
    if report.members.len() > 0 {
        let formula = format!("=SUM(B2:B{})", report.members.len() + 1);
        worksheet.write_formula_with_format(totals_row, 1, formula.as_str(), &totals_currency_format)
            .map_err(|e| format!("Failed to write totals formula: {}", e))?;
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

    let currency_format = Format::new()
        .set_num_format("R$ #,##0.00");

    let totals_format = Format::new()
        .set_bold();

    let totals_currency_format = Format::new()
        .set_num_format("R$ #,##0.00")
        .set_bold();

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
            let value_str = row.payments.get(&col.key).cloned().unwrap_or_default();

            // If there's a numeric value, write it as number with currency format
            if let Some(numeric_value) = extract_currency_value(&value_str) {
                worksheet.write_with_format(row_num, (col_idx + 2) as u16, numeric_value, &currency_format)
                    .map_err(|e| format!("Failed to write data: {}", e))?;
            } else {
                // Write dash or empty string as text
                worksheet.write(row_num, (col_idx + 2) as u16, &value_str)
                    .map_err(|e| format!("Failed to write data: {}", e))?;
            }
        }
    }

    // Write totals row
    if report.members.len() > 0 {
        let totals_row = (report.members.len() + 1) as u32;
        worksheet.write_with_format(totals_row, 0, "TOTAL", &totals_format)
            .map_err(|e| format!("Failed to write totals label: {}", e))?;

        // Write SUM formulas for each month column
        for (col_idx, _col) in report.month_columns.iter().enumerate() {
            let col_letter = excel_column_letter(col_idx + 2);
            let formula = format!("=SUM({}2:{}{})", col_letter, col_letter, report.members.len() + 1);
            worksheet.write_formula_with_format(totals_row, (col_idx + 2) as u16, formula.as_str(), &totals_currency_format)
                .map_err(|e| format!("Failed to write totals formula: {}", e))?;
        }
    }

    workbook.save(file_path)
        .map_err(|e| format!("Failed to save XLSX: {}", e))?;

    Ok(())
}

// Helper function to convert column index to Excel column letter
fn excel_column_letter(col_idx: usize) -> String {
    let mut col = col_idx;
    let mut result = String::new();

    while col >= 26 {
        result.insert(0, ((col % 26) as u8 + b'A') as char);
        col = col / 26 - 1;
    }
    result.insert(0, (col as u8 + b'A') as char);
    result
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
