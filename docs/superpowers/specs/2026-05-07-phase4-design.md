# Phase 4: Polish Features - Design Specification

**Version:** 1.0  
**Date:** 2026-05-07  
**Status:** Approved

## Overview

Phase 4 adds production-ready polish features to make the app more usable and complete for daily operation. This phase focuses on core UX improvements: password management, member search, financial visualization, and user guidance.

**Scope:** Four independent features implemented sequentially, each delivering immediate value.

**Out of scope for this phase:** Packaging/installers, icon/branding, comprehensive user manual. These are deferred to later work.

## Goals

1. **Password Change** - Allow users to update their password without losing access to encrypted data
2. **Member Search** - Quick lookup of members by name on the Members tab
3. **Dashboard Charts** - Visual representation of payment trends and debt evolution over 6 months
4. **About/Help Screen** - Concise in-app guidance and version information

## Architecture

### Technology Stack

**Unchanged from Phase 3:**
- Frontend: React 18 + TypeScript + Tailwind CSS
- Backend: Rust + Tauri + SQLCipher
- Build: Tauri 2.x

**New dependencies:**
- Frontend: `recharts` (lightweight charting library for React)
- Backend: None (use existing infrastructure)

### Implementation Approach

**Sequential feature implementation:**
1. Password change (most complex, security-critical)
2. Member search (simplest, frontend-only)
3. Dashboard charts (moderate complexity, backend + frontend)
4. About/Help screen (simple, frontend-only)

Each feature is independent and can be tested/validated before moving to the next.

---

## Feature 1: Password Change with Master Key

### Problem

Currently, the encryption key is derived directly from the user's password using PBKDF2. Changing the password would require re-deriving a new encryption key and re-encrypting the entire database, which is slow and complex.

### Solution

Introduce a **master encryption key** stored in the config file, encrypted with the user's password. When the password changes, only the master key encryption changes - the database remains untouched.

### Architecture

**Master Key Approach:**
- Generate a random 32-byte master key on first setup
- Store master key in config.json, encrypted with user's password
- Database is always encrypted with the master key (not the password-derived key)
- Password changes only re-encrypt the master key

**Security properties:**
- Master key never stored in plaintext
- Database encryption key never changes (stable)
- Password verification still uses bcrypt hash
- Equivalent security to current approach (password protects master key, master key protects database)

### Config File Structure Changes

**Current config.json:**
```json
{
  "password_hash": "bcrypt hash",
  "salt": [16 random bytes],
  "minimum_fee_brl": "15.00",
  "created_at": "2026-01-01T00:00:00Z"
}
```

**New config.json:**
```json
{
  "password_hash": "bcrypt hash",
  "salt": [16 random bytes],
  "master_key_encrypted": [encrypted 32-byte master key],
  "minimum_fee_brl": "15.00",
  "created_at": "2026-01-01T00:00:00Z"
}
```

### Migration Strategy

**First launch after Phase 4 update:**

1. Detect old config format (missing `master_key_encrypted` field)
2. Show migration modal: "Atualização necessária - Digite sua senha para continuar"
3. User enters current password
4. Backend derives encryption key from password + salt (existing logic)
5. Open database with current key
6. Generate new random 32-byte master key
7. Re-encrypt database with master key
8. Encrypt master key using password-derived key
9. Add `master_key_encrypted` to config
10. Save updated config
11. Continue to app (user stays logged in)

**Error handling:** If password is wrong or database can't be opened, show error and retry. No data is lost - migration is atomic.

### Password Change Flow

**UI Location:** Settings screen, below the minimum fee setting

**Form fields:**
- Current password (password input)
- New password (password input, min 8 chars)
- Confirm new password (password input, must match new password)

**Validation:**
- New password length >= 8 characters
- New password != current password
- New password == confirm password
- Current password must be verified by backend

**Backend logic:**
1. Verify current password against bcrypt hash
2. Derive current encryption key from current password + salt
3. Decrypt master key using current encryption key
4. Derive new encryption key from new password + salt
5. Re-encrypt master key using new encryption key
6. Generate new bcrypt hash for new password
7. Update config.json with new `password_hash` and `master_key_encrypted`
8. Return success

**User experience:**
- User stays logged in after password change (no re-auth required)
- Success message: "Senha alterada com sucesso"
- Error messages:
  - "Senha atual incorreta"
  - "A nova senha deve ter no mínimo 8 caracteres"
  - "As senhas não coincidem"

### Security Considerations

- Master key encryption uses AES-256-GCM
- Master key is 256 bits (32 bytes) of cryptographically random data
- Master key never leaves memory in plaintext
- PBKDF2 still used for password-to-key derivation (existing implementation)
- Database encryption unchanged (SQLCipher with 256-bit key)

---

## Feature 2: Member Search

### Problem

With 100+ members, finding a specific member requires scrolling through paginated lists. No quick lookup capability.

### Solution

Add a search input box at the top of the Members tab that filters the member list in real-time as the user types.

### UI Design

**Location:** Top of Members tab, above the "Active Members" section

**Component:**
- Text input with placeholder: "Buscar membro por nome..."
- Clear button (X icon) appears when text is entered
- Full-width or constrained to ~400px max width

**Visual feedback:**
- Show result count: "X membros encontrados" when search is active
- Show "Nenhum membro encontrado" when search returns zero results
- Gray out pagination controls when search is active (all results shown on one page)

### Search Behavior

**Matching logic:**
- Case-insensitive partial match on member name
- Matches anywhere in the name (not just start)
- Example: "silva" matches "João Silva", "Maria da Silva", "Silvana Costa"

**Scope:**
- Searches both active and inactive members
- Updates both tables simultaneously
- No backend call needed (client-side filter on existing data)

**Interaction:**
- Real-time filtering (no debounce needed for <200 members)
- Pagination resets to page 1 when search term changes
- Empty search box shows all members (default state)
- Clear button (X) resets to empty state

**Performance:** Filter is applied to the already-loaded members array. For 100-200 members, instant client-side filtering is sufficient.

### Implementation Notes

**Frontend only:** No backend changes needed. Use `members.filter(m => m.name.toLowerCase().includes(searchTerm.toLowerCase()))` before pagination logic.

**State management:** Add `memberSearchTerm` state to MainLayout component. Apply filter before splitting active/inactive and before pagination.

**Optional enhancement:** Highlight matched text in search results (bold or different color). Can be added later if desired.

---

## Feature 3: Dashboard Charts

### Problem

Dashboard shows static summary cards but no visual representation of trends over time. Users can't see if the club's financial situation is improving or declining.

### Solution

Add two stacked charts below the summary cards showing 6-month trends for payments and debt.

### UI Design

**Layout:**
```
┌─────────────────────────────────────┐
│  Dashboard Header  [Refresh Button] │
├─────────────────┬───────────────────┤
│  Total Debt     │  Active Members   │
│  R$ X,XXX.XX    │  XXX              │
├─────────────────┴───────────────────┤
│  Pagamentos Mensais (últimos 6 meses)│
│  [Green Bar Chart]                  │
├─────────────────────────────────────┤
│  Evolução da Dívida (últimos 6 meses)│
│  [Red Line Chart]                   │
└─────────────────────────────────────┘
```

### Chart 1: Monthly Payments

**Type:** Vertical bar chart (green bars)

**Data:**
- X-axis: Last 6 months (e.g., "Nov/25", "Dez/25", "Jan/26", "Fev/26", "Mar/26", "Abr/26")
- Y-axis: Total payment amount in BRL (R$)
- Bars: Total sum of all payments made in each month (by payment_date field)

**Visual:**
- Title: "Pagamentos Mensais (últimos 6 meses)"
- Bar color: Green (#10b981 or similar)
- Grid lines: Horizontal only, light gray
- Tooltip on hover: "Jan/2026: R$ 1.234,56"
- Y-axis format: "R$ X.XXX"

**Empty state:** If no payments in a month, show R$ 0,00 (zero-height bar)

### Chart 2: Debt Trends

**Type:** Line chart (red line)

**Data:**
- X-axis: Last 6 months (same as Chart 1)
- Y-axis: Total club debt in BRL (R$)
- Points: Total debt calculated as of the last day of each month

**Visual:**
- Title: "Evolução da Dívida (últimos 6 meses)"
- Line color: Red (#ef4444 or similar)
- Line width: 2-3px
- Point markers: Small circles at each data point
- Grid lines: Horizontal only, light gray
- Tooltip on hover: "Jan/2026: R$ 456,78"
- Y-axis format: "R$ X.XXX"

### Backend Data Query

**New command:** `get_dashboard_chart_data_cmd(password: String) -> ChartData`

**Return structure:**
```rust
struct ChartData {
    months: Vec<MonthData>,  // 6 elements, newest first
}

struct MonthData {
    month_key: String,       // "2026-01"
    month_display: String,   // "Jan/26"
    total_payments: f64,     // Sum of payments in this month
    total_debt: f64,         // Total club debt as of end of month
}
```

**Calculation logic:**

For each of the last 6 months:

1. **Total payments:** 
   ```sql
   SELECT SUM(amount_brl) FROM payments 
   WHERE strftime('%Y-%m', payment_date) = '2026-01'
   ```

2. **Total debt as of month end:**
   - Use existing `calculate_member_debt` function
   - Pass last day of month as "today" parameter
   - Sum debt across all active members

**Month range:** Current month + 5 previous months (e.g., if today is May 7, 2026, show: Dec 25, Jan 26, Feb 26, Mar 26, Apr 26, May 26)

### Frontend Implementation

**Library:** Recharts (https://recharts.org/)
- Lightweight (60kb gzipped)
- Good React/TypeScript integration
- Responsive by default
- Easy customization

**Component structure:**
- Create `DashboardCharts.tsx` component
- Import in `DashboardScreen.tsx`
- Load chart data on component mount
- Refresh button reloads both cards and charts

**Responsive behavior:**
- Desktop: Charts at full width
- Tablet/small screen: Charts shrink proportionally
- Min height: 200-250px per chart

**Loading state:** Show skeleton or spinner while loading chart data

**Error handling:** If chart data fails to load, show error message but keep summary cards visible

---

## Feature 4: About/Help Screen

### Problem

Users have no in-app reference for how to use the application or where to get help. No version information visible.

### Solution

Add a new "Ajuda" (Help) tab with concise app information and quick start guide.

### UI Design

**Location:** New tab in sidebar navigation
- Order: Dashboard, Membros, Pagamentos, Relatórios, **Ajuda**, Configurações
- Icon: Question mark or info icon (optional)
- Label: "Ajuda"

**Layout:** Single-column centered layout, max-width 800px, dark theme

### Content Structure

**Section 1: About**
```
Gestor do Clube
Versão 1.0.0

Aplicativo para gestão de mensalidades do clube.
Desenvolvido com Tauri + React.
```

**Section 2: Guia Rápido**

Short bullet points (max 2-3 lines each):

- **Adicionar membro:** Clique em "Membros" → "Adicionar Membro". Informe o nome e a data de início da participação no clube.

- **Registrar pagamento:** Use o botão "Adicionar Pagamento" no topo da tela. Selecione o membro, o mês de referência, e a data do pagamento.

- **Visualizar dívidas:** O Dashboard mostra a dívida total do clube. Para ver dívidas por membro, acesse a aba "Membros" e clique no nome do membro.

- **Cálculo de dívidas:** Um mês sem pagamento se torna dívida após o dia 10 do mês seguinte. Exemplo: sem pagamento em março → dívida após 10 de abril.

- **Exportar relatórios:** Acesse a aba "Relatórios". Escolha o tipo de relatório (dívidas ou histórico de pagamentos), configure as opções, e clique em "Exportar".

**Section 3: Segurança**

```
Segurança e Senha

Este aplicativo protege seus dados com criptografia. 
Sua senha é necessária para acessar o banco de dados.

⚠️ Importante: Não há recuperação de senha. 
Guarde sua senha em local seguro.
```

### Styling

- Section headers: Bold, larger font (text-lg or text-xl)
- Bullet points: Standard list styling with disc markers
- Paragraphs: Normal spacing, readable line height
- Warning (⚠️): Yellow text or yellow border box
- Background: Dark surface color (consistent with app theme)

### Version Number

**Source:** Read from `package.json` version field (or Cargo.toml)

**Display:** Show as "Versão X.Y.Z" in About section

**Implementation:** 
- Option A: Hardcode version string (update manually)
- Option B: Import from package.json at build time (preferred)

### Word Count Target

**Total content: ~300-400 words** (very concise, scannable)

**No backend needed** - all static content in frontend component

---

## Implementation Order

### 1. Password Change
**Priority:** Highest (security feature, most complex)
- Backend migration logic
- Config structure changes
- Settings UI update
- Testing with various scenarios

### 2. Member Search
**Priority:** Medium (simple, high user value)
- Frontend state management
- Search input component
- Filter logic
- Result display

### 3. Dashboard Charts
**Priority:** Medium (moderate complexity)
- Backend chart data query
- Install Recharts dependency
- Chart components
- Integration with Dashboard

### 4. About/Help Screen
**Priority:** Lowest (simple, informational)
- New tab in navigation
- Static content component
- Version number display

---

## Testing Strategy

### Password Change Testing
- Migration from old config to new config
- Change password with correct current password
- Reject change with wrong current password
- Verify database still opens after password change
- Test password validation (length, match)

### Member Search Testing
- Search with various terms (partial, full, case variations)
- Search with no results
- Clear search and verify full list returns
- Search with special characters in names

### Dashboard Charts Testing
- Verify calculations match manual tallies
- Test with sparse data (few payments/months)
- Test with zero data (empty database)
- Responsive behavior on different screen sizes

### About/Help Testing
- Verify version number displays correctly
- Check all links (if any)
- Verify text is readable and properly formatted

---

## Success Criteria

**Phase 4 is complete when:**

1. ✅ Users can change their password without losing database access
2. ✅ Config migration from old format to master key format works seamlessly
3. ✅ Member search filters list in real-time and shows result count
4. ✅ Dashboard shows 6-month bar chart for payments and line chart for debt
5. ✅ Chart data calculations are accurate
6. ✅ About/Help screen displays version and concise usage guide
7. ✅ All features work on both Windows and Linux
8. ✅ No regressions in existing functionality

---

## Future Enhancements (Post-Phase 4)

Features deferred from original Phase 4 plan:
- Keyboard shortcuts
- Advanced search filters (by debt status, date range)
- Chart export functionality
- Application installers (.exe, AppImage)
- Custom icon and branding
- Comprehensive user manual (PDF)

These can be addressed in Phase 5 or later based on user feedback.
