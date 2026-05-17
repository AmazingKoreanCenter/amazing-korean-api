-- =============================================================================
-- 보안 감사 2.3 — DB 최소권한: 앱 전용 NOSUPERUSER 소유 role (amk_app)
-- =============================================================================
-- 목적: 앱이 PostgreSQL `postgres`(cluster superuser)로 접속하던 것을 비-superuser
--   소유 role 로 전환할 수 있게 사전 프로비저닝. superuser 의 치명적 폭발 반경
--   (COPY ... PROGRAM = 서버 RCE / CREATE ROLE / 타 DB / ALTER SYSTEM / 서버
--   파일 읽기) 제거. 앱은 자기 스키마 객체 DDL+DML 만 필요(부팅 시 sqlx 마이그).
--
-- 멱등: 재실행 안전. 적용 경로 2가지로 비재발 보장 —
--   (1) 신규/재생성 DB: docker-entrypoint-initdb.d 자동 실행 (postgres superuser)
--   (2) 기존 prod 볼륨: 1회 수동 `docker exec ... psql -f` (AMK_DEPLOY_OPS §13)
--
-- ⚠️ Phase 1 = 본 SQL 적용만. 앱은 여전히 postgres 로 접속(런타임 영향 0).
--    Phase 2(별도 게이트) = DATABASE_URL user 를 amk_app 으로 교체.
--
-- psql 16 `\getenv` 로 컨테이너 env 의 APP_DB_PASSWORD 주입.
-- =============================================================================

\getenv app_pw APP_DB_PASSWORD

-- env 미존재 → \getenv 가 app_pw 미설정 → 중단
\if :{?app_pw}
\else
\echo '[2.3] APP_DB_PASSWORD 미설정 — 중단 (빈 비밀번호 role 생성 방지)'
\quit 1
\endif

-- 정의됐으나 빈/공백인 경우도 차단. (psql 은 $$...$$ 내부 :'var' 를 치환하지
-- 않으므로 평문 SELECT + \gset 으로 검사 — dollar-quote 안에서 하면 안 됨)
SELECT (btrim(:'app_pw') = '') AS pw_empty \gset
\if :pw_empty
\echo '[2.3] APP_DB_PASSWORD 가 비어있음 — 중단 (빈 비밀번호 role 방지)'
\quit 1
\endif

DO $$
BEGIN
  IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'amk_app') THEN
    CREATE ROLE amk_app LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION;
  END IF;
END
$$;

ALTER ROLE amk_app PASSWORD :'app_pw';

GRANT CONNECT ON DATABASE amazing_korean_db TO amk_app;
GRANT ALL ON SCHEMA public TO amk_app;
GRANT ALL ON ALL TABLES    IN SCHEMA public TO amk_app;
GRANT ALL ON ALL SEQUENCES IN SCHEMA public TO amk_app;

-- 전환 기간(앱이 아직 postgres 로 접속) 동안 postgres 가 만드는 신규 객체도
-- amk_app 이 쓸 수 있게 기본 권한 부여. Phase 2 후엔 amk_app 이 owner 라 무관.
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES    TO amk_app;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON SEQUENCES TO amk_app;

-- 기존 앱 객체 소유권 → amk_app 이전 (Phase 2 후 마이그 ALTER/DROP/ALTER TYPE
-- 가능하게). `REASSIGN OWNED BY postgres` 는 부트스트랩 superuser 라 PG 가 거부
-- (시스템 객체 포함). → public 스키마 앱 객체만 선별 ALTER OWNER. 멱등(같은
-- owner 재지정 무해), 신규 DB 시 객체 없으면 no-op. (Phase 1 = 앱이 아직
-- postgres superuser 라 소유권 무관하게 정상 동작 — Phase 2 대비 사전 이전)
DO $$
DECLARE r record;
BEGIN
  FOR r IN SELECT format('ALTER TABLE %I.%I OWNER TO amk_app', schemaname, tablename) c
           FROM pg_tables WHERE schemaname = 'public' LOOP EXECUTE r.c; END LOOP;
  FOR r IN SELECT format('ALTER SEQUENCE %I.%I OWNER TO amk_app', sequence_schema, sequence_name) c
           FROM information_schema.sequences WHERE sequence_schema = 'public' LOOP EXECUTE r.c; END LOOP;
  FOR r IN SELECT format('ALTER VIEW %I.%I OWNER TO amk_app', schemaname, viewname) c
           FROM pg_views WHERE schemaname = 'public' LOOP EXECUTE r.c; END LOOP;
  FOR r IN SELECT format('ALTER TYPE %I.%I OWNER TO amk_app', n.nspname, t.typname) c
           FROM pg_type t JOIN pg_namespace n ON n.oid = t.typnamespace
           WHERE n.nspname = 'public' AND t.typtype = 'e'  -- 앱 enum 타입
           LOOP EXECUTE r.c; END LOOP;
END
$$;
