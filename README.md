# Gestor do Clube (Club Manager)

Desktop application for club membership management built with Tauri 2.x, React 18, TypeScript, and Tailwind CSS.

## Prerequisites

Before running the application, you need to install:

1. **Rust** - Visit https://www.rust-lang.org/learn/get-started#installing-rust
2. **Linux System Dependencies** (webkit2gtk & rsvg2) - Visit https://tauri.app/guides/prerequisites/#linux

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

## Tech Stack

- **Frontend**: React 18 + TypeScript + Tailwind CSS 3.x
- **Desktop Framework**: Tauri 2.x
- **Build Tool**: Vite
- **Forms**: react-hook-form
- **Dates**: date-fns

## Development

```bash
# Install dependencies
npm install

# Run in development mode (requires Rust and system dependencies)
npm run tauri dev

# Build frontend only (useful for testing without Rust)
npm run build
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
