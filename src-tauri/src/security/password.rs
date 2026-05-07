use bcrypt::{hash, verify};
use ring::pbkdf2;
use std::num::NonZeroU32;
use thiserror::Error;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};

const PBKDF2_ITERATIONS: u32 = 100_000;

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

/// Encrypt master key with password-derived key
pub fn encrypt_master_key(master_key: &[u8; 32], password: &str, salt: &[u8]) -> Result<Vec<u8>, String> {
    // Derive encryption key from password
    let mut key_bytes = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
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
        NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
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
