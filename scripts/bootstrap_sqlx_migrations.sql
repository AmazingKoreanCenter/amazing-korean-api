-- =============================================================================
-- sqlx 자동 마이그레이션 전환을 위한 1회성 부트스트랩 스크립트
-- 프로덕션 DB에서 1회 실행 후 삭제
--
-- 용도: 기존에 수동으로 적용된 마이그레이션 이력을 _sqlx_migrations 테이블에 등록
--       이후 앱 부팅 시 sqlx::migrate!()가 이미 적용된 것으로 인식하여 건너뜀
--
-- 실행 방법:
--   docker exec -i amk-pg psql -U postgres -d amazing_korean_db < scripts/bootstrap_sqlx_migrations.sql
--
-- 주의: 이 스크립트는 migrations/ 폴더 밖에 있어야 함 (sqlx가 마이그레이션으로 인식하지 않도록)
-- =============================================================================

CREATE TABLE IF NOT EXISTS _sqlx_migrations (
    version         BIGINT PRIMARY KEY,
    description     TEXT NOT NULL,
    installed_on    TIMESTAMPTZ NOT NULL DEFAULT now(),
    success         BOOLEAN NOT NULL,
    checksum        BYTEA NOT NULL,
    execution_time  BIGINT NOT NULL
);

INSERT INTO _sqlx_migrations (version, description, installed_on, success, checksum, execution_time) VALUES
(20260208,       'AMK V1',                            now(), true, decode('eaaeca733d1af564dfb93c0874c8661a5cf9b207c644b84b73efe3d3166522cae58a702a3ac1305497745c55fff68b4e', 'hex'), 0),
(20260210,       'i18n add video content type',        now(), true, decode('ed40a6ca361cbbfec0f37a54203c480ef4a233f12ea6f7d960673f9486f7926b690dd3f45bbc34bbecc150220db5b24b', 'hex'), 0),
(20260210000001, 'i18n content translations',          now(), true, decode('17ce50629a14816e7bf90e4443c335187793e68c409bfcbbe7f52f57af17f95bd0113207767af430aeefe01b7d6e6dab', 'hex'), 0),
(20260212,       'add study task explain content type', now(), true, decode('3be8b33d4e34c4b7506798bad5c04f1ec441a556d4a62aa4ce71640a33ceb81150b740ab34f8b085ab9c4c5fc080aa33', 'hex'), 0),
(20260214,       'add mfa columns',                    now(), true, decode('d21d0651949781ee95e7b69048d137e3369c2d5b1e8f908c3e1fcea653f3a6394f648f04073c168bbaf7da27cb4d7616', 'hex'), 0),
(20260214000001, 'video log ip type fix',              now(), true, decode('d26dbeefbec568f23029a40410e43b1999c11107ffcad05df89a07b30e34d5039ae955bac52b12658900b661fb3b781c', 'hex'), 0),
(20260215,       'payment system',                     now(), true, decode('18158122655d318c35a17391a1e442eaa457b1a54336310ab8e2f07495fc092cf89f671e4a49b89a5cf5e3ccd6100b5d', 'hex'), 0),
(20260226,       'textbook',                           now(), true, decode('fc242d0362ccf8166e965fdf4aa98b547a49f9814226197b9e901beb5faa54fb98cf2c30ae64ddd2d82e241c2cfdbdfe', 'hex'), 0),
(20260303,       'textbook improvements',              now(), true, decode('14f0fed7cc3bd81b5df29071db22d414e367173bfa0b1920c5f8867de5b9e6a48b0bb7198b1af353431a5191bec5d457', 'hex'), 0),
(20260310,       'add tl language',                    now(), true, decode('b4e794a8ae1010c0e2f3989362f67f5809f9bc74a4ec8742185fe078ca17899d7400e09fe71ceea2741e9b9bd82b3c06', 'hex'), 0),
(20260310000001, 'ebook',                              now(), true, decode('c8cb44b455fb03113a903233ab2db942eb292df332d4d8fdeebcbddfca41ef7a5ef4bedc6a21649fd42e28042d9eeeb5', 'hex'), 0),
(20260312,       'ebook purchase code expand',         now(), true, decode('f0f0185d7762a70b8662ee8294645d5868afde0894ef74fb95603a1a5a947951990d6820b6ad81a214d97e5f875317f4', 'hex'), 0),
(20260323,       'textbook tax fields',                now(), true, decode('813865f0f515ed94ec8b5e991633e25fb0f5d721370f9d10441084f9d2e0f66b279977dc91163cb4ab46eb0f0122a6ba', 'hex'), 0)
ON CONFLICT (version) DO NOTHING;

-- 확인
SELECT version, description, success FROM _sqlx_migrations ORDER BY version;
