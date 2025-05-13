#!/bin/bash

# This script initializes the SQLite database with the required schema
# It's useful when we need to create tables before the application starts

# Get the database path from environment or use default
if [ -z "$DATABASE_URL" ]; then
  DB_PATH="/app/data/data.db"
else
  DB_PATH=${DATABASE_URL#sqlite:}
fi

echo "Initializing database at $DB_PATH"

# Make sure the database file exists
touch "$DB_PATH"
chmod 666 "$DB_PATH"

# Create tables
sqlite3 "$DB_PATH" <<EOF
-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create progress table to track user progress through lessons
CREATE TABLE IF NOT EXISTS progress (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    lesson_id TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0,
    completed_at TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id),
    UNIQUE (user_id, lesson_id)
);

-- Create sessions table
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    data TEXT,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

-- Create a demo user for testing
INSERT OR IGNORE INTO users (id, username, email, password_hash)
VALUES ('demo-user-id', 'demo', 'demo@example.com', 'dummy_hash');

-- Display schema when done
.schema
EOF

echo "Database initialization completed"
