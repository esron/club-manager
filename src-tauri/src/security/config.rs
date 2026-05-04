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
