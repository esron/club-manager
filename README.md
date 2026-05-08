# Gestor do Clube (Club Manager)

Desktop application for club membership management built with Tauri 2.x, React 18, TypeScript, and Tailwind CSS.

## Prerequisites

Before running the application, you need to install:

1. **Rust** - Visit https://www.rust-lang.org/learn/get-started#installing-rust
2. **Linux System Dependencies** (webkit2gtk & rsvg2) - Visit https://tauri.app/guides/prerequisites/#linux

## Phase 4 Features (Current)

**Password Management**
- Change password without losing database access
- Master key encryption for fast password changes
- Seamless migration from Phase 3 to Phase 4

**Member Search**
- Real-time search on Members tab
- Case-insensitive partial name matching
- Filters both active and inactive members
- Result count display

**Dashboard Visualizations**
- 6-month payment trends (bar chart)
- 6-month debt evolution (line chart)
- Interactive tooltips with formatted currency
- Responsive charts using Recharts library

**Help & Documentation**
- In-app help screen
- Quick start guide for common tasks
- Security information and warnings
- Version information display

## Previous Features

**Reports & Export (Phase 3)**
- Dedicated Reports screen with export functionality
- Debt Status report (current member debt summary)
- Payment History report (matrix-style payment grid)
- CSV and XLSX export formats
- Anonymization support
- Summary totals with XLSX formulas

**Dashboard & Member Management (Phase 2)**
- Overview of total outstanding debt
- Active member count
- Member detail view with payment history
- Debt calculation and visualization

**Payment System**
- Global payment modal
- Auto-fill from unpaid months
- Payment tracking by month and year

**Core Features (Phase 1)**
- Encrypted SQLCipher database
- Password-protected access
- Member CRUD operations
- Payment recording
- Portuguese interface
- Dark theme UI
- Configurable minimum membership fee

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
