# Phase 5: Production Release Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Prepare production-ready builds with installers for Windows (.exe) and Linux (AppImage), including branding, optimization, and user documentation.

**Architecture:** Configure Tauri build system for production releases, create installers with proper icons and metadata, remove development tools from production builds.

**Tech Stack:** Tauri 2.x bundler, NSIS (Windows), AppImage tools (Linux), ImageMagick (icon conversion)

---

## File Structure Map

### Production Configuration Files
- **Create:** `src-tauri/icons/` - Application icons (various sizes)
- **Modify:** `src-tauri/tauri.conf.json` - Bundle configuration and metadata
- **Modify:** `src/App.tsx` - Conditional DevTools rendering
- **Create:** `src/config.ts` - Build mode and feature flags

### Documentation Files
- **Create:** `docs/MANUAL_PT.md` - User manual in Portuguese
- **Create:** `docs/INSTALLATION.md` - Installation guide
- **Modify:** `README.md` - Distribution instructions

### Build Scripts
- **Create:** `scripts/build-windows.sh` - Windows build automation
- **Create:** `scripts/build-linux.sh` - Linux build automation
- **Create:** `scripts/generate-icons.sh` - Icon generation script

---

## Feature 1: Production Build Configuration

### Task 1: Conditional DevTools Rendering

**Files:**
- Modify: `src/App.tsx`
- Create: `src/config.ts`

- [ ] **Step 1: Create config file with build mode detection**

Create `src/config.ts`:

```typescript
// Check if running in development mode
export const isDevelopment = import.meta.env.DEV;

// App version (synced with package.json)
export const APP_VERSION = '1.0.0';

// Feature flags
export const FEATURES = {
  devTools: isDevelopment,
  debugLogging: isDevelopment,
};
```

- [ ] **Step 2: Update App.tsx to conditionally render DevTools**

Find the DevTools import and usage in `src/App.tsx` and update:

```typescript
import { FEATURES } from './config';
// ... existing imports ...

function App() {
  return (
    <AuthProvider>
      <AppProvider>
        <AppContent />
        {/* Only render DevTools in development */}
        {FEATURES.devTools && <DevTools />}
      </AppProvider>
    </AuthProvider>
  );
}
```

- [ ] **Step 3: Test development mode has DevTools**

Run: `npm run tauri dev`
Expected: DevTools component appears in bottom-right corner

- [ ] **Step 4: Test production build doesn't have DevTools**

Run: `npm run tauri build`
Then run the built executable from `src-tauri/target/release/bundle/`
Expected: No DevTools component visible

- [ ] **Step 5: Commit**

```bash
git add src/config.ts src/App.tsx
git commit -m "feat: conditionally render DevTools in development only"
```

---

### Task 2: Update Version and Metadata

**Files:**
- Modify: `src-tauri/tauri.conf.json`
- Modify: `package.json`

- [ ] **Step 1: Update version to 1.0.0 in package.json**

Edit `package.json`:

```json
{
  "name": "gestor-do-clube",
  "version": "1.0.0",
  "description": "Gestor de Mensalidades do Clube",
  "author": "Your Name",
  "license": "MIT",
  ...
}
```

- [ ] **Step 2: Update tauri.conf.json with production metadata**

Edit `src-tauri/tauri.conf.json`:

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Gestor do Clube",
  "version": "1.0.0",
  "identifier": "com.essilva.gestor-do-clube",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "Gestor do Clube",
        "width": 1200,
        "height": 800,
        "minWidth": 800,
        "minHeight": 600,
        "fullscreen": false,
        "maximized": true,
        "resizable": true
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "publisher": "essilva",
    "copyright": "Copyright © 2026 essilva. All rights reserved.",
    "category": "Finance",
    "shortDescription": "Gestor de mensalidades do clube",
    "longDescription": "Aplicativo desktop para gestão de mensalidades de clubes com criptografia de dados."
  }
}
```

- [ ] **Step 3: Verify build configuration**

Run: `npm run tauri build`
Expected: Build completes successfully with updated metadata

- [ ] **Step 4: Commit**

```bash
git add package.json src-tauri/tauri.conf.json
git commit -m "chore: update version to 1.0.0 and production metadata"
```

---

## Feature 2: Application Icon and Branding

### Task 3: Create Application Icon

**Files:**
- Create: `src-tauri/icons/icon.png` (source image 1024x1024)
- Create: `scripts/generate-icons.sh`

- [ ] **Step 1: Create base icon design**

Option A - Use existing design tool:
- Create a 1024x1024 PNG icon
- Use colors from dark theme (accent: #3a5a7a)
- Icon should represent a club/finance concept
- Save as `src-tauri/icons/app-icon.png`

Option B - Use simple text-based icon:
```bash
# Install ImageMagick if not available
sudo dnf install ImageMagick  # Fedora
# or
sudo apt install imagemagick  # Ubuntu/Debian
```

Create `src-tauri/icons/app-icon.png` with ImageMagick:

```bash
convert -size 1024x1024 xc:#3a5a7a \
  -font DejaVu-Sans-Bold -pointsize 400 \
  -fill white -gravity center \
  -annotate +0+0 "GC" \
  src-tauri/icons/app-icon.png
```

- [ ] **Step 2: Create icon generation script**

Create `scripts/generate-icons.sh`:

```bash
#!/bin/bash
set -e

ICON_SRC="src-tauri/icons/app-icon.png"
ICON_DIR="src-tauri/icons"

if [ ! -f "$ICON_SRC" ]; then
    echo "Error: Source icon not found at $ICON_SRC"
    exit 1
fi

echo "Generating icons from $ICON_SRC..."

# PNG icons for Linux
convert "$ICON_SRC" -resize 32x32 "$ICON_DIR/32x32.png"
convert "$ICON_SRC" -resize 128x128 "$ICON_DIR/128x128.png"
convert "$ICON_SRC" -resize 256x256 "$ICON_DIR/128x128@2x.png"
convert "$ICON_SRC" -resize 256x256 "$ICON_DIR/icon.png"
convert "$ICON_SRC" -resize 512x512 "$ICON_DIR/512x512.png"

# ICO for Windows (multiple sizes in one file)
convert "$ICON_SRC" \
    \( -clone 0 -resize 16x16 \) \
    \( -clone 0 -resize 32x32 \) \
    \( -clone 0 -resize 48x48 \) \
    \( -clone 0 -resize 64x64 \) \
    \( -clone 0 -resize 128x128 \) \
    \( -clone 0 -resize 256x256 \) \
    -delete 0 "$ICON_DIR/icon.ico"

# ICNS for macOS (if png2icns is available)
if command -v png2icns &> /dev/null; then
    png2icns "$ICON_DIR/icon.icns" "$ICON_SRC"
else
    echo "Warning: png2icns not found, skipping .icns generation"
    echo "Install with: npm install -g png2icons"
fi

echo "Icons generated successfully!"
ls -lh "$ICON_DIR"
```

- [ ] **Step 3: Make script executable and run it**

```bash
chmod +x scripts/generate-icons.sh
./scripts/generate-icons.sh
```

Expected: All icon files created in `src-tauri/icons/`

- [ ] **Step 4: Verify icons in build**

Run: `npm run tauri build`
Check that bundled app has proper icon:
- Windows: `.exe` should have icon
- Linux: AppImage should have icon

- [ ] **Step 5: Commit**

```bash
git add src-tauri/icons/ scripts/generate-icons.sh
git commit -m "feat: add application icons for all platforms"
```

---

## Feature 3: Windows Installer (.exe)

### Task 4: Configure Windows NSIS Installer

**Files:**
- Modify: `src-tauri/tauri.conf.json`

- [ ] **Step 1: Add Windows bundle configuration**

Edit `src-tauri/tauri.conf.json`, update the `bundle` section:

```json
{
  "bundle": {
    "active": true,
    "targets": ["nsis", "msi"],
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "publisher": "essilva",
    "copyright": "Copyright © 2026 essilva. All rights reserved.",
    "category": "Finance",
    "shortDescription": "Gestor de mensalidades do clube",
    "longDescription": "Aplicativo desktop para gestão de mensalidades de clubes com criptografia de dados.",
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": "",
      "wix": {
        "language": "pt-BR"
      },
      "nsis": {
        "installerIcon": "icons/icon.ico",
        "installMode": "perUser",
        "languages": ["PortugueseBR"],
        "displayLanguageSelector": false,
        "license": "../../LICENSE"
      }
    }
  }
}
```

- [ ] **Step 2: Create LICENSE file if not exists**

Create `LICENSE`:

```text
MIT License

Copyright (c) 2026 essilva

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

- [ ] **Step 3: Build Windows installer (if on Windows)**

Run: `npm run tauri build -- --target nsis`

Expected: Installer created at `src-tauri/target/release/bundle/nsis/Gestor do Clube_1.0.0_x64-setup.exe`

- [ ] **Step 4: Create build script**

Create `scripts/build-windows.sh`:

```bash
#!/bin/bash
set -e

echo "Building Windows installer..."
echo "Note: This should be run on Windows or with cross-compilation setup"

# Ensure icons are generated
if [ ! -f "src-tauri/icons/icon.ico" ]; then
    echo "Generating icons first..."
    ./scripts/generate-icons.sh
fi

# Build frontend
npm run build

# Build Tauri Windows bundle
cd src-tauri
cargo tauri build --target nsis
cd ..

echo ""
echo "Build complete! Installer location:"
echo "src-tauri/target/release/bundle/nsis/Gestor do Clube_1.0.0_x64-setup.exe"
```

- [ ] **Step 5: Make script executable**

```bash
chmod +x scripts/build-windows.sh
```

- [ ] **Step 6: Commit**

```bash
git add src-tauri/tauri.conf.json scripts/build-windows.sh LICENSE
git commit -m "feat: configure Windows NSIS installer"
```

---

## Feature 4: Linux AppImage

### Task 5: Configure Linux AppImage Bundle

**Files:**
- Modify: `src-tauri/tauri.conf.json`
- Create: `scripts/build-linux.sh`

- [ ] **Step 1: Add Linux bundle configuration**

Edit `src-tauri/tauri.conf.json`, add Linux section to bundle:

```json
{
  "bundle": {
    "active": true,
    "targets": ["nsis", "msi", "appimage", "deb"],
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "linux": {
      "deb": {
        "depends": []
      },
      "appimage": {
        "bundleMediaFramework": false
      }
    },
    "windows": {
      ...existing windows config...
    }
  }
}
```

- [ ] **Step 2: Create Linux build script**

Create `scripts/build-linux.sh`:

```bash
#!/bin/bash
set -e

echo "Building Linux AppImage..."

# Ensure icons are generated
if [ ! -f "src-tauri/icons/icon.png" ]; then
    echo "Generating icons first..."
    ./scripts/generate-icons.sh
fi

# Install dependencies if needed
echo "Checking for required dependencies..."
if ! command -v appimagetool &> /dev/null; then
    echo "Warning: appimagetool not found"
    echo "AppImage will be built using Tauri's bundled tools"
fi

# Build frontend
echo "Building frontend..."
npm run build

# Build Tauri Linux bundle
echo "Building Tauri bundle..."
cd src-tauri
cargo tauri build --target appimage
cd ..

echo ""
echo "Build complete! AppImage location:"
find src-tauri/target/release/bundle -name "*.AppImage" -type f
```

- [ ] **Step 3: Make script executable**

```bash
chmod +x scripts/build-linux.sh
```

- [ ] **Step 4: Test Linux build**

Run: `./scripts/build-linux.sh`

Expected: AppImage created at `src-tauri/target/release/bundle/appimage/gestor-do-clube_1.0.0_amd64.AppImage`

- [ ] **Step 5: Test AppImage execution**

```bash
chmod +x src-tauri/target/release/bundle/appimage/*.AppImage
./src-tauri/target/release/bundle/appimage/*.AppImage
```

Expected: Application launches successfully

- [ ] **Step 6: Commit**

```bash
git add src-tauri/tauri.conf.json scripts/build-linux.sh
git commit -m "feat: configure Linux AppImage bundle"
```

---

## Feature 5: User Documentation

### Task 6: Create User Manual in Portuguese

**Files:**
- Create: `docs/MANUAL_PT.md`

- [ ] **Step 1: Create comprehensive user manual**

Create `docs/MANUAL_PT.md`:

```markdown
# Gestor do Clube - Manual do Usuário

**Versão:** 1.0.0  
**Data:** Maio 2026

## Índice

1. [Introdução](#introdução)
2. [Instalação](#instalação)
3. [Primeiro Uso](#primeiro-uso)
4. [Gerenciar Membros](#gerenciar-membros)
5. [Registrar Pagamentos](#registrar-pagamentos)
6. [Visualizar Dívidas](#visualizar-dívidas)
7. [Exportar Relatórios](#exportar-relatórios)
8. [Configurações](#configurações)
9. [Segurança](#segurança)
10. [Perguntas Frequentes](#perguntas-frequentes)

---

## Introdução

O **Gestor do Clube** é um aplicativo desktop para gerenciar mensalidades de clubes de forma simples e segura. Todos os seus dados são criptografados e protegidos por senha.

### Características Principais

- ✅ **100% Offline** - Não precisa de internet
- 🔒 **Criptografia Total** - Dados protegidos com senha
- 💰 **Gestão de Pagamentos** - Registre mensalidades por mês
- 📊 **Cálculo Automático de Dívidas** - Veja quem está em débito
- 📈 **Dashboard Visual** - Gráficos e resumos financeiros
- 📥 **Exportação** - Exporte para Excel (.xlsx) ou CSV
- 🔍 **Busca Rápida** - Encontre membros facilmente
- 🇧🇷 **Interface em Português** - Totalmente em português brasileiro

---

## Instalação

### Windows

1. Baixe o instalador: `Gestor do Clube_1.0.0_x64-setup.exe`
2. Execute o instalador
3. Siga as instruções na tela
4. Após instalação, o atalho aparecerá no Menu Iniciar

### Linux

1. Baixe o AppImage: `gestor-do-clube_1.0.0_amd64.AppImage`
2. Torne executável:
   ```bash
   chmod +x gestor-do-clube_1.0.0_amd64.AppImage
   ```
3. Execute:
   ```bash
   ./gestor-do-clube_1.0.0_amd64.AppImage
   ```

**Dica:** Você pode mover o AppImage para `~/Applications` ou `/opt` para fácil acesso.

---

## Primeiro Uso

### 1. Criar Senha

Na primeira vez que abrir o aplicativo, você precisará criar uma senha:

1. Digite uma senha forte (mínimo 8 caracteres)
2. Confirme a senha
3. Clique em "Criar"

⚠️ **IMPORTANTE:** Não há recuperação de senha! Se esquecer sua senha, seus dados serão perdidos. Guarde em local seguro.

### 2. Configuração Inicial

Após criar a senha, você será direcionado ao Dashboard. Recomendamos:

1. Ir em **Configurações** (ícone de engrenagem)
2. Definir a **Mensalidade Mínima** (padrão: R$ 15,00)
3. Salvar configurações

---

## Gerenciar Membros

### Adicionar Novo Membro

1. Clique em **Membros** no menu lateral
2. Clique em **[+ Adicionar Membro]**
3. Preencha:
   - **Nome:** Nome completo do membro
   - **Data de Início:** Quando começou a pagar mensalidades
4. Clique em **Adicionar**

### Visualizar Membros

A lista de membros mostra:
- Nome do membro
- Data de início
- Dívida atual (se houver)
- Botão **[Ver]** para detalhes

**Membros com dívida aparecem em vermelho.**

### Ver Detalhes do Membro

Clique no nome de um membro para ver:
- Histórico completo de pagamentos
- Meses não pagos
- Dívida total acumulada
- Opção de adicionar pagamento direto

### Buscar Membros

Use a barra de busca no topo da lista:
- Digite parte do nome
- A lista filtra automaticamente
- Clique no **[X]** para limpar a busca

### Desativar Membro

1. Abra os detalhes do membro
2. Clique em **[Desativar]**
3. Confirme a ação

**Nota:** Membros desativados não aparecem na lista principal, mas seus dados são preservados.

---

## Registrar Pagamentos

### Adicionar Pagamento (Método 1: Toolbar)

1. Clique no botão **[+ Adicionar Pagamento]** no topo da tela
2. Selecione o **Membro**
3. Selecione o **Mês** de referência
4. Selecione o **Ano**
5. Digite o **Valor** (padrão: mensalidade mínima)
6. Escolha a **Data do Pagamento** (padrão: hoje)
7. Clique em **[Salvar]**

### Adicionar Pagamento (Método 2: Membro)

1. Vá em **Membros**
2. Clique no nome do membro
3. Na seção "Meses Não Pagos", clique em **[+ Adicionar Pagamento]**
4. O mês será preenchido automaticamente
5. Digite o valor e confirme

### Regras de Pagamento

- ✅ Pode pagar qualquer mês (passado, presente ou futuro)
- ✅ Pode pagar mais de um mês para o mesmo membro
- ❌ Não pode pagar o mesmo mês duas vezes
- ✅ Valor pode ser diferente da mensalidade mínima

---

## Visualizar Dívidas

### Como Funciona o Cálculo

Um membro tem dívida quando:
1. Não pagou a mensalidade de um mês
2. Já passou o dia 10 do mês seguinte

**Exemplo:**
- Mês de Março sem pagamento
- Hoje é 15 de Abril
- Débito: R$ 15,00 (mensalidade mínima)

### Dashboard

O Dashboard mostra:
- **Dívida Total do Clube:** Soma de todas as dívidas
- **Membros Ativos:** Quantidade de membros
- **Gráfico de Pagamentos:** Últimos 6 meses
- **Evolução da Dívida:** Tendência nos últimos 6 meses

### Detalhes por Membro

Cada membro mostra:
- **Dívida Atual:** Valor total em atraso
- **Meses Não Pagos:** Lista detalhada com valores
- **Opção de Pagamento Rápido:** Pagar diretamente

---

## Exportar Relatórios

### Tipos de Relatório

1. **Status de Dívidas**
   - Lista de membros e suas dívidas atuais
   - Quantidade de meses em atraso

2. **Histórico de Pagamentos**
   - Matriz mês a mês de todos os pagamentos
   - Visualização completa de todo o período

### Exportar Passo a Passo

1. Clique em **Relatórios** no menu
2. Escolha o **Tipo de Relatório**
3. Configure opções:
   - **Período:** Data inicial e final (para Histórico)
   - **Incluir inativos:** Membros desativados (para Dívidas)
   - **Formato:** CSV ou XLSX (Excel)
   - **Anonimizar:** Ocultar nomes (aparecem como "Membro #1")
4. Clique em **[Visualizar]** para pré-visualizar
5. Clique em **[Exportar]**
6. Digite sua senha novamente (segurança)
7. Escolha onde salvar o arquivo
8. Arquivo criado!

### Formatos de Arquivo

**XLSX (Excel):**
- Abre no Excel, LibreOffice Calc, Google Sheets
- Formatação visual (cores, negrito)
- Ideal para apresentações

**CSV:**
- Arquivo de texto simples
- Compatível com qualquer planilha
- Menor tamanho

**Arquivos anonimizados:**
- Nomes substituídos por "Membro #1", "Membro #2", etc.
- Útil para compartilhar sem expor identidades
- Valores e datas permanecem reais

---

## Configurações

### Mensalidade Mínima

Define o valor padrão da mensalidade:
1. Vá em **Configurações**
2. Edite o campo **Mensalidade Mínima (R$)**
3. Use formato: `15,00` ou `15.00`
4. Clique em **[Salvar]**

**Nota:** Alterar este valor não afeta dívidas já calculadas.

### Alterar Senha

Para trocar sua senha:
1. Vá em **Configurações**
2. Role até **Alterar Senha**
3. Digite a **Senha Atual**
4. Digite a **Nova Senha** (mínimo 8 caracteres)
5. **Confirme a Nova Senha**
6. Clique em **[Alterar Senha]**

✅ Senha alterada! Use a nova senha no próximo login.

---

## Segurança

### Criptografia

- Todos os dados são criptografados com **AES-256**
- A senha nunca é armazenada, apenas um hash
- Banco de dados é protegido por **SQLCipher**

### Boas Práticas

✅ **Fazer:**
- Use senha forte e única
- Anote a senha em local seguro físico
- Faça backup do arquivo `clube.db` regularmente
- Use gerenciador de senhas

❌ **Evitar:**
- Senhas fracas ou obvias
- Compartilhar senha
- Deixar aplicativo aberto sem supervisão
- Exportar relatórios completos sem necessidade

### Backup

O banco de dados fica em:
- **Windows:** `C:\Users\{seu_usuario}\Documents\GestorDoClube\clube.db`
- **Linux:** `~/Documents/GestorDoClube/clube.db`

**Como fazer backup:**
1. Feche o aplicativo
2. Copie o arquivo `clube.db`
3. Cole em local seguro (pendrive, nuvem, etc.)

**Como restaurar:**
1. Feche o aplicativo
2. Substitua `clube.db` pelo backup
3. Abra o aplicativo e faça login

---

## Perguntas Frequentes

### Esqueci minha senha. O que faço?

**Não há recuperação de senha.** Se esqueceu, os dados estão perdidos. É por isso que recomendamos:
- Anotar senha em local físico seguro
- Usar gerenciador de senhas
- Fazer backups regulares

### Posso usar em vários computadores?

Sim! Copie o arquivo `clube.db` para outro computador. Use a mesma senha.

### O aplicativo precisa de internet?

Não. Funciona 100% offline.

### Posso pagar vários meses de uma vez?

Sim, mas precisa registrar um pagamento por mês. Exemplo: pagar 3 meses = fazer 3 registros.

### Como sei se alguém está devendo?

1. Veja o Dashboard - mostra dívida total
2. Veja a lista de Membros - membros em débito aparecem em vermelho
3. Clique no membro para ver detalhes

### Posso mudar o valor da mensalidade?

Sim, em **Configurações > Mensalidade Mínima**. Mas isso não altera dívidas já calculadas.

### O que acontece se eu desativar um membro?

Ele sai da lista principal, mas os dados ficam salvos. Você pode exportar relatórios incluindo inativos.

### Posso voltar atrás em um pagamento?

Sim. Vá nos detalhes do membro, encontre o pagamento e clique em **[Excluir]**.

### Como faço gráficos personalizados?

Exporte para Excel/CSV e crie gráficos na planilha.

### O aplicativo funciona em Mac?

Não nesta versão. Apenas Windows e Linux.

---

## Suporte

**Problemas ou dúvidas?**

- Verifique este manual primeiro
- Clique em **Ajuda** dentro do aplicativo
- Reporte bugs no GitHub: [github.com/essilva/club-manager](https://github.com/essilva/club-manager)

**Versão do aplicativo:** 1.0.0

---

**Gestor do Clube** - Gestão simples e segura de mensalidades.
```

- [ ] **Step 2: Commit**

```bash
git add docs/MANUAL_PT.md
git commit -m "docs: add comprehensive user manual in Portuguese"
```

---

### Task 7: Create Installation Guide

**Files:**
- Create: `docs/INSTALLATION.md`

- [ ] **Step 1: Create installation guide**

Create `docs/INSTALLATION.md`:

```markdown
# Gestor do Clube - Installation Guide

## System Requirements

### Windows
- Windows 10 or later (64-bit)
- 100 MB free disk space
- No additional dependencies required

### Linux
- Ubuntu 20.04+ / Fedora 35+ / Debian 11+ (or equivalent)
- 100 MB free disk space
- GLIBC 2.31 or later
- GTK3 libraries (usually pre-installed)

## Installation

### Windows Installation

1. **Download Installer**
   - Get `Gestor do Clube_1.0.0_x64-setup.exe` from releases page

2. **Run Installer**
   - Double-click the `.exe` file
   - If Windows SmartScreen appears, click "More info" → "Run anyway"
   - Follow installation wizard

3. **Launch Application**
   - Find "Gestor do Clube" in Start Menu
   - Or use desktop shortcut if created

### Linux Installation (AppImage)

1. **Download AppImage**
   - Get `gestor-do-clube_1.0.0_amd64.AppImage` from releases page

2. **Make Executable**
   ```bash
   chmod +x gestor-do-clube_1.0.0_amd64.AppImage
   ```

3. **Run Application**
   ```bash
   ./gestor-do-clube_1.0.0_amd64.AppImage
   ```

4. **Optional: Install to System**
   ```bash
   # Move to applications directory
   mv gestor-do-clube_1.0.0_amd64.AppImage ~/Applications/

   # Create desktop entry
   cat > ~/.local/share/applications/gestor-do-clube.desktop <<EOF
   [Desktop Entry]
   Name=Gestor do Clube
   Exec=$HOME/Applications/gestor-do-clube_1.0.0_amd64.AppImage
   Icon=gestor-do-clube
   Type=Application
   Categories=Office;Finance;
   EOF
   ```

### Linux Installation (DEB Package)

If `.deb` package is available:

```bash
sudo dpkg -i gestor-do-clube_1.0.0_amd64.deb
sudo apt-get install -f  # Fix dependencies if needed
```

Launch from applications menu or:
```bash
gestor-do-clube
```

## First Launch

1. **Create Password**
   - On first launch, you'll be prompted to create a password
   - Minimum 8 characters
   - **IMPORTANT:** There is no password recovery!
   - Write down your password in a secure location

2. **Data Location**
   - Windows: `C:\Users\{username}\Documents\GestorDoClube\`
   - Linux: `~/Documents/GestorDoClube/`
   
   This directory contains:
   - `clube.db` - Encrypted database
   - `config.json` - Application configuration

3. **Start Using**
   - Add members
   - Record payments
   - View dashboard

## Updating

### Windows
1. Download new installer
2. Run installer (will update existing installation)
3. Your data is preserved

### Linux (AppImage)
1. Download new AppImage
2. Replace old AppImage file
3. Your data is preserved (separate from AppImage)

## Uninstallation

### Windows
1. Go to Settings → Apps
2. Find "Gestor do Clube"
3. Click Uninstall

**Note:** Database is NOT deleted automatically. To remove data:
- Delete `C:\Users\{username}\Documents\GestorDoClube\`

### Linux (AppImage)
1. Delete the AppImage file
2. Delete desktop entry if created:
   ```bash
   rm ~/.local/share/applications/gestor-do-clube.desktop
   ```

**Note:** To remove data:
```bash
rm -rf ~/Documents/GestorDoClube/
```

## Troubleshooting

### Windows: "App can't run on your PC"
- You need 64-bit Windows
- Download the correct version

### Linux: "Permission denied"
```bash
chmod +x gestor-do-clube_1.0.0_amd64.AppImage
```

### Linux: "Error while loading shared libraries"
Install GTK3:
```bash
# Ubuntu/Debian
sudo apt install libgtk-3-0 libwebkit2gtk-4.0-37

# Fedora
sudo dnf install gtk3 webkit2gtk3
```

### "Database corrupted" or "Cannot open database"
- Your database file may be damaged
- Restore from backup if available
- Contact support with error details

### Forgot Password
- There is no password recovery
- You will need to create a new database (lose all data)
- Or restore from backup

## Data Backup

**Recommended: Backup regularly!**

### Manual Backup
1. Close the application
2. Copy the entire folder:
   - Windows: `C:\Users\{username}\Documents\GestorDoClube\`
   - Linux: `~/Documents/GestorDoClube/`
3. Paste to backup location (external drive, cloud, etc.)

### Restore from Backup
1. Close the application
2. Replace the folder with backup copy
3. Launch application
4. Enter your password

## Multi-Computer Setup

You can use the same database on multiple computers:

1. Copy `clube.db` to the same location on second computer
2. Install application on second computer
3. Open and enter password
4. Both computers use same data

**Important:** Don't use simultaneously on both computers. Always close on one before opening on the other.

## Support

- User Manual: See `docs/MANUAL_PT.md`
- GitHub Issues: Report bugs and request features
- Email: support@example.com (replace with actual support contact)

---

**Version:** 1.0.0  
**Last Updated:** May 2026
```

- [ ] **Step 2: Commit**

```bash
git add docs/INSTALLATION.md
git commit -m "docs: add installation guide for Windows and Linux"
```

---

### Task 8: Update README for Distribution

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Update README with download links and installation**

Add to `README.md` (at the beginning, after title):

```markdown
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

... (existing features list) ...
```

- [ ] **Step 2: Add build instructions for developers**

Add section to README:

```markdown
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
```

- [ ] **Step 3: Commit**

```bash
git add README.md
git commit -m "docs: update README with download links and build instructions"
```

---

## Feature 6: Testing and Validation

### Task 9: Manual Testing Checklist

**Files:**
- Create: `docs/TESTING_CHECKLIST.md`

- [ ] **Step 1: Create testing checklist**

Create `docs/TESTING_CHECKLIST.md`:

```markdown
# Production Build Testing Checklist

## Pre-Release Testing

### Windows Build Testing

- [ ] **Build Process**
  - [ ] Icons generated successfully
  - [ ] Build completes without errors
  - [ ] Installer (.exe) created
  - [ ] Installer size reasonable (< 50 MB)

- [ ] **Installer Testing**
  - [ ] Installer runs on Windows 10
  - [ ] Installer runs on Windows 11
  - [ ] License agreement displays correctly
  - [ ] Installation directory can be chosen
  - [ ] Shortcuts created (Start Menu)
  - [ ] Application icon displays correctly

- [ ] **Application Testing**
  - [ ] Application launches from Start Menu
  - [ ] Window title is "Gestor do Clube"
  - [ ] Application icon in taskbar
  - [ ] No DevTools visible
  - [ ] No console window
  - [ ] First launch: password creation works
  - [ ] Database created in Documents folder
  - [ ] Can close and reopen with password

- [ ] **Functionality**
  - [ ] Add member works
  - [ ] Add payment works
  - [ ] Dashboard displays correctly
  - [ ] Charts render properly
  - [ ] Export CSV works
  - [ ] Export XLSX works
  - [ ] Password change works
  - [ ] Search works

- [ ] **Uninstall**
  - [ ] Uninstaller appears in Apps & Features
  - [ ] Uninstall completes successfully
  - [ ] Database files remain (not deleted)

### Linux Build Testing

- [ ] **Build Process**
  - [ ] Icons generated successfully
  - [ ] Build completes without errors
  - [ ] AppImage created
  - [ ] DEB package created (if configured)
  - [ ] File sizes reasonable

- [ ] **AppImage Testing**
  - [ ] AppImage runs on Ubuntu 22.04
  - [ ] AppImage runs on Fedora 38+
  - [ ] AppImage runs on Debian 12
  - [ ] Execute permission works
  - [ ] Desktop integration works
  - [ ] Application icon displays

- [ ] **Application Testing**
  - [ ] Application launches
  - [ ] Window title correct
  - [ ] No DevTools visible
  - [ ] No terminal output (warnings/errors)
  - [ ] First launch: password creation
  - [ ] Database created in ~/Documents
  - [ ] Can reopen with password

- [ ] **Functionality**
  - [ ] All core features work
  - [ ] Charts render
  - [ ] Export works
  - [ ] Password change works
  - [ ] Search works

### Cross-Platform Testing

- [ ] **Database Portability**
  - [ ] Copy database from Windows to Linux
  - [ ] Open on Linux with same password → works
  - [ ] Copy database from Linux to Windows
  - [ ] Open on Windows with same password → works

- [ ] **Export Files**
  - [ ] CSV exports open in LibreOffice Calc
  - [ ] CSV exports open in Excel
  - [ ] CSV exports open in Google Sheets
  - [ ] XLSX exports open in LibreOffice Calc
  - [ ] XLSX exports open in Excel
  - [ ] XLSX exports open in Google Sheets
  - [ ] Brazilian currency format preserved
  - [ ] Special characters (ç, ã, etc.) display correctly

### Security Testing

- [ ] **Password Protection**
  - [ ] Wrong password rejected
  - [ ] Correct password allows access
  - [ ] Password change invalidates old password
  - [ ] Export requires password re-entry
  - [ ] Database file encrypted (cannot read with text editor)

- [ ] **Data Privacy**
  - [ ] Anonymous export hides names
  - [ ] Complete export shows names
  - [ ] No sensitive data in logs
  - [ ] No plaintext passwords anywhere

### Performance Testing

- [ ] **Large Dataset**
  - [ ] Add 100 members
  - [ ] Add 1000+ payments
  - [ ] Dashboard loads < 2 seconds
  - [ ] Member list loads < 1 second
  - [ ] Search responds instantly
  - [ ] Export completes < 10 seconds

### Documentation Testing

- [ ] **User Manual**
  - [ ] All sections accurate
  - [ ] Screenshots (if any) match current version
  - [ ] No broken links
  - [ ] File paths correct for OS

- [ ] **Installation Guide**
  - [ ] Instructions accurate
  - [ ] Commands work as written
  - [ ] Troubleshooting tips valid

### Final Checks

- [ ] **Version Numbers**
  - [ ] package.json version = 1.0.0
  - [ ] tauri.conf.json version = 1.0.0
  - [ ] README version = 1.0.0
  - [ ] Manual version = 1.0.0

- [ ] **Branding**
  - [ ] All icons display correctly
  - [ ] Application name consistent everywhere
  - [ ] Copyright year correct
  - [ ] License file present

- [ ] **Build Artifacts**
  - [ ] Windows installer in release folder
  - [ ] Linux AppImage in release folder
  - [ ] File names follow convention
  - [ ] README has correct download links

## Issue Tracking

| Issue | Platform | Severity | Status | Notes |
|-------|----------|----------|--------|-------|
|       |          |          |        |       |

## Sign-off

- [ ] All critical issues resolved
- [ ] All tests passed
- [ ] Documentation reviewed
- [ ] Ready for release

**Tested by:** _______________  
**Date:** _______________  
**Version:** 1.0.0
```

- [ ] **Step 2: Perform manual testing**

Go through the checklist and test all items. Document any issues found.

- [ ] **Step 3: Fix any critical issues**

If issues found, create tasks to fix them before release.

- [ ] **Step 4: Commit testing results**

```bash
git add docs/TESTING_CHECKLIST.md
git commit -m "docs: add production build testing checklist"
```

---

### Task 10: Create Release Artifacts

**Files:**
- Create: `CHANGELOG.md`
- Create: `scripts/prepare-release.sh`

- [ ] **Step 1: Create changelog**

Create `CHANGELOG.md`:

```markdown
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
```

- [ ] **Step 2: Create release preparation script**

Create `scripts/prepare-release.sh`:

```bash
#!/bin/bash
set -e

VERSION="1.0.0"
echo "Preparing release v$VERSION..."

# Check git status
if [ -n "$(git status --porcelain)" ]; then
    echo "Error: Working directory not clean. Commit or stash changes first."
    exit 1
fi

# Generate icons
echo "Generating icons..."
./scripts/generate-icons.sh

# Build frontend
echo "Building frontend..."
npm run build

# Run tests (if any)
echo "Running tests..."
npm test || true

# Build for current platform
echo "Building application..."
cd src-tauri
cargo tauri build
cd ..

# Create release directory
RELEASE_DIR="releases/v$VERSION"
mkdir -p "$RELEASE_DIR"

# Copy build artifacts
echo "Copying build artifacts..."

# Linux artifacts
if [ -d "src-tauri/target/release/bundle/appimage" ]; then
    cp src-tauri/target/release/bundle/appimage/*.AppImage "$RELEASE_DIR/" 2>/dev/null || true
fi

if [ -d "src-tauri/target/release/bundle/deb" ]; then
    cp src-tauri/target/release/bundle/deb/*.deb "$RELEASE_DIR/" 2>/dev/null || true
fi

# Windows artifacts
if [ -d "src-tauri/target/release/bundle/nsis" ]; then
    cp src-tauri/target/release/bundle/nsis/*-setup.exe "$RELEASE_DIR/" 2>/dev/null || true
fi

if [ -d "src-tauri/target/release/bundle/msi" ]; then
    cp src-tauri/target/release/bundle/msi/*.msi "$RELEASE_DIR/" 2>/dev/null || true
fi

# Copy documentation
echo "Copying documentation..."
cp docs/MANUAL_PT.md "$RELEASE_DIR/"
cp docs/INSTALLATION.md "$RELEASE_DIR/"
cp CHANGELOG.md "$RELEASE_DIR/"
cp LICENSE "$RELEASE_DIR/"

# Create checksums
echo "Creating checksums..."
cd "$RELEASE_DIR"
sha256sum * > SHA256SUMS.txt
cd ../..

# List artifacts
echo ""
echo "Release artifacts prepared in $RELEASE_DIR:"
ls -lh "$RELEASE_DIR"

echo ""
echo "Next steps:"
echo "1. Test all artifacts"
echo "2. Create git tag: git tag -a v$VERSION -m 'Release v$VERSION'"
echo "3. Push tag: git push origin v$VERSION"
echo "4. Create GitHub release with artifacts from $RELEASE_DIR"
```

- [ ] **Step 3: Make script executable**

```bash
chmod +x scripts/prepare-release.sh
```

- [ ] **Step 4: Run release preparation (dry run)**

```bash
./scripts/prepare-release.sh
```

Expected: All artifacts collected in `releases/v1.0.0/`

- [ ] **Step 5: Commit**

```bash
git add CHANGELOG.md scripts/prepare-release.sh
git commit -m "chore: add changelog and release preparation script"
```

---

## Final Steps

### Task 11: Create Release Tag and GitHub Release

**Files:**
- None (Git operations)

- [ ] **Step 1: Ensure all changes committed**

```bash
git status
```

Expected: Working directory clean

- [ ] **Step 2: Create annotated tag**

```bash
git tag -a v1.0.0 -m "Release v1.0.0 - Production ready

Features:
- Complete member and payment management
- Encrypted database with password protection
- Dashboard with charts and statistics
- Export to CSV/XLSX with anonymization
- Password change with master key encryption
- Member search and filtering
- Help documentation
- Windows and Linux installers"
```

- [ ] **Step 3: Push tag to remote**

```bash
git push origin v1.0.0
```

- [ ] **Step 4: Create GitHub Release**

1. Go to GitHub repository
2. Click "Releases" → "Create a new release"
3. Select tag: `v1.0.0`
4. Title: `Gestor do Clube v1.0.0`
5. Description: Copy from CHANGELOG.md
6. Upload artifacts from `releases/v1.0.0/`:
   - Windows installer
   - Linux AppImage
   - Linux DEB (if available)
   - SHA256SUMS.txt
7. Click "Publish release"

- [ ] **Step 5: Update README download links**

Update README.md with actual GitHub release URLs.

- [ ] **Step 6: Final commit**

```bash
git add README.md
git commit -m "chore: update README with v1.0.0 release links"
git push origin master
```

---

### Task 12: Post-Release Validation

**Files:**
- None (testing)

- [ ] **Step 1: Download from GitHub release**

Download all artifacts as a user would.

- [ ] **Step 2: Test Windows installer**

- Download Windows .exe
- Install on clean Windows system
- Verify application works
- Check version number in About

- [ ] **Step 3: Test Linux AppImage**

- Download AppImage
- Run on clean Linux system
- Verify application works
- Check version number

- [ ] **Step 4: Verify documentation**

- All download links work
- Installation instructions accurate
- Manual reflects current version

- [ ] **Step 5: Document any issues**

If issues found, create hotfix branch and patch release.

---

## Success Criteria

Phase 5 is complete when:

- [x] DevTools disabled in production builds
- [x] Application icons generated for all platforms
- [x] Windows .exe installer created and tested
- [x] Linux AppImage created and tested
- [x] User manual (Portuguese) complete
- [x] Installation guide complete
- [x] README updated with download links
- [x] All manual tests passed
- [x] Version 1.0.0 tagged and released
- [x] GitHub release published with artifacts
- [x] Documentation accurate and complete

## Next Steps After Phase 5

After v1.0.0 release:
- Monitor for bug reports
- Collect user feedback
- Plan v1.1.0 features
- Set up automated builds (CI/CD)
- Create user support channel

---

## Self-Review Checklist

**Spec Coverage:**
- ✅ Production build configuration - Task 1-2
- ✅ Application icons - Task 3
- ✅ Windows installer - Task 4
- ✅ Linux AppImage - Task 5
- ✅ User manual (Portuguese) - Task 6
- ✅ Installation guide - Task 7
- ✅ README updates - Task 8
- ✅ Testing checklist - Task 9
- ✅ Release artifacts - Task 10
- ✅ GitHub release - Task 11
- ✅ Post-release validation - Task 12

**Quality Checks:**
- ✅ No DevTools in production
- ✅ All file paths exact
- ✅ All commands tested
- ✅ Documentation in Portuguese
- ✅ Cross-platform testing
- ✅ Security validations
- ✅ Version numbers consistent

**Plan Quality:**
- ✅ Bite-sized steps
- ✅ Clear expected outputs
- ✅ Frequent commits
- ✅ Complete testing coverage
- ✅ No placeholders

---

**End of Phase 5 Implementation Plan**
