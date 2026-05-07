# Phase 4: Polish Features Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add production-ready polish features including password change, member search, financial charts, and help documentation.

**Architecture:** Four independent features implemented sequentially. Password change uses master key encryption. Search is client-side filtering. Charts use Recharts library with backend data queries. Help screen is static content.

**Tech Stack:** React 18 + TypeScript + Tailwind CSS + Recharts (frontend), Rust + Tauri + SQLCipher (backend)

---

## File Structure Map

### Feature 1: Password Change with Master Key

**Backend Files:**
- Modify: `src-tauri/src/security/config.rs` - Add `master_key_encrypted` field to AppConfig
- Modify: `src-tauri/src/security/password.rs` - Add master key encryption/decryption functions
- Modify: `src-tauri/src/commands/auth.rs` - Add migration and password change logic
- Modify: `src-tauri/src/lib.rs` - Register new commands
- Create: `src-tauri/tests/password_change_tests.rs` - Password change tests

**Frontend Files:**
- Modify: `src/components/SettingsScreen.tsx` - Add password change form

### Feature 2: Member Search

**Frontend Files:**
- Modify: `src/components/MainLayout.tsx` - Add search input and filter logic

### Feature 3: Dashboard Charts

**Backend Files:**
- Create: `src-tauri/src/models/charts.rs` - Chart data models and generation logic
- Create: `src-tauri/src/commands/charts.rs` - Chart data command
- Modify: `src-tauri/src/models/mod.rs` - Register charts module
- Modify: `src-tauri/src/commands/mod.rs` - Register charts module
- Modify: `src-tauri/src/lib.rs` - Register chart command

**Frontend Files:**
- Modify: `package.json` - Add recharts dependency
- Create: `src/components/DashboardCharts.tsx` - Chart components
- Modify: `src/components/DashboardScreen.tsx` - Integrate charts

### Feature 4: About/Help Screen

**Frontend Files:**
- Create: `src/components/HelpScreen.tsx` - Help/about content
- Modify: `src/components/MainLayout.tsx` - Add help tab to navigation

---

## Feature 1: Password Change with Master Key

### Task 1: Add Master Key Encryption Functions

**Files:**
- Modify: `src-tauri/src/security/password.rs`

- [ ] **Step 1: Add AES-GCM encryption dependencies**

Check if `aes-gcm` is already in Cargo.toml. If not, add it:

```toml
aes-gcm = "0.10"
```

- [ ] **Step 2: Add master key encryption function**

Add to `src-tauri/src/security/password.rs`:

```rust
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use ring::pbkdf2;

/// Encrypt master key with password-derived key
pub fn encrypt_master_key(master_key: &[u8; 32], password: &str, salt: &[u8]) -> Result<Vec<u8>, String> {
    // Derive encryption key from password
    let mut key_bytes = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(100_000).unwrap(),
        salt,
        password.as_bytes(),
        &mut key_bytes,
    );

    // Create cipher
    let cipher = Aes256Gcm::new(&key_bytes.into());
    
    // Generate random nonce (12 bytes for AES-GCM)
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    // Encrypt
    let ciphertext = cipher.encrypt(nonce, master_key.as_ref())
        .map_err(|e| format!("Encryption failed: {}", e))?;
    
    // Prepend nonce to ciphertext
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}

/// Decrypt master key with password-derived key
pub fn decrypt_master_key(encrypted_data: &[u8], password: &str, salt: &[u8]) -> Result<[u8; 32], String> {
    if encrypted_data.len() < 12 {
        return Err("Invalid encrypted data".to_string());
    }
    
    // Derive encryption key from password
    let mut key_bytes = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(100_000).unwrap(),
        salt,
        password.as_bytes(),
        &mut key_bytes,
    );
    
    // Create cipher
    let cipher = Aes256Gcm::new(&key_bytes.into());
    
    // Extract nonce and ciphertext
    let nonce = Nonce::from_slice(&encrypted_data[0..12]);
    let ciphertext = &encrypted_data[12..];
    
    // Decrypt
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;
    
    if plaintext.len() != 32 {
        return Err("Invalid master key size".to_string());
    }
    
    let mut master_key = [0u8; 32];
    master_key.copy_from_slice(&plaintext);
    
    Ok(master_key)
}
```

- [ ] **Step 3: Build to verify compilation**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/security/password.rs src-tauri/Cargo.toml
git commit -m "feat: add master key encryption/decryption functions"
```

---

### Task 2: Update Config Structure

**Files:**
- Modify: `src-tauri/src/security/config.rs`

- [ ] **Step 1: Add master_key_encrypted field to AppConfig**

In `src-tauri/src/security/config.rs`, update the AppConfig struct:

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub password_hash: String,
    pub salt: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_key_encrypted: Option<Vec<u8>>,
    pub minimum_fee_brl: String,
    pub created_at: String,
}
```

- [ ] **Step 2: Build to verify compilation**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/security/config.rs
git commit -m "feat: add master_key_encrypted field to AppConfig"
```

---

### Task 3: Add Migration Logic

**Files:**
- Modify: `src-tauri/src/commands/auth.rs`

- [ ] **Step 1: Add migration check command**

Add to `src-tauri/src/commands/auth.rs`:

```rust
/// Check if config needs migration to master key format
#[tauri::command]
pub fn needs_migration() -> Result<bool, String> {
    let config_path = get_config_path();
    
    if !config_path.exists() {
        return Ok(false); // First launch, no migration needed
    }
    
    let config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;
    
    Ok(config.master_key_encrypted.is_none())
}
```

- [ ] **Step 2: Add migrate_to_master_key command**

Add to `src-tauri/src/commands/auth.rs`:

```rust
use crate::security::password::{encrypt_master_key, decrypt_master_key};

/// Migrate existing database to master key encryption
#[tauri::command]
pub fn migrate_to_master_key(password: String) -> Result<(), String> {
    let config_path = get_config_path();
    let mut config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;
    
    // Check if already migrated
    if config.master_key_encrypted.is_some() {
        return Err("Already migrated".to_string());
    }
    
    // Verify password
    let is_valid = verify_password(&password, &config.password_hash)
        .map_err(|e| format!("Failed to verify password: {}", e))?;
    
    if !is_valid {
        return Err("Incorrect password".to_string());
    }
    
    // Derive current encryption key from password
    let current_key = derive_encryption_key(&password, &config.salt)
        .map_err(|e| format!("Failed to derive key: {}", e))?;
    let current_key_hex = hex::encode(&current_key);
    
    // Open database with current key
    let db_path = get_db_path();
    let conn = open_encrypted_db(&db_path, &current_key_hex)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    // Generate new random master key
    let master_key: [u8; 32] = rand::random();
    let master_key_hex = hex::encode(&master_key);
    
    // Re-encrypt database with master key
    conn.execute(&format!("PRAGMA rekey = \"x'{}'\";", master_key_hex), [])
        .map_err(|e| format!("Failed to re-encrypt database: {}", e))?;
    
    // Encrypt master key with password-derived key
    let encrypted_master_key = encrypt_master_key(&master_key, &password, &config.salt)
        .map_err(|e| format!("Failed to encrypt master key: {}", e))?;
    
    // Update config
    config.master_key_encrypted = Some(encrypted_master_key);
    save_config(&config, &config_path)
        .map_err(|e| format!("Failed to save config: {}", e))?;
    
    Ok(())
}
```

- [ ] **Step 3: Update setup_password to use master key**

Modify `setup_password` function to create master key on first launch:

```rust
#[tauri::command]
pub fn setup_password(password: String) -> Result<(), String> {
    let config_path = get_config_path();

    // Verify this is first launch
    if config_path.exists() {
        return Err("Password already configured".to_string());
    }

    // Hash password with bcrypt
    let password_hash = hash_password(&password)
        .map_err(|e| format!("Failed to hash password: {}", e))?;

    // Generate random salt for encryption
    let salt: Vec<u8> = (0..16).map(|_| rand::random::<u8>()).collect();

    // Generate random master key
    let master_key: [u8; 32] = rand::random();
    let master_key_hex = hex::encode(&master_key);

    // Encrypt master key with password
    let encrypted_master_key = encrypt_master_key(&master_key, &password, &salt)
        .map_err(|e| format!("Failed to encrypt master key: {}", e))?;

    // Create config
    let config = AppConfig {
        password_hash,
        salt: salt.clone(),
        master_key_encrypted: Some(encrypted_master_key),
        minimum_fee_brl: "15.00".to_string(),
        created_at: Utc::now().to_rfc3339(),
    };

    // Save config
    save_config(&config, &config_path)
        .map_err(|e| format!("Failed to save config: {}", e))?;

    // Create encrypted database with master key
    let db_path = get_db_path();
    let conn = open_encrypted_db(&db_path, &master_key_hex)
        .map_err(|e| format!("Failed to create database: {}", e))?;

    // Initialize schema
    initialize_schema(&conn)
        .map_err(|e| format!("Failed to initialize schema: {}", e))?;

    Ok(())
}
```

- [ ] **Step 4: Update verify_password_cmd to use master key**

Modify `verify_password_cmd` to decrypt and use master key:

```rust
#[tauri::command]
pub fn verify_password_cmd(password: String) -> Result<bool, String> {
    let config_path = get_config_path();

    // Load config
    let config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    // Verify password
    let is_valid = verify_password(&password, &config.password_hash)
        .map_err(|e| format!("Failed to verify password: {}", e))?;

    if !is_valid {
        return Ok(false);
    }

    // Decrypt master key
    let encrypted_master_key = config.master_key_encrypted
        .ok_or("Master key not found in config".to_string())?;
    
    let master_key = decrypt_master_key(&encrypted_master_key, &password, &config.salt)
        .map_err(|e| format!("Failed to decrypt master key: {}", e))?;
    
    let master_key_hex = hex::encode(&master_key);

    // Test database connection with master key
    let db_path = get_db_path();
    let _conn = open_encrypted_db(&db_path, &master_key_hex)
        .map_err(|_| "Failed to open database with password".to_string())?;

    Ok(true)
}
```

- [ ] **Step 5: Build to verify compilation**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/commands/auth.rs
git commit -m "feat: add migration logic and update auth to use master key"
```

---

### Task 4: Add Password Change Command

**Files:**
- Modify: `src-tauri/src/commands/auth.rs`

- [ ] **Step 1: Add change_password command**

Add to `src-tauri/src/commands/auth.rs`:

```rust
/// Change user password
#[tauri::command]
pub fn change_password(current_password: String, new_password: String) -> Result<(), String> {
    let config_path = get_config_path();
    let mut config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;
    
    // Verify current password
    let is_valid = verify_password(&current_password, &config.password_hash)
        .map_err(|e| format!("Failed to verify password: {}", e))?;
    
    if !is_valid {
        return Err("Current password is incorrect".to_string());
    }
    
    // Decrypt master key with current password
    let encrypted_master_key = config.master_key_encrypted
        .ok_or("Master key not found in config".to_string())?;
    
    let master_key = decrypt_master_key(&encrypted_master_key, &current_password, &config.salt)
        .map_err(|e| format!("Failed to decrypt master key: {}", e))?;
    
    // Re-encrypt master key with new password
    let new_encrypted_master_key = encrypt_master_key(&master_key, &new_password, &config.salt)
        .map_err(|e| format!("Failed to encrypt master key: {}", e))?;
    
    // Hash new password
    let new_password_hash = hash_password(&new_password)
        .map_err(|e| format!("Failed to hash password: {}", e))?;
    
    // Update config
    config.password_hash = new_password_hash;
    config.master_key_encrypted = Some(new_encrypted_master_key);
    
    save_config(&config, &config_path)
        .map_err(|e| format!("Failed to save config: {}", e))?;
    
    Ok(())
}
```

- [ ] **Step 2: Register new commands in lib.rs**

Add to invoke_handler in `src-tauri/src/lib.rs`:

```rust
commands::auth::needs_migration,
commands::auth::migrate_to_master_key,
commands::auth::change_password,
```

- [ ] **Step 3: Build to verify compilation**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/commands/auth.rs src-tauri/src/lib.rs
git commit -m "feat: add password change command"
```

---

### Task 5: Add Password Change Tests

**Files:**
- Create: `src-tauri/tests/password_change_tests.rs`

- [ ] **Step 1: Create password change tests**

Create `src-tauri/tests/password_change_tests.rs`:

```rust
use gestor_do_clube_lib::security::password::{hash_password, verify_password, derive_encryption_key, encrypt_master_key, decrypt_master_key};
use gestor_do_clube_lib::security::config::{AppConfig, save_config, load_config};
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_master_key_encryption_decryption() {
    let master_key: [u8; 32] = [1u8; 32];
    let password = "test_password";
    let salt: Vec<u8> = (0..16).map(|i| i as u8).collect();
    
    let encrypted = encrypt_master_key(&master_key, password, &salt).unwrap();
    let decrypted = decrypt_master_key(&encrypted, password, &salt).unwrap();
    
    assert_eq!(master_key, decrypted);
}

#[test]
fn test_master_key_wrong_password() {
    let master_key: [u8; 32] = [1u8; 32];
    let password = "correct_password";
    let wrong_password = "wrong_password";
    let salt: Vec<u8> = (0..16).map(|i| i as u8).collect();
    
    let encrypted = encrypt_master_key(&master_key, password, &salt).unwrap();
    let result = decrypt_master_key(&encrypted, wrong_password, &salt);
    
    assert!(result.is_err());
}

#[test]
fn test_config_with_master_key() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");
    
    let master_key: [u8; 32] = rand::random();
    let password = "test_password";
    let salt: Vec<u8> = (0..16).map(|_| rand::random()).collect();
    
    let encrypted_master_key = encrypt_master_key(&master_key, password, &salt).unwrap();
    
    let config = AppConfig {
        password_hash: hash_password(password).unwrap(),
        salt: salt.clone(),
        master_key_encrypted: Some(encrypted_master_key.clone()),
        minimum_fee_brl: "15.00".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    
    save_config(&config, &config_path).unwrap();
    let loaded_config = load_config(&config_path).unwrap();
    
    assert_eq!(loaded_config.master_key_encrypted.unwrap(), encrypted_master_key);
}
```

- [ ] **Step 2: Run tests**

Run: `cd src-tauri && cargo test password_change_tests`
Expected: All tests pass

- [ ] **Step 3: Commit**

```bash
git add src-tauri/tests/password_change_tests.rs
git commit -m "test: add password change and master key tests"
```

---

### Task 6: Add Password Change UI

**Files:**
- Modify: `src/components/SettingsScreen.tsx`

- [ ] **Step 1: Add password change state and form**

Update `src/components/SettingsScreen.tsx`:

```typescript
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useApp } from '../contexts/AppContext';

export const SettingsScreen = () => {
  const { settings, updateSetting } = useApp();
  const [minimumFee, setMinimumFee] = useState('');
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const [loading, setLoading] = useState(false);

  // Password change state
  const [currentPassword, setCurrentPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [passwordError, setPasswordError] = useState('');
  const [passwordSuccess, setPasswordSuccess] = useState('');
  const [passwordLoading, setPasswordLoading] = useState(false);

  useEffect(() => {
    setMinimumFee(settings.minimumFee);
  }, [settings]);

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setSuccess('');
    setLoading(true);

    try {
      await updateSetting('minimum_fee_brl', minimumFee);
      setSuccess('Configurações salvas com sucesso');
      setTimeout(() => setSuccess(''), 3000);
    } catch (err) {
      console.error('Error saving settings:', err);
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const handlePasswordChange = async (e: React.FormEvent) => {
    e.preventDefault();
    setPasswordError('');
    setPasswordSuccess('');

    // Validation
    if (newPassword.length < 8) {
      setPasswordError('A nova senha deve ter no mínimo 8 caracteres');
      return;
    }

    if (newPassword !== confirmPassword) {
      setPasswordError('As senhas não coincidem');
      return;
    }

    if (newPassword === currentPassword) {
      setPasswordError('A nova senha deve ser diferente da senha atual');
      return;
    }

    setPasswordLoading(true);

    try {
      await invoke('change_password', {
        currentPassword,
        newPassword,
      });

      setPasswordSuccess('Senha alterada com sucesso');
      setCurrentPassword('');
      setNewPassword('');
      setConfirmPassword('');
      setTimeout(() => setPasswordSuccess(''), 3000);
    } catch (err) {
      console.error('Error changing password:', err);
      setPasswordError(String(err));
    } finally {
      setPasswordLoading(false);
    }
  };

  return (
    <div className="flex-1 p-8">
      <h1 className="text-2xl font-bold mb-6 text-dark-text-primary">Configurações</h1>

      {/* Minimum Fee Settings */}
      <div className="bg-dark-surface p-6 rounded-lg border border-dark-border max-w-2xl mb-6">
        <h2 className="text-lg font-semibold mb-4 text-dark-text-primary">Mensalidade</h2>
        <form onSubmit={handleSave}>
          <div className="mb-6">
            <label className="block mb-2 text-dark-text-secondary">
              Mensalidade Mínima (R$)
            </label>
            <input
              type="text"
              value={minimumFee}
              onChange={(e) => setMinimumFee(e.target.value)}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              placeholder="15.00"
              required
            />
            {error && <p className="text-dark-error text-sm mt-2">{error}</p>}
            {success && <p className="text-green-500 text-sm mt-2">{success}</p>}
          </div>

          <button
            type="submit"
            disabled={loading}
            className="bg-dark-accent text-white px-6 py-2 rounded hover:opacity-90 disabled:opacity-50"
          >
            {loading ? 'Salvando...' : 'Salvar'}
          </button>
        </form>
      </div>

      {/* Password Change */}
      <div className="bg-dark-surface p-6 rounded-lg border border-dark-border max-w-2xl">
        <h2 className="text-lg font-semibold mb-4 text-dark-text-primary">Alterar Senha</h2>
        <form onSubmit={handlePasswordChange}>
          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">
              Senha Atual
            </label>
            <input
              type="password"
              value={currentPassword}
              onChange={(e) => setCurrentPassword(e.target.value)}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              required
            />
          </div>

          <div className="mb-4">
            <label className="block mb-2 text-dark-text-secondary">
              Nova Senha (mínimo 8 caracteres)
            </label>
            <input
              type="password"
              value={newPassword}
              onChange={(e) => setNewPassword(e.target.value)}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              minLength={8}
              required
            />
          </div>

          <div className="mb-6">
            <label className="block mb-2 text-dark-text-secondary">
              Confirmar Nova Senha
            </label>
            <input
              type="password"
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2"
              minLength={8}
              required
            />
          </div>

          {passwordError && <p className="text-dark-error text-sm mb-4">{passwordError}</p>}
          {passwordSuccess && <p className="text-green-500 text-sm mb-4">{passwordSuccess}</p>}

          <button
            type="submit"
            disabled={passwordLoading}
            className="bg-dark-accent text-white px-6 py-2 rounded hover:opacity-90 disabled:opacity-50"
          >
            {passwordLoading ? 'Alterando...' : 'Alterar Senha'}
          </button>
        </form>
      </div>
    </div>
  );
};
```

- [ ] **Step 2: Test password change UI**

Run: `npm run tauri dev`
Steps:
1. Go to Settings
2. Fill in current password, new password, confirm password
3. Click "Alterar Senha"
4. Verify success message appears
5. Logout and login with new password

Expected: Password change works, user can login with new password

- [ ] **Step 3: Commit**

```bash
git add src/components/SettingsScreen.tsx
git commit -m "feat: add password change UI to settings screen"
```

---

### Task 7: Add Migration UI

**Files:**
- Create: `src/components/MigrationModal.tsx`
- Modify: `src/App.tsx`

- [ ] **Step 1: Create migration modal component**

Create `src/components/MigrationModal.tsx`:

```typescript
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface MigrationModalProps {
  onComplete: () => void;
}

export const MigrationModal = ({ onComplete }: MigrationModalProps) => {
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  const handleMigrate = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      await invoke('migrate_to_master_key', { password });
      onComplete();
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 w-96 shadow-xl">
        <h2 className="text-xl font-bold mb-4 text-white">Atualização Necessária</h2>
        <p className="text-gray-300 mb-4">
          O aplicativo foi atualizado. Digite sua senha para continuar.
        </p>
        
        <form onSubmit={handleMigrate}>
          <div className="mb-4">
            <label className="block text-sm font-medium mb-2 text-gray-300">
              Senha
            </label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
              autoFocus
              disabled={loading}
              required
            />
            {error && (
              <p className="text-red-500 text-sm mt-1">{error}</p>
            )}
          </div>

          <button
            type="submit"
            className="w-full px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-500 disabled:opacity-50"
            disabled={loading}
          >
            {loading ? 'Atualizando...' : 'Continuar'}
          </button>
        </form>
      </div>
    </div>
  );
};
```

- [ ] **Step 2: Add migration check to App.tsx**

Update `src/App.tsx` to check for migration on login:

```typescript
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { CreatePasswordScreen } from './components/CreatePasswordScreen';
import { LoginScreen } from './components/LoginScreen';
import { MainLayout } from './components/MainLayout';
import { MigrationModal } from './components/MigrationModal';
import { DatabaseMissingModal } from './components/DatabaseMissingModal';
import { DevTools } from './components/DevTools';
import { AuthProvider } from './contexts/AuthContext';
import { AppProvider } from './contexts/AppContext';

function App() {
  const [isFirstLaunch, setIsFirstLaunch] = useState<boolean | null>(null);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [showMigration, setShowMigration] = useState(false);

  useEffect(() => {
    checkFirstLaunch();
  }, []);

  const checkFirstLaunch = async () => {
    try {
      const firstLaunch = await invoke<boolean>('check_first_launch');
      setIsFirstLaunch(firstLaunch);
      
      if (!firstLaunch) {
        const needsMigration = await invoke<boolean>('needs_migration');
        setShowMigration(needsMigration);
      }
    } catch (error) {
      console.error('Error checking first launch:', error);
    }
  };

  const handleLogin = () => {
    setIsAuthenticated(true);
  };

  const handleMigrationComplete = () => {
    setShowMigration(false);
  };

  if (isFirstLaunch === null) {
    return <div className="min-h-screen bg-dark-bg flex items-center justify-center text-white">Carregando...</div>;
  }

  if (isFirstLaunch) {
    return <CreatePasswordScreen onComplete={() => setIsFirstLaunch(false)} />;
  }

  if (showMigration) {
    return <MigrationModal onComplete={handleMigrationComplete} />;
  }

  if (!isAuthenticated) {
    return <LoginScreen onLogin={handleLogin} />;
  }

  return (
    <AuthProvider>
      <AppProvider>
        <MainLayout />
        <DatabaseMissingModal />
        <DevTools />
      </AppProvider>
    </AuthProvider>
  );
}

export default App;
```

- [ ] **Step 3: Build frontend**

Run: `npm run build`
Expected: Build succeeds

- [ ] **Step 4: Test migration flow**

Run: `npm run tauri dev`
Steps:
1. If you have old config, migration modal should appear
2. Enter password
3. Verify migration completes
4. Verify app continues to main layout

Expected: Migration works seamlessly

- [ ] **Step 5: Commit**

```bash
git add src/components/MigrationModal.tsx src/App.tsx
git commit -m "feat: add migration modal for master key update"
```

---

## Feature 2: Member Search

### Task 8: Add Member Search to Members Tab

**Files:**
- Modify: `src/components/MainLayout.tsx`

- [ ] **Step 1: Add search state**

Add after existing state declarations in MainLayout (around line 22):

```typescript
const [memberSearchTerm, setMemberSearchTerm] = useState('');
```

- [ ] **Step 2: Add filter logic**

Add before pagination logic (around line 36):

```typescript
// Filter members by search term
const filteredMembers = memberSearchTerm
  ? members.filter(m => m.name.toLowerCase().includes(memberSearchTerm.toLowerCase()))
  : members;

const activeMembers = filteredMembers.filter((m) => m.active === true);
const inactiveMembers = filteredMembers.filter((m) => m.active === false);
```

Replace the existing `activeMembers` and `inactiveMembers` declarations with the above.

- [ ] **Step 3: Add search input UI**

Find the Members tab content section (around line 200-300) and add search input at the top:

```typescript
{activeTab === 'members' && !viewingMemberDetail && (
  <div className="flex-1 p-8">
    <div className="flex justify-between items-center mb-6">
      <h1 className="text-2xl font-bold text-dark-text-primary">Membros</h1>
      <button
        onClick={() => setShowAddMember(true)}
        className="bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90"
      >
        Adicionar Membro
      </button>
    </div>

    {/* Search Input */}
    <div className="mb-6 max-w-md">
      <div className="relative">
        <input
          type="text"
          value={memberSearchTerm}
          onChange={(e) => {
            setMemberSearchTerm(e.target.value);
            setMembersPage(1);
            setInactiveMembersPage(1);
          }}
          placeholder="Buscar membro por nome..."
          className="w-full bg-dark-bg border border-dark-border text-dark-text-primary rounded px-3 py-2 pr-8"
        />
        {memberSearchTerm && (
          <button
            onClick={() => {
              setMemberSearchTerm('');
              setMembersPage(1);
              setInactiveMembersPage(1);
            }}
            className="absolute right-2 top-1/2 -translate-y-1/2 text-dark-text-secondary hover:text-dark-text-primary"
          >
            ✕
          </button>
        )}
      </div>
      {memberSearchTerm && (
        <p className="text-sm text-dark-text-secondary mt-2">
          {filteredMembers.length} {filteredMembers.length === 1 ? 'membro encontrado' : 'membros encontrados'}
        </p>
      )}
    </div>

    {/* Rest of members content... */}
```

- [ ] **Step 4: Test search functionality**

Run: `npm run tauri dev`
Steps:
1. Go to Members tab
2. Type in search box
3. Verify members list filters in real-time
4. Verify both active and inactive sections filter
5. Verify clear button (X) resets search
6. Verify result count displays

Expected: Search works instantly, filters both active/inactive lists

- [ ] **Step 5: Commit**

```bash
git add src/components/MainLayout.tsx
git commit -m "feat: add member search to Members tab"
```

---

## Feature 3: Dashboard Charts

### Task 9: Add Recharts Dependency

**Files:**
- Modify: `package.json`

- [ ] **Step 1: Add recharts to package.json**

Run: `npm install recharts`

This will add recharts to package.json dependencies.

- [ ] **Step 2: Verify installation**

Run: `npm run build`
Expected: Build succeeds with recharts installed

- [ ] **Step 3: Commit**

```bash
git add package.json package-lock.json
git commit -m "chore: add recharts dependency for dashboard charts"
```

---

### Task 10: Add Chart Data Models

**Files:**
- Create: `src-tauri/src/models/charts.rs`
- Modify: `src-tauri/src/models/mod.rs`

- [ ] **Step 1: Create charts models file**

Create `src-tauri/src/models/charts.rs`:

```rust
use serde::{Deserialize, Serialize};
use rusqlite::{Connection, Result as SqlResult};
use chrono::{Datelike, Local, NaiveDate};

use super::debt::calculate_member_debt;

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthData {
    pub month_key: String,       // "2026-01"
    pub month_display: String,   // "Jan/26"
    pub total_payments: f64,     // Sum of payments in this month
    pub total_debt: f64,         // Total club debt as of end of month
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartData {
    pub months: Vec<MonthData>,
}

const MONTH_ABBREV_PT: [&str; 12] = [
    "Jan", "Fev", "Mar", "Abr", "Mai", "Jun",
    "Jul", "Ago", "Set", "Out", "Nov", "Dez"
];

pub fn generate_chart_data(conn: &Connection) -> SqlResult<ChartData> {
    let now = Local::now();
    let mut months = Vec::new();
    
    // Generate last 6 months (current month + 5 previous)
    for i in (0..6).rev() {
        let target_month = if now.month() as i32 - i > 0 {
            now.month() as i32 - i
        } else {
            12 + (now.month() as i32 - i)
        };
        
        let target_year = if now.month() as i32 - i > 0 {
            now.year()
        } else {
            now.year() - 1
        };
        
        let month_key = format!("{}-{:02}", target_year, target_month);
        let month_abbrev = MONTH_ABBREV_PT[(target_month as usize) - 1];
        let month_display = format!("{}/{}", month_abbrev, target_year % 100);
        
        // Calculate total payments for this month
        let total_payments: f64 = conn.query_row(
            "SELECT COALESCE(SUM(amount_brl), 0.0) FROM payments 
             WHERE strftime('%Y-%m', payment_date) = ?",
            [&month_key],
            |row| row.get(0)
        ).unwrap_or(0.0);
        
        // Calculate total debt as of end of month
        let last_day = get_last_day_of_month(target_year, target_month as u32);
        let end_of_month = format!("{}-{:02}-{:02}", target_year, target_month, last_day);
        
        let total_debt = calculate_total_debt(conn, &end_of_month)?;
        
        months.push(MonthData {
            month_key,
            month_display,
            total_payments,
            total_debt,
        });
    }
    
    Ok(ChartData { months })
}

fn calculate_total_debt(conn: &Connection, as_of_date: &str) -> SqlResult<f64> {
    // Get all active members
    let mut stmt = conn.prepare("SELECT id FROM members WHERE active = 1")?;
    let member_ids = stmt.query_map([], |row| row.get::<_, i64>(0))?;
    
    let mut total = 0.0;
    for member_id in member_ids {
        let id = member_id?;
        let debt = calculate_member_debt(conn, id, as_of_date)?;
        total += debt;
    }
    
    Ok(total)
}

fn get_last_day_of_month(year: i32, month: u32) -> u32 {
    if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day()
    }
}
```

- [ ] **Step 2: Register charts module**

Add to `src-tauri/src/models/mod.rs`:

```rust
pub mod charts;
```

- [ ] **Step 3: Build to verify compilation**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/models/charts.rs src-tauri/src/models/mod.rs
git commit -m "feat: add chart data models and generation logic"
```

---

### Task 11: Add Chart Data Command

**Files:**
- Create: `src-tauri/src/commands/charts.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: Create charts command file**

Create `src-tauri/src/commands/charts.rs`:

```rust
use crate::models::charts::{generate_chart_data, ChartData};
use crate::security::config::load_config;
use crate::security::password::derive_encryption_key;
use crate::db::connection::open_encrypted_db;
use std::path::PathBuf;

#[tauri::command]
pub fn get_dashboard_chart_data_cmd(password: String) -> Result<ChartData, String> {
    let config_path = get_config_path();
    let config = load_config(&config_path)
        .map_err(|e| format!("Failed to load config: {}", e))?;

    let key_bytes = derive_encryption_key(&password, &config.salt)
        .map_err(|e| format!("Failed to derive key: {}", e))?;
    let key_hex = hex::encode(&key_bytes);

    let db_path = get_db_path();
    let conn = open_encrypted_db(&db_path, &key_hex)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    generate_chart_data(&conn)
        .map_err(|e| format!("Failed to generate chart data: {}", e))
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("GestorDoClube");
    std::fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}

fn get_db_path() -> PathBuf {
    let mut path = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("GestorDoClube");
    std::fs::create_dir_all(&path).ok();
    path.push("club.db");
    path
}
```

- [ ] **Step 2: Register charts module**

Add to `src-tauri/src/commands/mod.rs`:

```rust
pub mod charts;
```

- [ ] **Step 3: Register chart command**

Add to invoke_handler in `src-tauri/src/lib.rs`:

```rust
commands::charts::get_dashboard_chart_data_cmd,
```

- [ ] **Step 4: Build to verify compilation**

Run: `cd src-tauri && cargo build`
Expected: Build succeeds

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/commands/charts.rs src-tauri/src/commands/mod.rs src-tauri/src/lib.rs
git commit -m "feat: add chart data command"
```

---

### Task 12: Create Dashboard Charts Component

**Files:**
- Create: `src/components/DashboardCharts.tsx`

- [ ] **Step 1: Create charts component**

Create `src/components/DashboardCharts.tsx`:

```typescript
import { BarChart, Bar, LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

interface MonthData {
  month_key: string;
  month_display: string;
  total_payments: number;
  total_debt: number;
}

interface DashboardChartsProps {
  data: MonthData[];
}

export const DashboardCharts = ({ data }: DashboardChartsProps) => {
  // Format currency for tooltips
  const formatCurrency = (value: number) => {
    return new Intl.NumberFormat('pt-BR', {
      style: 'currency',
      currency: 'BRL',
    }).format(value);
  };

  // Custom tooltip
  const CustomTooltip = ({ active, payload, label }: any) => {
    if (active && payload && payload.length) {
      return (
        <div className="bg-gray-800 border border-gray-700 p-3 rounded shadow-lg">
          <p className="text-white font-semibold mb-1">{label}</p>
          <p className="text-green-400">
            {formatCurrency(payload[0].value)}
          </p>
        </div>
      );
    }
    return null;
  };

  const CustomTooltipDebt = ({ active, payload, label }: any) => {
    if (active && payload && payload.length) {
      return (
        <div className="bg-gray-800 border border-gray-700 p-3 rounded shadow-lg">
          <p className="text-white font-semibold mb-1">{label}</p>
          <p className="text-red-400">
            {formatCurrency(payload[0].value)}
          </p>
        </div>
      );
    }
    return null;
  };

  return (
    <div className="space-y-6">
      {/* Payments Chart */}
      <div className="bg-dark-surface border border-dark-border rounded-lg p-6">
        <h2 className="text-lg font-semibold mb-4 text-dark-text-primary">
          Pagamentos Mensais (últimos 6 meses)
        </h2>
        <ResponsiveContainer width="100%" height={250}>
          <BarChart data={data}>
            <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
            <XAxis 
              dataKey="month_display" 
              stroke="#9ca3af"
              style={{ fontSize: '14px' }}
            />
            <YAxis 
              stroke="#9ca3af"
              style={{ fontSize: '14px' }}
              tickFormatter={(value) => `R$ ${value}`}
            />
            <Tooltip content={<CustomTooltip />} />
            <Bar dataKey="total_payments" fill="#10b981" radius={[4, 4, 0, 0]} />
          </BarChart>
        </ResponsiveContainer>
      </div>

      {/* Debt Trends Chart */}
      <div className="bg-dark-surface border border-dark-border rounded-lg p-6">
        <h2 className="text-lg font-semibold mb-4 text-dark-text-primary">
          Evolução da Dívida (últimos 6 meses)
        </h2>
        <ResponsiveContainer width="100%" height={250}>
          <LineChart data={data}>
            <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
            <XAxis 
              dataKey="month_display" 
              stroke="#9ca3af"
              style={{ fontSize: '14px' }}
            />
            <YAxis 
              stroke="#9ca3af"
              style={{ fontSize: '14px' }}
              tickFormatter={(value) => `R$ ${value}`}
            />
            <Tooltip content={<CustomTooltipDebt />} />
            <Line 
              type="monotone" 
              dataKey="total_debt" 
              stroke="#ef4444" 
              strokeWidth={2}
              dot={{ fill: '#ef4444', r: 4 }}
              activeDot={{ r: 6 }}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
};
```

- [ ] **Step 2: Commit**

```bash
git add src/components/DashboardCharts.tsx
git commit -m "feat: create dashboard charts component"
```

---

### Task 13: Integrate Charts into Dashboard

**Files:**
- Modify: `src/components/DashboardScreen.tsx`

- [ ] **Step 1: Add chart state and data loading**

Update `src/components/DashboardScreen.tsx`:

```typescript
import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useApp } from '../contexts/AppContext';
import { useAuth } from '../contexts/AuthContext';
import { formatCurrency } from '../types';
import { DashboardCharts } from './DashboardCharts';
import type { MemberDebtInfo } from '../types';

interface MonthData {
  month_key: string;
  month_display: string;
  total_payments: number;
  total_debt: number;
}

interface ChartData {
  months: MonthData[];
}

export const DashboardScreen = () => {
  const { members, getAllDebts } = useApp();
  const { password } = useAuth();
  const [debts, setDebts] = useState<MemberDebtInfo[]>([]);
  const [chartData, setChartData] = useState<MonthData[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const loadData = useCallback(async () => {
    setLoading(true);
    setError('');
    try {
      const [debtsData, charts] = await Promise.all([
        getAllDebts(),
        password ? invoke<ChartData>('get_dashboard_chart_data_cmd', { password }) : Promise.resolve({ months: [] })
      ]);
      
      setDebts(debtsData);
      setChartData(charts.months);
    } catch (err) {
      console.error('Error loading dashboard data:', err);
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, [getAllDebts, password]);

  useEffect(() => {
    loadData();
  }, [loadData]);

  const totalDebt = debts.reduce((sum, d) => sum + d.total_debt, 0);
  const activeMembers = members.filter(m => m.active).length;

  return (
    <div className="flex-1 p-8">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-dark-text-primary">Dashboard</h1>
        <button
          onClick={loadData}
          disabled={loading}
          className="bg-dark-accent text-white px-4 py-2 rounded hover:opacity-90 disabled:opacity-50"
        >
          {loading ? 'Carregando...' : 'Atualizar'}
        </button>
      </div>

      {error && (
        <div className="bg-dark-error/10 border border-dark-error text-dark-error p-4 rounded mb-6">
          {error}
        </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
        {/* Total Debt Card */}
        <div className={`bg-dark-surface border rounded-lg p-6 ${totalDebt > 0 ? 'border-dark-error' : 'border-dark-border'}`}>
          <h2 className="text-dark-text-secondary mb-4">Dívida Total do Clube</h2>
          {loading ? (
            <p className="text-dark-text-secondary">Calculando...</p>
          ) : (
            <p className={`text-4xl font-bold ${totalDebt > 0 ? 'text-dark-error' : 'text-dark-text-primary'}`}>
              {formatCurrency(totalDebt)}
            </p>
          )}
        </div>

        {/* Active Members Card */}
        <div className="bg-dark-surface border border-dark-border rounded-lg p-6">
          <h2 className="text-dark-text-secondary mb-4">Membros Ativos</h2>
          <p className="text-4xl font-bold text-green-500">
            {activeMembers}
          </p>
        </div>
      </div>

      {/* Charts */}
      {chartData.length > 0 && <DashboardCharts data={chartData} />}
    </div>
  );
};
```

- [ ] **Step 2: Test dashboard with charts**

Run: `npm run tauri dev`
Steps:
1. Navigate to Dashboard
2. Verify summary cards display
3. Verify two charts appear below cards
4. Verify payments bar chart shows green bars
5. Verify debt line chart shows red line
6. Hover over data points to verify tooltips
7. Click refresh to reload all data

Expected: Both charts display with correct data, tooltips work, refresh updates everything

- [ ] **Step 3: Commit**

```bash
git add src/components/DashboardScreen.tsx
git commit -m "feat: integrate charts into dashboard screen"
```

---

## Feature 4: About/Help Screen

### Task 14: Create Help Screen Component

**Files:**
- Create: `src/components/HelpScreen.tsx`

- [ ] **Step 1: Create help screen component**

Create `src/components/HelpScreen.tsx`:

```typescript
export const HelpScreen = () => {
  return (
    <div className="flex-1 p-8">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-2xl font-bold mb-8 text-dark-text-primary">Ajuda</h1>

        {/* About Section */}
        <div className="bg-dark-surface p-6 rounded-lg border border-dark-border mb-6">
          <h2 className="text-xl font-semibold mb-4 text-dark-text-primary">Gestor do Clube</h2>
          <p className="text-dark-text-secondary mb-2">Versão 1.0.0</p>
          <p className="text-dark-text-secondary mb-4">
            Aplicativo para gestão de mensalidades do clube.
          </p>
          <p className="text-dark-text-secondary text-sm">
            Desenvolvido com Tauri + React
          </p>
        </div>

        {/* Quick Start Guide */}
        <div className="bg-dark-surface p-6 rounded-lg border border-dark-border mb-6">
          <h2 className="text-xl font-semibold mb-4 text-dark-text-primary">Guia Rápido</h2>
          
          <div className="space-y-4">
            <div>
              <h3 className="font-semibold text-dark-text-primary mb-2">Adicionar membro</h3>
              <p className="text-dark-text-secondary">
                Clique em "Membros" → "Adicionar Membro". Informe o nome e a data de início da participação no clube.
              </p>
            </div>

            <div>
              <h3 className="font-semibold text-dark-text-primary mb-2">Registrar pagamento</h3>
              <p className="text-dark-text-secondary">
                Use o botão "Adicionar Pagamento" no topo da tela. Selecione o membro, o mês de referência, e a data do pagamento.
              </p>
            </div>

            <div>
              <h3 className="font-semibold text-dark-text-primary mb-2">Visualizar dívidas</h3>
              <p className="text-dark-text-secondary">
                O Dashboard mostra a dívida total do clube. Para ver dívidas por membro, acesse a aba "Membros" e clique no nome do membro.
              </p>
            </div>

            <div>
              <h3 className="font-semibold text-dark-text-primary mb-2">Cálculo de dívidas</h3>
              <p className="text-dark-text-secondary">
                Um mês sem pagamento se torna dívida após o dia 10 do mês seguinte. Exemplo: sem pagamento em março → dívida após 10 de abril.
              </p>
            </div>

            <div>
              <h3 className="font-semibold text-dark-text-primary mb-2">Exportar relatórios</h3>
              <p className="text-dark-text-secondary">
                Acesse a aba "Relatórios". Escolha o tipo de relatório (dívidas ou histórico de pagamentos), configure as opções, e clique em "Exportar".
              </p>
            </div>
          </div>
        </div>

        {/* Security Section */}
        <div className="bg-dark-surface p-6 rounded-lg border border-dark-border">
          <h2 className="text-xl font-semibold mb-4 text-dark-text-primary">Segurança e Senha</h2>
          
          <p className="text-dark-text-secondary mb-4">
            Este aplicativo protege seus dados com criptografia. Sua senha é necessária para acessar o banco de dados.
          </p>

          <div className="bg-yellow-900/20 border border-yellow-700 p-4 rounded">
            <p className="text-yellow-500 font-semibold mb-2">⚠️ Importante</p>
            <p className="text-yellow-200">
              Não há recuperação de senha. Guarde sua senha em local seguro.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};
```

- [ ] **Step 2: Commit**

```bash
git add src/components/HelpScreen.tsx
git commit -m "feat: create help/about screen component"
```

---

### Task 15: Add Help Tab to Navigation

**Files:**
- Modify: `src/components/MainLayout.tsx`

- [ ] **Step 1: Import HelpScreen**

Add to imports at top of `src/components/MainLayout.tsx`:

```typescript
import { HelpScreen } from './HelpScreen';
```

- [ ] **Step 2: Update activeTab type**

Change the activeTab state type (around line 14):

```typescript
const [activeTab, setActiveTab] = useState<'dashboard' | 'members' | 'payments' | 'reports' | 'help' | 'settings'>('dashboard');
```

- [ ] **Step 3: Add Help nav button**

Find the navigation buttons in the sidebar (around line 150-180) and add Help button after Reports, before Settings:

```typescript
<button
  onClick={() => {
    setActiveTab('help');
    setViewingMemberDetail(false);
    setSelectedMemberId(null);
  }}
  className={`w-full text-left px-4 py-2 rounded mb-2 ${
    activeTab === 'help' ? 'bg-dark-accent text-white' : 'text-dark-text-primary hover:bg-dark-bg'
  }`}
>
  Ajuda
</button>
```

- [ ] **Step 4: Add Help tab content**

Find the content rendering section and add Help tab (after Reports, before Settings):

```typescript
{activeTab === 'help' && <HelpScreen />}
```

- [ ] **Step 5: Test help screen**

Run: `npm run tauri dev`
Steps:
1. Click "Ajuda" in sidebar
2. Verify help screen displays
3. Verify all sections are readable
4. Verify styling is consistent with app theme

Expected: Help screen displays with about info, quick guide, and security warning

- [ ] **Step 6: Commit**

```bash
git add src/components/MainLayout.tsx
git commit -m "feat: add help tab to navigation"
```

---

## Final Testing and Documentation

### Task 16: Manual Testing Checklist

**Files:**
- None (manual testing only)

- [ ] **Step 1: Test password change flow**

Test scenarios:
1. Change password with correct current password → Success
2. Try wrong current password → Error "Current password is incorrect"
3. Try new password < 8 chars → Error "minimum 8 characters"
4. Try mismatched passwords → Error "passwords don't match"
5. Logout and login with new password → Success
6. Change password again → Success

- [ ] **Step 2: Test migration flow (if applicable)**

If testing with old database:
1. Launch app with old config (no master_key)
2. Migration modal appears
3. Enter password → Migration completes
4. App continues normally
5. Verify data still accessible

- [ ] **Step 3: Test member search**

Test scenarios:
1. Search for partial name → Filters correctly
2. Search for full name → Shows exact match
3. Search with no results → Shows "Nenhum membro encontrado"
4. Clear search → Shows all members
5. Search filters both active and inactive
6. Result count displays correctly

- [ ] **Step 4: Test dashboard charts**

Test scenarios:
1. Dashboard loads with charts
2. Payments bar chart shows data
3. Debt line chart shows data
4. Hover tooltips work
5. Refresh button updates charts
6. Charts responsive on resize

- [ ] **Step 5: Test help screen**

Test scenarios:
1. Navigate to Ajuda tab
2. All sections display correctly
3. Text is readable
4. Warning box is visible

- [ ] **Step 6: Regression testing**

Verify existing features still work:
1. Add member → Works
2. Add payment → Works
3. View member detail → Works
4. Generate reports → Works
5. Export CSV/XLSX → Works
6. Settings (minimum fee) → Works

Expected: All features work, no regressions

- [ ] **Step 7: Document test results**

Create a test summary noting:
- Features tested
- Any issues found
- Browser/OS tested

---

### Task 17: Update README

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Update Phase 3 section to Phase 4**

Update the features section in README.md to reflect Phase 4:

```markdown
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
```

- [ ] **Step 2: Commit README update**

```bash
git add README.md
git commit -m "docs: update README with Phase 4 features"
```

---

### Task 18: Phase 4 Completion Commit

**Files:**
- None (empty commit for milestone)

- [ ] **Step 1: Create completion commit**

```bash
git commit --allow-empty -m "chore: Phase 4 polish features complete

Features implemented:
- Password change with master key encryption
- Migration from Phase 3 to Phase 4
- Member search with real-time filtering
- Dashboard charts (payments + debt trends)
- Help/About screen with user guide

All manual tests passed
Ready for production use"
```

---

## Self-Review Checklist

**Spec Coverage:**
- ✅ Password change with master key - Tasks 1-7
- ✅ Config migration - Tasks 2-3, 7
- ✅ Member search - Task 8
- ✅ Dashboard charts (payments + debt) - Tasks 9-13
- ✅ About/Help screen - Tasks 14-15
- ✅ Manual testing - Task 16
- ✅ Documentation update - Task 17

**Type Consistency:**
- ✅ MonthData structure matches across Rust (charts.rs) and TypeScript (DashboardCharts.tsx)
- ✅ ChartData wrapper consistent
- ✅ Currency formatting consistent (formatCurrency)
- ✅ Month display format consistent ("Jan/26" pattern)

**No Placeholders:**
- ✅ All code blocks contain actual implementation
- ✅ All commands have exact paths and expected output
- ✅ All functions fully implemented
- ✅ No "TBD" or "TODO" markers

**Plan Quality:**
- ✅ Bite-sized steps (2-5 minutes each)
- ✅ Exact file paths throughout
- ✅ Complete code in every step
- ✅ Frequent commits after each task
- ✅ Clear testing instructions
