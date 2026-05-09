# Migration 정책

본 디렉터리의 SQL 파일은 sqlx 의 마이그레이션 시스템에서 관리됩니다. 신규 마이그레이션 추가 시 다음 정책을 따릅니다.

## SSoT

- **정책 본문** = `docs/AMK_DEPLOY_OPS.md §3` (마이그레이션 자동 적용 / 파일 네이밍)
- **부채 추적** = `docs/AMK_DEBTS.md` G16 (legacy 14자리 정렬 비호환)
- 본 README = 디렉터리 진입자용 빠른 참조 + 부채 회피 가이드

## 1. Timestamp 형식 = 8자리 (`YYYYMMDD`)

### 정책 (2026-03-23 INC 후 정착)

```
migrations/YYYYMMDD_<description>.sql
```

- **8자리 timestamp 만 사용**: `20260520_add_user_consent_timestamp.sql`
- **`HHMMSS` 접미사 (000001 등) 사용 금지** — 2026-03-23 사고로 서버 크래시 발생
- 이유: sqlx 는 첫 `_` 앞 숫자를 BIGINT 로 파싱 후 오름차순 실행. `20260310000001` 은 `20,260,310,000,001` 이 되어 `20260312` (`20,260,312`) 보다 큰 값 = 의존성 순서 뒤집힘.

### 같은 날 여러 migration = 다음 날짜 사용

```
✅ 정상:
20260310_add_field_a.sql
20260311_add_field_b.sql   ← 다음 날
20260312_add_field_c.sql   ← 다음 날

❌ 금지 (HHMMSS 접미사):
20260310_add_field_a.sql
20260310000001_add_field_b.sql   ← 정렬 뒤집힘 위험
```

## 2. 기존 14자리 migration = legacy (변경 금지)

본 정책 정착 (2026-03-23) 이전 추가된 두 개:
- `20260210000001_i18n_content_translations.sql`
- `20260214000001_video_log_ip_type_fix.sql`

→ **파일명 변경 절대 금지**. `_sqlx_migrations` 테이블의 version + checksum 과 불일치 = production DB 손상 위험.

→ production 환경에서는 점진 적용으로 우회됨 (한 번 적용된 마이그레이션은 skip).

→ Fresh DB 환경에서는 정렬 뒤집힘으로 fail (G16 부채):
```
type "content_type_enum" does not exist (code: 42704)
```

## 3. Fresh DB 셋업 시 우회 패턴 (G16)

CI service container 도입 / 신규 dev 환경 셋업 시:

### 옵션 A: 통합 테스트 = 기존 DB 유지 (현재 채택)

`tests/repo_integration.rs` 가 `#[tokio::test]` + 수동 PgPool + 기존 amk-pg DB 사용. sqlx::test 매크로 미사용 = migration 정렬 충돌 회피.

### 옵션 B: sqlx Migrator 정렬 옵션 변경

미평가. sqlx 0.8 의 Migrator API 검토 필요.

### 옵션 C: legacy 14자리 → 8자리 rename + production checksum 재정착

매우 위험. production `_sqlx_migrations` 백업 + version 재기록 필요. 권장하지 않음.

## 4. 신규 migration 작성 절차

```bash
# 1. 파일 생성 (8자리 timestamp)
touch migrations/$(date +"%Y%m%d")_descriptive_name.sql

# 2. SQL 작성 (CREATE TABLE / ALTER TYPE 등)
vi migrations/$(date +"%Y%m%d")_descriptive_name.sql

# 3. 로컬 검증
cargo run  # → "Database migrations applied" 로그 확인
# 또는: sqlx migrate run --database-url $DATABASE_URL

# 4. PR 생성 → 리뷰 → main 머지
# CI/CD 가 production 에 자동 배포 + sqlx::migrate!() 자동 적용
```

## 5. 한 번 적용된 파일 수정 금지

- production 의 `_sqlx_migrations.checksum` 과 SHA-384 불일치 = 서버 부팅 실패
- 변경 필요 시 = **새 마이그레이션 파일 추가** (이전 파일 그대로 유지)
