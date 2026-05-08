# Changelog

All notable changes to Gestor do Clube will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-05-08

### Added
- Encrypted database with SQLCipher (AES-256)
- Password-protected access with master key encryption
- Member management (add, view, search, deactivate)
- Payment tracking by month and year
- Automatic debt calculation (10th day rule)
- Dashboard with summary cards and charts
- 6-month payment trends chart (bar chart)
- 6-month debt evolution chart (line chart)
- Member detail view with full payment history
- Global payment modal with auto-fill
- Reports screen with preview functionality
- CSV export (compatible with Excel, LibreOffice, Google Sheets)
- XLSX export with formatting
- Anonymized export mode (Membro #1, #2, etc.)
- Configurable minimum monthly fee
- Password change feature
- Member search (real-time filtering)
- Help/About screen with user guide
- Brazilian Portuguese interface
- Dark theme UI
- Windows installer (.exe)
- Linux AppImage
- Comprehensive user manual in Portuguese

### Security
- AES-256 encryption for all data at rest
- PBKDF2-SHA256 key derivation (100,000 iterations)
- Bcrypt password hashing
- Re-authentication required for exports
- Master key encryption for fast password changes
- No password recovery (by design)

### Technical
- Built with Tauri 2.x + React 18 + TypeScript
- Offline-first, no internet required
- Cross-platform: Windows 10+ and Linux
- Database portable between platforms
- Production build without development tools

## [Unreleased]

### Planned
- Multi-currency support
- Recurring payment reminders
- Advanced reporting (custom date ranges)
- Attendance tracking
- Member notes/comments

---

[1.0.0]: https://github.com/essilva/club-manager/releases/tag/v1.0.0
