#!/usr/bin/env bash
# J3: deploy.yml + .env.example + config.rs 환경변수 정합성 검증 도구
#
# 사용:
#   bash scripts/check_env_consistency.sh
#
# Exit code:
#   0 = 정합 (config.rs 의 모든 변수 = deploy.yml + .env.example 에 명시)
#   1 = 차이 발견 (production panic 위험 또는 dev 시작 실패 가능)
#
# 검증 차원:
#   1. config.rs ↔ deploy.yml: production 미명시 변수 (panic 또는 default)
#   2. config.rs ↔ .env.example: dev 미명시 변수
#   3. deploy.yml ↔ config.rs: 사용되지 않는 secret (정보용, exit 1 안 함)
#
# 본 도구 = AMK_DEBTS J3 처리 (수동 정합성 점검 자동화).
# INC-001 패턴 (deploy.yml heredoc + Secrets + config.rs panic 게이트 동기화 누락) 회피.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

# ── 1. deploy.yml .env.prod heredoc 안 변수 추출 ──
# heredoc 블록 = `cat > .env.prod << 'EOF'` ... `EOF` 사이.
DEPLOY_VARS=$(awk "/cat > \.env\.prod << 'EOF'/,/^[[:space:]]*EOF[[:space:]]*$/" \
  .github/workflows/deploy.yml \
  | grep -E '^[[:space:]]*[A-Z_]+=' \
  | sed -E 's/^[[:space:]]*([A-Z_]+)=.*/\1/' \
  | sort -u)

# ── 2. .env.example 안 변수 ──
EXAMPLE_VARS=$(grep -E '^[A-Z_]+=' .env.example \
  | sed -E 's/^([A-Z_]+)=.*/\1/' \
  | sort -u)

# ── 3. config.rs 의 env::var() 호출 ──
CONFIG_VARS=$(grep -oE 'env::var\("[A-Z_]+"' src/config.rs \
  | sed -E 's/env::var\("([A-Z_]+)"/\1/' \
  | sort -u)

DEPLOY_COUNT=$(echo "$DEPLOY_VARS" | grep -c '.' || true)
EXAMPLE_COUNT=$(echo "$EXAMPLE_VARS" | grep -c '.' || true)
CONFIG_COUNT=$(echo "$CONFIG_VARS" | grep -c '.' || true)

echo "=== 변수 카운트 ==="
echo "  deploy.yml .env.prod : $DEPLOY_COUNT"
echo "  .env.example         : $EXAMPLE_COUNT"
echo "  src/config.rs        : $CONFIG_COUNT"

EXIT=0

# ── 차원 1: config.rs ↔ deploy.yml ──
echo ""
echo "=== ⚠ config.rs 사용 + deploy.yml 미명시 (production 부재 시 default/panic) ==="
DIFF1=$(comm -23 <(echo "$CONFIG_VARS") <(echo "$DEPLOY_VARS") || true)
if [ -n "$DIFF1" ]; then
  echo "$DIFF1" | sed 's/^/  - /'
  EXIT=1
else
  echo "  (없음)"
fi

# ── 차원 2: config.rs ↔ .env.example ──
echo ""
echo "=== ⚠ config.rs 사용 + .env.example 미명시 (dev 시작 default/panic) ==="
DIFF2=$(comm -23 <(echo "$CONFIG_VARS") <(echo "$EXAMPLE_VARS") || true)
if [ -n "$DIFF2" ]; then
  echo "$DIFF2" | sed 's/^/  - /'
  EXIT=1
else
  echo "  (없음)"
fi

# ── 차원 3: deploy.yml ↔ config.rs (정보용, exit 1 안 함) ──
echo ""
echo "=== ℹ deploy.yml 명시 + config.rs 미사용 (잠재 불필요 secret) ==="
DIFF3=$(comm -23 <(echo "$DEPLOY_VARS") <(echo "$CONFIG_VARS") || true)
if [ -n "$DIFF3" ]; then
  echo "$DIFF3" | sed 's/^/  - /'
  echo "  (DOMAIN / FRONTEND_URL / CORS_ORIGINS / ENCRYPTION_CURRENT_VERSION 등 시스템/외부 사용 변수일 수 있음. 무시 가능)"
else
  echo "  (없음)"
fi

echo ""
if [ $EXIT -eq 0 ]; then
  echo "✅ 정합 (production + dev 양쪽 모두 config.rs 의 모든 변수 명시됨)"
else
  echo "❌ 차이 발견 — 위 변수 추가 필요"
  echo "   참조: AMK_DEBTS.md §J1 (RATE_LIMIT_TEXTBOOK_*) / §J2 (APPLE_*) 패턴"
fi

exit $EXIT
