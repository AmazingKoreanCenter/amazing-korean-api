#!/usr/bin/env bash
# Amazing Korean API - Refresh Flow E2E Verification

API="${API:-http://localhost:3000}"
EMAIL="${EMAIL:-KKR@KKR.com}"
PASS="${PASS:-rudfbs1234}"
REDIS_CONTAINER="${REDIS_CONTAINER:-amk-redis}"

set -euo pipefail

log() { echo -e "\n\033[1;36m▶ $*\033[0m"; }
fail() { echo -e "\n\033[1;31m✗ $*\033[0m"; exit 1; }
ok()   { echo -e "\n\033[1;32m✓ $*\033[0m"; }

# 0) 서버 생존(선택)
curl -fsS "$API/health/live" >/dev/null || echo "(warn) health/live 호출 실패(서버 미기동일 수 있음)"

# 1) 로그인
log "로그인"
LOGIN_RAW=$(curl -i -s -X POST "$API/auth/login" -H 'Content-Type: application/json'   -d "{\"email\":\"$EMAIL\",\"password\":\"$PASS\"}")

REF_COOKIE=$(echo "$LOGIN_RAW" | tr -d '\r' | grep -i '^set-cookie:' | sed -n 's/.*ak_refresh=\([^;]*\).*/\1/p' | tail -n1)
[ -n "$REF_COOKIE" ] || fail "쿠키 추출 실패"

if command -v jq >/dev/null 2>&1; then
  USER_ID=$(echo "$LOGIN_RAW" | sed -n '/^{/,$p' | jq -r '.user.id')
else
  USER_ID=$(echo "$LOGIN_RAW" | sed -n '/^{/,$p' | sed -n 's/.*"id":\([0-9]*\).*/\1/p' | head -n1)
fi
[ -n "$USER_ID" ] || fail "USER_ID 추출 실패"

# 1-1) 세션ID
log "Redis에서 세션ID 찾기"
SID=$(docker exec -i "$REDIS_CONTAINER" redis-cli --raw SMEMBERS "ak:user_sessions:$USER_ID" | tail -n1 | tr -d '\r')
[ -n "$SID" ] || fail "세션ID 없음"

log "세션 해시/TTL 확인"
docker exec -i "$REDIS_CONTAINER" redis-cli HGETALL "ak:session:$SID"
TTL1=$(docker exec -i "$REDIS_CONTAINER" redis-cli TTL "ak:session:$SID" | tr -d '\r')
echo "TTL(session)=$TTL1"
OLD_HASH=$(docker exec -i "$REDIS_CONTAINER" redis-cli --raw HGET "ak:session:$SID" refresh_hash | tr -d '\r')
[ -n "$OLD_HASH" ] || fail "refresh_hash 없음"

log "해시→세션 매핑 확인 (ak:refresh:<hash> -> SID)"
MAP_SID=$(docker exec -i "$REDIS_CONTAINER" redis-cli --raw GET "ak:refresh:$OLD_HASH" | tr -d '\r')
echo "MAP_SID=$MAP_SID"
[ "$MAP_SID" = "$SID" ] || fail "매핑 불일치"

# 2) 리프레시(성공)
log "리프레시(성공) 호출"
REFRESH1=$(curl -i -s -X POST "$API/auth/refresh" -H "Cookie: ak_refresh=$REF_COOKIE")
STATUS1=$(echo "$REFRESH1" | head -n1 | awk '{print $2}')
echo "HTTP=$STATUS1"
[ "$STATUS1" = "200" ] || { echo "$REFRESH1" | sed -n '1,120p'; fail "리프레시 1회 실패"; }

NEW_COOKIE=$(echo "$REFRESH1" | tr -d '\r' | grep -i '^set-cookie:' | sed -n 's/.*ak_refresh=\([^;]*\).*/\1/p' | tail -n1)
[ -n "$NEW_COOKIE" ] || fail "새 쿠키 미수신(회전 안 됨)"

log "회전 결과 확인(세션 해시/rotation/TTL)"
NEW_HASH=$(docker exec -i "$REDIS_CONTAINER" redis-cli --raw HGET "ak:session:$SID" refresh_hash | tr -d '\r')
ROT=$(docker exec -i "$REDIS_CONTAINER" redis-cli --raw HGET "ak:session:$SID" rotation | tr -d '\r')
TTL2=$(docker exec -i "$REDIS_CONTAINER" redis-cli TTL "ak:session:$SID" | tr -d '\r')
echo "OLD_HASH=$OLD_HASH"
echo "NEW_HASH=$NEW_HASH"
echo "ROTATION=$ROT"
echo "TTL(after refresh)=$TTL2"

[ "$NEW_HASH" != "$OLD_HASH" ] || fail "해시가 갱신되지 않음"
[ "$ROT" = "1" ] || fail "rotation 증가 안 됨"

log "매핑 갱신 확인(구 매핑 삭제, 새 매핑 생성)"
EX_OLD=$(docker exec -i "$REDIS_CONTAINER" redis-cli EXISTS "ak:refresh:$OLD_HASH" | tr -d '\r')
MAP_SID_NEW=$(docker exec -i "$REDIS_CONTAINER" redis-cli --raw GET "ak:refresh:$NEW_HASH" | tr -d '\r')
echo "EXISTS old_map=$EX_OLD (0이어야 정상), NEW_MAP_SID=$MAP_SID_NEW"
[ "$EX_OLD" = "0" ] || fail "구 매핑이 남아있음"
[ "$MAP_SID_NEW" = "$SID" ] || fail "새 매핑의 세션ID 불일치"

TTL_MAP=$(docker exec -i "$REDIS_CONTAINER" redis-cli TTL "ak:refresh:$NEW_HASH" | tr -d '\r')
echo "TTL(map)=$TTL_MAP"
DIFF=$(( TTL2 > TTL_MAP ? TTL2 - TTL_MAP : TTL_MAP - TTL2 ))
[ "$DIFF" -le 3 ] || fail "세션 TTL과 매핑 TTL 차이가 큼: $DIFF s"

# 3) 재사용 공격(이전 쿠키) → 401
log "재사용 공격 테스트(이전 쿠키로 /auth/refresh)"
REPLAY=$(curl -i -s -X POST "$API/auth/refresh" -H "Cookie: ak_refresh=$REF_COOKIE")
REPLAY_STATUS=$(echo "$REPLAY" | head -n1 | awk '{print $2}')
echo "HTTP=$REPLAY_STATUS (401이어야 정상)"
[ "$REPLAY_STATUS" = "401" ] || fail "구 쿠키로 리프레시가 허용됨(취약)"

# 4) 동시성(동일 새 쿠키로 2회 동시) → 1 성공 / 1 실패
log "동시성 테스트(동일 새 쿠키로 2회 동시 /auth/refresh)"
RESP_A=$(mktemp); RESP_B=$(mktemp)
curl -i -s -X POST "$API/auth/refresh" -H "Cookie: ak_refresh=$NEW_COOKIE" >"$RESP_A" &
curl -i -s -X POST "$API/auth/refresh" -H "Cookie: ak_refresh=$NEW_COOKIE" >"$RESP_B" &
wait
A=$(head -n1 "$RESP_A" | awk '{print $2}')
B=$(head -n1 "$RESP_B" | awk '{print $2}')
echo "A=$A, B=$B  (하나는 200, 하나는 401 기대)"
if ! { [ "$A" = "200" -a "$B" = "401" ] || [ "$A" = "401" -a "$B" = "200" ]; }; then
  echo "--- RESP_A ---"; head -n40 "$RESP_A"
  echo "--- RESP_B ---"; head -n40 "$RESP_B"
  fail "동시성 회전 제어 실패(A=$A, B=$B)"
fi

# 5) 로그아웃 → 세션/매핑 제거
log "로그아웃(/auth/logout)"
LOGOUT=$(curl -i -s -X POST "$API/auth/logout" -H "Cookie: ak_refresh=$NEW_COOKIE")
echo "$LOGOUT" | head -n1
EX_SESS=$(docker exec -i "$REDIS_CONTAINER" redis-cli EXISTS "ak:session:$SID" | tr -d '\r')
EX_MAP=$(docker exec -i "$REDIS_CONTAINER" redis-cli EXISTS "ak:refresh:$NEW_HASH" | tr -d '\r')
echo "EXISTS session=$EX_SESS, map=$EX_MAP (둘 다 0이어야 정상)"
[ "$EX_SESS" = "0" ] && [ "$EX_MAP" = "0" ] || fail "로그아웃 후 잔존 키 있음"

ok "모든 검증 합격"
