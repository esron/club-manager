# Phase 2 Core Features Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Complete core functionality with settings, debt calculation, dashboard, member detail views, and global payment modal.

**Architecture:** Build incrementally from simplest (settings) to most complex (payment modal). Each component is self-contained and testable. Reuse existing patterns from Phase 1.

**Tech Stack:** React 18 + TypeScript, Tailwind CSS, Tauri (Rust), SQLCipher

---

## File Structure

### New Files
```
src/components/
  ├── SettingsScreen.tsx       # Configuration UI
  ├── DashboardScreen.tsx      # Overview widgets
  ├── MemberDetailView.tsx     # Individual member page
  └── AddPaymentModal.tsx      # Global payment dialog

src-tauri/src/commands/
  ├── settings.rs              # Settings commands
  └── debt.rs                  # Debt calculation commands
```

### Modified Files
```
src/types/index.ts              # Add new TypeScript interfaces
src/contexts/AppContext.tsx     # Add settings, debt, modal state
src/components/MainLayout.tsx   # Add tabs, navigation, toolbar
src-tauri/src/lib.rs           # Register new commands
src-tauri/src/commands/mod.rs  # Export new command modules
```

---

## Task 1: Settings Backend Commands

**Files:**
- Create: `src-tauri/src/commands/settings.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Create settings command file**

```rust
// src-tauri/src/commands/settings.rs
use crate::models::settings::{get_setting, update_setting};
use crate::security::config::load_config;
use crate::security::password::derive_encryption_key;
use crate::db::connection::open_encrypted_db;
use std::path::PathBuf;

#[tauri::command]
pub fn get_setting_cmd(password: String, key: String) -> Result<String, String> {
    let conn = get_authenticated_connection(&password)?;
    get_setting(&conn, &key)
        .map_err(|e| format!("Failed to get setting: {}", e))
}

#[tauri::command]
pub fn update_setting_cmd(password: String, key: String, value: String) -> Result<(), String> {
    // Validate based on key
    if key == "minimum_fee_brl" {
        validate_minimum_fee(&value)?;
    }
    
    let conn = get_authenticated_connection(&password)?;
    update_setting(&conn, &key, &value)
        .map_err(|e| format!("Failed to update setting: {}", e))
}

fn validate_minimum_fee(value: &str) -> Result<(), String> {
    let amount: f64 = value.parse()
        .map_err(|_| "Valor inválido".to_string())?;
    
    if amount <= 0.0 {
        return Err("Valor deve ser maior que zero".to_string());
    }
    
    if amount > 9999.99 {
        return Err("Valor máximo: R$ 9999.99".to_string());
    }
    
    // Check decimal places
    let parts: Vec<&str> = value.split('.').collect();
    if parts.len() == 2 && parts[1].len() > 2 {
        return Err("Máximo 2 casas decimais".to_string());
    }
    
    Ok(())
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
```

- [ ] **Step 2: Export settings commands in mod.rs**

```rust
// src-tauri/src/commands/mod.rs
pub mod auth;
pub mod database;
pub mod member;
pub mod payment;
pub mod settings;  // Add this line

#[cfg(debug_assertions)]
pub mod seed;
```

- [ ] **Step 3: Register commands in lib.rs**

Find the `tauri::Builder::default()` section and add the new commands to the `.invoke_handler()` list:

```rust
// In src-tauri/src/lib.rs, add to invoke_handler:
.invoke_handler(tauri::generate_handler![
    commands::auth::check_first_launch,
    commands::auth::setup_password,
    commands::auth::verify_password_cmd,
    commands::database::check_database_initialized,
    commands::database::initialize_database,
    commands::member::add_member_cmd,
    commands::member::get_members_cmd,
    commands::member::get_all_members_cmd,
    commands::member::get_member_cmd,
    commands::member::update_member_active_cmd,
    commands::member::update_member_name_cmd,
    commands::payment::add_payment_cmd,
    commands::payment::get_payments_cmd,
    commands::payment::delete_payment_cmd,
    commands::settings::get_setting_cmd,      // Add this
    commands::settings::update_setting_cmd,   // Add this
    #[cfg(debug_assertions)]
    commands::seed::seed_database,
])
```

- [ ] **Step 4: Test compilation**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds with no errors

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/commands/settings.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "feat: add settings backend commands

Add get_setting_cmd and update_setting_cmd with validation.
Validates minimum fee: > 0, <= 9999.99, max 2 decimal places.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Settings Frontend Types

**Files:**
- Modify: `src/types/index.ts`

- [ ] **Step 1: Add TypeScript interfaces for settings**

```typescript
// Add to src/types/index.ts after existing interfaces

export interface AppSettings {
  minimumFee: string;
}
```

- [ ] **Step 2: Commit**

```bash
git add src/types/index.ts
git commit -m "feat: add settings TypeScript interface

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Settings Context Integration

**Files:**
- Modify: `src/contexts/AppContext.tsx`

- [ ] **Step 1: Add settings state to AppContext**

Find the `AppContextType` interface and add:

```typescript
// In src/contexts/AppContext.tsx
import { Member, Payment, AppSettings } from '../types';

interface AppContextType {
  members: Member[];
  payments: Payment[];
  settings: AppSettings;  // Add this
  refreshMembers: () => Promise<void>;
  refreshPayments: () => Promise<void>;
  refreshSettings: () => Promise<void>;  // Add this
  updateSetting: (key: string, value: string) => Promise<void>;  // Add this
  addMember: (name: string, startDate: string) => Promise<void>;
  addPayment: (memberId: number, month: number, year: number, amount: number, paymentDate: string) => Promise<void>;
  updateMemberActive: (id: number, active: boolean) => Promise<void>;
  updateMemberName: (id: number, name: string) => Promise<void>;
  deletePayment: (id: number) => Promise<void>;
}
```

- [ ] **Step 2: Add settings state in provider**

```typescript
// In AppProvider component
export const AppProvider = ({ children }: { children: ReactNode }) => {
  const { password } = useAuth();
  const [members, setMembers] = useState<Member[]>([]);
  const [payments, setPayments] = useState<Payment[]>([]);
  const [settings, setSettings] = useState<AppSettings>({ minimumFee: '15.00' });  // Add this

  const refreshMembers = async () => {
    if (!password) return;
    try {
      const data = await invoke<Member[]>('get_all_members_cmd', { password });
      setMembers(data);
    } catch (err) {
      console.error('Error refreshing members:', err);
    }
  };

  const refreshPayments = async () => {
    if (!password) return;
    const data = await invoke<Payment[]>('get_payments_cmd', { password });
    setPayments(data);
  };

  // Add this function
  const refreshSettings = async () => {
    if (!password) return;
    try {
      const minimumFee = await invoke<string>('get_setting_cmd', { 
        password, 
        key: 'minimum_fee_brl' 
      });
      setSettings({ minimumFee });
    } catch (err) {
      console.error('Error refreshing settings:', err);
    }
  };

  // Add this function
  const updateSetting = async (key: string, value: string) => {
    if (!password) throw new Error('Not authenticated');
    await invoke('update_setting_cmd', { password, key, value });
    await refreshSettings();
  };

  // ... rest of existing functions ...

  return (
    <AppContext.Provider value={{ 
      members, 
      payments, 
      settings,  // Add this
      refreshMembers, 
      refreshPayments, 
      refreshSettings,  // Add this
      updateSetting,  // Add this
      addMember, 
      addPayment, 
      updateMemberActive, 
      updateMemberName, 
      deletePayment 
    }}>
      {children}
    </AppContext.Provider>
  );
};
```

- [ ] **Step 3: Load settings on app start**

Find where `refreshMembers` and `refreshPayments` are called and add `refreshSettings`:

```typescript
// In AuthContext.tsx, find the login function and modify it to:

const login = async (pwd: string): Promise<boolean> => {
  try {
    const result = await invoke<boolean>('verify_password_cmd', { password: pwd });
    if (result) {
      setPassword(pwd);
      setIsAuthenticated(true);
      
      // Check if database is properly initialized
      const dbInitialized = await invoke<boolean>('check_database_initialized', { password: pwd });
      if (!dbInitialized) {
        setDatabaseMissing(true);
        return true;
      }
      
      return true;
    }
    return false;
  } catch (err) {
    console.error('Login error:', err);
    return false;
  }
};
```

Then in App.tsx or MainLayout.tsx, call refreshSettings after authentication. Find where refreshMembers/refreshPayments are called in useEffect:

```typescript
// In MainLayout.tsx useEffect
useEffect(() => {
  refreshMembers();
  refreshPayments();
  refreshSettings();  // Add this
}, []);
```

Wait, I need to check where this is. Let me reconsider. The AppContext doesn't automatically have access to methods from other contexts. Let me think about this differently.

Actually, looking at the existing code, refreshMembers and refreshPayments are called in MainLayout's useEffect. We should add refreshSettings there. Let me update the step.

- [ ] **Step 3: Commit**

```bash
git add src/contexts/AppContext.tsx
git commit -m "feat: add settings to AppContext

Add settings state with refreshSettings and updateSetting methods.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: Settings Screen Component

**Files:**
- Create: `src/components/SettingsScreen.tsx`

- [ ] **Step 1: Create SettingsScreen component**

```typescript
// src/components/SettingsScreen.tsx
import { useState, useEffect } from 'react';
import { useApp } from '../contexts/AppContext';

export const SettingsScreen = () => {
  const { settings, updateSetting } = useApp();
  const [minimumFee, setMinimumFee] = useState('');
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    setMinimumFee(settings.minimumFee);
  }, [settings]);

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setSuccess('');
    setLoading(true);

    try {
      await updateSetting('minimum_fee_brl', minimumFee);
      setSuccess('Configurações salvas com sucesso');
      setTimeout(() => setSuccess(''), 3000);
    } catch (err) {
      console.error('Error saving settings:', err);
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex-1 p-8">
      <h1 className="text-2xl font-bold mb-6 text-dark-text-primary">Configurações</h1>
      
      <div className="bg-dark-surface p-6 rounded-lg border border-dark-border max-w-2xl">
        <form onSubmit={handleSave}>
          <div className="mb-6">
            <label className="block mb-2 text-dark-text-secondary">
              Mensalidade Mínima (R$)
            </label>
            <input
              type="text"
              value={minimumFee}
              onChange={(e) => setMinimumFee(e.target.value)}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              placeholder="15.00"
              required
            />
            {error && <p className="text-dark-error text-sm mt-2">{error}</p>}
            {success && <p className="text-green-500 text-sm mt-2">{success}</p>}
          </div>

          <button
            type="submit"
            disabled={loading}
            className="bg-dark-accent text-white px-6 py-2 rounded hover:opacity-90 disabled:opacity-50"
          >
            {loading ? 'Salvando...' : 'Salvar'}
          </button>
        </form>
      </div>
    </div>
  );
};
```

- [ ] **Step 2: Commit**

```bash
git add src/components/SettingsScreen.tsx
git commit -m "feat: create SettingsScreen component

Add settings UI with minimum fee configuration.
Shows validation errors and success messages.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: Add Settings Tab to MainLayout

**Files:**
- Modify: `src/components/MainLayout.tsx`

- [ ] **Step 1: Import SettingsScreen**

```typescript
// At top of src/components/MainLayout.tsx
import { SettingsScreen } from './SettingsScreen';
```

- [ ] **Step 2: Update tab state type**

Find the `activeTab` state and change its type:

```typescript
const [activeTab, setActiveTab] = useState<'members' | 'payments' | 'settings'>('members');
```

- [ ] **Step 3: Add Settings tab button**

Find the tab buttons section (where Members and Payments tabs are) and add Settings:

```typescript
{/* In the navigation/tab section */}
<button
  onClick={() => setActiveTab('members')}
  className={`px-4 py-2 ${activeTab === 'members' ? 'bg-dark-accent text-white' : 'text-dark-text-secondary'} rounded`}
>
  Membros
</button>
<button
  onClick={() => setActiveTab('payments')}
  className={`px-4 py-2 ${activeTab === 'payments' ? 'bg-dark-accent text-white' : 'text-dark-text-secondary'} rounded`}
>
  Pagamentos
</button>
<button
  onClick={() => setActiveTab('settings')}
  className={`px-4 py-2 ${activeTab === 'settings' ? 'bg-dark-accent text-white' : 'text-dark-text-secondary'} rounded`}
>
  Configurações
</button>
```

- [ ] **Step 4: Add Settings screen in content area**

Find where the tab content is rendered (where you show members/payments content) and add:

```typescript
{activeTab === 'members' && (
  // ... existing members content ...
)}

{activeTab === 'payments' && (
  // ... existing payments content ...
)}

{activeTab === 'settings' && (
  <SettingsScreen />
)}
```

- [ ] **Step 5: Call refreshSettings in useEffect**

Find the existing useEffect that calls refreshMembers and refreshPayments:

```typescript
useEffect(() => {
  refreshMembers();
  refreshPayments();
  refreshSettings();  // Add this line
}, []);
```

Wait, I need to make sure refreshSettings is available. Let me check the imports:

```typescript
// Update the useApp destructuring at the top
const { members, payments, settings, refreshMembers, refreshPayments, refreshSettings, addMember, addPayment, updateMemberActive, updateMemberName, deletePayment } = useApp();
```

- [ ] **Step 6: Test the UI**

Run: `npm run tauri dev`
Expected: App opens, Settings tab appears and shows minimum fee field

- [ ] **Step 7: Test saving a setting**

1. Click Settings tab
2. Change minimum fee to "20.00"
3. Click Salvar
4. Should see "Configurações salvas com sucesso"
5. Refresh page and verify value persists

- [ ] **Step 8: Test validation**

Try invalid values:
- "-10" → should show error
- "abc" → should show error
- "10000" → should show error
- "15.999" → should show error

- [ ] **Step 9: Commit**

```bash
git add src/components/MainLayout.tsx
git commit -m "feat: add Settings tab to navigation

Add Settings tab and integrate SettingsScreen component.
Load settings on app startup.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: Debt Calculation Backend Commands

**Files:**
- Create: `src-tauri/src/commands/debt.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Create debt command file**

```rust
// src-tauri/src/commands/debt.rs
use crate::models::debt::calculate_member_debt;
use crate::models::member::get_all_members;
use crate::models::payment::get_payments;
use crate::models::settings::get_setting;
use crate::security::config::load_config;
use crate::security::password::derive_encryption_key;
use crate::db::connection::open_encrypted_db;
use std::path::PathBuf;
use chrono::{Utc, NaiveDate, Datelike};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UnpaidMonth {
    month: i32,
    year: i32,
    amount: f64,
    display: String,
}

#[derive(Serialize, Deserialize)]
pub struct MemberDebtInfo {
    member_id: i64,
    member_name: String,
    total_debt: f64,
    unpaid_months: Vec<UnpaidMonth>,
}

#[tauri::command]
pub fn get_member_debt_cmd(password: String, member_id: i64) -> Result<MemberDebtInfo, String> {
    let conn = get_authenticated_connection(&password)?;
    
    // Get member details
    let member_name: String = conn.query_row(
        "SELECT name FROM members WHERE id = ?",
        [member_id],
        |row| row.get(0),
    ).map_err(|e| format!("Failed to get member: {}", e))?;
    
    let member_start: String = conn.query_row(
        "SELECT start_date FROM members WHERE id = ?",
        [member_id],
        |row| row.get(0),
    ).map_err(|e| format!("Failed to get member start date: {}", e))?;
    
    // Calculate total debt
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let total_debt = calculate_member_debt(&conn, member_id, &today)
        .map_err(|e| format!("Failed to calculate debt: {}", e))?;
    
    // Get unpaid months
    let unpaid_months = get_unpaid_months(&conn, member_id, &member_start, &today)?;
    
    Ok(MemberDebtInfo {
        member_id,
        member_name,
        total_debt,
        unpaid_months,
    })
}

#[tauri::command]
pub fn get_all_debts_cmd(password: String) -> Result<Vec<MemberDebtInfo>, String> {
    let conn = get_authenticated_connection(&password)?;
    
    // Get all active members
    let members = get_all_members(&conn)
        .map_err(|e| format!("Failed to get members: {}", e))?;
    
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let mut debts = Vec::new();
    
    for member in members {
        // Only calculate for active members
        if !member.active {
            continue;
        }
        
        let total_debt = calculate_member_debt(&conn, member.id, &today)
            .map_err(|e| format!("Failed to calculate debt: {}", e))?;
        
        let unpaid_months = get_unpaid_months(&conn, member.id, &member.start_date, &today)?;
        
        debts.push(MemberDebtInfo {
            member_id: member.id,
            member_name: member.name,
            total_debt,
            unpaid_months,
        });
    }
    
    Ok(debts)
}

fn get_unpaid_months(
    conn: &rusqlite::Connection,
    member_id: i64,
    start_date: &str,
    as_of_date: &str,
) -> Result<Vec<UnpaidMonth>, String> {
    let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
        .map_err(|_| "Invalid start date".to_string())?;
    let as_of = NaiveDate::parse_from_str(as_of_date, "%Y-%m-%d")
        .map_err(|_| "Invalid as_of date".to_string())?;
    
    // Get minimum fee
    let min_fee_str = get_setting(conn, "minimum_fee_brl")
        .map_err(|e| format!("Failed to get minimum fee: {}", e))?;
    let min_fee: f64 = min_fee_str.parse().unwrap_or(15.0);
    
    // Get all payments for this member
    let payments = get_payments(conn)
        .map_err(|e| format!("Failed to get payments: {}", e))?
        .into_iter()
        .filter(|p| p.member_id == member_id)
        .collect::<Vec<_>>();
    
    let mut unpaid = Vec::new();
    let mut current = start;
    
    while current <= as_of {
        let month = current.month() as i32;
        let year = current.year() as i32;
        
        // Check if payment exists
        let has_payment = payments.iter().any(|p| p.month == month && p.year == year);
        
        if !has_payment {
            // Check grace period
            let next_month = if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 10)
            } else {
                NaiveDate::from_ymd_opt(year, (month + 1) as u32, 10)
            };
            
            if let Some(deadline) = next_month {
                if as_of > deadline {
                    unpaid.push(UnpaidMonth {
                        month,
                        year,
                        amount: min_fee,
                        display: format_month_pt(month, year),
                    });
                }
            }
        }
        
        // Move to next month
        current = if current.month() == 12 {
            NaiveDate::from_ymd_opt(current.year() + 1, 1, 1).unwrap_or(current)
        } else {
            NaiveDate::from_ymd_opt(current.year(), current.month() + 1, 1).unwrap_or(current)
        };
    }
    
    Ok(unpaid)
}

fn format_month_pt(month: i32, year: i32) -> String {
    let month_name = match month {
        1 => "Janeiro",
        2 => "Fevereiro",
        3 => "Março",
        4 => "Abril",
        5 => "Maio",
        6 => "Junho",
        7 => "Julho",
        8 => "Agosto",
        9 => "Setembro",
        10 => "Outubro",
        11 => "Novembro",
        12 => "Dezembro",
        _ => "Desconhecido",
    };
    format!("{} {}", month_name, year)
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
```

- [ ] **Step 2: Export debt commands in mod.rs**

```rust
// src-tauri/src/commands/mod.rs
pub mod auth;
pub mod database;
pub mod debt;      // Add this line
pub mod member;
pub mod payment;
pub mod settings;

#[cfg(debug_assertions)]
pub mod seed;
```

- [ ] **Step 3: Register commands in lib.rs**

```rust
// In src-tauri/src/lib.rs, add to invoke_handler:
.invoke_handler(tauri::generate_handler![
    commands::auth::check_first_launch,
    commands::auth::setup_password,
    commands::auth::verify_password_cmd,
    commands::database::check_database_initialized,
    commands::database::initialize_database,
    commands::member::add_member_cmd,
    commands::member::get_members_cmd,
    commands::member::get_all_members_cmd,
    commands::member::get_member_cmd,
    commands::member::update_member_active_cmd,
    commands::member::update_member_name_cmd,
    commands::payment::add_payment_cmd,
    commands::payment::get_payments_cmd,
    commands::payment::delete_payment_cmd,
    commands::settings::get_setting_cmd,
    commands::settings::update_setting_cmd,
    commands::debt::get_member_debt_cmd,    // Add this
    commands::debt::get_all_debts_cmd,      // Add this
    #[cfg(debug_assertions)]
    commands::seed::seed_database,
])
```

- [ ] **Step 4: Test compilation**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds with no errors

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/commands/debt.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "feat: add debt calculation backend commands

Add get_member_debt_cmd and get_all_debts_cmd.
Includes unpaid months list with Portuguese month names.
Reuses existing debt calculation logic from models/debt.rs.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: Debt Calculation Frontend Types

**Files:**
- Modify: `src/types/index.ts`

- [ ] **Step 1: Add TypeScript interfaces for debt**

```typescript
// Add to src/types/index.ts

export interface UnpaidMonth {
  month: number;
  year: number;
  amount: number;
  display: string;
}

export interface MemberDebtInfo {
  member_id: number;
  member_name: string;
  total_debt: number;
  unpaid_months: UnpaidMonth[];
}

export const MONTH_NAMES_PT = [
  'Janeiro', 'Fevereiro', 'Março', 'Abril', 'Maio', 'Junho',
  'Julho', 'Agosto', 'Setembro', 'Outubro', 'Novembro', 'Dezembro'
];

export const formatMonthYear = (month: number, year: number): string => {
  return `${MONTH_NAMES_PT[month - 1]} ${year}`;
};
```

- [ ] **Step 2: Commit**

```bash
git add src/types/index.ts
git commit -m "feat: add debt TypeScript interfaces

Add MemberDebtInfo, UnpaidMonth types and month formatting utilities.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 8: Debt Calculation Context Integration

**Files:**
- Modify: `src/contexts/AppContext.tsx`

- [ ] **Step 1: Add debt methods to AppContext**

```typescript
// Update AppContextType interface
import { Member, Payment, AppSettings, MemberDebtInfo } from '../types';

interface AppContextType {
  members: Member[];
  payments: Payment[];
  settings: AppSettings;
  refreshMembers: () => Promise<void>;
  refreshPayments: () => Promise<void>;
  refreshSettings: () => Promise<void>;
  updateSetting: (key: string, value: string) => Promise<void>;
  getMemberDebt: (memberId: number) => Promise<MemberDebtInfo>;  // Add this
  getAllDebts: () => Promise<MemberDebtInfo[]>;  // Add this
  addMember: (name: string, startDate: string) => Promise<void>;
  addPayment: (memberId: number, month: number, year: number, amount: number, paymentDate: string) => Promise<void>;
  updateMemberActive: (id: number, active: boolean) => Promise<void>;
  updateMemberName: (id: number, name: string) => Promise<void>;
  deletePayment: (id: number) => Promise<void>;
}
```

- [ ] **Step 2: Implement debt methods in provider**

```typescript
// In AppProvider component, add these functions:

const getMemberDebt = async (memberId: number): Promise<MemberDebtInfo> => {
  if (!password) throw new Error('Not authenticated');
  const data = await invoke<MemberDebtInfo>('get_member_debt_cmd', { password, memberId });
  return data;
};

const getAllDebts = async (): Promise<MemberDebtInfo[]> => {
  if (!password) throw new Error('Not authenticated');
  const data = await invoke<MemberDebtInfo[]>('get_all_debts_cmd', { password });
  return data;
};

// Update the provider value:
return (
  <AppContext.Provider value={{ 
    members, 
    payments, 
    settings,
    refreshMembers, 
    refreshPayments, 
    refreshSettings,
    updateSetting,
    getMemberDebt,  // Add this
    getAllDebts,    // Add this
    addMember, 
    addPayment, 
    updateMemberActive, 
    updateMemberName, 
    deletePayment 
  }}>
    {children}
  </AppContext.Provider>
);
```

- [ ] **Step 3: Commit**

```bash
git add src/contexts/AppContext.tsx
git commit -m "feat: add debt methods to AppContext

Add getMemberDebt and getAllDebts methods.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 9: Dashboard Screen Component

**Files:**
- Create: `src/components/DashboardScreen.tsx`

- [ ] **Step 1: Create DashboardScreen component**

```typescript
// src/components/DashboardScreen.tsx
import { useState, useEffect } from 'react';
import { useApp } from '../contexts/AppContext';
import { formatCurrency } from '../types';
import type { MemberDebtInfo } from '../types';

export const DashboardScreen = () => {
  const { members, getAllDebts } = useApp();
  const [debts, setDebts] = useState<MemberDebtInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const loadDebts = async () => {
    setLoading(true);
    setError('');
    try {
      const data = await getAllDebts();
      setDebts(data);
    } catch (err) {
      console.error('Error loading debts:', err);
      setError('Erro ao calcular dívidas');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadDebts();
  }, []);

  const totalDebt = debts.reduce((sum, d) => sum + d.total_debt, 0);
  const activeMembers = members.filter(m => m.active).length;

  return (
    <div className="flex-1 p-8">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-dark-text-primary">Dashboard</h1>
        <button
          onClick={loadDebts}
          disabled={loading}
          className="bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90 disabled:opacity-50"
        >
          {loading ? 'Carregando...' : 'Atualizar'}
        </button>
      </div>

      {error && (
        <div className="bg-dark-error/10 border border-dark-error text-dark-error p-4 rounded mb-6">
          {error}
        </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Total Debt Card */}
        <div className={`bg-dark-surface border rounded-lg p-6 ${totalDebt > 0 ? 'border-dark-error' : 'border-dark-border'}`}>
          <h2 className="text-dark-text-secondary mb-4">Dívida Total do Clube</h2>
          {loading ? (
            <p className="text-dark-text-secondary">Calculando...</p>
          ) : (
            <p className={`text-4xl font-bold ${totalDebt > 0 ? 'text-dark-error' : 'text-dark-text-primary'}`}>
              {formatCurrency(totalDebt)}
            </p>
          )}
        </div>

        {/* Active Members Card */}
        <div className="bg-dark-surface border border-dark-border rounded-lg p-6">
          <h2 className="text-dark-text-secondary mb-4">Membros Ativos</h2>
          <p className="text-4xl font-bold text-green-500">
            {activeMembers}
          </p>
        </div>
      </div>
    </div>
  );
};
```

- [ ] **Step 2: Commit**

```bash
git add src/components/DashboardScreen.tsx
git commit -m "feat: create Dashboard component

Display total club debt and active member count.
Minimal design with two summary cards.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 10: Add Dashboard Tab to MainLayout

**Files:**
- Modify: `src/components/MainLayout.tsx`

- [ ] **Step 1: Import DashboardScreen**

```typescript
// At top of src/components/MainLayout.tsx
import { DashboardScreen } from './DashboardScreen';
```

- [ ] **Step 2: Update tab state type and default**

```typescript
const [activeTab, setActiveTab] = useState<'dashboard' | 'members' | 'payments' | 'settings'>('dashboard');
```

- [ ] **Step 3: Add Dashboard tab button (first position)**

```typescript
{/* In the navigation/tab section - add Dashboard FIRST */}
<button
  onClick={() => setActiveTab('dashboard')}
  className={`px-4 py-2 ${activeTab === 'dashboard' ? 'bg-dark-accent text-white' : 'text-dark-text-secondary'} rounded`}
>
  Dashboard
</button>
<button
  onClick={() => setActiveTab('members')}
  className={`px-4 py-2 ${activeTab === 'members' ? 'bg-dark-accent text-white' : 'text-dark-text-secondary'} rounded`}
>
  Membros
</button>
{/* ... rest of tabs ... */}
```

- [ ] **Step 4: Add Dashboard screen in content area**

```typescript
{activeTab === 'dashboard' && (
  <DashboardScreen />
)}

{activeTab === 'members' && (
  // ... existing members content ...
)}

{/* ... rest of content ... */}
```

- [ ] **Step 5: Update useApp destructuring**

```typescript
const { members, payments, settings, refreshMembers, refreshPayments, refreshSettings, getAllDebts, addMember, addPayment, updateMemberActive, updateMemberName, deletePayment } = useApp();
```

- [ ] **Step 6: Test the UI**

Run: `npm run tauri dev`
Expected: App opens to Dashboard tab by default, shows debt and member count

- [ ] **Step 7: Test debt calculation**

1. Add a member with start date in the past (e.g., January 2026)
2. Don't add any payments
3. Click Dashboard tab
4. Should see debt accumulating (multiple months × minimum fee)
5. Add a payment for one month
6. Refresh Dashboard
7. Should see debt reduced

- [ ] **Step 8: Commit**

```bash
git add src/components/MainLayout.tsx
git commit -m "feat: add Dashboard as default tab

Make Dashboard the first tab and default view.
Shows total debt and active member count on startup.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 11: Member Detail View Component Structure

**Files:**
- Create: `src/components/MemberDetailView.tsx`

- [ ] **Step 1: Create MemberDetailView component skeleton**

```typescript
// src/components/MemberDetailView.tsx
import { useState, useEffect } from 'react';
import { useApp } from '../contexts/AppContext';
import { formatCurrency, formatDate, MONTH_NAMES_PT } from '../types';
import type { Member, Payment, MemberDebtInfo } from '../types';

interface MemberDetailViewProps {
  memberId: number;
  onBack: () => void;
}

export const MemberDetailView = ({ memberId, onBack }: MemberDetailViewProps) => {
  const { members, payments, getMemberDebt, updateMemberActive, updateMemberName, deletePayment } = useApp();
  const [debtInfo, setDebtInfo] = useState<MemberDebtInfo | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [editingName, setEditingName] = useState(false);
  const [newName, setNewName] = useState('');
  const [memberPaymentsPage, setMemberPaymentsPage] = useState(1);
  const [memberPaymentsPageSize, setMemberPaymentsPageSize] = useState(15);

  const member = members.find(m => m.id === memberId);

  useEffect(() => {
    if (member) {
      setNewName(member.name);
      loadDebt();
    }
  }, [memberId, member]);

  const loadDebt = async () => {
    setLoading(true);
    setError('');
    try {
      const data = await getMemberDebt(memberId);
      setDebtInfo(data);
    } catch (err) {
      console.error('Error loading debt:', err);
      setError('Erro ao calcular dívida');
    } finally {
      setLoading(false);
    }
  };

  const handleSaveName = async () => {
    try {
      await updateMemberName(memberId, newName);
      setEditingName(false);
      await loadDebt();
    } catch (err) {
      console.error('Error updating name:', err);
      alert(String(err));
    }
  };

  const handleDeactivate = async () => {
    if (confirm('Tem certeza que deseja desativar este membro?')) {
      await updateMemberActive(memberId, false);
      onBack();
    }
  };

  const handleDeletePayment = async (paymentId: number) => {
    if (confirm('Tem certeza que deseja excluir este pagamento?')) {
      await deletePayment(paymentId);
      await loadDebt();
    }
  };

  if (!member) {
    return (
      <div className="flex-1 p-8">
        <p className="text-dark-text-secondary">Membro não encontrado</p>
        <button onClick={onBack} className="mt-4 text-dark-accent">
          ← Voltar para Membros
        </button>
      </div>
    );
  }

  const memberPayments = payments
    .filter(p => p.member_id === memberId)
    .sort((a, b) => new Date(b.payment_date).getTime() - new Date(a.payment_date).getTime());

  const paginatedMemberPayments = memberPayments.slice(
    (memberPaymentsPage - 1) * memberPaymentsPageSize,
    memberPaymentsPage * memberPaymentsPageSize
  );
  const memberPaymentsTotalPages = Math.ceil(memberPayments.length / memberPaymentsPageSize);

  return (
    <div className="flex-1 p-8">
      {/* Back button */}
      <button onClick={onBack} className="mb-4 text-dark-accent hover:underline">
        ← Voltar para Membros
      </button>

      {/* Header */}
      <div className="bg-dark-surface border border-dark-border rounded-lg p-6 mb-6">
        {editingName ? (
          <div className="flex items-center gap-2">
            <input
              type="text"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              className="bg-dark-bg border border-dark-border text-dark-text-primary rounded px-2 py-1 text-2xl font-bold"
            />
            <button onClick={handleSaveName} className="text-green-500 px-3 py-1">Salvar</button>
            <button onClick={() => { setEditingName(false); setNewName(member.name); }} className="text-dark-text-secondary px-3 py-1">Cancelar</button>
          </div>
        ) : (
          <h1 className="text-2xl font-bold text-dark-text-primary">{member.name}</h1>
        )}
        <p className="text-dark-text-secondary mt-2">Membro desde {formatDate(member.start_date)}</p>
        <div className="flex gap-2 mt-4">
          <button
            onClick={() => setEditingName(true)}
            className="bg-dark-accent text-white px-3 py-1 rounded text-sm hover:opacity-90"
          >
            Editar Nome
          </button>
          <button
            onClick={handleDeactivate}
            className="bg-dark-error text-white px-3 py-1 rounded text-sm hover:opacity-90"
          >
            Desativar
          </button>
        </div>
      </div>

      {/* Debt Summary Card */}
      {loading ? (
        <div className="bg-dark-surface border border-dark-border rounded-lg p-6 mb-6">
          <p className="text-dark-text-secondary">Calculando dívida...</p>
        </div>
      ) : error ? (
        <div className="bg-dark-error/10 border border-dark-error text-dark-error rounded-lg p-6 mb-6">
          {error}
        </div>
      ) : debtInfo ? (
        <>
          <div className={`border rounded-lg p-6 mb-6 ${debtInfo.total_debt > 0 ? 'bg-dark-error/10 border-dark-error' : 'bg-dark-surface border-dark-border'}`}>
            <h2 className="text-dark-text-secondary mb-2">Dívida Atual</h2>
            <p className={`text-3xl font-bold ${debtInfo.total_debt > 0 ? 'text-dark-error' : 'text-green-500'}`}>
              {formatCurrency(debtInfo.total_debt)}
            </p>
            <p className="text-dark-text-secondary mt-2">
              Meses em atraso: {debtInfo.unpaid_months.length}
            </p>
          </div>

          {/* Unpaid Months Section */}
          {debtInfo.unpaid_months.length > 0 && (
            <div className="bg-dark-surface border border-dark-border rounded-lg p-6 mb-6">
              <h2 className="text-xl font-bold text-dark-text-primary mb-4">Meses Não Pagos</h2>
              <table className="w-full">
                <thead>
                  <tr className="border-b border-dark-border">
                    <th className="text-left py-2 text-dark-text-secondary">Mês/Ano</th>
                    <th className="text-left py-2 text-dark-text-secondary">Valor</th>
                    <th className="text-left py-2 text-dark-text-secondary">Ação</th>
                  </tr>
                </thead>
                <tbody>
                  {debtInfo.unpaid_months.map((um, idx) => (
                    <tr key={idx} className="border-b border-dark-border">
                      <td className="py-2 text-dark-text-primary">{um.display}</td>
                      <td className="py-2 text-dark-text-primary">{formatCurrency(um.amount)}</td>
                      <td className="py-2">
                        <button className="text-dark-accent text-sm hover:underline">
                          + Adicionar Pagamento
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </>
      ) : null}

      {/* Payment History Section */}
      <div className="bg-dark-surface border border-dark-border rounded-lg p-6">
        <h2 className="text-xl font-bold text-dark-text-primary mb-4">Histórico de Pagamentos</h2>
        {memberPayments.length === 0 ? (
          <p className="text-dark-text-secondary">Nenhum pagamento registrado</p>
        ) : (
          <>
            <table className="w-full">
              <thead>
                <tr className="border-b border-dark-border">
                  <th className="text-left py-2 text-dark-text-secondary">Data Pago</th>
                  <th className="text-left py-2 text-dark-text-secondary">Ref. Mês/Ano</th>
                  <th className="text-left py-2 text-dark-text-secondary">Valor</th>
                  <th className="text-left py-2 text-dark-text-secondary">Ação</th>
                </tr>
              </thead>
              <tbody>
                {paginatedMemberPayments.map((payment) => (
                  <tr key={payment.id} className="border-b border-dark-border">
                    <td className="py-2 text-dark-text-primary">{formatDate(payment.payment_date)}</td>
                    <td className="py-2 text-dark-text-primary">
                      {MONTH_NAMES_PT[payment.month - 1]} {payment.year}
                    </td>
                    <td className="py-2 text-dark-text-primary">{formatCurrency(payment.amount_brl)}</td>
                    <td className="py-2">
                      <button
                        onClick={() => handleDeletePayment(payment.id)}
                        className="text-dark-error text-sm hover:underline"
                      >
                        Excluir
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>

            {/* Pagination */}
            {memberPaymentsTotalPages > 1 && (
              <div className="flex items-center justify-between mt-4">
                <div className="flex items-center gap-2">
                  <button
                    onClick={() => setMemberPaymentsPage(p => Math.max(1, p - 1))}
                    disabled={memberPaymentsPage === 1}
                    className="px-3 py-1 bg-dark-bg border border-dark-border rounded disabled:opacity-50"
                  >
                    Anterior
                  </button>
                  <span className="text-dark-text-secondary">
                    Página {memberPaymentsPage} de {memberPaymentsTotalPages}
                  </span>
                  <button
                    onClick={() => setMemberPaymentsPage(p => Math.min(memberPaymentsTotalPages, p + 1))}
                    disabled={memberPaymentsPage === memberPaymentsTotalPages}
                    className="px-3 py-1 bg-dark-bg border border-dark-border rounded disabled:opacity-50"
                  >
                    Próxima
                  </button>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-dark-text-secondary text-sm">Itens por página:</span>
                  <select
                    value={memberPaymentsPageSize}
                    onChange={(e) => {
                      setMemberPaymentsPageSize(Number(e.target.value));
                      setMemberPaymentsPage(1);
                    }}
                    className="bg-dark-bg border border-dark-border text-dark-text-primary rounded px-2 py-1 text-sm"
                  >
                    <option value="15">15</option>
                    <option value="30">30</option>
                    <option value="100">100</option>
                  </select>
                </div>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
};
```

- [ ] **Step 2: Commit**

```bash
git add src/components/MemberDetailView.tsx
git commit -m "feat: create MemberDetailView component

Display individual member details with:
- Header (name, start date, edit/deactivate buttons)
- Debt summary card
- Unpaid months table
- Payment history with pagination

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 12: Add Member Detail Navigation to MainLayout

**Files:**
- Modify: `src/components/MainLayout.tsx`

- [ ] **Step 1: Import MemberDetailView**

```typescript
// At top of src/components/MainLayout.tsx
import { MemberDetailView } from './MemberDetailView';
```

- [ ] **Step 2: Add state for selected member**

```typescript
// Add this state near the top of MainLayout component
const [selectedMemberId, setSelectedMemberId] = useState<number | null>(null);
const [viewingMemberDetail, setViewingMemberDetail] = useState(false);
```

- [ ] **Step 3: Add click handler for member names**

Find the members table where member names are displayed and make them clickable:

```typescript
{/* In the members table, find the name cell and update it: */}
<td className="py-2 text-dark-text-primary">
  <button
    onClick={() => {
      setSelectedMemberId(member.id);
      setViewingMemberDetail(true);
    }}
    className="text-dark-accent hover:underline text-left"
  >
    {editingMemberId === member.id ? (
      <input
        type="text"
        value={editingMemberName}
        onChange={(e) => setEditingMemberName(e.target.value)}
        className="bg-dark-bg border border-dark-border text-dark-text-primary rounded px-2 py-1"
        onClick={(e) => e.stopPropagation()}
      />
    ) : (
      member.name
    )}
  </button>
</td>
```

- [ ] **Step 4: Show MemberDetailView when member selected**

Replace the members tab content conditional with:

```typescript
{activeTab === 'members' && !viewingMemberDetail && (
  // ... existing members table content ...
)}

{activeTab === 'members' && viewingMemberDetail && selectedMemberId && (
  <MemberDetailView
    memberId={selectedMemberId}
    onBack={() => {
      setViewingMemberDetail(false);
      setSelectedMemberId(null);
      refreshMembers();
      refreshPayments();
    }}
  />
)}
```

- [ ] **Step 5: Test navigation**

Run: `npm run tauri dev`
1. Go to Members tab
2. Click on a member name
3. Should navigate to member detail view
4. Should show debt, unpaid months, and payment history
5. Click "← Voltar para Membros"
6. Should return to members list

- [ ] **Step 6: Commit**

```bash
git add src/components/MainLayout.tsx
git commit -m "feat: add member detail navigation

Make member names clickable to view detail page.
Show MemberDetailView when member selected.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 13: Global Payment Modal Component

**Files:**
- Create: `src/components/AddPaymentModal.tsx`

- [ ] **Step 1: Create AddPaymentModal component**

```typescript
// src/components/AddPaymentModal.tsx
import { useState, useEffect } from 'react';
import { useApp } from '../contexts/AppContext';
import { MONTH_NAMES_PT } from '../types';
import { DateInput } from './DateInput';

interface PaymentPrefill {
  memberId?: number;
  memberName?: string;
  month?: number;
  year?: number;
}

interface AddPaymentModalProps {
  isOpen: boolean;
  onClose: () => void;
  prefill?: PaymentPrefill;
}

export const AddPaymentModal = ({ isOpen, onClose, prefill }: AddPaymentModalProps) => {
  const { members, settings, addPayment } = useApp();
  const [selectedMemberName, setSelectedMemberName] = useState('');
  const [selectedMemberId, setSelectedMemberId] = useState(0);
  const [paymentMonth, setPaymentMonth] = useState(new Date().getMonth() + 1);
  const [paymentYear, setPaymentYear] = useState(new Date().getFullYear());
  const [paymentAmount, setPaymentAmount] = useState('15.00');
  const [paymentDate, setPaymentDate] = useState(new Date().toISOString().split('T')[0]);
  const [error, setError] = useState('');
  const [submitting, setSubmitting] = useState(false);

  const activeMembers = members.filter(m => m.active);

  useEffect(() => {
    if (isOpen) {
      // Reset or pre-fill form
      if (prefill) {
        setSelectedMemberId(prefill.memberId || 0);
        setSelectedMemberName(prefill.memberName || '');
        setPaymentMonth(prefill.month || new Date().getMonth() + 1);
        setPaymentYear(prefill.year || new Date().getFullYear());
      } else {
        setSelectedMemberId(0);
        setSelectedMemberName('');
        setPaymentMonth(new Date().getMonth() + 1);
        setPaymentYear(new Date().getFullYear());
      }
      setPaymentAmount(settings.minimumFee);
      setPaymentDate(new Date().toISOString().split('T')[0]);
      setError('');
    }
  }, [isOpen, prefill, settings.minimumFee]);

  const handleMemberInputChange = (value: string) => {
    setSelectedMemberName(value);
    const member = activeMembers.find(m => m.name === value);
    setSelectedMemberId(member?.id || 0);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setSubmitting(true);

    try {
      await addPayment(selectedMemberId, paymentMonth, paymentYear, parseFloat(paymentAmount), paymentDate);
      onClose();
    } catch (err) {
      console.error('Error adding payment:', err);
      setError(String(err));
    } finally {
      setSubmitting(false);
    }
  };

  const handleClose = () => {
    if (!submitting) {
      onClose();
    }
  };

  const handleBackdropClick = (e: React.MouseEvent) => {
    if (e.target === e.currentTarget) {
      handleClose();
    }
  };

  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        handleClose();
      }
    };

    if (isOpen) {
      document.addEventListener('keydown', handleEscape);
    }

    return () => {
      document.removeEventListener('keydown', handleEscape);
    };
  }, [isOpen, submitting]);

  if (!isOpen) return null;

  const isPrefilled = !!prefill?.memberId;

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
      onClick={handleBackdropClick}
    >
      <div className="bg-dark-surface border border-dark-border rounded-lg p-6 w-full max-w-md relative">
        <button
          onClick={handleClose}
          disabled={submitting}
          className="absolute top-4 right-4 text-dark-text-secondary hover:text-dark-text-primary disabled:opacity-50"
        >
          ✕
        </button>

        <h2 className="text-xl font-bold text-dark-text-primary mb-4">Adicionar Pagamento</h2>

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">Membro</label>
            <input
              list="member-list-modal"
              value={selectedMemberName}
              onChange={(e) => handleMemberInputChange(e.target.value)}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              placeholder="Digite o nome do membro"
              required
              disabled={isPrefilled}
            />
            <datalist id="member-list-modal">
              {activeMembers.map((member) => (
                <option key={member.id} value={member.name} />
              ))}
            </datalist>
          </div>

          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">Mês</label>
            <select
              value={paymentMonth}
              onChange={(e) => setPaymentMonth(Number(e.target.value))}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              required
              disabled={isPrefilled}
            >
              {MONTH_NAMES_PT.map((name, idx) => (
                <option key={idx} value={idx + 1}>
                  {name}
                </option>
              ))}
            </select>
          </div>

          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">Ano</label>
            <input
              type="number"
              value={paymentYear}
              onChange={(e) => setPaymentYear(Number(e.target.value))}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              required
              disabled={isPrefilled}
            />
          </div>

          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">Valor (R$)</label>
            <input
              type="text"
              value={paymentAmount}
              onChange={(e) => setPaymentAmount(e.target.value)}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              placeholder="15.00"
              required
            />
          </div>

          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">Data do Pagamento</label>
            <DateInput
              value={paymentDate}
              onChange={setPaymentDate}
              className="w-full"
            />
          </div>

          {error && <p className="text-dark-error text-sm mb-4">{error}</p>}

          <div className="flex gap-2 justify-end">
            <button
              type="button"
              onClick={handleClose}
              disabled={submitting}
              className="px-4 py-2 border border-dark-border text-dark-text-secondary rounded hover:bg-dark-bg disabled:opacity-50"
            >
              Cancelar
            </button>
            <button
              type="submit"
              disabled={submitting}
              className="px-4 py-2 bg-dark-accent text-white rounded hover:opacity-90 disabled:opacity-50"
            >
              {submitting ? 'Salvando...' : 'Salvar'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};
```

- [ ] **Step 2: Commit**

```bash
git add src/components/AddPaymentModal.tsx
git commit -m "feat: create AddPaymentModal component

Global payment modal with:
- Pre-fill support (locks member/month/year when pre-filled)
- Backdrop click and ESC key to close
- Form validation and error display
- Uses existing DateInput component

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 14: Payment Modal Context Integration

**Files:**
- Modify: `src/contexts/AppContext.tsx`
- Modify: `src/types/index.ts`

- [ ] **Step 1: Add PaymentPrefill type to types**

```typescript
// Add to src/types/index.ts
export interface PaymentPrefill {
  memberId?: number;
  memberName?: string;
  month?: number;
  year?: number;
}
```

- [ ] **Step 2: Add modal state to AppContext**

```typescript
// Update imports
import { Member, Payment, AppSettings, MemberDebtInfo, PaymentPrefill } from '../types';

interface AppContextType {
  members: Member[];
  payments: Payment[];
  settings: AppSettings;
  paymentModalOpen: boolean;  // Add this
  paymentModalPrefill?: PaymentPrefill;  // Add this
  refreshMembers: () => Promise<void>;
  refreshPayments: () => Promise<void>;
  refreshSettings: () => Promise<void>;
  updateSetting: (key: string, value: string) => Promise<void>;
  getMemberDebt: (memberId: number) => Promise<MemberDebtInfo>;
  getAllDebts: () => Promise<MemberDebtInfo[]>;
  openPaymentModal: (prefill?: PaymentPrefill) => void;  // Add this
  closePaymentModal: () => void;  // Add this
  addMember: (name: string, startDate: string) => Promise<void>;
  addPayment: (memberId: number, month: number, year: number, amount: number, paymentDate: string) => Promise<void>;
  updateMemberActive: (id: number, active: boolean) => Promise<void>;
  updateMemberName: (id: number, name: string) => Promise<void>;
  deletePayment: (id: number) => Promise<void>;
}
```

- [ ] **Step 3: Implement modal state in provider**

```typescript
// In AppProvider component, add state:
const [paymentModalOpen, setPaymentModalOpen] = useState(false);
const [paymentModalPrefill, setPaymentModalPrefill] = useState<PaymentPrefill>();

// Add functions:
const openPaymentModal = (prefill?: PaymentPrefill) => {
  setPaymentModalPrefill(prefill);
  setPaymentModalOpen(true);
};

const closePaymentModal = () => {
  setPaymentModalOpen(false);
  setPaymentModalPrefill(undefined);
};

// Update provider value:
return (
  <AppContext.Provider value={{ 
    members, 
    payments, 
    settings,
    paymentModalOpen,           // Add this
    paymentModalPrefill,        // Add this
    refreshMembers, 
    refreshPayments, 
    refreshSettings,
    updateSetting,
    getMemberDebt,
    getAllDebts,
    openPaymentModal,           // Add this
    closePaymentModal,          // Add this
    addMember, 
    addPayment, 
    updateMemberActive, 
    updateMemberName, 
    deletePayment 
  }}>
    {children}
  </AppContext.Provider>
);
```

- [ ] **Step 4: Commit**

```bash
git add src/types/index.ts src/contexts/AppContext.tsx
git commit -m "feat: add payment modal state to AppContext

Add PaymentPrefill type and modal state management.
Provides openPaymentModal and closePaymentModal methods.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 15: Wire Payment Modal to MainLayout

**Files:**
- Modify: `src/components/MainLayout.tsx`

- [ ] **Step 1: Import AddPaymentModal**

```typescript
// At top of src/components/MainLayout.tsx
import { AddPaymentModal } from './AddPaymentModal';
```

- [ ] **Step 2: Update useApp destructuring**

```typescript
const { 
  members, 
  payments, 
  settings, 
  paymentModalOpen,      // Add this
  paymentModalPrefill,   // Add this
  refreshMembers, 
  refreshPayments, 
  refreshSettings, 
  getAllDebts, 
  openPaymentModal,      // Add this
  closePaymentModal,     // Add this
  addMember, 
  addPayment, 
  updateMemberActive, 
  updateMemberName, 
  deletePayment 
} = useApp();
```

- [ ] **Step 3: Add toolbar button for payment modal**

Find the navigation/tab section and add a toolbar area with the payment button:

```typescript
{/* Add this above or beside the tab buttons */}
<div className="flex items-center gap-4 mb-6">
  <div className="flex gap-2">
    {/* Existing tab buttons here */}
  </div>
  
  <div className="ml-auto">
    <button
      onClick={() => openPaymentModal()}
      className="bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90"
    >
      + Adicionar Pagamento
    </button>
  </div>
</div>
```

- [ ] **Step 4: Render AddPaymentModal at end of component**

```typescript
{/* Add at the very end of MainLayout return, before closing div */}
<AddPaymentModal
  isOpen={paymentModalOpen}
  onClose={closePaymentModal}
  prefill={paymentModalPrefill}
/>
```

- [ ] **Step 5: Test toolbar button**

Run: `npm run tauri dev`
1. Click "+ Adicionar Pagamento" in toolbar
2. Modal should open
3. All fields should be empty/default
4. Fill form and submit
5. Modal should close and data refresh

- [ ] **Step 6: Commit**

```bash
git add src/components/MainLayout.tsx
git commit -m "feat: add payment modal to MainLayout

Add toolbar button to open payment modal from anywhere.
Render AddPaymentModal at root level.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 16: Wire Payment Modal to Member Detail Unpaid Months

**Files:**
- Modify: `src/components/MemberDetailView.tsx`

- [ ] **Step 1: Update useApp destructuring**

```typescript
const { 
  members, 
  payments, 
  getMemberDebt, 
  openPaymentModal,      // Add this
  updateMemberActive, 
  updateMemberName, 
  deletePayment 
} = useApp();
```

- [ ] **Step 2: Update unpaid month button click handler**

Find the "Adicionar Pagamento" button in the unpaid months table and update it:

```typescript
<button
  onClick={() => openPaymentModal({
    memberId: member.id,
    memberName: member.name,
    month: um.month,
    year: um.year,
  })}
  className="text-dark-accent text-sm hover:underline"
>
  + Adicionar Pagamento
</button>
```

- [ ] **Step 3: Test pre-filled modal**

Run: `npm run tauri dev`
1. Go to a member detail view with unpaid months
2. Click "+ Adicionar Pagamento" on an unpaid month row
3. Modal should open with member, month, year pre-filled and locked
4. Amount and date should be editable
5. Submit payment
6. Modal closes, debt recalculates, unpaid month disappears

- [ ] **Step 4: Commit**

```bash
git add src/components/MemberDetailView.tsx
git commit -m "feat: wire payment modal to unpaid months

Pre-fill modal with member/month/year when clicked from unpaid months.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 17: Remove Inline Payment Form from Payments Tab (Optional)

**Files:**
- Modify: `src/components/MainLayout.tsx`

Note: This step is optional. The design spec suggests keeping both the Payments tab form AND the global modal, or replacing the form with a button. We'll keep the existing form for now since it works and provides an alternative workflow.

- [ ] **Step 1: Decision point**

Choose one:
- **Option A:** Keep existing payment form in Payments tab (no changes needed)
- **Option B:** Replace payment form with button that opens modal

If choosing Option B, replace the payment form section with:

```typescript
{activeTab === 'payments' && (
  <div className="flex-1 p-8">
    <div className="flex justify-between items-center mb-6">
      <h2 className="text-xl font-bold text-dark-text-primary">Pagamentos</h2>
      <button
        onClick={() => openPaymentModal()}
        className="bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90"
      >
        + Adicionar Pagamento
      </button>
    </div>
    {/* Payment list table here */}
  </div>
)}
```

- [ ] **Step 2: Commit (if changed)**

```bash
git add src/components/MainLayout.tsx
git commit -m "refactor: replace Payments tab form with modal button

Use global payment modal for consistency.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 18: Enhanced Validation Messages

**Files:**
- Modify: `src/components/AddPaymentModal.tsx`
- Modify: `src/components/SettingsScreen.tsx`

- [ ] **Step 1: Improve payment modal validation messages**

The backend already returns Portuguese error messages. Ensure they're displayed properly:

```typescript
// In AddPaymentModal, the error display is already good:
{error && <p className="text-dark-error text-sm mb-4">{error}</p>}

// The backend returns messages like:
// "Já existe um pagamento para este membro neste mês"
// "Campo obrigatório"
// etc.
```

- [ ] **Step 2: Improve settings validation messages**

The backend already returns Portuguese validation errors. Settings component already displays them:

```typescript
// In SettingsScreen:
{error && <p className="text-dark-error text-sm mt-2">{error}</p>}

// Backend returns:
// "Valor inválido"
// "Valor deve ser maior que zero"
// "Valor máximo: R$ 9999.99"
// "Máximo 2 casas decimais"
```

- [ ] **Step 3: Verify error messages work end-to-end**

Test various error scenarios:
1. Settings: Try "abc", "-10", "10000", "15.999"
2. Payment modal: Try duplicate payment, invalid member, etc.
3. All error messages should be in Portuguese

- [ ] **Step 4: Commit (if any changes)**

```bash
git add src/components/AddPaymentModal.tsx src/components/SettingsScreen.tsx
git commit -m "chore: verify Portuguese error messages

Ensure all validation errors display in Portuguese.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 19: Final Testing and Bug Fixes

- [ ] **Step 1: Test Settings workflow**

1. Open Settings tab
2. Change minimum fee to "20.00"
3. Save
4. Go to Dashboard
5. Verify debt calculations use new fee (R$ 20 per month)
6. Change back to "15.00"
7. Verify Dashboard updates

- [ ] **Step 2: Test Dashboard workflow**

1. Open Dashboard (should be default)
2. Verify total debt matches sum of all member debts
3. Verify active member count is correct
4. Add/remove members, verify counts update
5. Click "Atualizar" button, verify refresh works

- [ ] **Step 3: Test Member Detail workflow**

1. Go to Members tab
2. Click a member name
3. Verify member detail opens with correct info
4. Verify debt calculation is accurate
5. Verify unpaid months list is correct
6. Verify payment history shows all payments
7. Test pagination on payment history
8. Click unpaid month "Adicionar Pagamento"
9. Verify modal opens pre-filled
10. Submit payment
11. Verify unpaid month disappears
12. Test Edit Name functionality
13. Test Deactivate functionality
14. Click "Voltar para Membros"
15. Verify returns to members list

- [ ] **Step 4: Test Payment Modal workflow**

1. Click toolbar "+ Adicionar Pagamento"
2. Verify modal opens empty
3. Test form validation (required fields, etc.)
4. Submit payment
5. Verify modal closes
6. Verify payment appears in Payments tab
7. Test ESC key closes modal
8. Test click outside closes modal
9. Test from Member Detail unpaid months
10. Verify pre-filled fields are locked

- [ ] **Step 5: Test edge cases**

1. Member with no start date (should handle gracefully)
2. Payment for future month (should work)
3. Very large debt amount (UI should handle)
4. Member with 100+ payments (pagination should work)
5. Settings with invalid values (should show errors)
6. Modal open when navigating away (should close cleanly)

- [ ] **Step 6: Fix any bugs found**

Document and fix any issues discovered during testing.

- [ ] **Step 7: Commit bug fixes (if any)**

```bash
git add .
git commit -m "fix: resolve issues found in Phase 2 testing

[Describe specific fixes here]

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 20: Documentation and Cleanup

- [ ] **Step 1: Update README or docs**

If project has a README, add Phase 2 features:
- Settings configuration
- Dashboard overview
- Member detail views
- Global payment modal

- [ ] **Step 2: Remove any debug code**

Check for console.log statements or debug code:

```bash
grep -r "console.log" src/components/*.tsx | grep -v "console.error"
```

Remove any debug logging that was added during development.

- [ ] **Step 3: Verify all TypeScript types**

Run type check:

```bash
npm run check
```

Expected: No type errors

- [ ] **Step 4: Final build test**

```bash
npm run tauri build
```

Expected: Build completes successfully

- [ ] **Step 5: Commit cleanup**

```bash
git add .
git commit -m "chore: Phase 2 cleanup and documentation

Remove debug code, update docs, verify types.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Success Criteria

Phase 2 is complete when:

- [x] Settings screen works (change minimum fee)
- [x] Dashboard shows total debt and member count
- [x] Member detail view accessible by clicking name
- [x] Member detail shows debt, unpaid months, payment history
- [x] Global payment modal accessible from toolbar
- [x] Payment modal can be pre-filled from unpaid months
- [x] All debt calculations are accurate
- [x] All UI text is in Portuguese
- [x] Dark theme is consistent
- [x] No TypeScript errors
- [x] App builds successfully

## Next Steps

After Phase 2:
- Phase 3: CSV/XLSX export with anonymization
- Phase 4: Password change, charts, search/filter, installers
