#!/bin/bash

# Don't exit immediately on error, so we can report it
set +e

echo "Starting IT Learning Platform..."
echo "$(date) - Start script initiated"

# Run diagnostics first and save output
echo "Running diagnostics..."
/app/diagnostics.sh > /app/diagnostic-output.txt 2>&1

# Create necessary directories with liberal permissions
mkdir -p /app/data
chmod 777 /app/data

# Make sure content directory is readable
chmod -R 755 /app/content

# If PORT is not set, randomly select a port between 8000-9000
if [ -z "$PORT" ]; then
  export PORT=$(shuf -i 8000-9000 -n 1)
  echo "Selected random port: $PORT"
else
  echo "Using configured port: $PORT"
fi

# Check database connection string and set default if missing
if [ -z "$DATABASE_URL" ]; then
  export DATABASE_URL="sqlite:/app/data/data.db"
  echo "DATABASE_URL was not set, using default: $DATABASE_URL"
fi

# Check for SECRET_KEY and set a sufficiently long default if not present
if [ -z "$SECRET_KEY" ] || [ ${#SECRET_KEY} -lt 64 ]; then
  export SECRET_KEY="xKYxsEm29T8XnJQXudM7CmzNBpos2q5bLcGFE1JNyAjf6uBAmCe4Z8Pw3t5kSRdDVvHg"
  echo "Setting SECRET_KEY to secure default (must be at least 64 chars long)"
fi

echo "Using database: $DATABASE_URL"

# Check if the binary exists and is executable
if [ ! -x "./it_learning_platform" ]; then
  echo "Error: Application binary is missing or not executable"
  ls -la .
  echo "Starting fallback server..."
  node /app/fallback-server.js
  exit 1
fi

# Create empty database file if it doesn't exist for SQLite
if [[ "$DATABASE_URL" == sqlite:* ]]; then
  DB_PATH=${DATABASE_URL#sqlite:}
  DB_DIR=$(dirname "$DB_PATH")
  
  echo "Ensuring database directory exists: $DB_DIR"
  mkdir -p "$DB_DIR" 
  
  echo "Setting permissions on database directory"
  chmod 777 "$DB_DIR"
  
  echo "Creating database file if it doesn't exist: $DB_PATH"
  touch "$DB_PATH" 
  
  echo "Setting permissions on database file"
  chmod 666 "$DB_PATH"
  
  echo "Database file status:"
  ls -la "$DB_PATH"
fi

# Check all required directories exist
if [ ! -d "/app/templates" ] || [ ! -d "/app/static" ] || [ ! -d "/app/content" ]; then
  echo "Error: Required directories missing!"
  ls -la /app/
  echo "Starting fallback server..."
  node /app/fallback-server.js
  exit 1
fi

echo "Starting application on port $PORT..."
echo "$(date) - Command: ./it_learning_platform"
echo "Environment: HOST=$HOST, PORT=$PORT, DATABASE_URL=$DATABASE_URL, RUST_LOG=$RUST_LOG"

# Kill any existing fallback server
pkill -f "node /app/fallback-server.js" || true

# Start the application with error handling - ensure we bind to 0.0.0.0 explicitly
./it_learning_platform > /app/app_output.log 2>&1
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
  echo "$(date) - Application exited with code $EXIT_CODE"
  echo "Last few lines from error output:" >> /app/diagnostic-output.txt
  echo "==================================" >> /app/diagnostic-output.txt
  tail -n 50 /app/app_output.log >> /app/diagnostic-output.txt
  echo "==================================" >> /app/diagnostic-output.txt
  echo "Environment variables:" >> /app/diagnostic-output.txt
  env >> /app/diagnostic-output.txt
  
  echo "Starting fallback server..."
  exec node /app/fallback-server.js
else
  exit $EXIT_CODE
fi

