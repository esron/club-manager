use rusqlite::{Connection, Result as SqlResult};
use std::path::Path;
use zeroize::Zeroizing;

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

    // Configure SQLCipher - wrap PRAGMA in Zeroizing to prevent key leakage
    let pragma = Zeroizing::new(format!(
        "PRAGMA cipher = 'aes-256-cbc';
         PRAGMA kdf_iter = 100000;
         PRAGMA cipher_page_size = 4096;
         PRAGMA key = \"x'{}'\";",
        key
    ));
    conn.execute_batch(&pragma)?;

    // Test that key is correct by executing a simple query
    conn.query_row("SELECT count(*) FROM sqlite_master", [], |_| Ok(()))?;

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

        conn.execute("CREATE TABLE test (id INTEGER)", [])
            .expect("Failed to create table");
    }
}
