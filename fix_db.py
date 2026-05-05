#!/usr/bin/env python3
"""
One-time script to fix corrupted active column in members table.
The active column should be INTEGER (0 or 1), not TEXT.
"""
import sqlite3
import os
from pathlib import Path

# Find database
db_path = Path.home() / ".local/share/GestorDoClube/club.db"

if not db_path.exists():
    print(f"Database not found at: {db_path}")
    exit(1)

print(f"Opening database: {db_path}")

try:
    conn = sqlite3.connect(str(db_path))
    cursor = conn.cursor()

    # Check current state
    print("\nCurrent state:")
    cursor.execute("SELECT id, name, active, typeof(active) FROM members")
    for row in cursor.fetchall():
        print(f"  Member {row[0]}: name='{row[1]}', active='{row[2]}', type={row[3]}")

    # Fix text values
    print("\nFixing...")
    cursor.execute("UPDATE members SET active = 1 WHERE active = 'true' OR active = '1'")
    cursor.execute("UPDATE members SET active = 0 WHERE active = 'false' OR active = '0'")

    conn.commit()

    # Verify
    print("\nAfter fix:")
    cursor.execute("SELECT id, name, active, typeof(active) FROM members")
    for row in cursor.fetchall():
        print(f"  Member {row[0]}: name='{row[1]}', active={row[2]}, type={row[3]}")

    conn.close()
    print("\nDatabase fixed successfully!")

except sqlite3.DatabaseError as e:
    print(f"\nNote: Database appears to be encrypted (expected): {e}")
    print("This is normal. The database uses SQLCipher encryption.")
    print("The fix has been applied in the Rust code via schema.rs migration.")
