#!/bin/bash

# A0-Guardrails: 개발 런타임 ‘무응답 대기’ 방지 장치
# 이 스크립트는 서버 기동 전 환경을 점검하고, 서버가 빠르게 기동되는지 확인합니다.

set -euo pipefail

PORT=3000
SERVER_START_TIMEOUT=10 # seconds

# --- 1. 포트 점유 확인 및 안내 ---
echo "\n--- 1. Checking port $PORT occupancy ---"
if command -v lsof &> /dev/null; then
    # macOS
    if lsof -iTCP:$PORT -sTCP:LISTEN -n -P &> /dev/null; then
        echo "⚠️ Port $PORT is already in use. Please terminate the process manually:"
        lsof -iTCP:$PORT -sTCP:LISTEN -n -P
        exit 1
    else
        echo "✅ Port $PORT is free."
    fi
elif command -v ss &> /dev/null; then
    # Linux
    if ss -lntp | grep :$PORT &> /dev/null; then
        echo "⚠️ Port $PORT is already in use. Please terminate the process manually:"
        ss -lntp | grep :$PORT
        exit 1
    else
        echo "✅ Port $PORT is free."
    fi
else
    echo "ℹ️ Neither 'lsof' nor 'ss' found. Cannot check port occupancy automatically."
    echo "   Please ensure port $PORT is free before proceeding."
fi

# --- 2. 환경 변수 및 DB 연결 확인 ---
echo "\n--- 2. Checking environment variables and DB connection ---"
if ! printenv | grep -E 'DATABASE_URL' &> /dev/null; then
    echo "❌ DATABASE_URL environment variable is not set. Please set it."
    exit 1
fi

if ! command -v psql &> /dev/null; then
    echo "ℹ️ 'psql' command not found. Cannot verify DB connection automatically."
    echo "   Please ensure your PostgreSQL database is running and accessible."
else
    echo "Attempting to connect to PostgreSQL..."
    if psql "$DATABASE_URL" -c \"select 1\" -t -q -P pager=off &> /dev/null; then
        echo "✅ PostgreSQL connection successful."
    else
        echo "❌ Failed to connect to PostgreSQL. Please check your DATABASE_URL and DB server status."
        exit 1
    fi
fi

# --- 3. SQLx 마이그레이션 상태 확인 ---
echo "\n--- 3. Checking SQLx migrations status ---"
if ! command -v sqlx &> /dev/null; then
    echo "ℹ️ 'sqlx' command not found. Cannot check migrations automatically."
    echo "   Please ensure sqlx-cli is installed and in your PATH."
else
    MIGRATION_INFO=$(sqlx migrate info)
    echo "$MIGRATION_INFO"
    if echo "$MIGRATION_INFO" | grep -q "Pending"; then
        echo "⚠️ There are pending migrations. Consider running 'sqlx migrate run'."
    else
        echo "✅ All migrations are applied."
    fi
fi

# --- 4. 서버 기동 및 빠른 실패 확인 ---
echo "\n--- 4. Starting server and checking for quick startup ---"

# Ensure previous background processes are killed if any
# This is a best effort, as the previous `cargo run &` might have already exited
# or been killed by the user. We use `|| true` to prevent `set -e` from exiting.
if command -v lsof &> /dev/null; then
    lsof -iTCP:$PORT -sTCP:LISTEN -n -P | awk 'NR>1 {print $2}' | xargs -r kill -TERM || true
elif command -v fuser &> /dev/null; then
    fuser -k $PORT/tcp || true
fi

# Start the server in the background and capture its output
echo "Starting server with RUST_LOG=info,sqlx=warn..."

# Use `timeout` to kill the `cargo run` process if it doesn't start within SERVER_START_TIMEOUT
# We redirect stderr to stdout to capture all logs for analysis
(timeout $SERVER_START_TIMEOUT bash -c "RUST_LOG=info,sqlx=warn DATABASE_URL=\"$DATABASE_URL\" cargo run 2>&1" & echo $! >&3) 3> server_pid.txt > server_output.log &

SERVER_PID=$(cat server_pid.txt)
rm server_pid.txt

# Wait for the server to log "listening on" or for the timeout
if grep -q -m 1 "listening on" <(tail -f server_output.log &) ; then
    echo "✅ Server started successfully!"
    echo "Server logs:"
    cat server_output.log
else
    echo "❌ Server did not start within $SERVER_START_TIMEOUT seconds."
    echo "Server output:"
    cat server_output.log
    kill -TERM $SERVER_PID || true # Ensure the server process is killed
    exit 1
fi

# --- 5. Health and Readiness Checks ---
echo "\n--- 5. Running Health and Readiness Checks ---"

# Give the server a moment to fully initialize after logging "listening on"
sleep 2

HEALTH_STATUS=$(curl -sS -w '\nHTTP %{http_code}\n' http://localhost:$PORT/health)
READY_STATUS=$(curl -sS -w '\nHTTP %{http_code}\n' http://localhost:$PORT/ready)
VIDEOS_HEALTH_STATUS=$(curl -sS -w '\nHTTP %{http_code}\n' http://localhost:$PORT/videos/health)

echo "/health: $HEALTH_STATUS"
echo "/ready: $READY_STATUS"
echo "/videos/health: $VIDEOS_HEALTH_STATUS"

if [[ "$HEALTH_STATUS" == *"HTTP 200"* && "$READY_STATUS" == *"HTTP 200"* && "$VIDEOS_HEALTH_STATUS" == *"HTTP 200"* ]]; then
    echo "✅ All health and readiness checks passed."
else
    echo "❌ One or more health/readiness checks failed."
    kill -TERM $SERVER_PID || true # Ensure the server process is killed
    exit 1
fi

echo "\n--- Preflight checks complete. Server is running in the background (PID: $SERVER_PID). ---"
echo "To stop the server, run: kill -TERM $SERVER_PID"

# Keep the server running in the background for further manual testing
# The script will exit, but the server process will continue.
# The user can manually kill it using the provided PID.
