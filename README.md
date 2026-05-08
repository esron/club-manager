# Gestor do Clube

Aplicativo desktop para gestão de mensalidades de clubes com criptografia de dados.

## 📥 Download

**Versão Atual: 1.0.0**

### Windows
- [Gestor do Clube 1.0.0 Setup.exe](releases/download/v1.0.0/Gestor-do-Clube_1.0.0_x64-setup.exe) (Instalador)
- Requisitos: Windows 10 ou superior (64-bit)

### Linux
- [Gestor do Clube 1.0.0 AppImage](releases/download/v1.0.0/gestor-do-clube_1.0.0_amd64.AppImage) (Portável)
- [Gestor do Clube 1.0.0 DEB](releases/download/v1.0.0/gestor-do-clube_1.0.0_amd64.deb) (Debian/Ubuntu)
- Requisitos: Ubuntu 20.04+, Fedora 35+, ou equivalente

## 🚀 Instalação Rápida

### Windows
1. Baixe o instalador `.exe`
2. Execute e siga as instruções
3. Abra pelo Menu Iniciar

### Linux (AppImage)
```bash
chmod +x gestor-do-clube_1.0.0_amd64.AppImage
./gestor-do-clube_1.0.0_amd64.AppImage
```

### Linux (DEB)
```bash
sudo dpkg -i gestor-do-clube_1.0.0_amd64.deb
```

📖 **Veja o [Guia de Instalação Completo](docs/INSTALLATION.md)**

## 📚 Documentação

- **[Manual do Usuário (PT-BR)](docs/MANUAL_PT.md)** - Guia completo de uso
- **[Guia de Instalação](docs/INSTALLATION.md)** - Instruções detalhadas
- **[Histórico de Versões](CHANGELOG.md)** - Mudanças e melhorias

## ✨ Características

### Phase 4 Features (Current)

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

## 🔨 Build do Código Fonte

### Pré-requisitos

- Node.js 18+
- Rust 1.75+
- npm ou yarn

### Desenvolvimento

```bash
# Clone o repositório
git clone https://github.com/essilva/club-manager.git
cd club-manager

# Instale dependências
npm install

# Execute em modo desenvolvimento
npm run tauri dev
```

### Build de Produção

**Linux:**
```bash
./scripts/build-linux.sh
```

**Windows:**
```bash
./scripts/build-windows.sh
```

Binários estarão em `src-tauri/target/release/bundle/`

### Gerar Ícones

```bash
./scripts/generate-icons.sh
```

Requer ImageMagick instalado.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
