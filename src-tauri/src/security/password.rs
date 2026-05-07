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

    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Invalid encrypted data")]
    InvalidEncryptedData,
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
pub fn encrypt_master_key(master_key: &[u8; 32], password: &str, salt: &[u8]) -> Result<Vec<u8>, PasswordError> {
    // Derive encryption key from password
    let key_bytes = derive_encryption_key(password, salt)?;

    // Create cipher
    let cipher = Aes256Gcm::new(key_bytes.as_slice().into());

    // Generate random nonce (12 bytes for AES-GCM)
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt
    let ciphertext = cipher.encrypt(nonce, master_key.as_ref())
        .map_err(|e| PasswordError::EncryptionFailed(e.to_string()))?;

    // Prepend nonce to ciphertext
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypt master key with password-derived key
pub fn decrypt_master_key(encrypted_data: &[u8], password: &str, salt: &[u8]) -> Result<[u8; 32], PasswordError> {
    // Check minimum size: 12 bytes nonce + 32 bytes key + 16 bytes auth tag = 60 bytes
    if encrypted_data.len() != 60 {
        return Err(PasswordError::InvalidEncryptedData);
    }

    // Derive encryption key from password
    let key_bytes = derive_encryption_key(password, salt)?;

    // Create cipher
    let cipher = Aes256Gcm::new(key_bytes.as_slice().into());

    // Extract nonce and ciphertext
    let nonce = Nonce::from_slice(&encrypted_data[0..12]);
    let ciphertext = &encrypted_data[12..];

    // Decrypt
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| PasswordError::DecryptionFailed(e.to_string()))?;

    if plaintext.len() != 32 {
        return Err(PasswordError::InvalidEncryptedData);
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

    #[test]
    fn test_master_key_round_trip() {
        let master_key = [42u8; 32];
        let password = "test_password";
        let salt = [1u8; 16];

        let encrypted = encrypt_master_key(&master_key, password, &salt).unwrap();
        let decrypted = decrypt_master_key(&encrypted, password, &salt).unwrap();

        assert_eq!(master_key, decrypted);
    }

    #[test]
    fn test_wrong_password_fails() {
        let master_key = [42u8; 32];
        let password = "correct_password";
        let wrong_password = "wrong_password";
        let salt = [1u8; 16];

        let encrypted = encrypt_master_key(&master_key, password, &salt).unwrap();
        let result = decrypt_master_key(&encrypted, wrong_password, &salt);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PasswordError::DecryptionFailed(_)));
    }

    #[test]
    fn test_truncated_ciphertext_fails() {
        let master_key = [42u8; 32];
        let password = "test_password";
        let salt = [1u8; 16];

        let encrypted = encrypt_master_key(&master_key, password, &salt).unwrap();

        // Truncate to less than required length
        let truncated = &encrypted[0..30];
        let result = decrypt_master_key(truncated, password, &salt);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PasswordError::InvalidEncryptedData));
    }

    #[test]
    fn test_tampered_ciphertext_fails() {
        let master_key = [42u8; 32];
        let password = "test_password";
        let salt = [1u8; 16];

        let encrypted = encrypt_master_key(&master_key, password, &salt).unwrap();

        // Tamper with the ciphertext (flip a bit in the encrypted portion)
        let mut tampered = encrypted.clone();
        tampered[20] ^= 0xFF;

        let result = decrypt_master_key(&tampered, password, &salt);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), PasswordError::DecryptionFailed(_)));
    }

    #[test]
    fn test_different_salts_produce_different_ciphertext() {
        let master_key = [42u8; 32];
        let password = "test_password";
        let salt1 = [1u8; 16];
        let salt2 = [2u8; 16];

        let encrypted1 = encrypt_master_key(&master_key, password, &salt1).unwrap();
        let encrypted2 = encrypt_master_key(&master_key, password, &salt2).unwrap();

        // Different salts should produce different encrypted data
        assert_ne!(encrypted1, encrypted2);

        // But both should decrypt correctly with their respective salts
        assert_eq!(decrypt_master_key(&encrypted1, password, &salt1).unwrap(), master_key);
        assert_eq!(decrypt_master_key(&encrypted2, password, &salt2).unwrap(), master_key);

        // Cross-decryption should fail
        assert!(decrypt_master_key(&encrypted1, password, &salt2).is_err());
        assert!(decrypt_master_key(&encrypted2, password, &salt1).is_err());
    }
}
