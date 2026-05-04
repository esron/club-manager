#[cfg(test)]
mod tests {
    use gestor_do_clube_lib::security::password::{hash_password, verify_password, derive_encryption_key};

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
}
