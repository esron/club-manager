use bcrypt::{hash, verify};
use ring::pbkdf2;
use std::num::NonZeroU32;
use thiserror::Error;

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
