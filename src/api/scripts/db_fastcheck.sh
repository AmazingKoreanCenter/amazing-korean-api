# scripts/db_fastcheck.sh
#!/bin/bash

# --- Configuration ---
PG_CONTAINER_NAME="amk-pg"
DB_USER="postgres"
DB_PASSWORD="postgres"
DB_NAME="amazing_korean_db"
SERVER_HOST="localhost"
SERVER_PORT="3000"
HEALTH_CHECK_PATH="/videos/health" # Or /health if /videos/health is not yet implemented
SERVER_STARTUP_TIMEOUT=10 # seconds
DB_CONNECT_TIMEOUT=5 # seconds

# --- Colors ---
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}--- Amazing Korean DB & Server Fast Check ---${NC}"

# 1) Docker Daemon/Container Status Check
echo -e "${YELLOW}1. Checking Docker daemon status...${NC}"
if ! docker info > /dev/null 2>&1; then
    echo -e "${RED}Error: Docker daemon is not running. Please start Docker.${NC}"
    exit 1
fi
echo -e "${GREEN}Docker daemon is running.${NC}"

echo -e "${YELLOW}2. Checking PostgreSQL container status...${NC}"
CONTAINER_ID=$(docker ps -a --filter "name=${PG_CONTAINER_NAME}" --format "{{.ID}}")

if [ -z "$CONTAINER_ID" ]; then
    echo -e "${YELLOW}Container '${PG_CONTAINER_NAME}' not found. Searching for other 'postgres' containers...${NC}"
    CONTAINER_ID=$(docker ps -a --filter "ancestor=postgres" --format "{{.ID}}" | head -n 1)
    if [ -z "$CONTAINER_ID" ]; then
        echo -e "${RED}Error: No PostgreSQL container found. Please ensure 'amk-pg' or another 'postgres' container is created.${NC}"
        exit 1
    else
        PG_CONTAINER_NAME=$(docker ps -a --filter "id=${CONTAINER_ID}" --format "{{.Names}}")
        echo -e "${YELLOW}Found PostgreSQL container: '${PG_CONTAINER_NAME}'. Using this one.${NC}"
    fi
fi

CONTAINER_STATUS=$(docker inspect -f '{{.State.Status}}' "$CONTAINER_ID")

if [ "$CONTAINER_STATUS" == "exited" ]; then
    echo -e "${YELLOW}Container '${PG_CONTAINER_NAME}' is stopped. Starting it...${NC}"
    if ! docker start "$CONTAINER_ID" > /dev/null; then
        echo -e "${RED}Error: Failed to start container '${PG_CONTAINER_NAME}'. Check docker logs for details.${NC}"
        docker logs "$CONTAINER_ID" | tail -n 20
        exit 1
    fi
    echo -e "${GREEN}Container '${PG_CONTAINER_NAME}' started.${NC}"
elif [ "$CONTAINER_STATUS" == "running" ]; then
    echo -e "${GREEN}Container '${PG_CONTAINER_NAME}' is already running.${NC}"
else
    echo -e "${RED}Error: Container '${PG_CONTAINER_NAME}' is in an unexpected state: ${CONTAINER_STATUS}.${NC}"
    exit 1
fi

# 3) Host Port Mapping Extraction
echo -e "${YELLOW}3. Extracting host port mapping for PostgreSQL...${NC}"
HOST_PORT=$(docker inspect --format='{{(index (index .NetworkSettings.Ports "5432/tcp") 0).HostPort}}' "$CONTAINER_ID" 2>/dev/null)

if [ -z "$HOST_PORT" ]; then
    echo -e "${YELLOW}Could not find host port mapping via 'docker inspect'. Trying 'docker compose port'...${NC}"
    # Assuming docker-compose.yml is in the parent directory of scripts/
    # Adjusting path for docker-compose.yml based on new script location
    COMPOSE_DIR=$(dirname "$(dirname "$(dirname "$(realpath "$0")")")") # Go up 3 levels from src/api/scripts
    if [ -f "${COMPOSE_DIR}/docker-compose.yml" ]; then
        HOST_PORT=$(docker compose -f "${COMPOSE_DIR}/docker-compose.yml" port "$PG_CONTAINER_NAME" 5432 | cut -d ':' -f 2)
    fi
fi

if [ -z "$HOST_PORT" ]; then
    echo -e "${RED}Error: Could not determine host port for PostgreSQL container '${PG_CONTAINER_NAME}'. Ensure port 5432 is mapped.${NC}"
    echo -e "${RED}Suggestion: Recreate the container with a port mapping (e.g., -p 5432:5432).${NC}"
    exit 1
fi
echo -e "${GREEN}PostgreSQL is mapped to host port: ${HOST_PORT}${NC}"

# 4) DATABASE_URL Auto-reconfiguration
DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${SERVER_HOST}:${HOST_PORT}/${DB_NAME}?connect_timeout=${DB_CONNECT_TIMEOUT}"
echo -e "${GREEN}Generated DATABASE_URL: postgres://${DB_USER}:*****@${SERVER_HOST}:${HOST_PORT}/${DB_NAME}?connect_timeout=${DB_CONNECT_TIMEOUT}${NC}"

# 5) psql Connection Test
echo -e "${YELLOW}5. Testing PostgreSQL connection from host...${NC}"
if PGPASSWORD="${DB_PASSWORD}" psql -h "${SERVER_HOST}" -p "${HOST_PORT}" -U "${DB_USER}" -d "${DB_NAME}" -c "SELECT 1" > /dev/null 2>&1; then
    echo -e "${GREEN}Host to DB connection successful!${NC}"
else
    echo -e "${RED}Error: Host to DB connection failed!${NC}"
    echo -e "${RED}Please check DB credentials, host, port, or firewall rules.${NC}"
    echo -e "${BLUE}Last 20 lines of docker logs for '${PG_CONTAINER_NAME}':${NC}"
    docker logs "$PG_CONTAINER_NAME" | tail -n 20
    exit 1
fi

echo -e "${YELLOW}6. Testing PostgreSQL connection inside container (optional, for deeper diagnostics)...${NC}"
if docker exec "$CONTAINER_ID" psql -U "${DB_USER}" -d "${DB_NAME}" -c "SELECT 1" > /dev/null 2>&1; then
    echo -e "${GREEN}Container internal DB connection successful!${NC}"
else
    echo -e "${YELLOW}Warning: Container internal DB connection failed. This might indicate an issue within the container itself.${NC}"
    echo -e "${BLUE}Last 20 lines of docker logs for '${PG_CONTAINER_NAME}':${NC}"
    docker logs "$PG_CONTAINER_NAME" | tail -n 20
fi

# 7) Server Startup (Fast Fail Mode)
echo -e "${YELLOW}7. Starting server with fast-fail mode and checking health endpoint...${NC}"
SERVER_LOG_FILE="/tmp/amazing_korean_api_server.log"
export DATABASE_URL="$DATABASE_URL" # Export for cargo run

# Ensure previous server instances are stopped
pkill -f "cargo run" > /dev/null 2>&1

# Start server in background
# Adjusting path for cargo run based on new script location
(cd "$(dirname "$(dirname "$(dirname "$(realpath "$0")")")")" && cargo run > "$SERVER_LOG_FILE" 2>&1) &
SERVER_PID=$!
echo -e "${BLUE}Server started with PID ${SERVER_PID}. Logs are being written to ${SERVER_LOG_FILE}${NC}"

# Wait for server to be ready
echo -e "${YELLOW}Waiting up to ${SERVER_STARTUP_TIMEOUT} seconds for server to respond to ${HEALTH_CHECK_PATH}...${NC}"
START_TIME=$(date +%s)
while true; do
    CURRENT_TIME=$(date +%s)
    ELAPSED_TIME=$((CURRENT_TIME - START_TIME))

    if [ "$ELAPSED_TIME" -ge "$SERVER_STARTUP_TIMEOUT" ]; then
        echo -e "${RED}Error: Server did not respond within ${SERVER_STARTUP_TIMEOUT} seconds.${NC}"
        kill "$SERVER_PID" > /dev/null 2>&1
        echo -e "${BLUE}Last 200 lines of server logs from ${SERVER_LOG_FILE}:${NC}"
        tail -n 200 "$SERVER_LOG_FILE"
        exit 1
    fi

    if curl -sS "http://${SERVER_HOST}:${SERVER_PORT}${HEALTH_CHECK_PATH}" > /dev/null; then
        echo -e "${GREEN}Server is listening and responded to ${HEALTH_CHECK_PATH}!${NC}"
        kill "$SERVER_PID" > /dev/null 2>&1 # Stop the background server
        rm "$SERVER_LOG_FILE"
        echo -e "${BLUE}--- Fast Check Complete: SUCCESS ---${NC}"
        exit 0
    fi
    sleep 1
done
