#!/bin/bash

# Find the database file
DB_PATH="$HOME/.local/share/GestorDoClube/club.db"

if [ ! -f "$DB_PATH" ]; then
    echo "Database not found at: $DB_PATH"
    exit 1
fi

echo "Database found at: $DB_PATH"
echo "==================================="
echo "Checking members table:"
echo "==================================="

# Try to query without encryption (will fail if encrypted)
sqlite3 "$DB_PATH" "SELECT id, name, start_date, active, typeof(active) as active_type FROM members;" 2>&1 || echo "Database is encrypted (expected)"

echo ""
echo "Note: The database is encrypted. We need to add a debug command to the Rust code."
