#[cfg(test)]
mod tests {
    use gestor_do_clube_lib::security::password::{hash_password, encrypt_master_key, decrypt_master_key};
    use gestor_do_clube_lib::security::config::{AppConfig, save_config, load_config};
    use tempfile::TempDir;

    #[test]
    fn test_master_key_encryption_decryption() {
        let master_key: [u8; 32] = [1u8; 32];
        let password = "test_password";
        let salt: [u8; 16] = [1u8; 16];

        let encrypted = encrypt_master_key(&master_key, password, &salt).unwrap();
        let decrypted = decrypt_master_key(&encrypted, password, &salt).unwrap();

        assert_eq!(master_key, *decrypted);
    }

    #[test]
    fn test_master_key_wrong_password() {
        let master_key: [u8; 32] = [1u8; 32];
        let password = "correct_password";
        let wrong_password = "wrong_password";
        let salt: [u8; 16] = [1u8; 16];

        let encrypted = encrypt_master_key(&master_key, password, &salt).unwrap();
        let result = decrypt_master_key(&encrypted, wrong_password, &salt);

        assert!(result.is_err());
    }

    #[test]
    fn test_config_with_master_key() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        let master_key: [u8; 32] = [42u8; 32];
        let password = "test_password";
        let salt: [u8; 16] = [1u8; 16];

        let encrypted_master_key = encrypt_master_key(&master_key, password, &salt).unwrap();

        let config = AppConfig {
            password_hash: hash_password(password).unwrap(),
            salt: salt.to_vec(),
            master_key_encrypted: Some(encrypted_master_key.clone()),
            minimum_fee_brl: "15.00".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        save_config(&config, &config_path).unwrap();
        let loaded_config = load_config(&config_path).unwrap();

        assert_eq!(loaded_config.master_key_encrypted.unwrap(), encrypted_master_key);
    }

    #[test]
    fn test_password_change_workflow() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");

        // Initial setup with old password
        let old_password = "old_password_123";
        let master_key: [u8; 32] = [99u8; 32];
        let salt: [u8; 16] = [5u8; 16];

        let encrypted_master_key = encrypt_master_key(&master_key, old_password, &salt).unwrap();

        let initial_config = AppConfig {
            password_hash: hash_password(old_password).unwrap(),
            salt: salt.to_vec(),
            master_key_encrypted: Some(encrypted_master_key.clone()),
            minimum_fee_brl: "15.00".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        save_config(&initial_config, &config_path).unwrap();

        // Load and verify we can decrypt with old password
        let loaded = load_config(&config_path).unwrap();
        let decrypted_old = decrypt_master_key(
            loaded.master_key_encrypted.as_ref().unwrap(),
            old_password,
            &salt
        ).unwrap();
        assert_eq!(master_key, *decrypted_old);

        // Change password
        let new_password = "new_password_456";
        let new_encrypted_master_key = encrypt_master_key(&master_key, new_password, &salt).unwrap();

        let updated_config = AppConfig {
            password_hash: hash_password(new_password).unwrap(),
            salt: salt.to_vec(),
            master_key_encrypted: Some(new_encrypted_master_key.clone()),
            minimum_fee_brl: "15.00".to_string(),
            created_at: initial_config.created_at.clone(),
        };

        save_config(&updated_config, &config_path).unwrap();

        // Verify we can decrypt with new password
        let loaded_updated = load_config(&config_path).unwrap();
        let decrypted_new = decrypt_master_key(
            loaded_updated.master_key_encrypted.as_ref().unwrap(),
            new_password,
            &salt
        ).unwrap();
        assert_eq!(master_key, *decrypted_new);

        // Verify old password no longer works
        let old_password_attempt = decrypt_master_key(
            loaded_updated.master_key_encrypted.as_ref().unwrap(),
            old_password,
            &salt
        );
        assert!(old_password_attempt.is_err());
    }

    #[test]
    fn test_master_key_persistence_across_password_changes() {
        let _temp_dir = TempDir::new().unwrap();

        // The master key should remain the same across password changes
        let master_key: [u8; 32] = [123u8; 32];
        let password1 = "password_one";
        let password2 = "password_two";
        let salt: [u8; 16] = [7u8; 16];

        // Encrypt with password1
        let encrypted1 = encrypt_master_key(&master_key, password1, &salt).unwrap();
        let decrypted1 = decrypt_master_key(&encrypted1, password1, &salt).unwrap();

        // Encrypt with password2
        let encrypted2 = encrypt_master_key(&master_key, password2, &salt).unwrap();
        let decrypted2 = decrypt_master_key(&encrypted2, password2, &salt).unwrap();

        // Both should decrypt to the same master key
        assert_eq!(master_key, *decrypted1);
        assert_eq!(master_key, *decrypted2);

        // Cross-password decryption should fail
        assert!(decrypt_master_key(&encrypted1, password2, &salt).is_err());
        assert!(decrypt_master_key(&encrypted2, password1, &salt).is_err());
    }

    #[test]
    fn test_encrypted_data_integrity() {
        let master_key: [u8; 32] = [55u8; 32];
        let password = "secure_password";
        let salt: [u8; 16] = [10u8; 16];

        let encrypted = encrypt_master_key(&master_key, password, &salt).unwrap();

        // Each encryption should produce different ciphertext due to random nonce
        let encrypted_again = encrypt_master_key(&master_key, password, &salt).unwrap();
        assert_ne!(encrypted, encrypted_again);

        // But both should decrypt to the same master key
        assert_eq!(*decrypt_master_key(&encrypted, password, &salt).unwrap(), master_key);
        assert_eq!(*decrypt_master_key(&encrypted_again, password, &salt).unwrap(), master_key);
    }

    #[test]
    fn test_tampered_encrypted_data_fails() {
        let master_key: [u8; 32] = [77u8; 32];
        let password = "test_password";
        let salt: [u8; 16] = [12u8; 16];

        let mut encrypted = encrypt_master_key(&master_key, password, &salt).unwrap();

        // Tamper with the ciphertext
        if encrypted.len() > 13 {
            encrypted[13] ^= 0xFF;
        }

        // Should fail to decrypt
        let result = decrypt_master_key(&encrypted, password, &salt);
        assert!(result.is_err());
    }
}
