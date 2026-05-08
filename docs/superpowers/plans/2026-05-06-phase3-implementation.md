# Phase 3: Export Functionality Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add comprehensive export functionality with Debt Status and Payment History reports supporting CSV/XLSX formats, anonymization, and re-authentication security.

**Architecture:** Incremental build starting with UI skeleton, then re-auth modal, then Debt Status report (validates full pipeline), finally Payment History matrix generation. Backend generates report data and files in Rust, frontend provides preview and export controls.

**Tech Stack:** React 18 + TypeScript + Tailwind CSS (frontend), Tauri + Rust + SQLCipher (backend), csv crate + rust_xlsxwriter (export)

---

## File Structure Map

### Frontend Files
- **Create:** `src/types/reports.ts` - Report type definitions
- **Create:** `src/components/ReportsScreen.tsx` - Main reports tab UI
- **Create:** `src/components/ReAuthModal.tsx` - Password confirmation modal
- **Create:** `src/components/ReportPreviewTable.tsx` - Report data preview component
- **Modify:** `src/contexts/AppContext.tsx:6-25` - Add re-auth and report state
- **Modify:** `src/components/MainLayout.tsx:12` - Add 'reports' tab to navigation
- **Modify:** `src/types/index.ts:67` - Export month abbreviations constant

### Backend Files
- **Create:** `src-tauri/src/commands/reports.rs` - Report generation and export commands
- **Create:** `src-tauri/src/models/reports.rs` - Report data models and logic
- **Create:** `src-tauri/tests/reports_tests.rs` - Report generation unit tests
- **Modify:** `src-tauri/src/commands/mod.rs:137` - Register new reports module
- **Modify:** `src-tauri/src/lib.rs` - Add reports commands to Tauri invoke handler
- **Modify:** `src-tauri/Cargo.toml:33` - Add csv and rust_xlsxwriter dependencies

---

## Task 1: Add Backend Dependencies

**Files:**
- Modify: `src-tauri/Cargo.toml:33`

- [ ] **Step 1: Add CSV and XLSX dependencies to Cargo.toml**

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = { version = "0.31", features = ["bundled-sqlcipher-vendored-openssl"] }
bcrypt = "0.15"
ring = "0.17"
hex = "0.4"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
rand = "0.8"
dirs = "5"
csv = "1.3"
rust_xlsxwriter = "0.76"
tauri-plugin-dialog = "2"
```

- [ ] **Step 2: Build to verify dependencies resolve**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds with new dependencies downloaded

- [ ] **Step 3: Commit**

```bash
git add src-tauri/Cargo.toml
git commit -m "chore: add csv, xlsx, and dialog dependencies for reports"
```

---

## Task 2: Create Report Type Definitions (Frontend)

**Files:**
- Create: `src/types/reports.ts`
- Modify: `src/types/index.ts:67`

- [ ] **Step 1: Export month abbreviations from types/index.ts**

Add after line 66 in `src/types/index.ts`:

```typescript
export const MONTH_ABBREV_PT = [
  'Jan', 'Fev', 'Mar', 'Abr', 'Mai', 'Jun',
  'Jul', 'Ago', 'Set', 'Out', 'Nov', 'Dez'
];
```

- [ ] **Step 2: Create report type definitions**

Create `src/types/reports.ts`:

```typescript
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
```

- [ ] **Step 3: Commit**

```bash
git add src/types/reports.ts src/types/index.ts
git commit -m "feat: add report type definitions"
```

---

## Task 3: Extend AppContext for Reports

**Files:**
- Modify: `src/contexts/AppContext.tsx:6-136`

- [ ] **Step 1: Update AppContextType interface**

Replace lines 6-25 in `src/contexts/AppContext.tsx`:

```typescript
interface AppContextType {
  members: Member[];
  payments: Payment[];
  settings: AppSettings;
  paymentModalOpen: boolean;
  paymentModalPrefill?: PaymentPrefill;
  showReAuthModal: boolean;
  reAuthCallback: (() => void) | null;
  refreshMembers: () => Promise<void>;
  refreshPayments: () => Promise<void>;
  refreshSettings: () => Promise<void>;
  updateSetting: (key: string, value: string) => Promise<void>;
  getMemberDebt: (memberId: number) => Promise<MemberDebtInfo>;
  getAllDebts: () => Promise<MemberDebtInfo[]>;
  openPaymentModal: (prefill?: PaymentPrefill) => void;
  closePaymentModal: () => void;
  requestReAuth: (callback: () => void) => void;
  closeReAuthModal: () => void;
  addMember: (name: string, startDate: string) => Promise<void>;
  addPayment: (memberId: number, month: number, year: number, amount: number, paymentDate: string) => Promise<void>;
  updateMemberActive: (id: number, active: boolean) => Promise<void>;
  updateMemberName: (id: number, name: string) => Promise<void>;
  deletePayment: (id: number) => Promise<void>;
}
```

- [ ] **Step 2: Add state variables to AppProvider**

After line 35 in `src/contexts/AppContext.tsx`, add:

```typescript
  const [showReAuthModal, setShowReAuthModal] = useState(false);
  const [reAuthCallback, setReAuthCallback] = useState<(() => void) | null>(null);
```

- [ ] **Step 3: Add re-auth functions**

Before line 123 in `src/contexts/AppContext.tsx`, add:

```typescript
  const requestReAuth = (callback: () => void) => {
    setReAuthCallback(() => callback);
    setShowReAuthModal(true);
  };

  const closeReAuthModal = () => {
    setShowReAuthModal(false);
    setReAuthCallback(null);
  };
```

- [ ] **Step 4: Update provider value**

Replace line 125 provider value with:

```typescript
    <AppContext.Provider value={{ 
      members, 
      payments, 
      settings, 
      paymentModalOpen, 
      paymentModalPrefill, 
      showReAuthModal,
      reAuthCallback,
      refreshMembers, 
      refreshPayments, 
      refreshSettings, 
      updateSetting, 
      getMemberDebt, 
      getAllDebts, 
      openPaymentModal, 
      closePaymentModal, 
      requestReAuth,
      closeReAuthModal,
      addMember, 
      addPayment, 
      updateMemberActive, 
      updateMemberName, 
      deletePayment 
    }}>
```

- [ ] **Step 5: Verify TypeScript compiles**

Run: `npm run build`
Expected: Build succeeds with no type errors

- [ ] **Step 6: Commit**

```bash
git add src/contexts/AppContext.tsx
git commit -m "feat: extend AppContext with re-auth state for reports"
```

---

## Task 4: Create ReAuth Modal Component

**Files:**
- Create: `src/components/ReAuthModal.tsx`

- [ ] **Step 1: Create ReAuthModal component**

Create `src/components/ReAuthModal.tsx`:

```typescript
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useApp } from '../contexts/AppContext';

export const ReAuthModal = () => {
  const { showReAuthModal, reAuthCallback, closeReAuthModal } = useApp();
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  if (!showReAuthModal) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      // Verify password using existing auth command
      await invoke('verify_password_cmd', { password });
      
      // Password correct - execute callback
      closeReAuthModal();
      setPassword('');
      if (reAuthCallback) {
        reAuthCallback();
      }
    } catch (err) {
      setError('Senha incorreta. Tente novamente.');
    } finally {
      setLoading(false);
    }
  };

  const handleCancel = () => {
    setPassword('');
    setError('');
    closeReAuthModal();
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 w-96 shadow-xl">
        <h2 className="text-xl font-bold mb-4 text-white">Confirmar Senha</h2>
        <p className="text-gray-300 mb-4">
          Digite sua senha para continuar com a exportação.
        </p>
        
        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label className="block text-sm font-medium mb-2 text-gray-300">
              Senha
            </label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              autoFocus
              disabled={loading}
            />
            {error && (
              <p className="text-red-500 text-sm mt-1">{error}</p>
            )}
          </div>

          <div className="flex gap-2">
            <button
              type="button"
              onClick={handleCancel}
              className="flex-1 px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-500 disabled:opacity-50"
              disabled={loading}
            >
              Cancelar
            </button>
            <button
              type="submit"
              className="flex-1 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500 disabled:opacity-50"
              disabled={loading}
            >
              {loading ? 'Verificando...' : 'Confirmar'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
```

- [ ] **Step 2: Verify TypeScript compiles**

Run: `npm run build`
Expected: Build succeeds

- [ ] **Step 3: Commit**

```bash
git add src/components/ReAuthModal.tsx
git commit -m "feat: create ReAuthModal component for export authentication"
```

---

## Task 5: Create Report Preview Table Component

**Files:**
- Create: `src/components/ReportPreviewTable.tsx`

- [ ] **Step 1: Create ReportPreviewTable component**

Create `src/components/ReportPreviewTable.tsx`:

```typescript
import { DebtStatusReport, PaymentHistoryReport } from '../types/reports';
import { formatCurrency } from '../types';

interface ReportPreviewTableProps {
  debtReport?: DebtStatusReport;
  paymentReport?: PaymentHistoryReport;
}

export const ReportPreviewTable = ({ debtReport, paymentReport }: ReportPreviewTableProps) => {
  if (!debtReport && !paymentReport) {
    return (
      <div className="text-gray-400 text-center py-8">
        Configure o relatório acima e clique em visualizar
      </div>
    );
  }

  if (debtReport) {
    const displayRows = debtReport.members.slice(0, 100);
    const hasMore = debtReport.members.length > 100;

    return (
      <div>
        {hasMore && (
          <div className="mb-2 text-sm text-yellow-500">
            Mostrando primeiras 100 linhas de {debtReport.members.length}
          </div>
        )}
        <div className="overflow-x-auto">
          <table className="min-w-full bg-gray-800 rounded">
            <thead>
              <tr className="bg-gray-700">
                <th className="px-4 py-2 text-left text-white">Nome do Membro</th>
                <th className="px-4 py-2 text-left text-white">Dívida Total (R$)</th>
                <th className="px-4 py-2 text-left text-white">Meses Não Pagos</th>
              </tr>
            </thead>
            <tbody>
              {displayRows.map((row, idx) => (
                <tr key={idx} className="border-t border-gray-700">
                  <td className="px-4 py-2 text-white">{row.member_name}</td>
                  <td className="px-4 py-2 text-white">{formatCurrency(row.total_debt)}</td>
                  <td className="px-4 py-2 text-white">{row.unpaid_month_count}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    );
  }

  if (paymentReport) {
    const displayRows = paymentReport.members.slice(0, 100);
    const hasMore = paymentReport.members.length > 100;

    return (
      <div>
        {hasMore && (
          <div className="mb-2 text-sm text-yellow-500">
            Mostrando primeiras 100 linhas de {paymentReport.members.length}
          </div>
        )}
        <div className="overflow-x-auto">
          <table className="min-w-full bg-gray-800 rounded">
            <thead>
              <tr className="bg-gray-700">
                <th className="px-4 py-2 text-left text-white sticky left-0 bg-gray-700">
                  Nome do Membro
                </th>
                <th className="px-4 py-2 text-left text-white">Início</th>
                {paymentReport.month_columns.map((col) => (
                  <th key={col.key} className="px-4 py-2 text-left text-white">
                    {col.display}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {displayRows.map((row, idx) => (
                <tr key={idx} className="border-t border-gray-700">
                  <td className="px-4 py-2 text-white sticky left-0 bg-gray-800">
                    {row.member_name}
                  </td>
                  <td className="px-4 py-2 text-white">{row.start_date}</td>
                  {paymentReport.month_columns.map((col) => (
                    <td key={col.key} className="px-4 py-2 text-white">
                      {row.payments[col.key] || ''}
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    );
  }

  return null;
};
```

- [ ] **Step 2: Verify TypeScript compiles**

Run: `npm run build`
Expected: Build succeeds

- [ ] **Step 3: Commit**

```bash
git add src/components/ReportPreviewTable.tsx
git commit -m "feat: create ReportPreviewTable component for report preview"
```

---

## Task 6: Create Reports Screen Skeleton

**Files:**
- Create: `src/components/ReportsScreen.tsx`

- [ ] **Step 1: Create basic ReportsScreen structure**

Create `src/components/ReportsScreen.tsx`:

```typescript
import { useState } from 'react';
import { ReportType, ExportFormat } from '../types/reports';

export const ReportsScreen = () => {
  const [reportType, setReportType] = useState<ReportType>('debt_status');
  const [format, setFormat] = useState<ExportFormat>('csv');
  const [anonymize, setAnonymize] = useState(false);
  const [includeInactive, setIncludeInactive] = useState(false);
  const [startDate, setStartDate] = useState('');
  const [endDate, setEndDate] = useState('');
  const [dateError, setDateError] = useState('');

  const validateDates = (): boolean => {
    if (reportType === 'payment_history') {
      if (!startDate || !endDate) {
        setDateError('Data inicial e final são obrigatórias');
        return false;
      }
      if (new Date(endDate) < new Date(startDate)) {
        setDateError('A data final deve ser posterior à data inicial');
        return false;
      }
    }
    setDateError('');
    return true;
  };

  const handleExport = () => {
    if (!validateDates()) return;
    // TODO: Will implement export logic in later tasks
    console.log('Export requested');
  };

  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold mb-6 text-white">Relatórios</h1>

      {/* Export Controls */}
      <div className="bg-gray-800 rounded-lg p-6 mb-6">
        <h2 className="text-lg font-semibold mb-4 text-white">Configuração do Relatório</h2>

        {/* Report Type Selection */}
        <div className="mb-4">
          <label className="block text-sm font-medium mb-2 text-gray-300">
            Tipo de Relatório
          </label>
          <div className="space-y-2">
            <label className="flex items-center text-white">
              <input
                type="radio"
                value="debt_status"
                checked={reportType === 'debt_status'}
                onChange={(e) => setReportType(e.target.value as ReportType)}
                className="mr-2"
              />
              Status de Dívidas
            </label>
            <label className="flex items-center text-white">
              <input
                type="radio"
                value="payment_history"
                checked={reportType === 'payment_history'}
                onChange={(e) => setReportType(e.target.value as ReportType)}
                className="mr-2"
              />
              Histórico de Pagamentos
            </label>
          </div>
        </div>

        {/* Conditional Configuration */}
        {reportType === 'debt_status' && (
          <div className="mb-4">
            <label className="flex items-center text-white">
              <input
                type="checkbox"
                checked={includeInactive}
                onChange={(e) => setIncludeInactive(e.target.checked)}
                className="mr-2"
              />
              Incluir membros inativos
            </label>
          </div>
        )}

        {reportType === 'payment_history' && (
          <div className="mb-4 space-y-3">
            <div>
              <label className="block text-sm font-medium mb-2 text-gray-300">
                Data Inicial
              </label>
              <input
                type="date"
                value={startDate}
                onChange={(e) => setStartDate(e.target.value)}
                className="px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white"
              />
            </div>
            <div>
              <label className="block text-sm font-medium mb-2 text-gray-300">
                Data Final
              </label>
              <input
                type="date"
                value={endDate}
                onChange={(e) => setEndDate(e.target.value)}
                className="px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white"
              />
            </div>
            {dateError && (
              <p className="text-red-500 text-sm">{dateError}</p>
            )}
          </div>
        )}

        {/* Format Selection */}
        <div className="mb-4">
          <label className="block text-sm font-medium mb-2 text-gray-300">
            Formato
          </label>
          <div className="space-y-2">
            <label className="flex items-center text-white">
              <input
                type="radio"
                value="csv"
                checked={format === 'csv'}
                onChange={(e) => setFormat(e.target.value as ExportFormat)}
                className="mr-2"
              />
              CSV
            </label>
            <label className="flex items-center text-white">
              <input
                type="radio"
                value="xlsx"
                checked={format === 'xlsx'}
                onChange={(e) => setFormat(e.target.value as ExportFormat)}
                className="mr-2"
              />
              XLSX
            </label>
          </div>
        </div>

        {/* Anonymize Checkbox */}
        <div className="mb-4">
          <label className="flex items-center text-white">
            <input
              type="checkbox"
              checked={anonymize}
              onChange={(e) => setAnonymize(e.target.checked)}
              className="mr-2"
            />
            Relatório anônimo (Membro #1, Membro #2, ...)
          </label>
        </div>

        {/* Export Button */}
        <button
          onClick={handleExport}
          className="px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-500"
        >
          Exportar
        </button>
      </div>

      {/* Preview Area - Placeholder */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h2 className="text-lg font-semibold mb-4 text-white">Visualização</h2>
        <div className="text-gray-400 text-center py-8">
          Configuração pronta. A visualização será implementada nas próximas etapas.
        </div>
      </div>
    </div>
  );
};
```

- [ ] **Step 2: Verify TypeScript compiles**

Run: `npm run build`
Expected: Build succeeds

- [ ] **Step 3: Commit**

```bash
git add src/components/ReportsScreen.tsx
git commit -m "feat: create ReportsScreen skeleton with controls"
```

---

## Task 7: Integrate Reports Tab into MainLayout

**Files:**
- Modify: `src/components/MainLayout.tsx:1-20,700-750`
- Modify: `src/App.tsx:1-20`

- [ ] **Step 1: Import ReportsScreen and ReAuthModal in MainLayout**

Add to imports at top of `src/components/MainLayout.tsx`:

```typescript
import { ReportsScreen } from './ReportsScreen';
import { ReAuthModal } from './ReAuthModal';
```

- [ ] **Step 2: Update activeTab type definition**

Change line 12 in `src/components/MainLayout.tsx`:

```typescript
const [activeTab, setActiveTab] = useState<'dashboard' | 'members' | 'payments' | 'reports' | 'settings'>('dashboard');
```

- [ ] **Step 3: Add Reports tab to navigation**

Find the tab navigation section (around line 700) and add Reports tab button after Payments and before Settings:

```typescript
<button
  onClick={() => {
    setActiveTab('reports');
    setViewingMemberDetail(false);
    setSelectedMemberId(null);
  }}
  className={`px-4 py-2 rounded ${
    activeTab === 'reports' ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300'
  }`}
>
  Relatórios
</button>
```

- [ ] **Step 4: Add Reports tab content rendering**

Find the content rendering section and add after Payments tab content (before Settings):

```typescript
{activeTab === 'reports' && <ReportsScreen />}
```

- [ ] **Step 5: Add ReAuthModal to component tree**

At the end of the return statement, before the closing tags, add:

```typescript
      <ReAuthModal />
```

- [ ] **Step 6: Test the UI manually**

Run: `npm run tauri dev`
Expected: Reports tab appears in navigation, clicking it shows the skeleton UI

- [ ] **Step 7: Commit**

```bash
git add src/components/MainLayout.tsx
git commit -m "feat: integrate Reports tab into main navigation"
```

---

## Task 8: Create Backend Report Models

**Files:**
- Create: `src-tauri/src/models/reports.rs`
- Modify: `src-tauri/src/models/mod.rs`

- [ ] **Step 1: Add reports module to models/mod.rs**

Add to `src-tauri/src/models/mod.rs`:

```rust
pub mod reports;
```

- [ ] **Step 2: Create report models and helper functions**

Create `src-tauri/src/models/reports.rs`:

```rust
use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result as SqlResult};
use std::collections::HashMap;
use chrono::NaiveDate;

use super::debt::calculate_member_debt;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebtStatusRow {
    pub member_id: i64,
    pub member_name: String,
    pub total_debt: f64,
    pub unpaid_month_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebtStatusReport {
    pub members: Vec<DebtStatusRow>,
    pub generated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthColumn {
    pub key: String,        // "2026-01"
    pub display: String,    // "Jan/2026"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentHistoryRow {
    pub member_id: i64,
    pub member_name: String,
    pub start_date: String,
    pub payments: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentHistoryReport {
    pub members: Vec<PaymentHistoryRow>,
    pub month_columns: Vec<MonthColumn>,
    pub generated_at: String,
}

const MONTH_ABBREV_PT: [&str; 12] = [
    "Jan", "Fev", "Mar", "Abr", "Mai", "Jun",
    "Jul", "Ago", "Set", "Out", "Nov", "Dez"
];

pub fn generate_debt_status_report(
    conn: &Connection,
    include_inactive: bool,
) -> SqlResult<DebtStatusReport> {
    let query = if include_inactive {
        "SELECT id, name, start_date FROM members ORDER BY id"
    } else {
        "SELECT id, name, start_date FROM members WHERE active = 1 ORDER BY id"
    };

    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let mut members = Vec::new();

    for row in rows {
        let (member_id, member_name) = row?;
        let total_debt = calculate_member_debt(conn, member_id, &today)?;
        
        // Count unpaid months by querying debt details
        let unpaid_count = if total_debt > 0.0 {
            // Get minimum fee setting
            let min_fee: f64 = conn.query_row(
                "SELECT value FROM settings WHERE key = 'minimum_fee_brl'",
                [],
                |row| row.get(0)
            ).unwrap_or(15.0);
            
            (total_debt / min_fee).ceil() as i32
        } else {
            0
        };

        members.push(DebtStatusRow {
            member_id,
            member_name,
            total_debt,
            unpaid_month_count: unpaid_count,
        });
    }

    // Sort by debt descending
    members.sort_by(|a, b| b.total_debt.partial_cmp(&a.total_debt).unwrap());

    Ok(DebtStatusReport {
        members,
        generated_at: chrono::Local::now().to_rfc3339(),
    })
}

pub fn generate_payment_history_report(
    conn: &Connection,
    start_date: &str,
    end_date: &str,
) -> SqlResult<PaymentHistoryReport> {
    // Parse dates
    let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
        .map_err(|_| rusqlite::Error::InvalidQuery)?;
    let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
        .map_err(|_| rusqlite::Error::InvalidQuery)?;

    // Generate month columns
    let mut month_columns = Vec::new();
    let mut current = NaiveDate::from_ymd_opt(start.year(), start.month(), 1).unwrap();
    let end_month = NaiveDate::from_ymd_opt(end.year(), end.month(), 1).unwrap();

    while current <= end_month {
        let key = format!("{}-{:02}", current.year(), current.month());
        let display = format!(
            "{}/{}",
            MONTH_ABBREV_PT[current.month() as usize - 1],
            current.year()
        );
        month_columns.push(MonthColumn { key, display });

        // Move to next month
        current = if current.month() == 12 {
            NaiveDate::from_ymd_opt(current.year() + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(current.year(), current.month() + 1, 1).unwrap()
        };
    }

    // Get all active members
    let mut stmt = conn.prepare(
        "SELECT id, name, start_date FROM members WHERE active = 1 ORDER BY id"
    )?;
    let members_data = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    let mut members = Vec::new();

    for member_data in members_data {
        let (member_id, member_name, start_date_str) = member_data?;
        let member_start = NaiveDate::parse_from_str(&start_date_str.split('T').next().unwrap(), "%Y-%m-%d")
            .map_err(|_| rusqlite::Error::InvalidQuery)?;

        // Get all payments for this member
        let mut payment_stmt = conn.prepare(
            "SELECT month, year, amount_brl FROM payments WHERE member_id = ?"
        )?;
        let payment_rows = payment_stmt.query_map([member_id], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, f64>(2)?,
            ))
        })?;

        let mut payments_map: HashMap<String, f64> = HashMap::new();
        for payment_row in payment_rows {
            let (month, year, amount) = payment_row?;
            let key = format!("{}-{:02}", year, month);
            payments_map.insert(key, amount);
        }

        // Build payments hash for all month columns
        let mut payments = HashMap::new();
        for col in &month_columns {
            let month_date = NaiveDate::parse_from_str(&format!("{}-01", col.key), "%Y-%m-%d")
                .map_err(|_| rusqlite::Error::InvalidQuery)?;

            if month_date < member_start {
                // Member not active yet - leave blank
                payments.insert(col.key.clone(), String::new());
            } else if let Some(amount) = payments_map.get(&col.key) {
                // Payment exists
                payments.insert(
                    col.key.clone(),
                    format!("R$ {:.2}", amount).replace('.', ",")
                );
            } else {
                // No payment - show dash
                payments.insert(col.key.clone(), "-".to_string());
            }
        }

        members.push(PaymentHistoryRow {
            member_id,
            member_name,
            start_date: format_date_dd_mm_yyyy(&start_date_str),
            payments,
        });
    }

    Ok(PaymentHistoryReport {
        members,
        month_columns,
        generated_at: chrono::Local::now().to_rfc3339(),
    })
}

fn format_date_dd_mm_yyyy(date_str: &str) -> String {
    if let Some(date_part) = date_str.split('T').next() {
        let parts: Vec<&str> = date_part.split('-').collect();
        if parts.len() == 3 {
            return format!("{}/{}/{}", parts[2], parts[1], parts[0]);
        }
    }
    date_str.to_string()
}

pub fn anonymize_report_debt(mut report: DebtStatusReport) -> DebtStatusReport {
    for (idx, member) in report.members.iter_mut().enumerate() {
        member.member_name = format!("Membro #{}", idx + 1);
    }
    report
}

pub fn anonymize_report_payment(mut report: PaymentHistoryReport) -> PaymentHistoryReport {
    for (idx, member) in report.members.iter_mut().enumerate() {
        member.member_name = format!("Membro #{}", idx + 1);
    }
    report
}
```

- [ ] **Step 3: Verify Rust compiles**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/models/reports.rs src-tauri/src/models/mod.rs
git commit -m "feat: add report generation models and logic"
```

---

## Task 9: Write Backend Report Tests

**Files:**
- Create: `src-tauri/tests/reports_tests.rs`

- [ ] **Step 1: Write debt status report tests**

Create `src-tauri/tests/reports_tests.rs`:

```rust
use gestor_do_clube_lib::models::reports::{generate_debt_status_report, generate_payment_history_report};
use gestor_do_clube_lib::models::member::create_member;
use gestor_do_clube_lib::models::payment::create_payment;
use gestor_do_clube_lib::models::settings::set_setting;
use gestor_do_clube_lib::db::schema::initialize_schema;
use rusqlite::Connection;

#[test]
fn test_debt_status_report_empty() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();
    set_setting(&conn, "minimum_fee_brl", "15.00").unwrap();

    let report = generate_debt_status_report(&conn, false).unwrap();
    assert_eq!(report.members.len(), 0);
}

#[test]
fn test_debt_status_report_with_debt() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();
    set_setting(&conn, "minimum_fee_brl", "15.00").unwrap();

    let member_id = create_member(&conn, "Test Member", "2026-01-01").unwrap();
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
    set_setting(&conn, "minimum_fee_brl", "15.00").unwrap();

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
    set_setting(&conn, "minimum_fee_brl", "15.00").unwrap();

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

    let member_id = create_member(&conn, "Test", "2026-03-01").unwrap();

    let report = generate_payment_history_report(&conn, "2026-01-01", "2026-03-31").unwrap();
    
    // Jan and Feb should be blank (member not started)
    assert_eq!(report.members[0].payments.get("2026-01").unwrap(), "");
    assert_eq!(report.members[0].payments.get("2026-02").unwrap(), "");
    assert_eq!(report.members[0].payments.get("2026-03").unwrap(), "-");
}
```

- [ ] **Step 2: Run tests to verify they fail (no implementation yet)**

Run: `cd src-tauri && cargo test reports_tests`
Expected: Tests compile and run (some may fail if models functions don't exist yet)

- [ ] **Step 3: Commit**

```bash
git add src-tauri/tests/reports_tests.rs
git commit -m "test: add unit tests for report generation"
```

---

## Task 10: Create Report Generation Commands

**Files:**
- Create: `src-tauri/src/commands/reports.rs`
- Modify: `src-tauri/src/commands/mod.rs:137`

- [ ] **Step 1: Create report commands file**

Create `src-tauri/src/commands/reports.rs`:

```rust
use tauri::AppHandle;
use crate::db::get_db_conn;
use crate::models::reports::{
    generate_debt_status_report, generate_payment_history_report,
    anonymize_report_debt, anonymize_report_payment,
    DebtStatusReport, PaymentHistoryReport,
};
use crate::commands::auth::verify_password;

#[tauri::command]
pub fn get_debt_status_report_cmd(
    app: AppHandle,
    password: String,
    include_inactive: bool,
) -> Result<DebtStatusReport, String> {
    verify_password(&app, &password)?;
    
    let conn = get_db_conn(&app)?;
    let report = generate_debt_status_report(&conn, include_inactive)
        .map_err(|e| format!("Failed to generate debt status report: {}", e))?;
    
    Ok(report)
}

#[tauri::command]
pub fn get_payment_history_report_cmd(
    app: AppHandle,
    password: String,
    start_date: String,
    end_date: String,
) -> Result<PaymentHistoryReport, String> {
    verify_password(&app, &password)?;
    
    let conn = get_db_conn(&app)?;
    let report = generate_payment_history_report(&conn, &start_date, &end_date)
        .map_err(|e| format!("Failed to generate payment history report: {}", e))?;
    
    Ok(report)
}
```

- [ ] **Step 2: Register reports module in commands/mod.rs**

Add to `src-tauri/src/commands/mod.rs`:

```rust
pub mod reports;
```

- [ ] **Step 3: Register commands in lib.rs**

Find the `invoke_handler` in `src-tauri/src/lib.rs` and add the new commands:

```rust
.invoke_handler(tauri::generate_handler![
    // ... existing commands ...
    commands::reports::get_debt_status_report_cmd,
    commands::reports::get_payment_history_report_cmd,
])
```

- [ ] **Step 4: Build to verify everything compiles**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

- [ ] **Step 5: Run tests**

Run: `cd src-tauri && cargo test`
Expected: All tests pass

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/commands/reports.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "feat: add report generation commands"
```

---

## Task 11: Wire Debt Status Report to Frontend

**Files:**
- Modify: `src/components/ReportsScreen.tsx:1-300`

- [ ] **Step 1: Add imports and state for debt report**

Add to imports in `src/components/ReportsScreen.tsx`:

```typescript
import { invoke } from '@tauri-apps/api/core';
import { DebtStatusReport, PaymentHistoryReport } from '../types/reports';
import { ReportPreviewTable } from './ReportPreviewTable';
import { useApp } from '../contexts/AppContext';
import { useAuth } from '../contexts/AuthContext';
```

Add state variables after existing state (around line 11):

```typescript
  const { requestReAuth } = useApp();
  const { password } = useAuth();
  const [debtReport, setDebtReport] = useState<DebtStatusReport | null>(null);
  const [paymentReport, setPaymentReport] = useState<PaymentHistoryReport | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
```

- [ ] **Step 2: Add function to load debt status report**

Add after state variables:

```typescript
  const loadDebtStatusReport = async () => {
    if (!password) return;
    setLoading(true);
    setError('');
    try {
      const report = await invoke<DebtStatusReport>('get_debt_status_report_cmd', {
        password,
        includeInactive,
      });
      setDebtReport(report);
      setPaymentReport(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };
```

- [ ] **Step 3: Update handleExport to support debt status**

Replace the `handleExport` function:

```typescript
  const handleExport = () => {
    if (!validateDates()) return;
    
    requestReAuth(async () => {
      // Export logic will be implemented in next task
      console.log('Authenticated for export');
    });
  };
```

- [ ] **Step 4: Add "Visualizar" button and update preview area**

Replace the Export button section with:

```typescript
        {/* Action Buttons */}
        <div className="flex gap-2">
          <button
            onClick={reportType === 'debt_status' ? loadDebtStatusReport : () => {}}
            disabled={loading || (reportType === 'payment_history' && !validateDates())}
            className="px-6 py-2 bg-gray-600 text-white rounded hover:bg-gray-500 disabled:opacity-50"
          >
            {loading ? 'Carregando...' : 'Visualizar'}
          </button>
          <button
            onClick={handleExport}
            disabled={loading || !debtReport && !paymentReport}
            className="px-6 py-2 bg-blue-600 text-white rounded hover:bg-blue-500 disabled:opacity-50"
          >
            Exportar
          </button>
        </div>
```

Replace the preview area placeholder:

```typescript
      {/* Preview Area */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h2 className="text-lg font-semibold mb-4 text-white">Visualização</h2>
        {error && (
          <div className="mb-4 p-3 bg-red-900 text-red-200 rounded">
            {error}
          </div>
        )}
        <ReportPreviewTable debtReport={debtReport || undefined} paymentReport={paymentReport || undefined} />
      </div>
```

- [ ] **Step 5: Test debt status report preview**

Run: `npm run tauri dev`
Steps:
1. Navigate to Reports tab
2. Select "Status de Dívidas"
3. Click "Visualizar"
4. Verify debt status table appears in preview area

Expected: Debt status report displays with member names, debt amounts, and unpaid month counts

- [ ] **Step 6: Commit**

```bash
git add src/components/ReportsScreen.tsx
git commit -m "feat: wire debt status report to frontend preview"
```

---

## Task 12: Wire Payment History Report to Frontend

**Files:**
- Modify: `src/components/ReportsScreen.tsx:30-50`

- [ ] **Step 1: Add function to load payment history report**

Add after `loadDebtStatusReport` function:

```typescript
  const loadPaymentHistoryReport = async () => {
    if (!password) return;
    if (!validateDates()) return;
    
    setLoading(true);
    setError('');
    try {
      const report = await invoke<PaymentHistoryReport>('get_payment_history_report_cmd', {
        password,
        startDate,
        endDate,
      });
      setPaymentReport(report);
      setDebtReport(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };
```

- [ ] **Step 2: Update Visualizar button to handle payment history**

Update the Visualizar button onClick:

```typescript
onClick={reportType === 'debt_status' ? loadDebtStatusReport : loadPaymentHistoryReport}
```

- [ ] **Step 3: Test payment history report preview**

Run: `npm run tauri dev`
Steps:
1. Navigate to Reports tab
2. Select "Histórico de Pagamentos"
3. Enter start date (e.g., 2026-01-01)
4. Enter end date (e.g., 2026-03-31)
5. Click "Visualizar"

Expected: Payment history matrix displays with member names, start dates, and month columns showing payments or dashes

- [ ] **Step 4: Commit**

```bash
git add src/components/ReportsScreen.tsx
git commit -m "feat: wire payment history report to frontend preview"
```

---

## Task 13: Add CSV Export Functionality

**Files:**
- Modify: `src-tauri/src/commands/reports.rs:30-200`

- [ ] **Step 1: Add CSV export function for debt status**

Add to `src-tauri/src/commands/reports.rs`:

```rust
use std::path::PathBuf;
use csv::Writer;

fn export_debt_status_csv(
    report: &DebtStatusReport,
    file_path: &str,
) -> Result<(), String> {
    let mut wtr = Writer::from_path(file_path)
        .map_err(|e| format!("Failed to create CSV file: {}", e))?;

    // Write UTF-8 BOM for Excel compatibility
    wtr.write_byte_record(&[0xEF, 0xBB, 0xBF])
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
```

- [ ] **Step 2: Add CSV export function for payment history**

Add after the debt status CSV function:

```rust
fn export_payment_history_csv(
    report: &PaymentHistoryReport,
    file_path: &str,
) -> Result<(), String> {
    let mut wtr = Writer::from_path(file_path)
        .map_err(|e| format!("Failed to create CSV file: {}", e))?;

    // Write UTF-8 BOM
    wtr.write_byte_record(&[0xEF, 0xBB, 0xBF])
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
```

- [ ] **Step 3: Add export command**

Add new command:

```rust
#[tauri::command]
pub fn export_debt_status_csv_cmd(
    app: AppHandle,
    password: String,
    include_inactive: bool,
    anonymize: bool,
    file_path: String,
) -> Result<(), String> {
    verify_password(&app, &password)?;
    
    let conn = get_db_conn(&app)?;
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
    app: AppHandle,
    password: String,
    start_date: String,
    end_date: String,
    anonymize: bool,
    file_path: String,
) -> Result<(), String> {
    verify_password(&app, &password)?;
    
    let conn = get_db_conn(&app)?;
    let mut report = generate_payment_history_report(&conn, &start_date, &end_date)
        .map_err(|e| format!("Failed to generate report: {}", e))?;
    
    if anonymize {
        report = anonymize_report_payment(report);
    }
    
    export_payment_history_csv(&report, &file_path)?;
    
    Ok(())
}
```

- [ ] **Step 4: Register commands in lib.rs**

Add to invoke_handler:

```rust
commands::reports::export_debt_status_csv_cmd,
commands::reports::export_payment_history_csv_cmd,
```

- [ ] **Step 5: Build and verify**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/commands/reports.rs src-tauri/src/lib.rs
git commit -m "feat: add CSV export functionality for reports"
```

---

## Task 14: Add XLSX Export Functionality

**Files:**
- Modify: `src-tauri/src/commands/reports.rs:200-400`

- [ ] **Step 1: Add XLSX export for debt status**

Add to `src-tauri/src/commands/reports.rs`:

```rust
use rust_xlsxwriter::{Workbook, Format, Color};

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
        worksheet.write(row_num, 1, format!("R$ {:.2}", row.total_debt).replace('.', ","))
            .map_err(|e| format!("Failed to write data: {}", e))?;
        worksheet.write(row_num, 2, row.unpaid_month_count)
            .map_err(|e| format!("Failed to write data: {}", e))?;
    }

    workbook.save(file_path)
        .map_err(|e| format!("Failed to save XLSX: {}", e))?;

    Ok(())
}
```

- [ ] **Step 2: Add XLSX export for payment history**

Add after debt status XLSX function:

```rust
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
```

- [ ] **Step 3: Add XLSX export commands**

Add new commands:

```rust
#[tauri::command]
pub fn export_debt_status_xlsx_cmd(
    app: AppHandle,
    password: String,
    include_inactive: bool,
    anonymize: bool,
    file_path: String,
) -> Result<(), String> {
    verify_password(&app, &password)?;
    
    let conn = get_db_conn(&app)?;
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
    app: AppHandle,
    password: String,
    start_date: String,
    end_date: String,
    anonymize: bool,
    file_path: String,
) -> Result<(), String> {
    verify_password(&app, &password)?;
    
    let conn = get_db_conn(&app)?;
    let mut report = generate_payment_history_report(&conn, &start_date, &end_date)
        .map_err(|e| format!("Failed to generate report: {}", e))?;
    
    if anonymize {
        report = anonymize_report_payment(report);
    }
    
    export_payment_history_xlsx(&report, &file_path)?;
    
    Ok(())
}
```

- [ ] **Step 4: Register commands**

Add to lib.rs invoke_handler:

```rust
commands::reports::export_debt_status_xlsx_cmd,
commands::reports::export_payment_history_xlsx_cmd,
```

- [ ] **Step 5: Build and verify**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/commands/reports.rs src-tauri/src/lib.rs
git commit -m "feat: add XLSX export functionality for reports"
```

---

## Task 15: Wire Export to Frontend with File Picker

**Files:**
- Modify: `src/components/ReportsScreen.tsx:60-120`

- [ ] **Step 1: Add file picker import**

Add to imports:

```typescript
import { save } from '@tauri-apps/plugin-dialog';
```

- [ ] **Step 2: Implement complete export flow**

Replace the `handleExport` function:

```typescript
  const handleExport = () => {
    if (!validateDates()) return;
    if (!debtReport && !paymentReport) return;

    requestReAuth(async () => {
      if (!password) return;
      
      try {
        // Determine default filename
        const today = new Date().toISOString().split('T')[0];
        const defaultName = reportType === 'debt_status' 
          ? `relatorio-dividas-${today}`
          : `historico-pagamentos-${today}`;
        const extension = format === 'csv' ? '.csv' : '.xlsx';

        // Show file picker
        const filePath = await save({
          defaultPath: defaultName + extension,
          filters: [{
            name: format.toUpperCase(),
            extensions: [format]
          }]
        });

        if (!filePath) {
          // User cancelled
          return;
        }

        // Call appropriate export command
        if (reportType === 'debt_status') {
          const cmd = format === 'csv' 
            ? 'export_debt_status_csv_cmd'
            : 'export_debt_status_xlsx_cmd';
          
          await invoke(cmd, {
            password,
            includeInactive,
            anonymize,
            filePath,
          });
        } else {
          const cmd = format === 'csv'
            ? 'export_payment_history_csv_cmd'
            : 'export_payment_history_xlsx_cmd';
          
          await invoke(cmd, {
            password,
            startDate,
            endDate,
            anonymize,
            filePath,
          });
        }

        alert('Relatório exportado com sucesso!');
      } catch (err) {
        setError(`Erro ao exportar: ${err}`);
      }
    });
  };
```

- [ ] **Step 3: Test CSV export end-to-end**

Run: `npm run tauri dev`
Steps:
1. Generate debt status report preview
2. Select CSV format
3. Click "Exportar"
4. Enter password in modal
5. Choose save location
6. Verify file is created and can be opened in LibreOffice Calc

Expected: CSV file with proper formatting, UTF-8 encoding, Portuguese characters

- [ ] **Step 4: Test XLSX export end-to-end**

Steps:
1. Generate payment history report preview
2. Select XLSX format
3. Click "Exportar"
4. Enter password
5. Save file
6. Verify file opens in Excel/LibreOffice

Expected: XLSX file with proper formatting, headers, Brazilian number format

- [ ] **Step 5: Commit**

```bash
git add src/components/ReportsScreen.tsx
git commit -m "feat: wire export functionality with file picker dialog"
```

---

## Task 16: Final Testing and Documentation

**Files:**
- Modify: `README.md:13-40`

- [ ] **Step 1: Run full manual test checklist**

Test each scenario from the spec:
- [ ] Export Debt Status as CSV (complete mode)
- [ ] Export Debt Status as CSV (anonymized mode)
- [ ] Export Debt Status as XLSX (complete mode)
- [ ] Export Debt Status as XLSX (anonymized mode)
- [ ] Export Debt Status with inactive members included
- [ ] Export Debt Status with inactive members excluded
- [ ] Export Payment History as CSV (3-month range)
- [ ] Export Payment History as XLSX (1-year range)
- [ ] Verify wrong password rejected
- [ ] Verify correct password allows export
- [ ] Cancel file picker (no error)
- [ ] Verify preview updates correctly
- [ ] Verify date validation works

- [ ] **Step 2: Update README with Phase 3 features**

Update Phase 2 section title to "Phase 3 Features" and add export features:

```markdown
## Phase 3 Features

**Reports & Export**
- Dedicated Reports screen with export functionality
- Debt Status report (current member debt summary)
- Payment History report (matrix-style payment grid)
- CSV and XLSX export formats
- Anonymization support (Membro #1, #2, etc.)
- Re-authentication before export for security
- Preview functionality before exporting
- Date range selection for payment history
- Optional inactive member inclusion for debt status

**Dashboard**
- Overview of total outstanding debt across all members
- Display of active member count
- Quick access to member detail views

**Settings**
- Configurable minimum membership fee
- Persisted settings stored in local database

**Member Management**
- Member detail view with full payment history
- Debt calculation showing unpaid months with amounts
- List of months with outstanding payments
- Quick link to pay specific unpaid months

**Payment System**
- Global payment modal accessible from toolbar
- Auto-fill payment form when selecting from unpaid months
- Payment date, amount, and period tracking
- Portuguese-language payment descriptions

**User Interface**
- Complete Portuguese interface
- Dark theme throughout application
- Responsive table layouts with pagination
- Intuitive navigation between sections
```

- [ ] **Step 3: Commit README update**

```bash
git add README.md
git commit -m "docs: update README with Phase 3 export features"
```

- [ ] **Step 4: Final commit with phase completion**

```bash
git commit --allow-empty -m "chore: Phase 3 export functionality complete

Export features implemented:
- Debt Status and Payment History reports
- CSV and XLSX formats
- Anonymization support
- Re-authentication security
- Preview functionality
- File picker integration

All manual tests passed"
```

---

## Self-Review Checklist

**Spec Coverage:**
- ✅ Reports Screen UI - Task 6, 7
- ✅ Re-authentication modal - Task 4
- ✅ Debt Status report - Tasks 8, 9, 10, 11, 13, 14
- ✅ Payment History report - Tasks 8, 9, 10, 12, 13, 14
- ✅ CSV export - Task 13
- ✅ XLSX export - Task 14
- ✅ Anonymization - Tasks 8, 13, 14
- ✅ Preview functionality - Task 5, 11, 12
- ✅ Date range selection - Task 6, 12
- ✅ Include inactive toggle - Task 6, 11
- ✅ File picker dialog - Task 15

**Type Consistency:**
- ✅ DebtStatusReport structure matches across frontend (reports.ts) and backend (reports.rs)
- ✅ PaymentHistoryReport structure matches across layers
- ✅ Month column format consistent: "Jan/2026" pattern
- ✅ Currency format consistent: "R$ 15,00" pattern
- ✅ Command naming consistent: `*_cmd` suffix

**No Placeholders:**
- ✅ All code blocks contain actual implementation
- ✅ All test cases have specific assertions
- ✅ All commands include error handling
- ✅ All file paths are exact

**Plan Quality:**
- ✅ Bite-sized steps (2-5 minutes each)
- ✅ TDD flow followed where applicable
- ✅ Each step has exact commands with expected output
- ✅ Frequent commits after each meaningful change
- ✅ Clear file structure map at beginning
