use bcrypt::{hash, verify};
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
