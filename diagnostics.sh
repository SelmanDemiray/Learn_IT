#!/bin/bash

echo "==== Running IT Learning Platform Diagnostics ===="
echo "Date: $(date)"
echo "Hostname: $(hostname)"
echo "Current directory: $(pwd)"
echo "User: $(whoami)"

echo -e "\n== Environment Variables =="
echo "HOST: $HOST"
echo "PORT: $PORT"
echo "DATABASE_URL: $DATABASE_URL"
echo "RUST_LOG: $RUST_LOG"

echo -e "\n== Directory Structure =="
ls -la /app
echo -e "\nContent directory:"
ls -la /app/content || echo "Content directory not found!"
echo -e "\nTemplates directory:"
ls -la /app/templates || echo "Templates directory not found!"
echo -e "\nStatic directory:"
ls -la /app/static || echo "Static directory not found!"
echo -e "\nData directory:"
ls -la /app/data || echo "Data directory not found!"

echo -e "\n== File Permissions =="
echo "Application executable:"
ls -la /app/it_learning_platform
echo "Start script:"
ls -la /app/start.sh

echo -e "\n== Testing Application Dependencies =="
ldd /app/it_learning_platform || echo "ldd not available"

echo -e "\n== Testing Database Access =="
if command -v sqlite3 &>/dev/null; then
    if [[ "$DATABASE_URL" == sqlite:* ]]; then
        DB_PATH=${DATABASE_URL#sqlite:}
        echo "Testing database at $DB_PATH"
        if [ -f "$DB_PATH" ]; then
            sqlite3 "$DB_PATH" "PRAGMA quick_check;" || echo "Database check failed"
        else
            echo "Database file does not exist"
            touch "$DB_PATH" && echo "Created empty database file"
            chmod 666 "$DB_PATH" && echo "Set database file permissions"
        fi
    else
        echo "Non-SQLite database URL: $DATABASE_URL"
    fi
else
    echo "sqlite3 command not found"
fi

echo -e "\n== Testing Network ===="
curl -s -S -I -m 5 http://localhost:${PORT:-8080} || echo "App not responding on localhost:${PORT:-8080}"

echo -e "\n== System Resources =="
echo "Memory:"
free -h || echo "free command not available"
echo "Disk space:"
df -h || echo "df command not available"

echo "==== Diagnostics Complete ===="
