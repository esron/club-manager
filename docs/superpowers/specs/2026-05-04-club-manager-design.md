# Gestor do Clube - Design Specification

**Version:** 1.0  
**Date:** 2026-05-04  
**Status:** Approved

## Overview

A secure, offline desktop application for managing member contributions in a private club. The application tracks monthly membership fees, calculates debts, and generates reports while maintaining data security through encryption.

## Core Requirements

### Functional Requirements

1. **Member Management**
   - Add members (name + start date only)
   - View member list with debt status
   - View individual member payment history
   - Soft delete (mark inactive) to preserve history

2. **Payment Tracking**
   - Register payments for any member/month combination
   - User selects which month the payment applies to
   - Support payments for current, past, and future months
   - Prevent duplicate payments (one payment per member per month)
   - Track payment date and amount in BRL (R$)

3. **Debt Calculation**
   - Debt = any month without payment that is past the 10th of the following month
   - Example: No payment for March → after April 10th → R$ 15 debt (minimum fee)
   - Debt accumulates monthly until paid
   - No automatic FIFO payment application - user chooses which month to pay

4. **Reporting & Export**
   - Export data to CSV or XLSX format
   - Date range selection
   - Two export modes:
     - **Complete:** Shows member names
     - **Anonymized:** Shows "Membro #1", "Membro #2", etc.
   - Re-authentication required before export (security)
   - Compatible with LibreOffice Calc, Microsoft Excel, Google Sheets

5. **Configuration**
   - Configurable minimum monthly fee (default: R$ 15,00)
   - Password change functionality

### Non-Functional Requirements

1. **Platform**
   - Desktop application (Windows + Linux)
   - Standalone executable (no browser or internet required)
   - Single-user, offline operation

2. **Security**
   - Password-protected access
   - Encrypted data storage
   - Database file transferable while encrypted
   - No password recovery mechanism
   - Re-authentication for sensitive operations

3. **Language**
   - Brazilian Portuguese UI
   - BRL currency format (R$ 15,00)
   - Brazilian date format (DD/MM/YYYY)

4. **User Experience**
   - Dark theme UI
   - Easy-to-use interface
   - Fast performance (handles 100+ members)

## Architecture

### Technology Stack

**Framework:** Tauri (Rust + React)

**Frontend:**
- React 18 with TypeScript
- Tailwind CSS (dark theme)
- State management: React Context API
- Form handling: React Hook Form
- Date handling: date-fns

**Backend:**
- Rust (Tauri)
- SQLCipher for encrypted database
- PBKDF2 for key derivation
- rust_xlsxwriter for XLSX exports
- csv crate for CSV exports

**Build Targets:**
- Windows: `.exe` installer
- Linux: AppImage or .deb package

### System Architecture

```
┌─────────────────────────────────────────┐
│          Tauri Desktop App              │
├─────────────────────────────────────────┤
│  Frontend (React + TypeScript)          │
│  - Dashboard, Members, Payments, Reports│
│  - Dark theme UI with Tailwind CSS      │
│  - Form validation and state management │
├─────────────────────────────────────────┤
│  Tauri Commands (Rust Backend)          │
│  - Database operations (CRUD)           │
│  - Password management & hashing        │
│  - File export (CSV/XLSX)              │
│  - Encryption key derivation (PBKDF2)   │
├─────────────────────────────────────────┤
│  SQLCipher Database (AES-256)           │
│  - Encrypted SQLite database            │
│  - Password-protected                   │
│  - Single transferable .db file         │
└─────────────────────────────────────────┘
```

### Application Flow

1. **First Launch:**
   - User sets password (8+ characters)
   - Password → PBKDF2 (100,000 iterations, SHA-256) → encryption key
   - Create encrypted SQLCipher database
   - Store password hash (bcrypt) for login validation

2. **Subsequent Launches:**
   - User enters password
   - Verify password hash
   - Derive encryption key from password
   - Unlock SQLCipher database
   - Load application state

3. **Data Operations:**
   - All operations via Tauri commands (Rust backend)
   - Frontend sends commands → Rust executes → Database updates
   - Database automatically encrypted at rest (SQLCipher)

4. **Exit:**
   - Database auto-locks
   - Sensitive data cleared from memory
   - Requires password on next launch

### File Storage

**Windows:**
- Database: `C:\Users\{username}\Documents\GestorDoClube\clube.db`
- Config: `C:\Users\{username}\Documents\GestorDoClube\config.json`

**Linux:**
- Database: `~/Documents/GestorDoClube/clube.db`
- Config: `~/Documents/GestorDoClube/config.json`

**config.json** contains:
```json
{
  "password_hash": "bcrypt_hash_here",
  "minimum_fee_brl": "15.00",
  "created_at": "2026-05-04T10:00:00Z"
}
```

## Database Schema

### Tables

```sql
-- Members table
CREATE TABLE members (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    start_date TEXT NOT NULL,  -- ISO format: YYYY-MM-DD
    created_at TEXT NOT NULL,
    active BOOLEAN DEFAULT 1   -- Soft delete flag
);

-- Payments table
CREATE TABLE payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    member_id INTEGER NOT NULL,
    month INTEGER NOT NULL,        -- 1-12
    year INTEGER NOT NULL,          -- 2026, 2027, etc.
    amount_brl REAL NOT NULL,       -- Amount in BRL (e.g., 15.00)
    payment_date TEXT NOT NULL,     -- ISO format: YYYY-MM-DD
    created_at TEXT NOT NULL,
    FOREIGN KEY (member_id) REFERENCES members(id),
    UNIQUE(member_id, month, year)  -- Prevent duplicate payments
);

-- Settings table
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- Initial data
INSERT INTO settings (key, value) VALUES ('minimum_fee_brl', '15.00');
```

### Indexes

```sql
CREATE INDEX idx_payments_member ON payments(member_id);
CREATE INDEX idx_payments_date ON payments(year, month);
CREATE INDEX idx_members_active ON members(active);
```

### Data Types & Formats

- **Dates:** Stored as ISO 8601 (YYYY-MM-DD) for sorting/querying
- **Currency:** Stored as REAL (e.g., 15.50), displayed as R$ 15,50
- **Month/Year:** Separate INTEGER columns for efficient querying

## Security Design

### Password & Encryption

**Password Requirements:**
- Minimum 8 characters
- No maximum (reasonable limit: 128 chars)
- No complexity requirements (user choice)
- Validation on input

**Encryption Flow:**

1. **Password → Key Derivation:**
   ```
   Password (user input)
     ↓
   PBKDF2-SHA256 (100,000 iterations, random salt)
     ↓
   256-bit encryption key
     ↓
   SQLCipher database unlock
   ```

2. **Password Hashing (for login validation):**
   ```
   Password (user input)
     ↓
   bcrypt (cost factor: 12)
     ↓
   Stored in config.json
   ```

**SQLCipher Configuration:**
```rust
PRAGMA cipher = 'aes-256-cbc';
PRAGMA kdf_iter = 100000;
PRAGMA cipher_page_size = 4096;
```

### Password Change Process

1. User enters current password + new password
2. Verify current password hash
3. Derive new encryption key from new password
4. Execute `PRAGMA rekey = 'new_derived_key'`
5. SQLCipher re-encrypts entire database
6. Update password hash in config.json
7. Success confirmation

**Important:** Old password becomes invalid immediately after re-encryption.

### Export Security

Before exporting data:
1. Show password re-entry dialog
2. Verify password matches current hash
3. Proceed with export only if valid
4. Export files are NOT encrypted (user responsibility)

### Security Warnings

Display on first launch:
```
⚠️ IMPORTANTE: Não há recuperação de senha!

Se você esquecer sua senha, seus dados serão 
perdidos permanentemente. Guarde sua senha em 
local seguro.

Recomendamos:
- Anotar em local seguro físico
- Usar gerenciador de senhas
- Fazer backup regular do arquivo clube.db
```

## User Interface Design

### Theme: Dark Mode

**Color Palette:**
- Background: `#1a1a1a`
- Surface: `#2d2d2d`
- Border: `#404040`
- Text Primary: `#e0e0e0`
- Text Secondary: `#888888`
- Accent: `#3a5a7a` (blue)
- Success: `#4ade80` (green)
- Error: `#f87171` (red)
- Warning: `#ffc107` (yellow/amber)

### Navigation Structure

**Sidebar Navigation (180px wide):**
```
📊 Gestor do Clube
─────────────────
Dashboard     ◀ (selected state)
Membros
Pagamentos
Relatórios
Configurações
```

**Main Content Area:**
- Full height minus top padding
- Scrollable content
- Responsive to window resize

### Screen Layouts

#### 1. Login Screen (First Launch)

```
┌────────────────────────────────────┐
│                                    │
│         📊 Gestor do Clube         │
│                                    │
│    ┌──────────────────────────┐   │
│    │  Criar Senha             │   │
│    │  [____________]          │   │
│    │                          │   │
│    │  Confirmar Senha         │   │
│    │  [____________]          │   │
│    │                          │   │
│    │  [  Criar  ]             │   │
│    └──────────────────────────┘   │
│                                    │
│    ⚠️ Aviso sobre senha           │
│                                    │
└────────────────────────────────────┘
```

#### 2. Dashboard

**Widgets (as approved):**

**Monthly Summary Cards (Top):**
```
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│  R$ 450,00   │ │  R$ 225,00   │ │    30/40     │
│  Arrecadado  │ │   Débito     │ │  Em Dia      │
└──────────────┘ └──────────────┘ └──────────────┘
```

**Debt Alerts Widget:**
```
⚠️ Membros com Débito (10)

Maria Santos       R$ 45,00 (3 meses)
Pedro Costa        R$ 30,00 (2 meses)
Ana Lima           R$ 15,00 (1 mês)
...

💡 Lembrete: Hoje é dia 10 - pendências 
de Abril viraram débito!
```

**Monthly Chart Widget:**
```
📊 Últimos 6 Meses

Bar chart showing collection per month
(last 6 months with values)
```

#### 3. Membros (Members List)

```
┌────────────────────────────────────────────────┐
│ Lista de Membros (40)        [+ Novo Membro]  │
├────────────────────────────────────────────────┤
│ Nome          │ Data Início │ Débito │ Ações  │
├────────────────────────────────────────────────┤
│ João Silva    │ 01/01/2026  │ R$ 0   │ [Ver]  │
│ Maria Santos  │ 15/02/2026  │ R$ 45  │ [Ver]  │
│ Pedro Costa   │ 20/01/2026  │ R$ 0   │ [Ver]  │
└────────────────────────────────────────────────┘
```

**Add Member Dialog:**
```
┌──────────────────────────┐
│ Adicionar Membro         │
├──────────────────────────┤
│ Nome:                    │
│ [________________]       │
│                          │
│ Data de Início:          │
│ [__/__/____]             │
│                          │
│ [Cancelar] [Adicionar]   │
└──────────────────────────┘
```

#### 4. Pagamentos (Hybrid Payment Entry)

```
┌────────────────────────────────────────────────────┐
│ [Maio 2026 ▼]                                      │
├────────────────────────────────────────────────────┤
│ Membro        │ Status      │ Débito   │ Ação     │
├────────────────────────────────────────────────────┤
│ João Silva    │ ✓ R$ 15,00  │ R$ 0,00  │ [💰]     │
│ Maria Santos  │ ✗ Pendente  │ R$ 45,00 │ [💰] ◀   │
│ Pedro Costa   │ ✓ R$ 20,00  │ R$ 0,00  │ [💰]     │
└────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────┐
│ 💰 Registrar Pagamento - Maria Santos          │
├─────────────────────────────────────────────────┤
│ Valor (R$)  │ Mês/Ano  │                       │
│ [15,00]     │ [05/2026]│  [✓ Salvar]  [✗]     │
└─────────────────────────────────────────────────┘
```

**Workflow:**
1. User selects month from dropdown (top)
2. List shows all members with status for that month
3. Click 💰 button → opens inline payment dialog
4. User enters amount and selects month/year
5. Click ✓ Salvar → payment recorded
6. List updates to show new status

#### 5. Relatórios (Reports & Export)

```
┌─────────────────────────────────────────┐
│ 📊 Exportar Dados                       │
├─────────────────────────────────────────┤
│ Período:                                │
│ [01/01/2026] até [31/05/2026]          │
│                                         │
│ Tipo:                                   │
│ [Anonimizado ▼]                        │
│   - Anonimizado (nomes ocultos)        │
│   - Completo (com nomes)               │
│                                         │
│ Formato:                                │
│ [Excel (.xlsx) ▼]                      │
│   - Excel (.xlsx)                      │
│   - CSV (.csv)                         │
│                                         │
│ ┌───────────────────────────────────┐  │
│ │ 🔒 Confirmação de Senha           │  │
│ │ Digite sua senha novamente        │  │
│ └───────────────────────────────────┘  │
│                                         │
│ [📥 Exportar]                          │
└─────────────────────────────────────────┘
```

#### 6. Configurações (Settings)

```
┌─────────────────────────────────────┐
│ ⚙️ Geral                            │
├─────────────────────────────────────┤
│ Mensalidade Mínima (R$):           │
│ [15,00]                             │
│                                     │
│ [Salvar]                            │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│ 🔒 Segurança                        │
├─────────────────────────────────────┤
│ [Alterar Senha]                     │
└─────────────────────────────────────┘
```

## Export File Format

### Spreadsheet Structure

**Columns:**
- Column A: `Membro` (name or Membro #N)
- Columns B-N: One column per month in period (e.g., `Jan/2026`, `Fev/2026`)
- Last column: `Débito Total`

**Rows:**
- Header row (bold)
- One row per active member
- Separator row (empty)
- Totals row: `Arrecadado` per month
- Totals row: `Débito Total`

### Example: Complete Export

```
Membro       | Jan/2026 | Fev/2026 | Mar/2026 | Abr/2026 | Mai/2026 | Débito Total
─────────────────────────────────────────────────────────────────────────────────────
João Silva   | R$ 15,00 | R$ 15,00 | R$ 15,00 | R$ 15,00 | R$ 15,00 | R$ 0,00
Maria Santos | R$ 15,00 | R$ 15,00 | -        | -        | -        | R$ 45,00
Pedro Costa  | R$ 20,00 | R$ 15,00 | R$ 15,00 | R$ 15,00 | R$ 15,00 | R$ 0,00
Ana Lima     | -        | R$ 15,00 | R$ 15,00 | -        | -        | R$ 30,00
─────────────────────────────────────────────────────────────────────────────────────
Arrecadado   | R$ 50,00 | R$ 60,00 | R$ 45,00 | R$ 30,00 | R$ 30,00 | R$ 215,00
Débito Total |          |          |          |          |          | R$ 75,00
```

### Example: Anonymized Export

```
Membro    | Jan/2026 | Fev/2026 | Mar/2026 | Abr/2026 | Mai/2026 | Débito Total
──────────────────────────────────────────────────────────────────────────────────
Membro #1 | R$ 15,00 | R$ 15,00 | R$ 15,00 | R$ 15,00 | R$ 15,00 | R$ 0,00
Membro #2 | R$ 15,00 | R$ 15,00 | -        | -        | -        | R$ 45,00
Membro #3 | R$ 20,00 | R$ 15,00 | R$ 15,00 | R$ 15,00 | R$ 15,00 | R$ 0,00
Membro #4 | -        | R$ 15,00 | R$ 15,00 | -        | -        | R$ 30,00
──────────────────────────────────────────────────────────────────────────────────
Arrecadado| R$ 50,00 | R$ 60,00 | R$ 45,00 | R$ 30,00 | R$ 30,00 | R$ 215,00
Débito Total |       |          |          |          |          | R$ 75,00
```

### Cell Values

- **Payment exists:** `R$ X,XX` (Brazilian format)
- **No payment:** `-` (dash)
- **Debt:** Calculated as: months without payment * minimum_fee (if past 10th)

### File Naming Convention

**Format:**
```
relatorio_{type}_{start_date}_{end_date}.{ext}

Examples:
relatorio_completo_2026-01-01_2026-05-31.xlsx
relatorio_anonimo_2026-01-01_2026-05-31.csv
```

### XLSX Formatting

- Header row: **Bold**, background `#2d2d2d`, text white
- Currency cells: Number format `R$ #,##0.00`
- Totals row: **Bold**, top border
- Auto-fit column widths
- Freeze header row

### CSV Format

- UTF-8 encoding with BOM (Excel compatibility)
- Comma separator
- Values quoted if containing commas
- Brazilian decimal separator (`,` not `.`)

## Data Validation

### Input Validation Rules

**Member Name:**
- Required
- 2-100 characters
- Trim leading/trailing whitespace
- Allow duplicates (with warning message)

**Member Start Date:**
- Required
- Valid date in DD/MM/YYYY format
- Cannot be in the future
- Stored as YYYY-MM-DD in database

**Payment Amount:**
- Required
- Positive number > 0
- Accept both comma and period as decimal separator
- Display with Brazilian format (R$ X,XX)
- Max 2 decimal places
- Reasonable max: R$ 999.999,99

**Payment Month/Year:**
- Required
- Month: 1-12
- Year: 2000-2099 (reasonable range)
- Format: MM/YYYY for display

**Payment Date:**
- Optional (defaults to today)
- Valid date in DD/MM/YYYY format
- Cannot be in future

**Minimum Fee Setting:**
- Required
- Positive number > 0
- Format: R$ X,XX

**Password:**
- Minimum 8 characters
- Maximum 128 characters
- No special character requirements

### Validation Messages (Portuguese)

```
✗ Nome é obrigatório
✗ Nome deve ter entre 2 e 100 caracteres
✗ Data inválida. Use o formato DD/MM/YYYY
✗ Data não pode ser no futuro
✗ Valor é obrigatório
✗ Valor deve ser maior que zero
✗ Valor inválido. Use o formato: 15,00
✗ Mês inválido (use 01-12)
✗ Ano inválido
✗ Já existe pagamento para este mês
✗ Senha deve ter no mínimo 8 caracteres
✗ As senhas não coincidem

✓ Membro adicionado com sucesso
✓ Pagamento registrado
✓ Configuração salva
✓ Senha alterada com sucesso
✓ Relatório exportado: {filename}
```

### Error Handling

**Database Errors:**

```
Wrong password:
"Senha incorreta. Tente novamente."

Database corrupted:
"Erro ao abrir banco de dados. O arquivo pode estar corrompido."

Disk full:
"Espaço insuficiente em disco."

Foreign key violation:
"Erro: membro não encontrado."
```

**Network/File Errors:**

```
Export path not writable:
"Não foi possível salvar o arquivo. Verifique as permissões."

File already exists (prompt):
"Arquivo já existe. Deseja substituir?"
```

**Validation Display:**
- Inline validation (below form fields)
- Red text with ✗ icon
- Prevent form submission until valid
- Clear error when field is corrected

## Business Logic

### Debt Calculation Algorithm

**Definition:** A member has debt for a month if:
1. No payment exists for that month in the database, AND
2. Current date > 10th day of the following month, AND
3. Month is >= member start_date month

**Pseudocode:**
```python
def calculate_debt(member_id, current_date):
    debt = 0.0
    member = get_member(member_id)
    minimum_fee = get_setting('minimum_fee_brl')
    
    # Start from member's start month
    current_month = member.start_date.month
    current_year = member.start_date.year
    
    # Loop through months until current month
    while (current_year, current_month) < current_date.to_month():
        # Check if payment exists
        payment = get_payment(member_id, current_month, current_year)
        
        if payment is None:
            # Check if past the 10th of next month
            cutoff_date = Date(current_year, current_month, 1).add_months(1).add_days(10)
            
            if current_date > cutoff_date:
                debt += minimum_fee
        
        # Move to next month
        current_month += 1
        if current_month > 12:
            current_month = 1
            current_year += 1
    
    return debt
```

**Examples:**

```
Scenario 1:
- Member start: 2026-01-01
- Current date: 2026-05-15
- Payments: Jan (R$ 15), Feb (R$ 15), Mar (none), Apr (none), May (R$ 15)
- Debt calculation:
  - Jan: paid ✓
  - Feb: paid ✓
  - Mar: no payment, 05/15 > 04/10, add R$ 15
  - Apr: no payment, 05/15 > 05/10, add R$ 15
  - May: paid ✓
- Total debt: R$ 30,00

Scenario 2:
- Member start: 2026-04-01
- Current date: 2026-05-08
- Payments: Apr (none), May (none)
- Debt calculation:
  - Apr: no payment, but 05/08 < 05/10, no debt yet
  - May: not yet evaluated (current month)
- Total debt: R$ 0,00

Scenario 3 (after 10th):
- Same as Scenario 2 but current date: 2026-05-11
- Debt calculation:
  - Apr: no payment, 05/11 > 05/10, add R$ 15
  - May: not yet evaluated (current month)
- Total debt: R$ 15,00
```

### Payment Status Calculation

For a given member and month:

```python
def get_payment_status(member_id, month, year):
    payment = get_payment(member_id, month, year)
    
    if payment:
        return {
            'status': 'paid',
            'amount': payment.amount_brl,
            'date': payment.payment_date
        }
    else:
        current_date = today()
        cutoff_date = Date(year, month, 1).add_months(1).add_days(10)
        
        if current_date > cutoff_date:
            return {
                'status': 'debt',
                'amount': 0,
                'debt_amount': get_setting('minimum_fee_brl')
            }
        else:
            return {
                'status': 'pending',
                'amount': 0
            }
```

### Dashboard Statistics

**Monthly Summary (Current Month):**

```python
def calculate_monthly_summary(month, year):
    total_collected = sum(payments where month=month and year=year)
    
    total_debt = sum(calculate_debt(m.id) for m in active_members)
    
    members_up_to_date = count(members where calculate_debt(m.id) == 0)
    total_active_members = count(active_members)
    
    return {
        'collected': total_collected,
        'total_debt': total_debt,
        'up_to_date': f"{members_up_to_date}/{total_active_members}"
    }
```

**Debt Alerts List:**

```python
def get_debt_alerts():
    alerts = []
    for member in active_members:
        debt = calculate_debt(member.id)
        if debt > 0:
            months_in_debt = debt / get_setting('minimum_fee_brl')
            alerts.append({
                'member_name': member.name,
                'debt_amount': debt,
                'months_count': int(months_in_debt)
            })
    
    # Sort by debt amount descending
    return sorted(alerts, key=lambda x: x['debt_amount'], reverse=True)
```

## Testing Strategy

### Unit Tests (Rust)

**Tauri Commands:**
- `create_member(name, start_date)` → verify DB insert
- `get_members()` → verify query returns correct data
- `create_payment(member_id, month, year, amount)` → verify insert, handle duplicates
- `calculate_member_debt(member_id)` → test debt calculation logic
- `generate_export(start_date, end_date, anonymize, format)` → verify file contents
- `change_password(old_pass, new_pass)` → verify re-encryption

**Encryption:**
- PBKDF2 key derivation with known test vectors
- SQLCipher unlock/lock operations
- Password hash verification (bcrypt)

**Edge Cases:**
- Member with no payments (all debt)
- Member who started mid-month
- Payment on exact 10th day boundary
- Future payments (should not affect debt)
- Minimum fee changed mid-period

### Integration Tests

**Full Workflows:**

1. **New Database Setup:**
   - Create password
   - Verify database created and encrypted
   - Verify can reopen with password
   - Verify wrong password fails

2. **Member & Payment Flow:**
   - Add member
   - Register payment for current month
   - Verify payment appears in DB
   - Verify debt = 0
   - Advance simulated date past 10th of next month
   - Verify debt appears

3. **Export Flow:**
   - Create test data (5 members, various payment patterns)
   - Export CSV (complete)
   - Verify CSV content matches expected
   - Export XLSX (anonymized)
   - Verify XLSX can be opened in LibreOffice

4. **Password Change:**
   - Create database with password "old123"
   - Change password to "new456"
   - Close and reopen with "new456" → success
   - Try to open with "old123" → failure

### Manual Testing Checklist

**Initial Setup:**
- [ ] First launch: password creation screen
- [ ] Set password with < 8 chars → error
- [ ] Set valid password → database created
- [ ] Close and reopen → password prompt
- [ ] Enter wrong password → error message
- [ ] Enter correct password → app opens

**Member Management:**
- [ ] Add member with valid data → success
- [ ] Add member with invalid date → error
- [ ] Add member with empty name → error
- [ ] View member list → all members shown
- [ ] Member with debt shows red amount
- [ ] Member without debt shows green R$ 0,00

**Payment Entry:**
- [ ] Select current month from dropdown
- [ ] Click 💰 on pending member → dialog opens
- [ ] Enter amount and save → payment recorded
- [ ] Payment shows in list as green ✓
- [ ] Try to pay same month again → error
- [ ] Pay for future month → allowed
- [ ] Pay for past month → allowed, debt recalculated

**Dashboard:**
- [ ] Monthly summary shows correct totals
- [ ] Debt alerts list shows members with debt
- [ ] Chart shows last 6 months data
- [ ] All values in BRL format (R$ X,XX)

**Export:**
- [ ] Select date range
- [ ] Choose "Complete" + "XLSX"
- [ ] Enter wrong password → error
- [ ] Enter correct password → file saved
- [ ] Open file in LibreOffice → correct data
- [ ] Export "Anonymized" + "CSV"
- [ ] Open CSV in Excel → names are "Membro #N"

**Settings:**
- [ ] Change minimum fee from R$ 15 to R$ 20
- [ ] Verify new debt calculations use R$ 20
- [ ] Change password
- [ ] Close and reopen with new password → success

**Performance:**
- [ ] Add 50 members
- [ ] Register 500+ payments
- [ ] Dashboard loads quickly (< 1 second)
- [ ] Export completes quickly (< 5 seconds)

**Platform Testing:**
- [ ] Build Windows .exe
- [ ] Install and run on Windows 10/11
- [ ] Build Linux AppImage
- [ ] Run on Ubuntu 22.04+
- [ ] Copy clube.db from Windows to Linux → opens correctly
- [ ] Copy clube.db from Linux to Windows → opens correctly

### Test Data Generator

Create a test data script for manual testing:

```rust
// Generate test database with realistic data
fn generate_test_data() {
    // 20 members with varying start dates
    // 80% payment compliance (realistic)
    // Mix of members with 0, 1, 2, 3+ months debt
    // Some future payments
    // Date range: Jan 2026 - May 2026
}
```

## Implementation Phases

### Phase 1: Foundation (MVP)
**Goal:** Encrypted database + basic CRUD

- [ ] Tauri project setup
- [ ] SQLCipher integration
- [ ] Password creation/verification
- [ ] Database schema creation
- [ ] Member CRUD (add, list, view)
- [ ] Payment CRUD (add, list)
- [ ] Basic UI shell (dark theme, sidebar nav)

**Deliverable:** Can add members and payments, data is encrypted

### Phase 2: Core Features
**Goal:** Complete core functionality

- [ ] Debt calculation logic
- [ ] Dashboard with widgets
- [ ] Payment hybrid UI (list + inline dialog)
- [ ] Member detail view with payment history
- [ ] Settings (minimum fee)
- [ ] Validation on all forms
- [ ] Error handling

**Deliverable:** Fully functional app for single-user tracking

### Phase 3: Reporting
**Goal:** Export functionality

- [ ] CSV export
- [ ] XLSX export
- [ ] Anonymization logic
- [ ] Date range selection
- [ ] Password re-authentication for export
- [ ] File save dialog

**Deliverable:** Can export data to spreadsheets

### Phase 4: Polish
**Goal:** Production-ready

- [ ] Password change feature
- [ ] Chart on dashboard (6-month bar chart)
- [ ] Search/filter members
- [ ] Soft delete members (archive)
- [ ] About/help screen
- [ ] Installer creation (Windows .exe, Linux AppImage)
- [ ] Icon and branding
- [ ] User manual (Portuguese)

**Deliverable:** Distributable application

### Phase 5: Testing & Release
**Goal:** Validated, stable release

- [ ] Complete test suite (unit + integration)
- [ ] Manual testing checklist completion
- [ ] Cross-platform testing (Windows + Linux)
- [ ] Database transfer testing
- [ ] Security audit (basic)
- [ ] Performance optimization (if needed)
- [ ] Release v1.0

**Deliverable:** Version 1.0 ready for use

## Future Enhancements (Out of Scope for v1.0)

**Potential future features (not currently planned):**

- Multiple clubs support (separate databases)
- Member categories (types of membership)
- Payment methods tracking (cash, transfer, etc.)
- Recurring payment reminders/notifications
- Backup/restore within app
- Dark/light theme toggle
- Member notes/comments
- Payment receipts (PDF generation)
- Multi-currency support
- Attendance tracking
- Advanced reporting (charts, trends)
- Email integration for reminders

**These are explicitly NOT in v1.0 scope.**

## Success Criteria

**v1.0 is considered successful when:**

1. ✅ User can create encrypted database with password
2. ✅ User can add members and record payments
3. ✅ Debt is calculated correctly based on the 10th day rule
4. ✅ Dashboard shows accurate financial summary
5. ✅ User can export data to CSV and XLSX (both anonymized and complete)
6. ✅ Exported files open correctly in LibreOffice/Excel/Sheets
7. ✅ Database file can be transferred between Windows and Linux
8. ✅ Wrong password prevents access (security)
9. ✅ App works 100% offline
10. ✅ No crashes during normal operation
11. ✅ All UI text is in Brazilian Portuguese
12. ✅ Performance is acceptable with 100+ members

## Non-Goals

**Explicitly out of scope:**

- ❌ Web version or mobile apps
- ❌ Multi-user support or collaboration
- ❌ Cloud sync or online backup
- ❌ Automatic payment imports (bank integration)
- ❌ Email/SMS notifications
- ❌ Advanced access control (roles, permissions)
- ❌ Audit logging
- ❌ Multi-language support (Portuguese only)

## Glossary

- **Débito (Debt):** Amount owed by a member for months without payment after the 10th of the following month
- **Mensalidade (Monthly Fee):** The configured minimum amount members should pay each month
- **Membro (Member):** A person in the club who pays monthly fees
- **Pagamento (Payment):** A recorded payment by a member for a specific month
- **Anonimizado (Anonymized):** Export mode that replaces member names with "Membro #N"
- **SQLCipher:** Encrypted SQLite database library
- **Tauri:** Framework for building desktop apps with web technologies + Rust
- **PBKDF2:** Password-Based Key Derivation Function (for encryption key)

---

**End of Design Specification**
