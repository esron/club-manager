# Gestor do Clube (Club Manager)

Desktop application for club membership management built with Tauri 2.x, React 18, TypeScript, and Tailwind CSS.

## Prerequisites

Before running the application, you need to install:

1. **Rust** - Visit https://www.rust-lang.org/learn/get-started#installing-rust
2. **Linux System Dependencies** (webkit2gtk & rsvg2) - Visit https://tauri.app/guides/prerequisites/#linux

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
