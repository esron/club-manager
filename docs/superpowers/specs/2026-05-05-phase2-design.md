# Gestor do Clube - Phase 2 Design Specification

**Version:** 1.0  
**Date:** 2026-05-05  
**Status:** Draft

## Overview

Phase 2 builds on the Phase 1 MVP foundation to complete core functionality. This phase adds debt calculation integration, dashboard overview, individual member detail views, settings configuration, and improved payment workflow with a global modal.

## Phase 2 Scope

### Features Included

1. **Settings Screen** - Configure minimum monthly fee
2. **Debt Calculation Integration** - Wire backend logic to frontend
3. **Dashboard** - Minimal overview with total debt and member count
4. **Member Detail View** - Individual member page with payment history and debt breakdown
5. **Global Payment Modal** - Add payments from anywhere in the app
6. **Enhanced Validation** - Improved form validation and error handling

### Out of Scope (Future Phases)

- CSV/XLSX export (Phase 3)
- Password change (Phase 4)
- Charts/graphs (Phase 4)
- Search/filter (Phase 4)

## Implementation Approach

**Build Order (Incremental):**

1. Settings screen (simplest, standalone)
2. Debt calculation commands (backend integration)
3. Dashboard with minimal widgets
4. Member detail view
5. Global payment modal
6. Enhanced error handling and validation

**Why this order:** Each piece builds on the previous. Settings is simple and lets us test debt calculations with different fees. Dashboard uses debt calculations. Member detail view uses both. Payment modal is last since it's used in multiple contexts.

## Architecture

### Technology Stack

No changes from Phase 1:
- **Frontend:** React 18 + TypeScript, Tailwind CSS
- **Backend:** Tauri (Rust), SQLCipher database
- **State:** React Context API

### New Components

```
src/components/
  ├── SettingsScreen.tsx       (configuration UI)
  ├── DashboardScreen.tsx      (overview widgets)
  ├── MemberDetailView.tsx     (individual member page)
  └── AddPaymentModal.tsx      (global payment dialog)
```

### New Backend Commands

```rust
// src-tauri/src/commands/settings.rs (new file)
get_setting_cmd(password: String, key: String) -> Result<String, String>
update_setting_cmd(password: String, key: String, value: String) -> Result<(), String>

// src-tauri/src/commands/debt.rs (new file)
get_member_debt_cmd(password: String, member_id: i64) -> Result<MemberDebtInfo, String>
get_all_debts_cmd(password: String) -> Result<Vec<MemberDebtInfo>, String>
```

### Data Structures

```typescript
interface MemberDebtInfo {
  member_id: number;
  member_name: string;
  total_debt: number;  // BRL amount
  unpaid_months: UnpaidMonth[];
}

interface UnpaidMonth {
  month: number;        // 1-12
  year: number;
  amount: number;       // minimum fee for that period
  display: string;      // "Janeiro 2026" (Portuguese)
}

interface PaymentPrefill {
  memberId?: number;
  memberName?: string;
  month?: number;
  year?: number;
}
```

## Feature Specifications

### 1. Settings Screen

#### Purpose
Allow users to configure application settings, starting with minimum monthly fee.

#### UI Layout
- Full-screen component (replaces MainLayout content when Settings tab is active)
- Simple form with labeled input fields
- Save button at bottom
- Success/error messages inline

#### Fields
1. **Minimum Monthly Fee (R$)**
   - Text input
   - Label: "Mensalidade Mínima (R$)"
   - Placeholder: "15.00"
   - Format: decimal with 2 places (XX.XX)
   - Current value loaded from database on mount

2. **Future Fields** (Phase 4)
   - Password change section (placeholder for now)

#### Backend Integration

**Get Setting:**
```rust
#[tauri::command]
pub fn get_setting_cmd(password: String, key: String) -> Result<String, String>
```
- Opens encrypted database with password
- Calls `get_setting(conn, key)` from models/settings.rs
- Returns setting value as string

**Update Setting:**
```rust
#[tauri::command]
pub fn update_setting_cmd(password: String, key: String, value: String) -> Result<(), String>
```
- Validates value format based on key
- For "minimum_fee_brl": must be valid decimal > 0, max 9999.99
- Opens encrypted database with password
- Calls `update_setting(conn, key, value)` from models/settings.rs
- Returns error if validation fails

#### Validation Rules
- **Minimum fee:**
  - Must be numeric
  - Must be > 0
  - Maximum: 9999.99
  - Format: up to 2 decimal places
  - Example valid: "15.00", "20", "100.50"
  - Example invalid: "abc", "-10", "10000", "15.999"

#### Error Handling
- Show validation errors inline below field (red text)
- Database errors: "Erro ao salvar configurações"
- Success message: "Configurações salvas com sucesso" (green text, 3 seconds)
- Password errors: trigger re-authentication flow (same as other commands)

#### AppContext Integration
```typescript
// Add to AppContext:
interface AppContextType {
  // ... existing ...
  minimumFee: string;
  refreshSettings: () => Promise<void>;
  updateSetting: (key: string, value: string) => Promise<void>;
}
```
- Load settings on app startup (after authentication)
- Store `minimumFee` in context for use in calculations
- Refresh settings after update

### 2. Debt Calculation Commands

#### Purpose
Expose existing debt calculation logic from `src-tauri/src/models/debt.rs` to frontend.

#### Backend Commands

**Get Member Debt:**
```rust
#[tauri::command]
pub fn get_member_debt_cmd(password: String, member_id: i64) -> Result<MemberDebtInfo, String>
```

**Implementation:**
1. Open encrypted database with password
2. Call `calculate_member_debt(conn, member_id, today)`
   - Uses current date (Utc::now().format("%Y-%m-%d"))
3. Get member details (name)
4. Build list of unpaid months by iterating through member's timeline
5. Return `MemberDebtInfo` with:
   - member_id, member_name
   - total_debt (from calculation)
   - unpaid_months array

**Get All Debts:**
```rust
#[tauri::command]
pub fn get_all_debts_cmd(password: String) -> Result<Vec<MemberDebtInfo>, String>
```

**Implementation:**
1. Get all active members
2. For each member, call `calculate_member_debt()`
3. Return array of `MemberDebtInfo`
4. Filter out members with 0 debt? No - return all so Dashboard can show total

#### Frontend Integration

**AppContext additions:**
```typescript
getMemberDebt: (memberId: number) => Promise<MemberDebtInfo>;
getAllDebts: () => Promise<MemberDebtInfo[]>;
```

**Month Name Translation:**
```typescript
const MONTH_NAMES_PT = [
  'Janeiro', 'Fevereiro', 'Março', 'Abril', 'Maio', 'Junho',
  'Julho', 'Agosto', 'Setembro', 'Outubro', 'Novembro', 'Dezembro'
];

// Format unpaid month display
const formatMonth = (month: number, year: number) => 
  `${MONTH_NAMES_PT[month - 1]} ${year}`;
```

#### Performance
- Calculation for 100 members: ~100ms
- No caching - always calculate fresh (ensures accuracy)
- Can optimize in future if needed

### 3. Dashboard Screen

#### Purpose
Provide quick overview of club status with minimal essential metrics.

#### UI Layout
- Full-screen component
- First tab in navigation (before Members)
- Two large summary cards side-by-side (stack on mobile)
- Manual refresh button: "Atualizar"

#### Widgets

**1. Total Club Debt Card**
```
┌─────────────────────────┐
│ Dívida Total do Clube   │
│                         │
│      R$ XXX.XX          │  <- Large, prominent
│                         │
└─────────────────────────┘
```
- Label: "Dívida Total do Clube"
- Value: Sum of all member debts from `get_all_debts_cmd()`
- Color: Red accent if debt > 0, gray if 0
- Font size: 2xl or 3xl for amount

**2. Active Members Card**
```
┌─────────────────────────┐
│ Membros Ativos          │
│                         │
│         XX              │  <- Large count
│                         │
└─────────────────────────┘
```
- Label: "Membros Ativos"
- Value: Count of active members (from existing members array)
- Color: Green accent
- Font size: 2xl or 3xl for count

#### Behavior
- Load data on mount (useEffect)
- Show loading state while calculating: "Calculando..."
- Show error message if calculation fails
- Refresh button triggers re-load

#### Implementation Notes
- Reuse Tailwind dark theme classes
- Card component: `bg-dark-surface border border-dark-border rounded-lg p-6`
- Responsive: `grid grid-cols-1 md:grid-cols-2 gap-4`

### 4. Member Detail View

#### Purpose
Show complete information for a single member: debt breakdown, unpaid months, and payment history.

#### Navigation
- Click member name in Members table → navigate to detail page
- URL routing: `/member/:id` or similar pattern
- Back button: "← Voltar para Membros" (returns to Members tab)

#### Layout Sections

**1. Header**
```
┌────────────────────────────────────────┐
│ João Silva                              │  <- Large, bold
│ Membro desde 01/01/2026                 │
│ [Editar Nome] [Desativar]               │  <- Same as table
└────────────────────────────────────────┘
```
- Member name (text-2xl font-bold)
- Start date label
- Edit/Deactivate buttons (reuse existing functions)

**2. Debt Summary Card**
```
┌────────────────────────────────────────┐
│ Dívida Atual                            │
│                                         │
│ R$ XX.XX                                │  <- Color coded
│ Meses em atraso: X                      │
└────────────────────────────────────────┘
```
- Total debt (green if 0, red if > 0)
- Count of unpaid months
- Loaded from `get_member_debt_cmd()`

**3. Unpaid Months Section** (conditional - only if debt > 0)
```
┌────────────────────────────────────────┐
│ Meses Não Pagos                         │
│                                         │
│ Mês/Ano        Valor      Ação          │
│ Janeiro 2026   R$ 15.00   [+ Pagar]     │
│ Fevereiro 2026 R$ 15.00   [+ Pagar]     │
└────────────────────────────────────────┘
```
- Table of unpaid months from `MemberDebtInfo.unpaid_months`
- "Adicionar Pagamento" button per row
- Clicking button opens global payment modal pre-filled with member/month/year

**4. Payment History Section**
```
┌────────────────────────────────────────┐
│ Histórico de Pagamentos                 │
│                                         │
│ Data Pago   Ref. Mês/Ano   Valor   Ação │
│ 15/01/2026  Dez 2025       R$15   [X]   │
│ 10/02/2026  Jan 2026       R$15   [X]   │
│                                         │
│ [1] 2 3 ... [15 ▼]                      │  <- Pagination
└────────────────────────────────────────┘
```
- All payments for this member (filtered from all payments)
- Columns: Payment Date, Month/Year Paid For, Amount, Delete
- Pagination (15/30/100 per page selector)
- Newest payments first (sort by payment_date DESC)

#### State Management
- Component receives member ID via props/route params
- Load on mount:
  - Member details (from members context)
  - Member debt (call `getMemberDebt(id)`)
  - Payments (filter from payments context)
- Refresh after payment added/deleted
- Loading states for debt calculation

#### Error Handling
- Member not found: redirect to Members tab
- Debt calculation error: show message, allow viewing payment history
- Database errors: show error message with retry button

### 5. Global Payment Modal

#### Purpose
Provide consistent payment entry from anywhere in the app, with optional pre-filling for contextual additions.

#### Trigger Locations

1. **Toolbar Button** (always visible)
   - "Adicionar Pagamento" button in top toolbar/sidebar
   - Opens modal with empty form

2. **Member Detail View**
   - "Adicionar Pagamento" button in each unpaid month row
   - Opens modal pre-filled with member, month, year

3. **Payments Tab** (optional refactor)
   - Option A: Keep existing inline form
   - Option B: Replace with button that opens modal (consistency)
   - Recommendation: Keep both for flexibility

#### Modal Design

**Structure:**
```
┌─ Backdrop (semi-transparent) ───────────┐
│                                          │
│  ┌─ Modal Card ───────────────────┐  X  │
│  │ Adicionar Pagamento             │     │
│  │                                 │     │
│  │ Membro: [Dropdown ▼]           │     │
│  │ Mês: [Janeiro ▼]               │     │
│  │ Ano: [2026]                    │     │
│  │ Valor (R$): [15.00]            │     │
│  │ Data do Pagamento: [DD/MM/AAAA]│     │
│  │                                 │     │
│  │         [Cancelar] [Salvar]    │     │
│  └────────────────────────────────┘     │
│                                          │
└──────────────────────────────────────────┘
```

**Components:**
- Dark backdrop (bg-black/50)
- Centered white card (max-w-md)
- Close X button (top-right)
- Click outside or ESC key to close
- Same form fields as current payment form

#### Form Fields

1. **Membro** - Searchable dropdown with datalist (reuse existing implementation)
2. **Mês** - Dropdown 1-12 (labels in Portuguese: Janeiro, Fevereiro, etc.)
3. **Ano** - Number input (default: current year)
4. **Valor (R$)** - Text input (default: minimum fee from settings)
5. **Data do Pagamento** - DateInput component (default: today)

#### Pre-filling Behavior

**From unpaid month row:**
```typescript
openPaymentModal({
  memberId: 123,
  memberName: "João Silva",
  month: 1,
  year: 2026
})
```
- Member, month, year are pre-filled and locked/disabled
- Amount = minimum fee
- Payment date = today
- User can only change amount and date

**From toolbar:**
```typescript
openPaymentModal()
```
- All fields empty/default
- Member dropdown requires selection
- Month defaults to current month
- Year defaults to current year
- Amount = minimum fee
- Payment date = today

#### Submission

**Validation:**
- Same rules as current payment form
- Prevent duplicate payments (same member + month + year)
- Amount must be > 0
- Date must be valid

**Success Flow:**
1. Call `addPayment()` from AppContext
2. Close modal
3. Clear form fields
4. Refresh relevant data:
   - If on Member Detail: refresh member debt
   - If on Dashboard: refresh total debt
   - If on Payments tab: refresh payment list

**Error Flow:**
1. Show error message inline in modal (red text below field)
2. Don't close modal
3. Allow user to fix and retry

#### AppContext Integration

```typescript
interface AppContextType {
  // ... existing ...
  paymentModalOpen: boolean;
  paymentModalPrefill?: PaymentPrefill;
  openPaymentModal: (prefill?: PaymentPrefill) => void;
  closePaymentModal: () => void;
}
```

**Implementation:**
```typescript
// In AppContext
const [paymentModalOpen, setPaymentModalOpen] = useState(false);
const [paymentModalPrefill, setPaymentModalPrefill] = useState<PaymentPrefill>();

const openPaymentModal = (prefill?: PaymentPrefill) => {
  setPaymentModalPrefill(prefill);
  setPaymentModalOpen(true);
};

const closePaymentModal = () => {
  setPaymentModalOpen(false);
  setPaymentModalPrefill(undefined);
};

// In App.tsx or MainLayout.tsx
{paymentModalOpen && <AddPaymentModal />}
```

### 6. Enhanced Validation & Error Handling

#### Form Validation Improvements

**Settings Screen:**
- Real-time validation on minimum fee input
- Show error immediately on blur if invalid
- Disable save button if validation fails
- Clear error on focus

**Payment Modal:**
- Validate all fields before submission
- Show field-specific errors (which field is wrong)
- Better error messages in Portuguese:
  - "Campo obrigatório" (required)
  - "Valor inválido" (invalid format)
  - "Data inválida" (invalid date)
  - "Pagamento já existe para este mês" (duplicate)

#### Error Message Consistency

**Format:**
- All error messages in Brazilian Portuguese
- Consistent styling: red text (text-dark-error class)
- Position: below the relevant field or action
- Auto-dismiss success messages after 3 seconds

**Common Errors:**
- Database errors: "Erro ao acessar o banco de dados"
- Network/Tauri errors: "Erro de comunicação com o sistema"
- Validation errors: specific field-level messages
- Authentication errors: trigger re-login flow

## Navigation Structure

### Tab Order
1. **Dashboard** (new) - overview
2. **Membros** (existing) - member list
3. **Pagamentos** (existing) - payment list
4. **Configurações** (new) - settings

### Routing (if implementing)
- `/dashboard` - Dashboard screen
- `/members` - Members list
- `/member/:id` - Member detail view
- `/payments` - Payments list
- `/settings` - Settings screen

**Note:** Routing implementation is optional. Can use tabs with conditional rendering if routing adds complexity.

## Data Flow

### Application Startup
1. User authenticates (existing)
2. Load settings (`get_setting_cmd("minimum_fee_brl")`)
3. Store in AppContext
4. Load members (existing)
5. Load payments (existing)
6. Navigate to Dashboard

### Dashboard Flow
```
User opens Dashboard tab
  → Call get_all_debts_cmd()
  → Calculate total debt
  → Display widgets
```

### Member Detail Flow
```
User clicks member name
  → Navigate to Member Detail (pass member ID)
  → Load member debt (get_member_debt_cmd)
  → Filter payments for this member
  → Display sections
```

### Payment Addition Flow
```
User clicks "Adicionar Pagamento" (toolbar or unpaid month)
  → Open modal (with optional prefill)
  → User fills/confirms form
  → Submit via addPayment()
  → Refresh relevant views
  → Close modal
```

### Settings Update Flow
```
User opens Settings tab
  → Load current settings (get_setting_cmd)
  → User changes minimum fee
  → Validate input
  → Save (update_setting_cmd)
  → Refresh settings in AppContext
  → Show success message
```

## Testing Considerations

### Manual Testing Checklist

**Settings:**
- [ ] Load settings shows current value
- [ ] Save valid fee updates database
- [ ] Invalid fee shows error, doesn't save
- [ ] Very large/small values handled
- [ ] Success message appears and disappears

**Debt Calculation:**
- [ ] Member with no payments shows correct debt
- [ ] Member with partial payments shows correct debt
- [ ] Member with all payments shows R$ 0.00
- [ ] Debt calculation respects grace period (10th of next month)
- [ ] Debt calculation respects member start date

**Dashboard:**
- [ ] Total debt sums all active members
- [ ] Active member count is correct
- [ ] Refresh button updates data
- [ ] Loading states appear
- [ ] Error handling works

**Member Detail:**
- [ ] Clicking name navigates correctly
- [ ] Back button returns to Members
- [ ] Debt summary is accurate
- [ ] Unpaid months list is correct
- [ ] Payment history shows all payments for member
- [ ] Pagination works
- [ ] Delete payment refreshes view

**Payment Modal:**
- [ ] Opens from toolbar (empty)
- [ ] Opens from unpaid month (pre-filled)
- [ ] Pre-filled fields are locked
- [ ] Submission works
- [ ] Duplicate validation works
- [ ] Close button works
- [ ] Click outside closes
- [ ] ESC key closes
- [ ] Success refreshes all relevant views

### Edge Cases

1. **Member with start date in future** - should have 0 debt
2. **Member starting mid-month** - debt calculation starts from their month
3. **Payment date vs. month paid for** - ensure correct unpaid month detection
4. **Minimum fee change** - new calculations use new fee, old debts don't recalculate
5. **Deactivated member** - doesn't appear in debt calculations
6. **Very large debt** - UI handles large numbers gracefully
7. **Modal open when navigating** - should close automatically

## Implementation Notes

### Code Organization

**New files:**
```
src/components/
  ├── SettingsScreen.tsx
  ├── DashboardScreen.tsx
  ├── MemberDetailView.tsx
  └── AddPaymentModal.tsx

src-tauri/src/commands/
  ├── settings.rs    (new - settings commands)
  └── debt.rs        (new - debt commands)
```

**Modified files:**
```
src/contexts/AppContext.tsx    (add settings, debt, modal state)
src/components/MainLayout.tsx  (add Dashboard/Settings tabs, member name click handler)
src-tauri/src/lib.rs          (register new commands)
src-tauri/src/commands/mod.rs (add new command modules)
```

### Styling Consistency

**Reuse existing Tailwind classes:**
- Cards: `bg-dark-surface border border-dark-border rounded-lg`
- Buttons: `bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90`
- Inputs: existing input styles from MainLayout
- Errors: `text-dark-error text-sm`
- Success: `text-green-500 text-sm`

**Dark theme colors:**
- Background: `bg-dark-bg`
- Surface: `bg-dark-surface`
- Border: `border-dark-border`
- Text primary: `text-dark-text-primary`
- Text secondary: `text-dark-text-secondary`
- Accent: `bg-dark-accent`
- Error: `text-dark-error`

### Performance Optimization

**Current approach:** Calculate debts on-demand, no caching
- Simple to implement
- Always accurate
- Fast enough for 100 members

**Future optimization (if needed):**
- Cache debt calculations with expiry
- Invalidate cache on payment add/delete or settings change
- Background calculation on app startup

### Internationalization Notes

**Portuguese month names:**
```typescript
const MONTH_NAMES_PT = [
  'Janeiro', 'Fevereiro', 'Março', 'Abril', 
  'Maio', 'Junho', 'Julho', 'Agosto',
  'Setembro', 'Outubro', 'Novembro', 'Dezembro'
];
```

**All UI text in Portuguese:**
- Dashboard: "Dívida Total do Clube", "Membros Ativos"
- Member Detail: "Membro desde", "Dívida Atual", "Meses em atraso", "Meses Não Pagos", "Histórico de Pagamentos"
- Payment Modal: "Adicionar Pagamento", "Membro", "Mês", "Ano", "Valor", "Data do Pagamento"
- Settings: "Configurações", "Mensalidade Mínima"
- Buttons: "Salvar", "Cancelar", "Voltar para Membros", "Atualizar"

## Success Criteria

Phase 2 is complete when:

1. ✅ Settings screen allows changing minimum fee
2. ✅ Dashboard shows total club debt and active member count
3. ✅ Member detail view shows individual debt breakdown and payment history
4. ✅ Clicking member name navigates to detail view
5. ✅ Global payment modal works from toolbar and unpaid month rows
6. ✅ All debt calculations are accurate and respect grace periods
7. ✅ All text is in Brazilian Portuguese
8. ✅ Error handling is consistent and helpful
9. ✅ Dark theme styling is consistent across new components
10. ✅ Manual testing checklist passes

## Future Enhancements (Phase 3 & 4)

**Phase 3:**
- CSV/XLSX export with anonymization
- Date range selection for exports
- Password re-authentication for exports

**Phase 4:**
- Password change functionality
- 6-month bar chart on Dashboard
- Search/filter members
- Installer creation

## Appendix: Debt Calculation Logic

**Reusing existing implementation from `src-tauri/src/models/debt.rs`:**

```rust
pub fn calculate_member_debt(
    conn: &Connection,
    member_id: i64,
    as_of_date: &str,
) -> Result<f64>
```

**Business rules:**
1. Start from member's start_date
2. For each month until as_of_date:
   - Check if payment exists for (member_id, month, year)
   - If no payment AND past grace period (10th of next month):
     - Add minimum_fee to debt
3. Return total debt

**Grace period calculation:**
- Month M unpaid → debt accrues after (M+1)/10
- Example: January 2026 unpaid → debt after February 10, 2026

**No changes needed to existing logic** - just expose via commands.
