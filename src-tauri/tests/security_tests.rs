#[cfg(test)]
mod tests {
    use gestor_do_clube_lib::security::password::{hash_password, verify_password};

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
