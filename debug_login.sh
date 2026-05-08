#!/bin/bash

# Diagnostic script to check login issue
echo "=== Login Diagnostic ==="
echo ""
echo "What's your password? (it will be visible)"
read -r PASSWORD

echo ""
echo "Testing with password: $PASSWORD"
echo ""

# Create a temporary Rust program to test
cat > /tmp/test_login.rs << 'EOF'
use std::path::PathBuf;

fn main() {
    let password = std::env::args().nth(1).expect("Need password");

    // Load config
    let mut config_path = dirs::data_local_dir().unwrap();
    config_path.push("GestorDoClube");
    config_path.push("config.json");

    let config_json = std::fs::read_to_string(&config_path).unwrap();
    println!("✓ Config loaded");

    // Parse the password hash
    let config: serde_json::Value = serde_json::from_str(&config_json).unwrap();
    let password_hash = config["password_hash"].as_str().unwrap();

    // Test bcrypt verification
    let is_valid = bcrypt::verify(&password, password_hash).unwrap();
    println!("Bcrypt verification: {}", if is_valid { "✓ PASS" } else { "✗ FAIL - Wrong password!" });

    if !is_valid {
        println!("\nThe password you entered doesn't match the stored hash.");
        println!("This means either:");
        println!("1. You're entering the wrong password");
        println!("2. The password hash was corrupted during migration");
        return;
    }

    println!("\nPassword is correct! Checking master key decryption...");

    // The password is correct, so the issue is with master key or database
    println!("\nNext steps:");
    println!("1. Try to decrypt the master key");
    println!("2. Try to open database with master key");
    println!("3. If that fails, database might still be on old key");
}
EOF

cd src-tauri
rustc /tmp/test_login.rs \
  --edition 2021 \
  --extern bcrypt=target/release/deps/libbcrypt-*.rlib \
  --extern dirs=target/release/deps/libdirs-*.rlib \
  --extern serde_json=target/release/deps/libserde_json-*.rlib \
  -L target/release/deps \
  -o /tmp/test_login 2>&1 | head -20

if [ -f /tmp/test_login ]; then
    /tmp/test_login "$PASSWORD"
else
    echo "Build failed, let me try a simpler approach..."
fi
