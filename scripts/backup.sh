#!/usr/bin/env bash
# DB + Redis 일일 백업 (A4-4 옵션 A 수동 정기 — EC2 측 archive 생성, 사용자 PC scp pull 대상)
#
# 등록 (EC2 ubuntu user crontab):
#   crontab -e
#   0 3 * * * /home/ubuntu/amazing-korean-api/scripts/backup.sh >> /home/ubuntu/backup/backup.log 2>&1
#
# 환경변수:
#   BACKUP_DIR              백업 디렉터리 (기본 $HOME/backup)
#   BACKUP_RETENTION_DAYS   archive 보관 일수 (기본 7)
#   ENV_FILE                .env 파일 경로 (기본 $HOME/amazing-korean-api/.env, REDIS_PASSWORD 로드용)
#
# 산출물: $BACKUP_DIR/amk-YYYYMMDD-HHMMSS.tar.gz (db.sql.gz + redis.rdb)
# 회전: BACKUP_RETENTION_DAYS 초과 archive 자동 삭제

set -euo pipefail

BACKUP_DIR="${BACKUP_DIR:-$HOME/backup}"
RETENTION_DAYS="${BACKUP_RETENTION_DAYS:-7}"
ENV_FILE="${ENV_FILE:-$HOME/amazing-korean-api/.env}"

if [[ -f "$ENV_FILE" ]]; then
  set -a
  # shellcheck disable=SC1090
  . "$ENV_FILE"
  set +a
fi

mkdir -p "$BACKUP_DIR"
TS="$(date +%Y%m%d-%H%M%S)"
ARCHIVE="amk-${TS}.tar.gz"
DB_FILE="db-${TS}.sql.gz"
REDIS_FILE="redis-${TS}.rdb"

log() { echo "[$(date -Iseconds)] $*"; }

log "backup start ts=${TS}"

log "step 1/4 — PostgreSQL pg_dump"
docker exec amk-pg pg_dump -U postgres -d amazing_korean_db \
  --exclude-table=_sqlx_migrations \
  | gzip > "${BACKUP_DIR}/${DB_FILE}"

log "step 2/4 — Redis BGSAVE + LASTSAVE polling"
PRE_LASTSAVE="$(docker exec amk-redis sh -c 'redis-cli -a "$REDIS_PASSWORD" --no-auth-warning LASTSAVE')"
docker exec amk-redis sh -c 'redis-cli -a "$REDIS_PASSWORD" --no-auth-warning BGSAVE' > /dev/null

POLL_MAX=60
for i in $(seq 1 $POLL_MAX); do
  POST_LASTSAVE="$(docker exec amk-redis sh -c 'redis-cli -a "$REDIS_PASSWORD" --no-auth-warning LASTSAVE')"
  if [[ "$POST_LASTSAVE" != "$PRE_LASTSAVE" ]]; then
    log "  BGSAVE complete after ${i}s"
    break
  fi
  if [[ $i -eq $POLL_MAX ]]; then
    log "  WARN: BGSAVE polling timeout (${POLL_MAX}s) — proceeding with current dump.rdb"
  fi
  sleep 1
done

docker cp amk-redis:/data/dump.rdb "${BACKUP_DIR}/${REDIS_FILE}"

log "step 3/4 — tar.gz archive"
tar -czf "${BACKUP_DIR}/${ARCHIVE}" -C "${BACKUP_DIR}" "${DB_FILE}" "${REDIS_FILE}"
rm "${BACKUP_DIR}/${DB_FILE}" "${BACKUP_DIR}/${REDIS_FILE}"

log "step 4/4 — rotation (>${RETENTION_DAYS}d)"
find "${BACKUP_DIR}" -maxdepth 1 -name 'amk-*.tar.gz' -mtime +"${RETENTION_DAYS}" -print -delete || true

SIZE="$(du -h "${BACKUP_DIR}/${ARCHIVE}" | cut -f1)"
log "backup done archive=${ARCHIVE} size=${SIZE}"
