// Debt Status Report
export interface DebtStatusReport {
  members: DebtStatusRow[];
  generated_at: string;
}

export interface DebtStatusRow {
  member_id: number;
  member_name: string;
  total_debt: number;
  unpaid_month_count: number;
}

// Payment History Report (Matrix)
export interface PaymentHistoryReport {
  members: PaymentHistoryRow[];
  month_columns: MonthColumn[];
  generated_at: string;
}

export interface PaymentHistoryRow {
  member_id: number;
  member_name: string;
  start_date: string;
  payments: { [monthKey: string]: string };
}

export interface MonthColumn {
  key: string;        // "2026-01"
  display: string;    // "Jan/2026"
}

export type ReportType = 'debt_status' | 'payment_history';
export type ExportFormat = 'csv' | 'xlsx';

export interface ReportConfig {
  reportType: ReportType;
  format: ExportFormat;
  anonymize: boolean;
  includeInactive?: boolean;  // For debt status
  startDate?: string;         // For payment history (YYYY-MM-DD)
  endDate?: string;           // For payment history (YYYY-MM-DD)
}
