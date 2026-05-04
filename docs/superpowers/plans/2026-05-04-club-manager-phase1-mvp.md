# Gestor do Clube - Phase 1 (MVP) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the foundation of the club manager app with encrypted database, basic member/payment CRUD, and minimal UI.

**Architecture:** Tauri desktop app with Rust backend (SQLCipher encrypted database) and React frontend (TypeScript + Tailwind CSS dark theme). Password-based encryption using PBKDF2 for key derivation and bcrypt for authentication.

**Tech Stack:** Tauri 2.x, Rust 1.75+, SQLCipher, React 18, TypeScript 5.x, Tailwind CSS 3.x, bcrypt, date-fns

---

## File Structure Overview

**Backend (src-tauri/):**
```
src-tauri/
├── Cargo.toml
├── tauri.conf.json
├── build.rs
└── src/
    ├── main.rs
    ├── lib.rs
    ├── db/
    │   ├── mod.rs
    │   └── schema.rs
    ├── models/
    │   ├── mod.rs
    │   ├── member.rs
    │   ├── payment.rs
    │   └── settings.rs
    ├── security/
    │   ├── mod.rs
    │   ├── password.rs
    │   └── config.rs
    └── commands/
        ├── mod.rs
        ├── auth.rs
        ├── members.rs
        ├── payments.rs
        └── settings.rs
```

**Frontend (src/):**
```
src/
├── main.tsx
├── App.tsx
├── index.css
├── contexts/
│   ├── AuthContext.tsx
│   └── AppContext.tsx
├── components/
│   ├── Layout/
│   │   ├── Sidebar.tsx
│   │   └── MainLayout.tsx
│   ├── Auth/
│   │   ├── LoginScreen.tsx
│   │   └── CreatePasswordScreen.tsx
│   ├── Members/
│   │   ├── MembersList.tsx
│   │   └── AddMemberDialog.tsx
│   └── Payments/
│       ├── PaymentsList.tsx
│       └── PaymentDialog.tsx
├── types/
│   └── index.ts
└── utils/
    ├── formatters.ts
    └── validators.ts
```

---

## Task 1: Initialize Tauri Project

**Files:**
- Create: `package.json`
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/tauri.conf.json`

- [ ] **Step 1: Install Tauri CLI**

Run the following commands:

```bash
npm create tauri-app@latest
```

When prompted:
- Project name: `gestor-do-clube`
- Package manager: `npm`
- UI template: `React + TypeScript`
- Tauri manager: Use recommended settings

- [ ] **Step 2: Verify project structure**

Run:
```bash
cd gestor-do-clube
ls -la
```

Expected: Should see `package.json`, `src-tauri/` directory, `src/` directory

- [ ] **Step 3: Install frontend dependencies**

```bash
npm install
npm install -D tailwindcss postcss autoprefixer
npm install date-fns react-hook-form
npx tailwindcss init -p
```

- [ ] **Step 4: Verify Tauri dev mode works**

```bash
npm run tauri dev
```

Expected: Tauri window opens with default React app

- [ ] **Step 5: Commit**

```bash
git add .
git commit -m "feat: initialize Tauri project with React + TypeScript

Initialize base Tauri project structure with React frontend.
Ready for custom implementation.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: Configure Tailwind CSS Dark Theme

**Files:**
- Modify: `tailwind.config.js`
- Modify: `src/index.css`

- [ ] **Step 1: Configure Tailwind with dark theme colors**

Edit `tailwind.config.js`:

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        dark: {
          bg: '#1a1a1a',
          surface: '#2d2d2d',
          border: '#404040',
          text: {
            primary: '#e0e0e0',
            secondary: '#888888',
          },
          accent: '#3a5a7a',
          success: '#4ade80',
          error: '#f87171',
          warning: '#ffc107',
        },
      },
    },
  },
  plugins: [],
}
```

- [ ] **Step 2: Set up global styles**

Edit `src/index.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  body {
    @apply bg-dark-bg text-dark-text-primary;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  }

  input, select, textarea {
    @apply bg-dark-surface border border-dark-border text-dark-text-primary;
    @apply rounded px-3 py-2 focus:outline-none focus:ring-2 focus:ring-dark-accent;
  }

  button {
    @apply bg-dark-accent text-white px-4 py-2 rounded;
    @apply hover:opacity-90 transition-opacity;
  }
}
```

- [ ] **Step 3: Test dark theme**

Run:
```bash
npm run tauri dev
```

Expected: App background should be dark (#1a1a1a)

- [ ] **Step 4: Commit**

```bash
git add tailwind.config.js src/index.css
git commit -m "feat: configure Tailwind CSS with dark theme

Set up dark theme color palette matching design spec:
- Background: #1a1a1a
- Surface: #2d2d2d  
- Accent: #3a5a7a
- Success/Error/Warning colors

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: Add Rust Dependencies for SQLCipher and Encryption

**Files:**
- Modify: `src-tauri/Cargo.toml`

- [ ] **Step 1: Add dependencies to Cargo.toml**

Edit `src-tauri/Cargo.toml`, add to `[dependencies]`:

```toml
[dependencies]
tauri = { version = "2", features = [] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = { version = "0.31", features = ["bundled-sqlcipher-vendored-openssl"] }
bcrypt = "0.15"
ring = "0.17"
hex = "0.4"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
```

- [ ] **Step 2: Verify dependencies compile**

Run:
```bash
cd src-tauri
cargo build
```

Expected: Build succeeds (may take a while for first build)

- [ ] **Step 3: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/Cargo.lock
git commit -m "feat: add Rust dependencies for encryption and database

Add SQLCipher (via rusqlite), bcrypt for password hashing,
ring for PBKDF2, and other core dependencies.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: Implement Password Hashing Module

**Files:**
- Create: `src-tauri/src/security/mod.rs`
- Create: `src-tauri/src/security/password.rs`
- Create: `src-tauri/tests/security_tests.rs`

- [ ] **Step 1: Write failing test for password hashing**

Create `src-tauri/tests/security_tests.rs`:

```rust
#[cfg(test)]
mod tests {
    use gestor_do_clube::security::password::{hash_password, verify_password};

    #[test]
    fn test_password_hash_and_verify() {
        let password = "test_password_123";
        let hash = hash_password(password).expect("Failed to hash password");
        
        assert!(verify_password(password, &hash).expect("Failed to verify"));
        assert!(!verify_password("wrong_password", &hash).expect("Failed to verify"));
    }

    #[test]
    fn test_different_passwords_different_hashes() {
        let password = "same_password";
        let hash1 = hash_password(password).expect("Failed to hash");
        let hash2 = hash_password(password).expect("Failed to hash");
        
        // bcrypt includes salt, so hashes should differ
        assert_ne!(hash1, hash2);
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:
```bash
cd src-tauri
cargo test test_password_hash_and_verify
```

Expected: FAIL with module not found error

- [ ] **Step 3: Create security module structure**

Create `src-tauri/src/security/mod.rs`:

```rust
pub mod password;
```

- [ ] **Step 4: Implement password hashing**

Create `src-tauri/src/security/password.rs`:

```rust
use bcrypt::{hash, verify, DEFAULT_COST};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Failed to hash password: {0}")]
    HashError(#[from] bcrypt::BcryptError),
    
    #[error("Password verification failed")]
    VerificationError,
}

/// Hash a password using bcrypt with cost factor 12
pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    hash(password, 12).map_err(PasswordError::HashError)
}

/// Verify a password against a bcrypt hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, PasswordError> {
    verify(password, hash).map_err(PasswordError::HashError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let password = "secure_password_123";
        let hash = hash_password(password).unwrap();
        
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong", &hash).unwrap());
    }
}
```

- [ ] **Step 5: Update lib.rs to export security module**

Edit `src-tauri/src/lib.rs` (create if doesn't exist):

```rust
pub mod security;
```

- [ ] **Step 6: Run tests to verify they pass**

Run:
```bash
cd src-tauri
cargo test
```

Expected: All tests PASS

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/security/ src-tauri/src/lib.rs src-tauri/tests/
git commit -m "feat: implement password hashing with bcrypt

Add password hashing and verification using bcrypt with cost factor 12.
Includes unit tests for hash generation and verification.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: Implement PBKDF2 Key Derivation

**Files:**
- Modify: `src-tauri/src/security/password.rs`

- [ ] **Step 1: Write failing test for key derivation**

Add to `src-tauri/tests/security_tests.rs`:

```rust
use gestor_do_clube::security::password::derive_encryption_key;

#[test]
fn test_derive_encryption_key() {
    let password = "user_password";
    let salt = b"random_salt_1234"; // 16 bytes
    
    let key = derive_encryption_key(password, salt).expect("Key derivation failed");
    
    // Should produce 32-byte (256-bit) key
    assert_eq!(key.len(), 32);
}

#[test]
fn test_same_password_same_salt_same_key() {
    let password = "test";
    let salt = b"fixed_salt_12345";
    
    let key1 = derive_encryption_key(password, salt).unwrap();
    let key2 = derive_encryption_key(password, salt).unwrap();
    
    assert_eq!(key1, key2);
}

#[test]
fn test_different_salt_different_key() {
    let password = "test";
    let salt1 = b"salt_version_001";
    let salt2 = b"salt_version_002";
    
    let key1 = derive_encryption_key(password, salt1).unwrap();
    let key2 = derive_encryption_key(password, salt2).unwrap();
    
    assert_ne!(key1, key2);
}
```

- [ ] **Step 2: Run test to verify failure**

Run:
```bash
cd src-tauri
cargo test test_derive_encryption_key
```

Expected: FAIL - function not found

- [ ] **Step 3: Implement PBKDF2 key derivation**

Add to `src-tauri/src/security/password.rs`:

```rust
use ring::pbkdf2;
use std::num::NonZeroU32;

const PBKDF2_ITERATIONS: u32 = 100_000;

/// Derive a 256-bit encryption key from password using PBKDF2-SHA256
///
/// # Arguments
/// * `password` - User password
/// * `salt` - 16-byte salt (should be random and stored with config)
///
/// # Returns
/// 32-byte encryption key suitable for AES-256
pub fn derive_encryption_key(password: &str, salt: &[u8]) -> Result<Vec<u8>, PasswordError> {
    let iterations = NonZeroU32::new(PBKDF2_ITERATIONS)
        .ok_or(PasswordError::VerificationError)?;
    
    let mut key = vec![0u8; 32]; // 256 bits
    
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        iterations,
        salt,
        password.as_bytes(),
        &mut key,
    );
    
    Ok(key)
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run:
```bash
cd src-tauri
cargo test
```

Expected: All tests PASS

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/security/password.rs src-tauri/tests/security_tests.rs
git commit -m "feat: implement PBKDF2 key derivation for encryption

Add PBKDF2-SHA256 with 100,000 iterations to derive 256-bit
encryption keys from user passwords. Includes comprehensive tests.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: Implement Config File Management

**Files:**
- Create: `src-tauri/src/security/config.rs`
- Modify: `src-tauri/src/security/mod.rs`

- [ ] **Step 1: Write failing test for config operations**

Add to `src-tauri/tests/security_tests.rs`:

```rust
use gestor_do_clube::security::config::{AppConfig, save_config, load_config};
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_save_and_load_config() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");
    
    let config = AppConfig {
        password_hash: "test_hash".to_string(),
        salt: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        minimum_fee_brl: "15.00".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    save_config(&config, &config_path).expect("Failed to save config");
    let loaded = load_config(&config_path).expect("Failed to load config");
    
    assert_eq!(config.password_hash, loaded.password_hash);
    assert_eq!(config.salt, loaded.salt);
    assert_eq!(config.minimum_fee_brl, loaded.minimum_fee_brl);
}
```

- [ ] **Step 2: Add tempfile dependency**

Add to `src-tauri/Cargo.toml` under `[dev-dependencies]`:

```toml
[dev-dependencies]
tempfile = "3"
```

- [ ] **Step 3: Run test to verify failure**

Run:
```bash
cd src-tauri
cargo test test_save_and_load_config
```

Expected: FAIL - module not found

- [ ] **Step 4: Implement config module**

Create `src-tauri/src/security/config.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub password_hash: String,
    pub salt: Vec<u8>,
    pub minimum_fee_brl: String,
    pub created_at: String,
}

/// Save app configuration to JSON file
pub fn save_config(config: &AppConfig, path: &Path) -> Result<(), ConfigError> {
    let json = serde_json::to_string_pretty(config)?;
    
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(path, json)?;
    Ok(())
}

/// Load app configuration from JSON file
pub fn load_config(path: &Path) -> Result<AppConfig, ConfigError> {
    let json = fs::read_to_string(path)?;
    let config = serde_json::from_str(&json)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_config_serialization() {
        let config = AppConfig {
            password_hash: "hash123".to_string(),
            salt: vec![1, 2, 3, 4],
            minimum_fee_brl: "15.00".to_string(),
            created_at: "2026-05-04T10:00:00Z".to_string(),
        };
        
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.password_hash, deserialized.password_hash);
    }
}
```

- [ ] **Step 5: Update security mod.rs**

Edit `src-tauri/src/security/mod.rs`:

```rust
pub mod password;
pub mod config;
```

- [ ] **Step 6: Run tests to verify they pass**

Run:
```bash
cd src-tauri
cargo test
```

Expected: All tests PASS

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/security/config.rs src-tauri/src/security/mod.rs src-tauri/Cargo.toml src-tauri/tests/
git commit -m "feat: implement config file management

Add AppConfig struct and save/load functions for managing
config.json file with password hash, salt, and settings.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: Implement Database Schema Initialization

**Files:**
- Create: `src-tauri/src/db/mod.rs`
- Create: `src-tauri/src/db/schema.rs`

- [ ] **Step 1: Write failing test for schema creation**

Create `src-tauri/tests/db_tests.rs`:

```rust
use gestor_do_clube::db::schema::initialize_schema;
use rusqlite::Connection;
use tempfile::NamedTempFile;

#[test]
fn test_schema_initialization() {
    let temp_file = NamedTempFile::new().unwrap();
    let conn = Connection::open(temp_file.path()).unwrap();
    
    initialize_schema(&conn).expect("Schema initialization failed");
    
    // Verify tables exist
    let tables: Vec<String> = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
        .unwrap()
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    
    assert!(tables.contains(&"members".to_string()));
    assert!(tables.contains(&"payments".to_string()));
    assert!(tables.contains(&"settings".to_string()));
}

#[test]
fn test_settings_default_data() {
    let temp_file = NamedTempFile::new().unwrap();
    let conn = Connection::open(temp_file.path()).unwrap();
    
    initialize_schema(&conn).unwrap();
    
    let min_fee: String = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'minimum_fee_brl'",
            [],
            |row| row.get(0)
        )
        .unwrap();
    
    assert_eq!(min_fee, "15.00");
}
```

- [ ] **Step 2: Run test to verify failure**

Run:
```bash
cd src-tauri
cargo test test_schema_initialization
```

Expected: FAIL - module not found

- [ ] **Step 3: Create database module structure**

Create `src-tauri/src/db/mod.rs`:

```rust
pub mod schema;
```

Update `src-tauri/src/lib.rs`:

```rust
pub mod security;
pub mod db;
```

- [ ] **Step 4: Implement schema initialization**

Create `src-tauri/src/db/schema.rs`:

```rust
use rusqlite::{Connection, Result};

pub fn initialize_schema(conn: &Connection) -> Result<()> {
    // Create members table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS members (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            start_date TEXT NOT NULL,
            created_at TEXT NOT NULL,
            active BOOLEAN DEFAULT 1
        )",
        [],
    )?;
    
    // Create payments table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS payments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            member_id INTEGER NOT NULL,
            month INTEGER NOT NULL,
            year INTEGER NOT NULL,
            amount_brl REAL NOT NULL,
            payment_date TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (member_id) REFERENCES members(id),
            UNIQUE(member_id, month, year)
        )",
        [],
    )?;
    
    // Create settings table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;
    
    // Create indexes
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_payments_member ON payments(member_id)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_payments_date ON payments(year, month)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_members_active ON members(active)",
        [],
    )?;
    
    // Insert default settings if not exists
    conn.execute(
        "INSERT OR IGNORE INTO settings (key, value) VALUES ('minimum_fee_brl', '15.00')",
        [],
    )?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creation() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        
        // Test that we can query the tables
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM members", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run:
```bash
cd src-tauri
cargo test
```

Expected: All tests PASS

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/db/ src-tauri/src/lib.rs src-tauri/tests/db_tests.rs
git commit -m "feat: implement database schema initialization

Create members, payments, and settings tables with indexes.
Insert default minimum fee setting. Includes comprehensive tests.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 8: Implement Database Connection with SQLCipher

**Files:**
- Modify: `src-tauri/src/db/mod.rs`
- Create: `src-tauri/src/db/connection.rs`

- [ ] **Step 1: Write failing test for encrypted connection**

Add to `src-tauri/tests/db_tests.rs`:

```rust
use gestor_do_clube::db::connection::open_encrypted_db;
use tempfile::NamedTempFile;

#[test]
fn test_open_encrypted_database() {
    let temp_file = NamedTempFile::new().unwrap();
    let key = "test_encryption_key_32_bytes!!";
    
    // Create encrypted database
    let conn = open_encrypted_db(temp_file.path(), key).expect("Failed to open encrypted DB");
    
    // Verify we can use it
    conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY)", []).unwrap();
    drop(conn);
    
    // Verify we can reopen with same key
    let conn2 = open_encrypted_db(temp_file.path(), key).expect("Failed to reopen");
    let count: i64 = conn2
        .query_row("SELECT COUNT(*) FROM test", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_wrong_key_fails() {
    let temp_file = NamedTempFile::new().unwrap();
    let key1 = "correct_key_12345678901234567";
    let key2 = "wrong_key_123456789012345678";
    
    // Create with key1
    let conn = open_encrypted_db(temp_file.path(), key1).unwrap();
    conn.execute("CREATE TABLE test (id INTEGER)", []).unwrap();
    drop(conn);
    
    // Try to open with key2 - should fail
    let result = open_encrypted_db(temp_file.path(), key2);
    assert!(result.is_err());
}
```

- [ ] **Step 2: Run test to verify failure**

Run:
```bash
cd src-tauri
cargo test test_open_encrypted_database
```

Expected: FAIL - function not found

- [ ] **Step 3: Implement encrypted database connection**

Create `src-tauri/src/db/connection.rs`:

```rust
use rusqlite::{Connection, Result as SqlResult};
use std::path::Path;

/// Open an encrypted SQLCipher database
///
/// # Arguments
/// * `path` - Path to database file
/// * `key` - Encryption key (hex-encoded)
///
/// # Returns
/// SQLite connection with encryption enabled
pub fn open_encrypted_db(path: &Path, key: &str) -> SqlResult<Connection> {
    let conn = Connection::open(path)?;
    
    // Configure SQLCipher
    conn.pragma_update(None, "cipher", "aes-256-cbc")?;
    conn.pragma_update(None, "kdf_iter", 100000)?;
    conn.pragma_update(None, "cipher_page_size", 4096)?;
    
    // Set encryption key
    conn.pragma_update(None, "key", format!("\"x'{}'\"", key))?;
    
    // Test that key is correct by executing a simple query
    conn.execute("SELECT count(*) FROM sqlite_master", [])?;
    
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_connection() {
        let temp = NamedTempFile::new().unwrap();
        let conn = open_encrypted_db(temp.path(), "testkey123").unwrap();
        
        conn.execute("CREATE TABLE test (id INTEGER)", []).unwrap();
    }
}
```

- [ ] **Step 4: Update db/mod.rs**

Edit `src-tauri/src/db/mod.rs`:

```rust
pub mod schema;
pub mod connection;
```

- [ ] **Step 5: Run tests to verify they pass**

Run:
```bash
cd src-tauri
cargo test
```

Expected: All tests PASS

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/db/connection.rs src-tauri/src/db/mod.rs src-tauri/tests/db_tests.rs
git commit -m "feat: implement encrypted database connection with SQLCipher

Add function to open SQLCipher encrypted database with AES-256-CBC.
Includes tests for encryption and key verification.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 9: Implement Member Model and CRUD

**Files:**
- Create: `src-tauri/src/models/mod.rs`
- Create: `src-tauri/src/models/member.rs`

- [ ] **Step 1: Write failing tests for member operations**

Create `src-tauri/tests/member_tests.rs`:

```rust
use gestor_do_clube::models::member::{Member, create_member, get_members, get_member_by_id};
use gestor_do_clube::db::schema::initialize_schema;
use rusqlite::Connection;

#[test]
fn test_create_and_get_member() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();
    
    let member_id = create_member(&conn, "João Silva", "2026-01-15").unwrap();
    let member = get_member_by_id(&conn, member_id).unwrap();
    
    assert_eq!(member.name, "João Silva");
    assert_eq!(member.start_date, "2026-01-15");
    assert_eq!(member.active, true);
}

#[test]
fn test_get_all_members() {
    let conn = Connection::open_in_memory().unwrap();
    initialize_schema(&conn).unwrap();
    
    create_member(&conn, "Member 1", "2026-01-01").unwrap();
    create_member(&conn, "Member 2", "2026-02-01").unwrap();
    
    let members = get_members(&conn).unwrap();
    assert_eq!(members.len(), 2);
}
```

- [ ] **Step 2: Run test to verify failure**

Run:
```bash
cd src-tauri
cargo test test_create_and_get_member
```

Expected: FAIL - module not found

- [ ] **Step 3: Create models module structure**

Create `src-tauri/src/models/mod.rs`:

```rust
pub mod member;
```

Update `src-tauri/src/lib.rs`:

```rust
pub mod security;
pub mod db;
pub mod models;
```

- [ ] **Step 4: Implement Member model**

Create `src-tauri/src/models/member.rs`:

```rust
use rusqlite::{Connection, Result, Row};
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: i64,
    pub name: String,
    pub start_date: String,
    pub created_at: String,
    pub active: bool,
}

impl Member {
    fn from_row(row: &Row) -> Result<Self> {
        Ok(Member {
            id: row.get(0)?,
            name: row.get(1)?,
            start_date: row.get(2)?,
            created_at: row.get(3)?,
            active: row.get(4)?,
        })
    }
}

/// Create a new member
pub fn create_member(conn: &Connection, name: &str, start_date: &str) -> Result<i64> {
    let created_at = Utc::now().to_rfc3339();
    
    conn.execute(
        "INSERT INTO members (name, start_date, created_at, active) VALUES (?, ?, ?, 1)",
        [name, start_date, &created_at],
    )?;
    
    Ok(conn.last_insert_rowid())
}

/// Get all active members
pub fn get_members(conn: &Connection) -> Result<Vec<Member>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, start_date, created_at, active FROM members WHERE active = 1 ORDER BY name"
    )?;
    
    let members = stmt.query_map([], Member::from_row)?
        .collect::<Result<Vec<_>>>()?;
    
    Ok(members)
}

/// Get member by ID
pub fn get_member_by_id(conn: &Connection, id: i64) -> Result<Member> {
    conn.query_row(
        "SELECT id, name, start_date, created_at, active FROM members WHERE id = ?",
        [id],
        Member::from_row,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schema::initialize_schema;

    #[test]
    fn test_member_crud() {
        let conn = Connection::open_in_memory().unwrap();
        initialize_schema(&conn).unwrap();
        
        let id = create_member(&conn, "Test Member", "2026-01-01").unwrap();
        let member = get_member_by_id(&conn, id).unwrap();
        
        assert_eq!(member.name, "Test Member");
        assert!(member.active);
    }
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run:
```bash
cd src-tauri
cargo test
```

Expected: All tests PASS

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/models/ src-tauri/tests/member_tests.rs src-tauri/src/lib.rs
git commit -m "feat: implement Member model with CRUD operations

Add Member struct and database operations:
- create_member
- get_members
- get_member_by_id

Includes comprehensive unit tests.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

---

## Summary: Remaining Phase 1 Tasks

The plan above covers Tasks 1-9 (Project setup through Member model). Phase 1 MVP requires completing Tasks 10-20 following the same TDD pattern:

**Tasks 10-12: Core Models**
- Task 10: Payment Model (create_payment, get_payments, get_payment_by_member_month)
- Task 11: Settings Model (get_setting, update_setting)
- Task 12: Debt Calculation Logic (calculate_member_debt function)

**Tasks 13-15: Tauri Commands (Backend→Frontend Bridge)**
- Task 13: Authentication Commands (setup_password, verify_password, check_first_launch)
- Task 14: Member Commands (add_member_cmd, get_members_cmd, get_member_cmd)
- Task 15: Payment Commands (add_payment_cmd, get_payments_cmd)

**Tasks 16-18: Frontend Foundation**
- Task 16: TypeScript Types & Utils (Member, Payment, formatCurrency, formatDate, validators)
- Task 17: Auth Context (password management, app initialization state)
- Task 18: App Context (members list, payments list, refresh functions)

**Tasks 19-20: UI Components**
- Task 19: Auth Screens (CreatePasswordScreen, LoginScreen with security warning)
- Task 20: Main Layout (Sidebar, Dashboard placeholder, Members list, Payments list)

**Each task follows TDD:**
1. Write failing test
2. Run to verify failure
3. Implement minimal code
4. Run to verify pass
5. Commit with descriptive message

**Phase 1 Deliverable:**
- ✅ Encrypted database with SQLCipher
- ✅ Password-based authentication
- ✅ Member CRUD (add, list, view)
- ✅ Payment CRUD (add, list)
- ✅ Dark theme UI with sidebar navigation
- ✅ Brazilian Portuguese labels
- ✅ All data encrypted at rest

**To execute this plan:**

Use `superpowers:subagent-driven-development` (recommended) or `superpowers:executing-plans` skill to implement tasks 1-9 first, then use the same pattern to implement tasks 10-20. Each task is self-contained and can be implemented independently by following the TDD steps.

**Not in Phase 1 (saved for Phase 2):**
- Dashboard widgets (summary cards, charts)
- Debt calculation UI display
- Payment hybrid UI (inline dialog)
- Member detail view with payment history
- Validation error messages
- Export functionality

---

## Phase 1 Complete Definition of Done

**Code:**
- [ ] All 20 tasks completed and committed
- [ ] All unit tests pass (`cargo test` in src-tauri/)
- [ ] App builds without warnings (`npm run tauri build`)
- [ ] Dark theme applied throughout UI

**Functionality:**
- [ ] User can create password on first launch
- [ ] Database file is created encrypted
- [ ] User can login with correct password
- [ ] Wrong password is rejected
- [ ] User can add members (name + start date)
- [ ] Members list displays all active members
- [ ] User can add payments (member + month/year + amount)
- [ ] Payments list shows recorded payments
- [ ] Duplicate payments are prevented (same member/month)
- [ ] App works 100% offline

**Testing:**
- [ ] Manual test: Create password, add 3 members, add 5 payments
- [ ] Manual test: Close app, reopen, verify password works
- [ ] Manual test: Try wrong password, verify rejection
- [ ] Manual test: Verify database file exists in ~/Documents/GestorDoClube/

**Documentation:**
- [ ] README.md with build instructions
- [ ] Phase 2 planning document referencing this as base

---

**End of Phase 1 Implementation Plan**

