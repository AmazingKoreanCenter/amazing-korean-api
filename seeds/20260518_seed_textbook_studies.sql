-- 20260518_seed_textbook_studies.sql
-- 출처: amazing-korean-books/scripts/guide-v2/seed_output/20260504_seed_textbook_studies.sql
--   (20260425 세대와 byte-identical 확인. api 스키마 정합 수동 검증: basic_500/study_idx/컬럼 일치)
-- ⚠️ books 원본과의 유일한 차이: study_state 'open' → 'ready' (67행 전부).
--   사유: 숨김 시딩(staged rollout). 서빙은 study_state='open' 만 노출(repo.rs 169/216/290 등)
--   → 'ready' = 전 사용자 비노출. 검증·Mac Mini 번역 도착 후 공개 flip:
--     UPDATE study SET study_state='open' WHERE study_idx LIKE 'amk500-%';
--   재시딩 안전: ON CONFLICT DO UPDATE SET 은 title/subtitle/description 만 갱신,
--   study_state 미포함 → flip 한 'open' 을 재실행이 되돌리지 않음.
--
-- 20260504_seed_textbook_studies.sql
-- 자동 생성: scripts/guide-v2/gen_seed_sql.py
-- 67개 study INSERT (해설집 500 세부 목차)
--
-- ⚠️ 배포 전 프로덕션에서 다음 확인:
--   docker exec amk-pg-prod psql -U postgres -d amazing_korean_db -c "\dt study"
--   docker exec amk-pg-prod psql -U postgres -d amazing_korean_db -c "SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1"
-- ⚠️ HYMN 계정이 prod에 없으면 마이그레이션 abort됨 (안전장치).

-- ==== HYMN 계정 사전 검증 (실패 시 즉시 abort) ====
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM users WHERE user_auth = 'HYMN') THEN
    RAISE EXCEPTION 'HYMN account not found — abort migration. user_auth=HYMN 계정을 prod에 먼저 생성하세요.';
  END IF;
END $$;

-- ==== 67개 study INSERT (UPSERT, 멱등) ====

INSERT INTO study (
    study_idx, study_state, study_access, study_program,
    study_title, study_subtitle, study_description, updated_by_user_id
) VALUES
('amk500-01-01-subject-adjective', 'ready', 'public', 'basic_500', '주어 + 서술어 (형용사)', 'Subject + Adjective (문장 1~10)', '문장구조 / 주어 + 서술어 (형용사) 패턴 10문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-01-02-subject-verb', 'ready', 'public', 'basic_500', '주어 + 서술어 (동사)', 'Subject + Verb (문장 11~20)', '문장구조 / 주어 + 서술어 (동사) 패턴 10문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-01-03-subject-object-verb', 'ready', 'public', 'basic_500', '주어 + 목적어 + 서술어', 'Subject + Object + Verb (문장 21~30)', '문장구조 / 주어 + 목적어 + 서술어 패턴 10문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-01-04-subject-time-place-object-verb', 'ready', 'public', 'basic_500', '주어 + 시간 + 장소 + 목적어 + 서술어', 'Subject + Time + Place + Object + Verb (문장 31~40)', '문장구조 / 주어 + 시간 + 장소 + 목적어 + 서술어 패턴 10문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-01-05-subject-indirect-obj-direct-obj-verb', 'ready', 'public', 'basic_500', '주어 + 간접목적어 + 직접목적어 + 서술어', 'Subject + Indirect Obj + Direct Obj + Verb (문장 41~50)', '문장구조 / 주어 + 간접목적어 + 직접목적어 + 서술어 패턴 10문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-01-06-who-when', 'ready', 'public', 'basic_500', '의문사 — 누구 (Who), 언제 (When)', 'Who & When (문장 51~60)', '문장구조 / 의문사 — 누구 (Who), 언제 (When) 패턴 10문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-01-07-where-what', 'ready', 'public', 'basic_500', '의문사 — 어디 (Where), 무엇 (What)', 'Where & What (문장 61~70)', '문장구조 / 의문사 — 어디 (Where), 무엇 (What) 패턴 10문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-01-08-what-kind-of', 'ready', 'public', 'basic_500', '의문사 — 어떤 (What kind of)', 'What kind of (문장 71~78)', '문장구조 / 의문사 — 어떤 (What kind of) 패턴 8문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-01-09-and-or-but', 'ready', 'public', 'basic_500', '접속 — 과/와 (and), 이나/나 (or), ~고 (and), ~지만 (but), ~거나 (or)', 'And, Or, But (문장 79~86)', '문장구조 / 접속 — 과/와 (and), 이나/나 (or), ~고 (and), ~지만 (but), ~거나 (or) 패턴 8문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-01-noun-predicate', 'ready', 'public', 'basic_500', 'A = B (명사) — 이다', 'A = B (Noun) — 이다 (문장 87~96)', '서술어 문법 / A = B (명사) — 이다 패턴 10문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-02-present-progressive', 'ready', 'public', 'basic_500', '현재진행 — ''~고 있다''', 'Present Progressive (be V-ing) (문장 97~104)', '서술어 문법 / 현재진행 — ''~고 있다'' 패턴 8문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-03-ability-possibility', 'ready', 'public', 'basic_500', '능력, 가능 — ''~ㄹ/을 수 있다/없다'', ''~ㄹ/을 줄 알다/모르다''', 'Ability / Possibility (Can) (문장 105~114)', '서술어 문법 / 능력, 가능 — ''~ㄹ/을 수 있다/없다'', ''~ㄹ/을 줄 알다/모르다'' 패턴 10문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-04-suggestion', 'ready', 'public', 'basic_500', '청유 — ''~ㅂ시다/읍시다'' (Let''s), ''~ㄹ/을까요?'' (Shall we?)', 'Suggestion (Let''s, Shall we) (문장 115~122)', '서술어 문법 / 청유 — ''~ㅂ시다/읍시다'' (Let''s), ''~ㄹ/을까요?'' (Shall we?) 패턴 8문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-05-negation', 'ready', 'public', 'basic_500', '부정 — 안 (don''t), 못 (can''t), ~지 않다, ~지 못하다', 'Negation (Don''t, Can''t) (문장 123~141)', '서술어 문법 / 부정 — 안 (don''t), 못 (can''t), ~지 않다, ~지 못하다 패턴 19문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-06-want-to', 'ready', 'public', 'basic_500', '희망 — ~고 싶다 (want to)', 'Want to (Wish / Hope) (문장 142~149)', '서술어 문법 / 희망 — ~고 싶다 (want to) 패턴 8문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-07-i-wish', 'ready', 'public', 'basic_500', '희망 2 — ~았/었으면 좋겠다 (I wish / I hope)', 'I wish (I hope) (문장 150~156)', '서술어 문법 / 희망 2 — ~았/었으면 좋겠다 (I wish / I hope) 패턴 7문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-08-plan-near-future', 'ready', 'public', 'basic_500', '계획, 가까운 미래 — ~려고 하다 (be going to / be about to)', 'Plan / Near Future (문장 157~163)', '서술어 문법 / 계획, 가까운 미래 — ~려고 하다 (be going to / be about to) 패턴 7문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-09-decision-promise', 'ready', 'public', 'basic_500', '결정, 결심, 약속 — ~기로 했다 (decided to)', 'Decision / Promise (문장 164~168)', '서술어 문법 / 결정, 결심, 약속 — ~기로 했다 (decided to) 패턴 5문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-10-modal-permissions', 'ready', 'public', 'basic_500', '허락, 금지, 의무, 면제', 'Permission / Prohibition / Obligation / Exemption (문장 169~180)', '서술어 문법 / 허락, 금지, 의무, 면제 패턴 12문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-11-request-favor', 'ready', 'public', 'basic_500', '부탁, 요구', 'Request / Favor (문장 181~189)', '서술어 문법 / 부탁, 요구 패턴 9문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-12-experience', 'ready', 'public', 'basic_500', '경험 — ~ㄴ/은 적이 있다/없다, ~아/어 보다', 'Experience (문장 190~198)', '서술어 문법 / 경험 — ~ㄴ/은 적이 있다/없다, ~아/어 보다 패턴 9문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-13-action-for-others', 'ready', 'public', 'basic_500', '다른 사람을 위한 행동 — ~아/어 주다', 'Action for Others (문장 199~203)', '서술어 문법 / 다른 사람을 위한 행동 — ~아/어 주다 패턴 5문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-14-change-of-state', 'ready', 'public', 'basic_500', '변화 1 — ~아/어지다 (became/got)', 'Change of State (문장 204~208)', '서술어 문법 / 변화 1 — ~아/어지다 (became/got) 패턴 5문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-15-change-of-situation', 'ready', 'public', 'basic_500', '변화 2 — ~게 되다 (come to / end up)', 'Change of Situation (문장 209~212)', '서술어 문법 / 변화 2 — ~게 되다 (come to / end up) 패턴 4문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-16-guess-supposition', 'ready', 'public', 'basic_500', '추측 — ~것 같다 (It seems / I think)', 'Guess / Supposition (문장 213~217)', '서술어 문법 / 추측 — ~것 같다 (It seems / I think) 패턴 5문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-17-empathy-guess', 'ready', 'public', 'basic_500', '공감, 추측 — ~겠다 (must be)', 'Empathy / Guess (문장 218~222)', '서술어 문법 / 공감, 추측 — ~겠다 (must be) 패턴 5문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-18-exclamation-discovery', 'ready', 'public', 'basic_500', '감탄, 발견 — ~네요 (Wow!)', 'Exclamation / Discovery (문장 223~231)', '서술어 문법 / 감탄, 발견 — ~네요 (Wow!) 패턴 9문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-19-confirmation', 'ready', 'public', 'basic_500', '확인, 동의 — ~지요/죠 (right? / isn''t it?)', 'Confirmation (문장 232~236)', '서술어 문법 / 확인, 동의 — ~지요/죠 (right? / isn''t it?) 패턴 5문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-02-20-polite-command-everyday-expressions', 'ready', 'public', 'basic_500', '정중한 명령, 일상 표현 — ~(으)세요', 'Polite Command / Everyday Expressions (문장 237~247)', '서술어 문법 / 정중한 명령, 일상 표현 — ~(으)세요 패턴 11문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-01-time-when-while', 'ready', 'public', 'basic_500', '시간 — 명사 + 때, 동사/형용사 + ~ㄹ/을 때', 'Time — when, while (때) (문장 248~252)', '부사어 문법 / 시간 — 명사 + 때, 동사/형용사 + ~ㄹ/을 때 패턴 5문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-02-duration', 'ready', 'public', 'basic_500', '기간 — 명사 + 동안, 동사 + ~는 동안', 'Duration (동안) (문장 253~259)', '부사어 문법 / 기간 — 명사 + 동안, 동사 + ~는 동안 패턴 7문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-03-before', 'ready', 'public', 'basic_500', '전 — 명사 + 전에, 동사 + ~기 전에', 'Before (전에) (문장 260~265)', '부사어 문법 / 전 — 명사 + 전에, 동사 + ~기 전에 패턴 6문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-04-after', 'ready', 'public', 'basic_500', '후 — 명사 + 후에, 동사 + ~ㄴ/은 후에', 'After (후에) (문장 266~270)', '부사어 문법 / 후 — 명사 + 후에, 동사 + ~ㄴ/은 후에 패턴 5문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-05-sequential-actions', 'ready', 'public', 'basic_500', '순서 — 동사 + ~고', 'Sequential Actions (~고) (문장 271~276)', '부사어 문법 / 순서 — 동사 + ~고 패턴 6문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-06-sequential-causation', 'ready', 'public', 'basic_500', '순서/원인 — 동사 + ~아/어서', 'Sequential Causation (~아/어서) (문장 277~283)', '부사어 문법 / 순서/원인 — 동사 + ~아/어서 패턴 7문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-07-simultaneous-action', 'ready', 'public', 'basic_500', '동시 동작 — 동사 + ~(으)면서', 'Simultaneous Action (~(으)면서) (문장 284~291)', '부사어 문법 / 동시 동작 — 동사 + ~(으)면서 패턴 8문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-08-as-soon-as', 'ready', 'public', 'basic_500', '즉시 — 동사 + ~자마자', 'As Soon As (~자마자) (문장 292~295)', '부사어 문법 / 즉시 — 동사 + ~자마자 패턴 4문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-09-action-transition', 'ready', 'public', 'basic_500', '동작 전환 — 동사 + ~다가', 'Action Transition (~다가) (문장 296~299)', '부사어 문법 / 동작 전환 — 동사 + ~다가 패턴 4문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-10-from-to', 'ready', 'public', 'basic_500', '범위 — 부터 ~ 까지 (시간/장소)', 'From ~ To (부터 ~ 까지) (문장 300~306)', '부사어 문법 / 범위 — 부터 ~ 까지 (시간/장소) 패턴 7문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-11-because-of', 'ready', 'public', 'basic_500', '원인 — 명사 + 때문에', 'Because of (때문에) (문장 307~313)', '부사어 문법 / 원인 — 명사 + 때문에 패턴 7문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-12-thanks-to', 'ready', 'public', 'basic_500', '원인 — 명사 + 덕분에 (긍정적 결과)', 'Thanks to (덕분에) (문장 314~315)', '부사어 문법 / 원인 — 명사 + 덕분에 (긍정적 결과) 패턴 2문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-13-because', 'ready', 'public', 'basic_500', '원인 — 동사/형용사 + ~아/어서', 'Because (아/어서) (문장 316~324)', '부사어 문법 / 원인 — 동사/형용사 + ~아/어서 패턴 9문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-14-since-because', 'ready', 'public', 'basic_500', '원인 — ~(으)니까 + 명령/청유', 'Since/Because (~니까) (문장 325~331)', '부사어 문법 / 원인 — ~(으)니까 + 명령/청유 패턴 7문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-15-for-in-order-to', 'ready', 'public', 'basic_500', '목적 — 명사 + ~를 위해서/위해/위하여, 동사 + ~기 위해서/위해/위하여', 'For / In order to (위해서/위해/위하여) (문장 332~337)', '부사어 문법 / 목적 — 명사 + ~를 위해서/위해/위하여, 동사 + ~기 위해서/위해/위하여 패턴 6문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-16-in-order-to', 'ready', 'public', 'basic_500', '목적 — 동사 + ~(으)려고', 'In order to (~(으)려고) (문장 338~345)', '부사어 문법 / 목적 — 동사 + ~(으)려고 패턴 8문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-17-go-come-to', 'ready', 'public', 'basic_500', '목적 — 동사 + ~(으)러 가다/오다', 'Go/Come to (~(으)러 가다/오다) (문장 346~351)', '부사어 문법 / 목적 — 동사 + ~(으)러 가다/오다 패턴 6문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-18-conditional-if', 'ready', 'public', 'basic_500', '가정/조건 — ~(으)면', 'If (~(으)면) (문장 352~355)', '부사어 문법 / 가정/조건 — ~(으)면 패턴 4문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-19-if-you-want-to', 'ready', 'public', 'basic_500', '필수 조건 — ~(으)려면 ~해야 하다', 'If you want to (~(으)려면 ~해야 한다) (문장 356~361)', '부사어 문법 / 필수 조건 — ~(으)려면 ~해야 하다 패턴 6문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-03-20-even-if', 'ready', 'public', 'basic_500', '예상과 다른 결과 — ~아/어도', 'Even if (~아/어도) (문장 362~365)', '부사어 문법 / 예상과 다른 결과 — ~아/어도 패턴 4문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-01-background-context', 'ready', 'public', 'basic_500', '배경/상황 — ~는데/~ㄴ데/~인데', 'Background/Context (~는데) (문장 366~369)', '기타 문법 / 배경/상황 — ~는데/~ㄴ데/~인데 패턴 4문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-02-comparative-superlative', 'ready', 'public', 'basic_500', '비교급 / 최상급 — ~처럼, ~보다 더, ~보다 덜, 가장', 'Comparative / Superlative (비교급 / 최상급) (문장 370~383)', '기타 문법 / 비교급 / 최상급 — ~처럼, ~보다 더, ~보다 덜, 가장 패턴 14문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-03-verb-adj-noun', 'ready', 'public', 'basic_500', '명사형 — ~ㄴ/은/는 것, ~기, ~ㅁ/음', 'Verb/Adj → Noun (명사형) (문장 384~387)', '기타 문법 / 명사형 — ~ㄴ/은/는 것, ~기, ~ㅁ/음 패턴 4문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-04-adjective-adverb', 'ready', 'public', 'basic_500', '부사형 — ~게, ~히, ~이', 'Adjective → Adverb (부사형) (문장 388~396)', '기타 문법 / 부사형 — ~게, ~히, ~이 패턴 9문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-05-adjective-verb', 'ready', 'public', 'basic_500', '동사화 — 형용사 + ~아/어하다', 'Adjective → Verb (동사화) (문장 397~400)', '기타 문법 / 동사화 — 형용사 + ~아/어하다 패턴 4문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-06-only', 'ready', 'public', 'basic_500', '한정 — "만" / "밖에"', 'Only (만 / 밖에) (문장 401~407)', '기타 문법 / 한정 — "만" / "밖에" 패턴 7문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-07-method-direction', 'ready', 'public', 'basic_500', '방향, 수단, 재료, 원인, 신분, 변화 — ~로/으로', 'Method / Direction (~로/으로) (문장 408~413)', '기타 문법 / 방향, 수단, 재료, 원인, 신분, 변화 — ~로/으로 패턴 6문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-08-adverbs-of-frequency', 'ready', 'public', 'basic_500', '빈도부사 — 항상, 자주, 가끔, 마다', 'Adverbs of Frequency (빈도부사) (문장 414~418)', '기타 문법 / 빈도부사 — 항상, 자주, 가끔, 마다 패턴 5문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-09-classifiers', 'ready', 'public', 'basic_500', '종별사 — 단위 명사 (명, 개, 마리, 권...)', 'Classifiers (종별사) (문장 419~435)', '기타 문법 / 종별사 — 단위 명사 (명, 개, 마리, 권...) 패턴 17문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-10-rieul-irregular', 'ready', 'public', 'basic_500', 'ㄹ 불규칙 — ㄹ + ㄴ/ㅂ/ㅅ → ㄹ 탈락', 'ㄹ Irregular (ㄹ 불규칙) (문장 436~440)', '기타 문법 / ㄹ 불규칙 — ㄹ + ㄴ/ㅂ/ㅅ → ㄹ 탈락 패턴 5문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-11-eu-irregular', 'ready', 'public', 'basic_500', '"ㅡ" 불규칙 — "ㅡ" + 아/어 → "ㅡ" 탈락', '"ㅡ" Irregular ("ㅡ" 불규칙) (문장 441~444)', '기타 문법 / "ㅡ" 불규칙 — "ㅡ" + 아/어 → "ㅡ" 탈락 패턴 4문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-12-reu-irregular', 'ready', 'public', 'basic_500', '"르" 불규칙 — "르" + 았/었습니다 → ㄹ랐/렀습니다', '르 Irregular (르 불규칙) (문장 445~448)', '기타 문법 / "르" 불규칙 — "르" + 았/었습니다 → ㄹ랐/렀습니다 패턴 4문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-13-bieup-irregular', 'ready', 'public', 'basic_500', '"ㅂ" 불규칙 — "ㅂ" + 아/어 → 오/우 + 아/어', '"ㅂ" Irregular ("ㅂ" 불규칙) (문장 449~453)', '기타 문법 / "ㅂ" 불규칙 — "ㅂ" + 아/어 → 오/우 + 아/어 패턴 5문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-14-digeut-irregular', 'ready', 'public', 'basic_500', 'ㄷ 불규칙 — ㄷ + 모음 → ㄹ', 'ㄷ Irregular (ㄷ 불규칙) (문장 454~459)', '기타 문법 / ㄷ 불규칙 — ㄷ + 모음 → ㄹ 패턴 6문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-15-siot-irregular', 'ready', 'public', 'basic_500', 'ㅅ 불규칙 — ㅅ + 아/어 → ㅅ 탈락', 'ㅅ Irregular (ㅅ 불규칙) (문장 460~461)', '기타 문법 / ㅅ 불규칙 — ㅅ + 아/어 → ㅅ 탈락 패턴 2문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-16-indirect-speech', 'ready', 'public', 'basic_500', '간접화법 — ~다고, ~이라고, ~냐고, ~(으)라고, ~자고 하다', 'Indirect Speech (간접화법) (문장 462~473)', '기타 문법 / 간접화법 — ~다고, ~이라고, ~냐고, ~(으)라고, ~자고 하다 패턴 12문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-17-modifier', 'ready', 'public', 'basic_500', '관형어 — ~ㄴ/은, ~는, ~ㄹ/을', 'Modifier (관형어) (문장 474~488)', '기타 문법 / 관형어 — ~ㄴ/은, ~는, ~ㄹ/을 패턴 15문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1)),
('amk500-04-18-honorifics', 'ready', 'public', 'basic_500', '존댓말 — 께서, ~시다, 드시다, 주무시다 등', 'Honorifics (존댓말) (문장 489~500)', '기타 문법 / 존댓말 — 께서, ~시다, 드시다, 주무시다 등 패턴 12문장', (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1))
ON CONFLICT (study_idx) DO UPDATE SET
    study_title = EXCLUDED.study_title,
    study_subtitle = EXCLUDED.study_subtitle,
    study_description = EXCLUDED.study_description,
    study_updated_at = NOW();

