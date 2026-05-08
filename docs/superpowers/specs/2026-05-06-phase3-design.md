# Gestor do Clube - Phase 3 Design Specification

**Version:** 1.0  
**Date:** 2026-05-06  
**Status:** Approved

## Overview

Phase 3 adds comprehensive export functionality to the club manager application. This phase introduces a dedicated Reports screen with two export types: Payment History (matrix format) and Debt Status (summary list). Both support CSV and XLSX formats, anonymization, and require re-authentication for security.

## Phase 3 Scope

### Features Included

1. **Reports Screen** - New dedicated tab in main navigation
2. **Re-Authentication Modal** - Password confirmation before export
3. **Debt Status Report** - Current member debt summary with optional inactive member inclusion
4. **Payment History Report** - Matrix-style payment grid with date range selection
5. **Dual Format Support** - CSV and XLSX export options
6. **Anonymization** - Optional name masking (Membro #1, #2, etc.) for both report types
7. **Preview Functionality** - Display report data before exporting

### Out of Scope (Future Phases)

- Password change functionality
- Charts and graphs
- Advanced filtering (search, sort in UI beyond what the reports provide)
- Scheduled/automated exports
- Email integration

## Implementation Approach

**Build Order (Incremental - Approach 1: Simple First):**

1. Reports tab UI skeleton (controls + preview area)
2. Re-authentication modal component
3. Debt Status report (simpler - validates full export pipeline)
4. Payment History report (complex matrix layout)

**Why this order:** Starting with the simpler Debt Status report de-risks the project by validating the entire export flow (UI → backend → file generation) before tackling the complex Payment History matrix generation. The preview functionality provides user confidence, and if matrix generation encounters issues, we still have a working export feature.

**Estimated timeline:** 4-5 implementation sessions

## Architecture

### Technology Stack

No changes from Phase 2:
- **Frontend:** React 18 + TypeScript, Tailwind CSS (dark theme)
- **Backend:** Tauri (Rust), SQLCipher database
- **State:** React Context API
- **Export Libraries:** csv crate, rust_xlsxwriter

### New Components

```
src/components/
  ├── ReportsScreen.tsx          (main reports tab UI)
  ├── ReAuthModal.tsx            (password confirmation before export)
  └── ReportPreviewTable.tsx     (displays preview of report data)
```

### New Backend Commands

```rust
// src-tauri/src/commands/reports.rs (new file)

// Debt Status Report
get_debt_status_report_cmd(
  password: String, 
  include_inactive: bool
) -> Result<DebtStatusReport, String>

// Payment History Report  
get_payment_history_report_cmd(
  password: String,
  start_date: String,  // "YYYY-MM-DD"
  end_date: String     // "YYYY-MM-DD"
) -> Result<PaymentHistoryReport, String>

// Export to file
export_report_cmd(
  password: String,
  report_data: ReportData,
  format: String,      // "csv" or "xlsx"
  anonymize: bool,
  file_path: String    // from file picker dialog
) -> Result<(), String>
```

### Data Structures

**Debt Status Report:**

```typescript
interface DebtStatusReport {
  members: DebtStatusRow[];
  generated_at: string;
}

interface DebtStatusRow {
  member_id: number;
  member_name: string;
  total_debt: number;
  unpaid_month_count: number;
}
```

**Payment History Report (Matrix):**

```typescript
interface PaymentHistoryReport {
  members: PaymentHistoryRow[];
  month_columns: MonthColumn[];  // The months to display as columns
  generated_at: string;
}

interface PaymentHistoryRow {
  member_id: number;
  member_name: string;
  start_date: string;  // Member's start date
  payments: { [monthKey: string]: string };  // e.g., {"2026-01": "R$ 15,00", "2026-02": "-"}
}

interface MonthColumn {
  key: string;        // "2026-01"
  display: string;    // "Jan/2026"
}
```

**State Management:**

```typescript
// Add to AppContext
interface AppContextType {
  // ... existing state ...
  showReAuthModal: boolean;
  setShowReAuthModal: (show: boolean) => void;
  pendingExport: (() => void) | null;
  setPendingExport: (fn: (() => void) | null) => void;
}
```

## Feature Specifications

### 1. Reports Screen UI/UX

**Layout Structure:**

The Reports screen follows the existing dark theme and is divided into two main sections:

**Top Section - Export Controls:**
- **Report Type selector** (radio buttons):
  - "Histórico de Pagamentos" (Payment History)
  - "Status de Dívidas" (Debt Status)
- **Configuration area** (changes based on selected report type):
  - **Payment History:** 
    - Start Date input (DD/MM/YYYY format)
    - End Date input (DD/MM/YYYY format)
  - **Debt Status:** 
    - Checkbox: "Incluir membros inativos"
- **Format selector** (radio buttons):
  - CSV
  - XLSX
- **Anonymize checkbox:** "Relatório anônimo (Membro #1, Membro #2, ...)"
- **Export button** (primary blue button): "Exportar"

**Bottom Section - Preview Area:**
- Shows preview table of the data that will be exported
- For Payment History: horizontally scrollable table with month columns
- For Debt Status: simple 3-column table
- Maximum 100 rows displayed in preview
- If more than 100 rows exist, show note: "Mostrando primeiras 100 linhas"
- Preview updates automatically when user changes filters/dates

**Navigation:**
- Add "Relatórios" tab to MainLayout navigation bar
- Position between existing tabs in logical order

### 2. Re-Authentication Flow

**Process:**
1. User configures report and clicks "Exportar" button
2. ReAuthModal appears as overlay/dialog
3. User enters password in password input field
4. On correct password:
   - Modal closes
   - File picker dialog opens (native Tauri dialog)
   - User chooses location and filename
   - Backend generates file
   - Success message: "Relatório exportado com sucesso!"
5. On wrong password:
   - Error message displayed: "Senha incorreta. Tente novamente."
   - Modal remains open for retry

**Security:**
- Password required before every export operation
- No session-based bypass
- Password not stored or cached

### 3. Debt Status Report

**Purpose:** Current snapshot of member debt status across all members.

**Backend Generation Logic:**
1. Authenticate password
2. Query all members (optionally filter out inactive based on `include_inactive` parameter)
3. For each member, calculate current debt using existing debt calculation logic
4. Return list sorted by debt amount descending (highest debt first)

**Data Columns:**
1. **Nome do Membro** - Member name (or "Membro #N" if anonymized)
2. **Dívida Total (R$)** - Total outstanding debt in BRL format (e.g., "R$ 45,00")
3. **Meses Não Pagos** - Count of unpaid months (e.g., "3")

**Export Format:**
- **CSV:** UTF-8 encoding with BOM (for Excel compatibility), comma-separated
- **XLSX:** Brazilian number formatting, header row with bold text
- Both formats use Brazilian Portuguese column names

### 4. Payment History Report

**Purpose:** Matrix-style view of payments across time periods, with one row per member and one column per month.

**Backend Generation Logic:**
1. Authenticate password
2. Validate start_date <= end_date
3. Generate list of all months between start and end dates (inclusive)
4. Query all active members with their start dates
5. For each member:
   - Get all payments in the date range
   - For each month column:
     - If month < member start date: leave cell blank (member not yet active)
     - If payment exists for that month: show amount "R$ XX,XX"
     - If no payment and member was active: show "-"
6. Return matrix data structure

**Data Columns:**
1. **Nome do Membro** - Member name (or "Membro #N" if anonymized)
2. **Início** - Member start date in DD/MM/YYYY format
3. **Dynamic month columns** - One column per month in date range
   - Column header format: "Mmm/YYYY" (e.g., "Jan/2026", "Fev/2026")
   - Cell values: payment amount "R$ 15,00" or "-" for unpaid

**Month Column Generation:**
- Portuguese month abbreviations: ["Jan", "Fev", "Mar", "Abr", "Mai", "Jun", "Jul", "Ago", "Set", "Out", "Nov", "Dez"]
- Generate from start_date to end_date inclusive
- Format: "Mmm/YYYY"

**Export Format:**
- **CSV:** First column "Nome do Membro", second column "Início", then month columns left-to-right
- **XLSX:** Same structure with additional formatting (aligned columns, header row styling)
- Horizontally scrollable if many month columns

### 5. Anonymization

**Behavior:**
- Applied at export time (not during report generation)
- Replace member names with "Membro #1", "Membro #2", etc.
- Numbering is consistent within a single report (same member = same number)
- Members sorted by member_id before assigning numbers (ensures consistency)
- Applies to both Debt Status and Payment History reports

**Purpose:**
- Allow sharing reports externally without exposing member identities
- Useful for showing club statistics to external auditors or stakeholders

## Error Handling & Validation

### Input Validation

**Frontend Validation (pre-submission):**
- Start Date and End Date required for Payment History report
- End Date must be >= Start Date
- Dates must be valid calendar dates
- Show inline error messages below invalid fields in red text
- Disable "Exportar" button while validation errors exist

**Backend Validation:**
- Password authentication (uses existing auth logic)
- Date string parsing - reject malformed date strings
- File path validation - ensure target directory is writable
- Report size check - warn if generating very large reports (>10,000 rows)

### Error Scenarios & Messages

| Scenario | User-Facing Message (Portuguese) |
|----------|----------------------------------|
| Wrong password in ReAuthModal | "Senha incorreta. Tente novamente." |
| Invalid date range (end < start) | "A data final deve ser posterior à data inicial." |
| No members in date range | "Nenhum membro encontrado no período selecionado." |
| File save fails (permission denied) | "Erro ao salvar arquivo. Verifique as permissões." |
| File save fails (disk full) | "Erro ao salvar arquivo. Espaço em disco insuficiente." |
| Database error during generation | "Erro ao gerar relatório. Tente novamente." |
| Export cancelled by user | No message - just close file picker dialog |

### Loading States

- Show loading spinner while generating preview (after user changes dates/filters)
- Show loading spinner in ReAuthModal during password verification
- Show progress indicator during file export:
  - "Gerando relatório..." (while backend generates data)
  - "Salvando arquivo..." (while writing to disk)
- Disable all controls while operations are in progress to prevent duplicate submissions

### Edge Cases

1. **Empty reports:** Display message "Nenhum dado disponível para o período selecionado" in preview area
2. **Very large date ranges:** Preview shows first 100 rows with note indicating truncation
3. **Member with no start date:** Should not happen (required field in member creation), but handle gracefully by using earliest possible date or skipping member
4. **Payment amount formatting:** Handle both integer (R$ 15,00) and decimal values correctly (R$ 15,50)
5. **Special characters in member names:** Ensure proper CSV escaping and XLSX encoding

## Testing Strategy

### Backend Unit Tests (Rust)

**Test coverage for `src-tauri/src/commands/reports.rs`:**

1. **Debt Status Report:**
   - Generate report with active members only
   - Generate report including inactive members
   - Verify sorting by debt amount (descending)
   - Handle empty member list

2. **Payment History Report:**
   - Single month date range
   - Multi-month date range within same year
   - Multi-year date range (e.g., Jan 2024 - Dec 2026)
   - Date range spanning year boundary (Nov 2025 - Feb 2026)
   - Member start date before report period
   - Member start date in middle of report period
   - Member start date after report period
   - All months paid scenario
   - No months paid scenario
   - Partial payments scenario

3. **Month Column Generation:**
   - Verify correct Portuguese month names
   - Verify correct ordering (chronological)
   - Handle year boundaries correctly

4. **Payment Matrix Population:**
   - Paid months show correct amount
   - Unpaid months show "-"
   - Pre-start-date months are blank
   - Amount formatting: "R$ 15,00" format

5. **Anonymization:**
   - Consistent numbering across same report
   - Proper sorting before numbering
   - Verify "Membro #N" format

6. **CSV Generation:**
   - Proper UTF-8 BOM for Excel compatibility
   - Comma separation
   - Proper escaping of special characters
   - Header row present

7. **XLSX Generation:**
   - Proper number formatting
   - Header row styling
   - Brazilian locale formatting (comma as decimal separator)

### Test Data Scenarios

1. Member with all months paid in range
2. Member with no payments in range
3. Member with partial payments (some paid, some unpaid)
4. Member start date before report period
5. Member start date in middle of report period
6. Member start date after report period
7. Report period before any members existed
8. Single month report
9. Multi-year report (e.g., Jan 2024 - Dec 2026)
10. Database with 0 members
11. Database with 100+ members (performance test)

### Manual Testing Checklist

**Before Phase 3 is considered complete:**

- [ ] Export Debt Status as CSV (complete mode)
- [ ] Export Debt Status as CSV (anonymized mode)
- [ ] Export Debt Status as XLSX (complete mode)
- [ ] Export Debt Status as XLSX (anonymized mode)
- [ ] Export Debt Status with inactive members included
- [ ] Export Debt Status with inactive members excluded
- [ ] Export Payment History as CSV (3-month range)
- [ ] Export Payment History as XLSX (1-year range)
- [ ] Export Payment History with multi-year range
- [ ] Verify wrong password is rejected in ReAuthModal
- [ ] Verify correct password allows export to proceed
- [ ] Test with 0 members (empty database scenario)
- [ ] Test with 100+ members (performance check)
- [ ] Open exported CSV in LibreOffice Calc (verify Portuguese characters display correctly)
- [ ] Open exported XLSX in Microsoft Excel (verify formatting)
- [ ] Open exported XLSX in LibreOffice Calc (verify compatibility)
- [ ] Cancel file picker dialog (ensure no error or crash)
- [ ] Verify "Include inactive members" toggle works correctly
- [ ] Verify preview updates when changing date range
- [ ] Verify preview updates when toggling inactive members
- [ ] Verify date validation (end date before start date shows error)
- [ ] Export button disabled during invalid input
- [ ] Test large date range (5+ years) for performance

## Implementation Notes

### Brazilian Formatting Standards

**Dates:**
- Input format: DD/MM/YYYY
- Display format: DD/MM/YYYY
- Internal storage: YYYY-MM-DD (ISO 8601)

**Currency:**
- Format: R$ 15,00 (space after R$, comma as decimal separator)
- Thousands separator: . (dot) for values >= R$ 1.000,00
- Example: R$ 1.234,56

**Month Names:**
- Full: Janeiro, Fevereiro, Março, Abril, Maio, Junho, Julho, Agosto, Setembro, Outubro, Novembro, Dezembro
- Abbreviated: Jan, Fev, Mar, Abr, Mai, Jun, Jul, Ago, Set, Out, Nov, Dez

### File Naming Suggestions

**Default filenames when opening file picker:**
- Debt Status: `relatorio-dividas-YYYY-MM-DD.csv` or `.xlsx`
- Payment History: `historico-pagamentos-YYYY-MM-DD.csv` or `.xlsx`
- Use generation date for YYYY-MM-DD portion

### Performance Considerations

**Large Reports:**
- Payment History with 100 members × 36 months (3 years) = 3,600 cells
- Preview limited to 100 rows to maintain UI responsiveness
- Backend should handle up to 500 members × 60 months without significant delay (<5 seconds)
- If generation takes >3 seconds, consider adding progress indicator

**Memory Usage:**
- Generate report data in streaming fashion if possible
- Avoid loading entire report into memory before export
- XLSX format is more memory-intensive than CSV

## Success Criteria

Phase 3 is complete when:

1. ✅ Reports tab accessible from main navigation
2. ✅ Debt Status report generates correctly (CSV and XLSX)
3. ✅ Payment History report generates correctly with matrix layout (CSV and XLSX)
4. ✅ Date range selection works for Payment History
5. ✅ "Include inactive members" toggle works for Debt Status
6. ✅ Anonymization works for both report types
7. ✅ Re-authentication required before every export
8. ✅ Preview displays data accurately before export
9. ✅ File picker dialog allows user to choose save location
10. ✅ Exported files open correctly in LibreOffice Calc and Microsoft Excel
11. ✅ All manual testing checklist items pass
12. ✅ Error handling covers all identified scenarios
13. ✅ UI follows existing dark theme and design patterns

## Future Enhancements (Out of Scope)

- Scheduled exports (daily/weekly automatic generation)
- Email integration (send reports via email)
- Additional report types (payment trends, member growth charts)
- PDF export format
- Print functionality
- Custom column selection for reports
- Report templates (saved configurations)
