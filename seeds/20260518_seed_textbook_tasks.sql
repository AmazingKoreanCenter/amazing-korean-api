-- 20260518_seed_textbook_tasks.sql
-- 출처: amazing-korean-books/.../20260505_seed_textbook_tasks.sql (20260426 와 byte-identical)
-- 무변경. study_task 500(kind=typing) + study_task_typing 500. 멱등 ON CONFLICT.
-- study_task 가시성 = 부모 study.study_state JOIN (repo.rs 290/442) → 부모 'ready' 면 자동 숨김.
--
-- 20260505_seed_textbook_tasks.sql
-- 자동 생성: scripts/guide-v2/gen_seed_sql.py
-- 500개 study_task + study_task_typing INSERT
--
-- 한 문장(num) → 1 task (study_task_kind='typing', study_task_idx='amk500-sent-{num:03d}')
-- 정답 = korean (전처리 없이 원문). 정규화는 런타임 비교 로직 (D9).
-- question = en (영어 원문), 35개 언어 번역은 M07에서 별도.
--
-- ⚠️ M05 (studies)가 먼저 적용되어야 함 (study_id 참조).

-- #1: amk500-01-01-subject-adjective
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-01-subject-adjective'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-001'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I am happy.', '저는 행복합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #2: amk500-01-01-subject-adjective
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-01-subject-adjective'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-002'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She is kind.', '그녀는 친절합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #3: amk500-01-01-subject-adjective
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-01-subject-adjective'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-003'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi is busy.', '수미는 바쁩니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #4: amk500-01-01-subject-adjective
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-01-subject-adjective'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-004'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The weather is nice.', '날씨가 좋습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #5: amk500-01-01-subject-adjective
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-01-subject-adjective'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-005'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The mountain is high.', '산이 높습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #6: amk500-01-01-subject-adjective
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-01-subject-adjective'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-006'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The room is small.', '방이 작습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #7: amk500-01-01-subject-adjective
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-01-subject-adjective'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-007'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'There are a lot of people.', '사람들이 많습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #8: amk500-01-01-subject-adjective
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-01-subject-adjective'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-008'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The bag is heavy.', '가방이 무겁습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #9: amk500-01-01-subject-adjective
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-01-subject-adjective'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-009'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The train is fast.', '기차가 빠릅니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #10: amk500-01-01-subject-adjective
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-01-subject-adjective'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-010'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The movie is interesting.', '영화가 재미있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #11: amk500-01-02-subject-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-02-subject-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-011'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He slept.', '그는 잤습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #12: amk500-01-02-subject-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-02-subject-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-012'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He got up.', '그는 일어났습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #13: amk500-01-02-subject-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-02-subject-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-013'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He walked.', '그는 걸었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #14: amk500-01-02-subject-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-02-subject-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-014'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He ran.', '그는 뛰었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #15: amk500-01-02-subject-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-02-subject-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-015'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He worked.', '그는 일했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #16: amk500-01-02-subject-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-02-subject-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-016'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He rested.', '그는 쉬었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #17: amk500-01-02-subject-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-02-subject-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-017'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He thought.', '그는 생각했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #18: amk500-01-02-subject-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-02-subject-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-018'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He spoke.', '그는 말했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #19: amk500-01-02-subject-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-02-subject-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-019'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He listened.', '그는 들었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #20: amk500-01-02-subject-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-02-subject-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-020'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He went.', '그는 갔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #21: amk500-01-03-subject-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-03-subject-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-021'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I eat breakfast.', '저는 아침을 먹습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #22: amk500-01-03-subject-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-03-subject-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-022'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I exercise.', '저는 운동을 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #23: amk500-01-03-subject-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-03-subject-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-023'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I draw a picture.', '저는 그림을 그립니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #24: amk500-01-03-subject-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-03-subject-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-024'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I listen to music.', '저는 음악을 듣습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #25: amk500-01-03-subject-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-03-subject-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-025'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I watch a movie.', '저는 영화를 봅니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #26: amk500-01-03-subject-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-03-subject-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-026'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I wear clothes.', '저는 옷을 입습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #27: amk500-01-03-subject-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-03-subject-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-027'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I clean.', '저는 청소를 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #28: amk500-01-03-subject-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-03-subject-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-028'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I read a book.', '저는 책을 읽습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #29: amk500-01-03-subject-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-03-subject-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-029'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I learn Korean.', '저는 한국어를 배웁니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #30: amk500-01-03-subject-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-03-subject-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-030'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I meet a friend.', '저는 친구를 만납니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #31: amk500-01-04-subject-time-place-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-04-subject-time-place-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-031'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I bought.', '저는 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #32: amk500-01-04-subject-time-place-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-04-subject-time-place-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-032'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I bought a gift.', '저는 선물을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #33: amk500-01-04-subject-time-place-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-04-subject-time-place-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-033'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I bought a gift at the department store.', '저는 백화점에서 선물을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #34: amk500-01-04-subject-time-place-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-04-subject-time-place-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-034'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I bought a gift at the department store yesterday.', '저는 어제 백화점에서 선물을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #35: amk500-01-04-subject-time-place-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-04-subject-time-place-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-035'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I met.', '저는 만났습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #36: amk500-01-04-subject-time-place-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-04-subject-time-place-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-036'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I met a friend.', '저는 친구를 만났습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #37: amk500-01-04-subject-time-place-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-04-subject-time-place-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-037'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I met a friend at the café.', '저는 커피숍에서 친구를 만났습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #38: amk500-01-04-subject-time-place-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-04-subject-time-place-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-038'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I met a friend at the café last weekend.', '저는 지난 주말에 커피숍에서 친구를 만났습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #39: amk500-01-04-subject-time-place-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-04-subject-time-place-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-039'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'We saw a movie last weekend.', '우리는 지난 주말에 영화를 보았습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #40: amk500-01-04-subject-time-place-object-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-04-subject-time-place-object-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-040'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'We watched a movie at the theater last weekend.', '우리는 지난 주말에 영화관에서 영화를 보았습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #41: amk500-01-05-subject-indirect-obj-direct-obj-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-05-subject-indirect-obj-direct-obj-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-041'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I gave.', '저는 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #42: amk500-01-05-subject-indirect-obj-direct-obj-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-05-subject-indirect-obj-direct-obj-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-042'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I gave a gift.', '저는 선물을 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #43: amk500-01-05-subject-indirect-obj-direct-obj-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-05-subject-indirect-obj-direct-obj-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-043'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I gave her a gift.', '저는 그녀에게 선물을 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #44: amk500-01-05-subject-indirect-obj-direct-obj-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-05-subject-indirect-obj-direct-obj-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-044'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I gave the cat a fish.', '저는 고양이에게 생선을 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #45: amk500-01-05-subject-indirect-obj-direct-obj-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-05-subject-indirect-obj-direct-obj-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-045'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I gave water to the flowers.', '저는 꽃에 물을 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #46: amk500-01-05-subject-indirect-obj-direct-obj-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-05-subject-indirect-obj-direct-obj-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-046'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I made a call.', '저는 전화를 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #47: amk500-01-05-subject-indirect-obj-direct-obj-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-05-subject-indirect-obj-direct-obj-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-047'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I called my friend.', '저는 친구에게 전화를 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #48: amk500-01-05-subject-indirect-obj-direct-obj-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-05-subject-indirect-obj-direct-obj-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-048'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I called home.', '저는 집에 전화를 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #49: amk500-01-05-subject-indirect-obj-direct-obj-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-05-subject-indirect-obj-direct-obj-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-049'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I called the office.', '저는 회사에 전화를 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #50: amk500-01-05-subject-indirect-obj-direct-obj-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-05-subject-indirect-obj-direct-obj-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-050'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I called the office from home.', '저는 집에서 회사에 전화를 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #51: amk500-01-06-who-when
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-06-who-when'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-051'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Who is Sumi?', '누가 수미입니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #52: amk500-01-06-who-when
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-06-who-when'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-052'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I am Sumi.', '제가 수미입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #53: amk500-01-06-who-when
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-06-who-when'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-053'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Who did you meet yesterday?', '당신은 어제 누구를 만났습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #54: amk500-01-06-who-when
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-06-who-when'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-054'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I met Kyoungjin yesterday.', '저는 어제 경진을 만났습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #55: amk500-01-06-who-when
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-06-who-when'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-055'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Who is that person?', '저 사람은 누구입니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #56: amk500-01-06-who-when
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-06-who-when'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-056'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'That person is my boyfriend.', '저 사람은 제 남자 친구입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #57: amk500-01-06-who-when
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-06-who-when'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-057'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'When do you go to the language institute?', '당신은 언제 어학원에 갑니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #58: amk500-01-06-who-when
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-06-who-when'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-058'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I go to the language institute on Monday.', '저는 월요일에 어학원에 갑니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #59: amk500-01-06-who-when
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-06-who-when'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-059'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'When is your birthday?', '당신의 생일은 언제입니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #60: amk500-01-06-who-when
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-06-who-when'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-060'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My birthday is January 1st.', '제 생일은 1월 1일입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #61: amk500-01-07-where-what
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-07-where-what'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-061'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Where did you go yesterday?', '당신은 어제 어디에 갔습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #62: amk500-01-07-where-what
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-07-where-what'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-062'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I went to Seoul yesterday.', '저는 어제 서울에 갔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #63: amk500-01-07-where-what
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-07-where-what'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-063'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Where do you study Korean?', '당신은 어디에서 한국어를 공부합니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #64: amk500-01-07-where-what
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-07-where-what'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-064'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I study Korean at the language institute.', '저는 어학원에서 한국어를 공부합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #65: amk500-01-07-where-what
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-07-where-what'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-065'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Where is that place?', '저기는 어디입니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #66: amk500-01-07-where-what
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-07-where-what'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-066'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'That is a market.', '저기는 시장입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #67: amk500-01-07-where-what
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-07-where-what'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-067'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'What is this?', '이것은 무엇입니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #68: amk500-01-07-where-what
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-07-where-what'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-068'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This is a fan.', '이것은 부채입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #69: amk500-01-07-where-what
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-07-where-what'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-069'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'What do you like?', '당신은 무엇을 좋아합니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #70: amk500-01-07-where-what
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-07-where-what'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-070'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I like K-pop.', '저는 케이팝을 좋아합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #71: amk500-01-08-what-kind-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-08-what-kind-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-071'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'What kind of food do you like?', '당신은 어떤 음식을 좋아합니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #72: amk500-01-08-what-kind-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-08-what-kind-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-072'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I like bulgogi.', '저는 불고기를 좋아합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #73: amk500-01-08-what-kind-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-08-what-kind-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-073'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'What kind of music do you like?', '당신은 어떤 음악을 좋아합니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #74: amk500-01-08-what-kind-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-08-what-kind-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-074'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I like K-pop.', '저는 케이팝을 좋아합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #75: amk500-01-08-what-kind-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-08-what-kind-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-075'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'What color do you like?', '당신은 어떤 색을 좋아합니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #76: amk500-01-08-what-kind-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-08-what-kind-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-076'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I like blue.', '저는 파란색을 좋아합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #77: amk500-01-08-what-kind-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-08-what-kind-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-077'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'What kind of men do you like?', '당신은 어떤 남자를 좋아합니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #78: amk500-01-08-what-kind-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-08-what-kind-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-078'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I like a kind man.', '저는 친절한 남자를 좋아합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #79: amk500-01-09-and-or-but
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-09-and-or-but'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-079'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I go to the Korean language school on Monday and Wednesday.', '저는 월요일과 수요일에 한국어 학원에 갑니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #80: amk500-01-09-and-or-but
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-09-and-or-but'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-080'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I bought fruits, vegetables, and fish at the market yesterday.', '저는 어제 시장에서 과일과 야채와 생선을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #81: amk500-01-09-and-or-but
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-09-and-or-but'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-081'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I eat bread or rice in the morning.', '저는 아침에 빵이나 밥을 먹습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #82: amk500-01-09-and-or-but
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-09-and-or-but'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-082'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I watch movies or dramas at home on weekends.', '저는 주말에 집에서 영화나 드라마를 봅니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #83: amk500-01-09-and-or-but
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-09-and-or-but'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-083'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She is pretty and smart.', '그녀는 예쁘고 똑똑합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #84: amk500-01-09-and-or-but
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-09-and-or-but'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-084'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Today it is cold, the wind is blowing, and it is raining.', '오늘은 춥고 바람이 불고 비가 옵니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #85: amk500-01-09-and-or-but
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-09-and-or-but'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-085'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I meet friends or exercise on weekends.', '저는 주말에 친구를 만나거나 운동을 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #86: amk500-01-09-and-or-but
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-01-09-and-or-but'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-086'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Kimchi is spicy but delicious.', '김치는 맵지만 맛있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #87: amk500-02-01-noun-predicate
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-01-noun-predicate'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-087'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I am Kim Kyoungjin.', '저는 김경진입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #88: amk500-02-01-noun-predicate
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-01-noun-predicate'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-088'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I am a university student.', '저는 대학생입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #89: amk500-02-01-noun-predicate
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-01-noun-predicate'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-089'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My major is computer science.', '제 전공은 컴퓨터공학입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #90: amk500-02-01-noun-predicate
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-01-noun-predicate'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-090'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My hometown is Seoul.', '제 고향은 서울입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #91: amk500-02-01-noun-predicate
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-01-noun-predicate'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-091'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My hobby is traveling.', '제 취미는 여행입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #92: amk500-02-01-noun-predicate
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-01-noun-predicate'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-092'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Today is Saturday.', '오늘은 토요일입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #93: amk500-02-01-noun-predicate
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-01-noun-predicate'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-093'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Tomorrow is Sunday.', '내일은 일요일입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #94: amk500-02-01-noun-predicate
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-01-noun-predicate'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-094'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This is my laptop.', '이것은 제 노트북입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #95: amk500-02-01-noun-predicate
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-01-noun-predicate'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-095'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Here is Seoul Station.', '여기는 서울역입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #96: amk500-02-01-noun-predicate
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-01-noun-predicate'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-096'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'That is Namsan Mountain.', '저기는 남산입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #97: amk500-02-02-present-progressive
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-02-present-progressive'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-097'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I am going to school.', '저는 학교에 가고 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #98: amk500-02-02-present-progressive
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-02-present-progressive'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-098'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I am waiting for my friend at the coffee shop.', '저는 커피숍에서 친구를 기다리고 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #99: amk500-02-02-present-progressive
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-02-present-progressive'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-099'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I am listening to music.', '저는 음악을 듣고 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #100: amk500-02-02-present-progressive
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-02-present-progressive'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-100'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Snow is falling from the sky.', '하늘에서 눈이 내리고 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #101: amk500-02-02-present-progressive
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-02-present-progressive'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-101'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I am learning Korean these days.', '저는 요즘 한국어를 배우고 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #102: amk500-02-02-present-progressive
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-02-present-progressive'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-102'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She lives in Seoul.', '그녀는 서울에서 살고 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #103: amk500-02-02-present-progressive
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-02-present-progressive'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-103'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The boss is in a meeting now.', '사장님은 지금 회의 중입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #104: amk500-02-02-present-progressive
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-02-present-progressive'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-104'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The manager is on a business trip now.', '과장님은 지금 출장 중입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #105: amk500-02-03-ability-possibility
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-03-ability-possibility'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-105'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Can you speak Korean?', '당신은 한국어를 할 수 있습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #106: amk500-02-03-ability-possibility
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-03-ability-possibility'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-106'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Yes, I can speak Korean.', '예, 저는 한국어를 할 수 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #107: amk500-02-03-ability-possibility
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-03-ability-possibility'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-107'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'No, I can''t speak Korean.', '아니요, 저는 한국어를 할 수 없습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #108: amk500-02-03-ability-possibility
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-03-ability-possibility'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-108'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Do you know how to speak Korean?', '당신은 한국어를 할 줄 압니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #109: amk500-02-03-ability-possibility
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-03-ability-possibility'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-109'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Yes, I know how to speak Korean.', '예, 저는 한국어를 할 줄 압니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #110: amk500-02-03-ability-possibility
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-03-ability-possibility'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-110'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'No, I don''t know how to speak Korean.', '아니요, 저는 한국어를 할 줄 모릅니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #111: amk500-02-03-ability-possibility
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-03-ability-possibility'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-111'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I can swim.', '저는 수영을 할 수 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #112: amk500-02-03-ability-possibility
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-03-ability-possibility'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-112'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I know how to swim.', '저는 수영을 할 줄 압니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #113: amk500-02-03-ability-possibility
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-03-ability-possibility'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-113'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I don''t know how to swim.', '저는 수영을 할 줄 모릅니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #114: amk500-02-03-ability-possibility
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-03-ability-possibility'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-114'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I can go to Seoul tomorrow.', '저는 내일 서울에 갈 수 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #115: amk500-02-04-suggestion
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-04-suggestion'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-115'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Shall we watch a movie this weekend?', '이번 주말에 영화를 볼까요?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #116: amk500-02-04-suggestion
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-04-suggestion'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-116'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Let''s watch a movie this weekend.', '이번 주말에 영화를 봅시다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #117: amk500-02-04-suggestion
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-04-suggestion'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-117'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Where shall we meet tomorrow?', '내일 어디에서 만날까요?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #118: amk500-02-04-suggestion
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-04-suggestion'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-118'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Let''s meet at a coffee shop.', '커피숍에서 만납시다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #119: amk500-02-04-suggestion
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-04-suggestion'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-119'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'What shall we eat?', '무엇을 먹을까요?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #120: amk500-02-04-suggestion
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-04-suggestion'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-120'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Let''s eat bulgogi.', '불고기를 먹읍시다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #121: amk500-02-04-suggestion
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-04-suggestion'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-121'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Shall we leave now?', '지금 출발할까요?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #122: amk500-02-04-suggestion
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-04-suggestion'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-122'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Let''s leave now.', '지금 출발합시다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #123: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-123'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I eat breakfast.', '저는 아침을 먹습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #124: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-124'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I don''t eat breakfast.', '저는 아침을 안 먹습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #125: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-125'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I can''t eat breakfast.', '저는 아침을 못 먹습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #126: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-126'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He plays the piano.', '그는 피아노를 칩니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #127: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-127'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He doesn''t play the piano.', '그는 피아노를 안 칩니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #128: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-128'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He can''t play the piano.', '그는 피아노를 못 칩니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #129: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-129'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I love him.', '저는 그를 사랑합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #130: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-130'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I don''t love him.', '저는 그를 사랑하지 않습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #131: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-131'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I can''t love him.', '저는 그를 사랑하지 못합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #132: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-132'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He is my friend.', '그는 제 친구입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #133: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 11, 'amk500-sent-133'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He is not my friend.', '그는 제 친구가 아닙니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #134: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 12, 'amk500-sent-134'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He is healthy.', '그는 건강합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #135: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 13, 'amk500-sent-135'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He is not healthy.', '그는 건강하지 않습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #136: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 14, 'amk500-sent-136'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Please go to the mountain tomorrow.', '내일 산에 가세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #137: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 15, 'amk500-sent-137'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Don''t go to the mountain tomorrow.', '내일 산에 가지 마세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #138: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 16, 'amk500-sent-138'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Shall we go to the mountain tomorrow?', '내일 산에 갈까요?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #139: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 17, 'amk500-sent-139'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Shall we not go to the mountain tomorrow?', '내일 산에 가지 말까요?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #140: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 18, 'amk500-sent-140'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Let''s go to the mountain tomorrow.', '내일 산에 갑시다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #141: amk500-02-05-negation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-05-negation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 19, 'amk500-sent-141'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Let''s not go to the mountain tomorrow.', '내일 산에 가지 맙시다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #142: amk500-02-06-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-06-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-142'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Where do you want to go?', '당신은 어디에 가고 싶습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #143: amk500-02-06-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-06-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-143'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I want to go to Korea.', '저는 한국에 가고 싶습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #144: amk500-02-06-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-06-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-144'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'When do you want to go?', '당신은 언제 가고 싶습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #145: amk500-02-06-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-06-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-145'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I want to go next fall.', '저는 내년 가을에 가고 싶습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #146: amk500-02-06-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-06-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-146'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'What do you want to eat?', '당신은 무엇을 먹고 싶습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #147: amk500-02-06-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-06-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-147'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I want to eat kimchi.', '저는 김치를 먹고 싶습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #148: amk500-02-06-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-06-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-148'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Who do you want to meet?', '당신은 누구를 만나고 싶습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #149: amk500-02-06-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-06-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-149'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I want to meet BTS.', '저는 BTS를 만나고 싶습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #150: amk500-02-07-i-wish
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-07-i-wish'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-150'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I wish I had a car.', '차가 있었으면 좋겠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #151: amk500-02-07-i-wish
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-07-i-wish'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-151'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I wish I had a lot of money.', '돈이 많았으면 좋겠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #152: amk500-02-07-i-wish
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-07-i-wish'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-152'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I wish I had no worries.', '걱정이 없었으면 좋겠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #153: amk500-02-07-i-wish
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-07-i-wish'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-153'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I hope my parents are healthy.', '부모님이 건강했으면 좋겠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #154: amk500-02-07-i-wish
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-07-i-wish'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-154'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I wish I could speak Korean well.', '한국어를 잘했으면 좋겠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #155: amk500-02-07-i-wish
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-07-i-wish'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-155'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I hope it doesn''t rain tomorrow.', '내일 비가 안 왔으면 좋겠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #156: amk500-02-07-i-wish
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-07-i-wish'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-156'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I hope our team wins.', '우리 팀이 이겼으면 좋겠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #157: amk500-02-08-plan-near-future
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-08-plan-near-future'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-157'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I''m going to go to Korea next year.', '저는 내년에 한국에 가려고 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #158: amk500-02-08-plan-near-future
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-08-plan-near-future'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-158'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I''m going to meet my friend this weekend.', '저는 이번 주말에 친구를 만나려고 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #159: amk500-02-08-plan-near-future
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-08-plan-near-future'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-159'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I''m going to watch a movie this Sunday.', '저는 이번 주 일요일에 영화를 보려고 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #160: amk500-02-08-plan-near-future
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-08-plan-near-future'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-160'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I''m going to buy a car next month.', '저는 다음 달에 차를 사려고 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #161: amk500-02-08-plan-near-future
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-08-plan-near-future'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-161'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I''m going to go to the library this afternoon.', '저는 오늘 오후에 도서관에 가려고 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #162: amk500-02-08-plan-near-future
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-08-plan-near-future'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-162'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'It''s about to rain now.', '지금 비가 오려고 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #163: amk500-02-08-plan-near-future
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-08-plan-near-future'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-163'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The train is about to depart now.', '지금 기차가 출발하려고 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #164: amk500-02-09-decision-promise
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-09-decision-promise'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-164'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I have decided to study abroad next year.', '저는 내년에 유학을 가기로 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #165: amk500-02-09-decision-promise
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-09-decision-promise'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-165'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I have decided to lose weight this year.', '저는 올해 살을 빼기로 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #166: amk500-02-09-decision-promise
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-09-decision-promise'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-166'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I have decided to quit smoking.', '저는 담배를 끊기로 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #167: amk500-02-09-decision-promise
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-09-decision-promise'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-167'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'We decided to meet again tomorrow.', '우리는 내일 다시 만나기로 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #168: amk500-02-09-decision-promise
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-09-decision-promise'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-168'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'We decided to get married next spring.', '우리는 내년 봄에 결혼하기로 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #169: amk500-02-10-modal-permissions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-10-modal-permissions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-169'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'May I take a picture?', '사진을 찍어도 됩니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #170: amk500-02-10-modal-permissions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-10-modal-permissions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-170'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Yes, you may take a picture.', '예, 사진을 찍어도 됩니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #171: amk500-02-10-modal-permissions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-10-modal-permissions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-171'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'No, you must not take a picture.', '아니요, 사진을 찍으면 안 됩니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #172: amk500-02-10-modal-permissions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-10-modal-permissions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-172'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Don''t take a picture.', '사진을 찍지 마세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #173: amk500-02-10-modal-permissions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-10-modal-permissions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-173'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'You must take a picture.', '사진을 찍어야 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #174: amk500-02-10-modal-permissions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-10-modal-permissions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-174'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'You don''t have to take a picture.', '사진을 안 찍어도 됩니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #175: amk500-02-10-modal-permissions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-10-modal-permissions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-175'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'May I sit here?', '여기에 앉아도 됩니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #176: amk500-02-10-modal-permissions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-10-modal-permissions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-176'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'You may sit here.', '여기에 앉아도 됩니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #177: amk500-02-10-modal-permissions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-10-modal-permissions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-177'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'You must not sit here.', '여기에 앉으면 안 됩니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #178: amk500-02-10-modal-permissions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-10-modal-permissions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-178'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Don''t sit here.', '여기에 앉지 마세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #179: amk500-02-10-modal-permissions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-10-modal-permissions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 11, 'amk500-sent-179'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'You must sit here.', '여기에 앉아야 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #180: amk500-02-10-modal-permissions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-10-modal-permissions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 12, 'amk500-sent-180'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'You don''t have to sit here.', '여기에 안 앉아도 됩니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #181: amk500-02-11-request-favor
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-11-request-favor'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-181'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Please help me.', '도와 주세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #182: amk500-02-11-request-favor
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-11-request-favor'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-182'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Could you please help me?', '도와 주시겠습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #183: amk500-02-11-request-favor
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-11-request-favor'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-183'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Please say it again.', '다시 말씀해 주세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #184: amk500-02-11-request-favor
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-11-request-favor'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-184'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Could you please say it again?', '다시 말씀해 주시겠습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #185: amk500-02-11-request-favor
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-11-request-favor'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-185'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Please explain it again.', '다시 설명해 주세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #186: amk500-02-11-request-favor
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-11-request-favor'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-186'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Could you please explain it again?', '다시 설명해 주시겠습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #187: amk500-02-11-request-favor
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-11-request-favor'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-187'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Please wait a moment.', '잠깐만 기다려 주세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #188: amk500-02-11-request-favor
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-11-request-favor'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-188'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Could you please wait a moment?', '잠깐만 기다려 주시겠습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #189: amk500-02-11-request-favor
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-11-request-favor'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-189'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Could you exchange dollars for Korean won?', '달러를 한국 돈으로 바꿔 주시겠습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #190: amk500-02-12-experience
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-12-experience'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-190'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Have you ever been to Korea?', '당신은 한국에 간 적이 있습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #191: amk500-02-12-experience
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-12-experience'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-191'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I have been to Korea.', '저는 한국에 간 적이 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #192: amk500-02-12-experience
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-12-experience'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-192'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I have never been to Korea.', '저는 한국에 간 적이 없습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #193: amk500-02-12-experience
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-12-experience'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-193'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Have you ever been to Korea?', '당신은 한국에 가 보았습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #194: amk500-02-12-experience
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-12-experience'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-194'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I have been to Korea.', '저는 한국에 가 보았습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #195: amk500-02-12-experience
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-12-experience'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-195'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I haven''t been to Korea yet.', '저는 한국에 가 보지 못했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #196: amk500-02-12-experience
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-12-experience'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-196'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Please try this food.', '이 음식을 먹어 보세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #197: amk500-02-12-experience
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-12-experience'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-197'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I have tried making kimchi.', '저는 김치를 만들어 보았습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #198: amk500-02-12-experience
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-12-experience'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-198'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I want to visit Jeju Island someday.', '제주도에 한 번 가 보고 싶습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #199: amk500-02-13-action-for-others
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-13-action-for-others'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-199'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I helped my friend.', '저는 제 친구를 도와 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #200: amk500-02-13-action-for-others
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-13-action-for-others'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-200'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He waited for me.', '그는 저를 기다려 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #201: amk500-02-13-action-for-others
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-13-action-for-others'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-201'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He sang a song for me.', '그는 저에게 노래를 불러 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #202: amk500-02-13-action-for-others
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-13-action-for-others'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-202'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He made Korean food for me.', '그는 저에게 한국 음식을 만들어 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #203: amk500-02-13-action-for-others
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-13-action-for-others'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-203'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The father read a book to his son.', '아버지는 아들에게 책을 읽어 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #204: amk500-02-14-change-of-state
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-14-change-of-state'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-204'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The weather got hot.', '날씨가 더워졌습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #205: amk500-02-14-change-of-state
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-14-change-of-state'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-205'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The weather got cold.', '날씨가 추워졌습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #206: amk500-02-14-change-of-state
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-14-change-of-state'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-206'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'His grades got better.', '그는 성적이 좋아졌습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #207: amk500-02-14-change-of-state
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-14-change-of-state'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-207'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Her skin has improved.', '그녀는 피부가 좋아졌습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #208: amk500-02-14-change-of-state
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-14-change-of-state'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-208'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'His health became bad.', '그는 건강이 나빠졌습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #209: amk500-02-15-change-of-situation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-15-change-of-situation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-209'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I came to like K-pop.', '저는 케이팝을 좋아하게 되었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #210: amk500-02-15-change-of-situation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-15-change-of-situation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-210'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I came to understand Korean culture.', '저는 한국 문화를 이해하게 되었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #211: amk500-02-15-change-of-situation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-15-change-of-situation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-211'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I ended up being hospitalized.', '저는 병원에 입원하게 되었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #212: amk500-02-15-change-of-situation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-15-change-of-situation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-212'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I came to work at another company.', '저는 다른 회사에서 일하게 되었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #213: amk500-02-16-guess-supposition
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-16-guess-supposition'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-213'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'It seems like it rained last night.', '어젯밤에 비가 온 것 같습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #214: amk500-02-16-guess-supposition
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-16-guess-supposition'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-214'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'It seems like it''s raining now.', '지금 비가 오는 것 같습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #215: amk500-02-16-guess-supposition
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-16-guess-supposition'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-215'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'It looks like it will rain tomorrow.', '내일 비가 올 것 같습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #216: amk500-02-16-guess-supposition
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-16-guess-supposition'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-216'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I think this exam will be difficult.', '이번 시험이 어려울 것 같습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #217: amk500-02-16-guess-supposition
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-16-guess-supposition'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-217'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I think the movie will be interesting.', '그 영화는 재미있을 것 같습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #218: amk500-02-17-empathy-guess
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-17-empathy-guess'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-218'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'You must feel good.', '기분이 좋겠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #219: amk500-02-17-empathy-guess
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-17-empathy-guess'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-219'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'That must be heartbreaking.', '마음이 아프겠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #220: amk500-02-17-empathy-guess
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-17-empathy-guess'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-220'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'You must be hungry.', '배가 고프겠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #221: amk500-02-17-empathy-guess
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-17-empathy-guess'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-221'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'You must be happy.', '기쁘겠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #222: amk500-02-17-empathy-guess
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-17-empathy-guess'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-222'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'You must be worried.', '걱정이 많겠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #223: amk500-02-18-exclamation-discovery
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-18-exclamation-discovery'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-223'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The flowers are so beautiful.', '꽃이 참 예쁘네요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #224: amk500-02-18-exclamation-discovery
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-18-exclamation-discovery'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-224'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The scenery is really nice.', '경치가 참 좋네요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #225: amk500-02-18-exclamation-discovery
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-18-exclamation-discovery'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-225'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'It''s so cold today.', '날씨가 참 춥네요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #226: amk500-02-18-exclamation-discovery
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-18-exclamation-discovery'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-226'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This food is really delicious.', '이 음식은 정말 맛있네요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #227: amk500-02-18-exclamation-discovery
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-18-exclamation-discovery'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-227'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The room is really clean!', '방이 정말 깨끗하네요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #228: amk500-02-18-exclamation-discovery
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-18-exclamation-discovery'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-228'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The price is really cheap.', '가격이 정말 싸네요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #229: amk500-02-18-exclamation-discovery
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-18-exclamation-discovery'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-229'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'It''s snowing so much!', '눈이 정말 많이 오네요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #230: amk500-02-18-exclamation-discovery
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-18-exclamation-discovery'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-230'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'You sing really well.', '당신은 노래를 정말 잘하네요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #231: amk500-02-18-exclamation-discovery
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-18-exclamation-discovery'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-231'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'A big restaurant opened over there.', '저기에 큰 식당이 생겼네요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #232: amk500-02-19-confirmation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-19-confirmation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-232'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The weather is really nice today, isn''t it?', '오늘 날씨가 참 좋죠?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #233: amk500-02-19-confirmation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-19-confirmation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-233'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This coffee is really good, isn''t it?', '이 커피 정말 맛있죠?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #234: amk500-02-19-confirmation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-19-confirmation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-234'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The class is at 11 tomorrow, right?', '내일 수업이 11시죠?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #235: amk500-02-19-confirmation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-19-confirmation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-235'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'You were really tired yesterday, weren''t you?', '어제 너무 피곤했죠?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #236: amk500-02-19-confirmation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-19-confirmation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-236'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This song is really good, isn''t it?', '이 노래 정말 좋죠?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #237: amk500-02-20-polite-command-everyday-expressions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-20-polite-command-everyday-expressions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-237'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Hi, how are you?', '안녕하세요?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #238: amk500-02-20-polite-command-everyday-expressions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-20-polite-command-everyday-expressions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-238'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Welcome.', '어서 오세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #239: amk500-02-20-polite-command-everyday-expressions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-20-polite-command-everyday-expressions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-239'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Please sit down here.', '여기에 앉으세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #240: amk500-02-20-polite-command-everyday-expressions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-20-polite-command-everyday-expressions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-240'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'One bibimbap, please.', '비빔밥 하나 주세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #241: amk500-02-20-polite-command-everyday-expressions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-20-polite-command-everyday-expressions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-241'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Enjoy your meal.', '맛있게 드세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #242: amk500-02-20-polite-command-everyday-expressions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-20-polite-command-everyday-expressions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-242'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Goodbye. (to person leaving)', '안녕히 가세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #243: amk500-02-20-polite-command-everyday-expressions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-20-polite-command-everyday-expressions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-243'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Goodbye. (to person staying)', '안녕히 계세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #244: amk500-02-20-polite-command-everyday-expressions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-20-polite-command-everyday-expressions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-244'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Have a nice day.', '좋은 하루 되세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #245: amk500-02-20-polite-command-everyday-expressions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-20-polite-command-everyday-expressions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-245'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Always be happy.', '늘 행복하세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #246: amk500-02-20-polite-command-everyday-expressions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-20-polite-command-everyday-expressions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-246'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Stay healthy always.', '늘 건강하세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #247: amk500-02-20-polite-command-everyday-expressions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-02-20-polite-command-everyday-expressions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 11, 'amk500-sent-247'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Happy new year!', '새해 복 많이 받으세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #248: amk500-03-01-time-when-while
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-01-time-when-while'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-248'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'What did you do during the vacation?', '당신은 방학 때 무엇을 했습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #249: amk500-03-01-time-when-while
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-01-time-when-while'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-249'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I went backpacking during the vacation.', '저는 방학 때 배낭여행을 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #250: amk500-03-01-time-when-while
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-01-time-when-while'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-250'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He did volunteer work during his vacation.', '그는 휴가 때 봉사활동을 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #251: amk500-03-01-time-when-while
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-01-time-when-while'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-251'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He listens to music when he studies.', '그는 공부할 때 음악을 듣습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #252: amk500-03-01-time-when-while
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-01-time-when-while'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-252'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I sing when I''m in a good mood.', '저는 기분이 좋을 때 노래를 부릅니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #253: amk500-03-02-duration
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-02-duration'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-253'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'How long did you live in Korea?', '당신은 얼마 동안 한국에서 살았습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #254: amk500-03-02-duration
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-02-duration'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-254'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I lived in Korea for two years.', '저는 2년 동안 한국에서 살았습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #255: amk500-03-02-duration
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-02-duration'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-255'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She studied Korean for three hours.', '그녀는 세 시간 동안 한국어를 공부했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #256: amk500-03-02-duration
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-02-duration'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-256'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He was in the hospital for a week.', '그는 일주일 동안 병원에 있었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #257: amk500-03-02-duration
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-02-duration'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-257'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I worked part-time during the vacation.', '저는 방학 동안 아르바이트를 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #258: amk500-03-02-duration
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-02-duration'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-258'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I couldn''t eat anything for two days.', '저는 이틀 동안 아무것도 먹지 못했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #259: amk500-03-02-duration
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-02-duration'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-259'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'It snowed while we were sleeping.', '우리가 자는 동안 눈이 왔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #260: amk500-03-03-before
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-03-before'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-260'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The students cleaned the classroom before class.', '학생들은 수업 전에 교실을 청소했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #261: amk500-03-03-before
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-03-before'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-261'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She prepared the materials before the meeting.', '그녀는 회의 전에 자료를 준비했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #262: amk500-03-03-before
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-03-before'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-262'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi plans to get TOPIK Level 6 before graduation.', '수미는 졸업 전에 토픽 6급을 따려고 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #263: amk500-03-03-before
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-03-before'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-263'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She traveled around Southeast Asia before coming to Korea.', '그녀는 한국에 오기 전에 동남아를 여행했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #264: amk500-03-03-before
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-03-before'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-264'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'We booked a hotel before going on the trip.', '우리는 여행하기 전에 호텔을 예약했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #265: amk500-03-03-before
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-03-before'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-265'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He says a prayer before eating.', '그는 식사하기 전에 기도를 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #266: amk500-03-04-after
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-04-after'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-266'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I met my friend after class.', '저는 수업 후에 친구를 만났습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #267: amk500-03-04-after
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-04-after'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-267'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My father drinks coffee after a meal.', '아버지는 식사 후에 커피를 마십니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #268: amk500-03-04-after
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-04-after'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-268'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'After cleaning, the street was clean.', '청소 후에 거리가 깨끗해졌습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #269: amk500-03-04-after
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-04-after'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-269'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He took a shower after exercising.', '그는 운동한 후에 샤워를 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #270: amk500-03-04-after
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-04-after'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-270'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She wrote a book report after reading the book.', '그녀는 책을 읽은 후에 독후감을 썼습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #271: amk500-03-05-sequential-actions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-05-sequential-actions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-271'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My father eats breakfast and then drinks coffee.', '아버지는 아침 식사를 하고 커피를 마십니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #272: amk500-03-05-sequential-actions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-05-sequential-actions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-272'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My father drinks coffee and reads the newspaper.', '아버지는 커피를 마시고 신문을 봅니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #273: amk500-03-05-sequential-actions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-05-sequential-actions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-273'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My father reads the newspaper and then goes to work.', '아버지는 신문을 보고 회사에 갑니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #274: amk500-03-05-sequential-actions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-05-sequential-actions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-274'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She washed her face and then put on makeup.', '그녀는 세수를 하고 화장을 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #275: amk500-03-05-sequential-actions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-05-sequential-actions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-275'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He finished work and met his friend.', '그는 퇴근하고 친구를 만났습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #276: amk500-03-05-sequential-actions
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-05-sequential-actions'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-276'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He cleaned, did laundry, and met a friend on the weekend.', '그는 주말에 청소하고, 빨래하고, 친구를 만났습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #277: amk500-03-06-sequential-causation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-06-sequential-causation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-277'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi went to a café and met her friend.', '수미는 커피숍에 가서 친구를 만났습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #278: amk500-03-06-sequential-causation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-06-sequential-causation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-278'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi met her friend and went to the theater.', '수미는 친구를 만나서 극장에 갔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #279: amk500-03-06-sequential-causation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-06-sequential-causation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-279'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi went to the theater and watched a movie.', '수미는 극장에 가서 영화를 보았습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #280: amk500-03-06-sequential-causation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-06-sequential-causation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-280'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sujae went to the department store and bought a gift.', '수재는 백화점에 가서 선물을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #281: amk500-03-06-sequential-causation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-06-sequential-causation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-281'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sujae bought flowers and gave them to his friend.', '수재는 꽃을 사서 친구에게 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #282: amk500-03-06-sequential-causation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-06-sequential-causation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-282'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She went to the gym and exercised.', '그녀는 체육관에 가서 운동을 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #283: amk500-03-06-sequential-causation
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-06-sequential-causation'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-283'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He went to the bank and withdrew money.', '그는 은행에 가서 돈을 찾았습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #284: amk500-03-07-simultaneous-action
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-07-simultaneous-action'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-284'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I study while listening to music.', '저는 음악을 들으면서 공부를 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #285: amk500-03-07-simultaneous-action
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-07-simultaneous-action'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-285'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My father eats a meal while watching the news.', '아버지는 뉴스를 보면서 식사를 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #286: amk500-03-07-simultaneous-action
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-07-simultaneous-action'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-286'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My mother cooks while singing a song.', '어머니는 노래를 부르면서 요리를 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #287: amk500-03-07-simultaneous-action
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-07-simultaneous-action'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-287'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She reads a book while drinking coffee.', '그녀는 커피를 마시면서 책을 읽습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #288: amk500-03-07-simultaneous-action
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-07-simultaneous-action'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-288'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'We talked while drinking coffee.', '우리는 커피를 마시면서 이야기했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #289: amk500-03-07-simultaneous-action
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-07-simultaneous-action'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-289'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'You should not call while driving.', '운전하면서 전화하면 안 됩니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #290: amk500-03-07-simultaneous-action
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-07-simultaneous-action'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-290'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He met many good people while traveling.', '그는 여행을 하면서 좋은 사람들을 만났습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #291: amk500-03-07-simultaneous-action
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-07-simultaneous-action'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-291'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He is both a singer and an actor.', '그는 가수이면서 영화배우입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #292: amk500-03-08-as-soon-as
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-08-as-soon-as'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-292'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He drank water as soon as he woke up.', '그는 일어나자마자 물을 마셨습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #293: amk500-03-08-as-soon-as
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-08-as-soon-as'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-293'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The power went out as soon as I turned on the computer.', '제가 컴퓨터를 켜자마자 전기가 나갔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #294: amk500-03-08-as-soon-as
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-08-as-soon-as'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-294'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He checked in as soon as he arrived at the airport.', '그는 공항에 도착하자마자 체크인을 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #295: amk500-03-08-as-soon-as
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-08-as-soon-as'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-295'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'We parted as soon as we met.', '우리는 만나자마자 헤어졌습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #296: amk500-03-09-action-transition
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-09-action-transition'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-296'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She fell asleep while watching TV.', '그녀는 텔레비전을 보다가 잠이 들었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #297: amk500-03-09-action-transition
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-09-action-transition'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-297'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He met a friend while walking on the street.', '그는 길을 걷다가 친구를 만났습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #298: amk500-03-09-action-transition
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-09-action-transition'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-298'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He broke a dish while washing the dishes.', '그는 설거지를 하다가 그릇을 깨뜨렸습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #299: amk500-03-09-action-transition
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-09-action-transition'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-299'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He had a nightmare while sleeping.', '그는 잠을 자다가 무서운 꿈을 꾸었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #300: amk500-03-10-from-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-10-from-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-300'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Summer vacation starts today.', '오늘부터 여름방학입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #301: amk500-03-10-from-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-10-from-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-301'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'We have a vacation from this Monday to next Sunday.', '이번 주 월요일부터 다음 주 일요일까지 휴가입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #302: amk500-03-10-from-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-10-from-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-302'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi studied Korean hard from morning to night.', '수미는 아침부터 밤까지 한국어를 열심히 공부했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #303: amk500-03-10-from-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-10-from-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-303'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The exam range is from number 1 to 100.', '시험 범위는 1번부터 100번까지입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #304: amk500-03-10-from-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-10-from-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-304'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The exam time is from 9 AM to 1 PM.', '시험 시간은 오전 9시부터 오후 1시까지입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #305: amk500-03-10-from-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-10-from-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-305'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'It takes three hours from Seoul to Busan by KTX.', '서울에서 부산까지 KTX로 3시간 걸립니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #306: amk500-03-10-from-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-10-from-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-306'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'How much is the taxi fare from Incheon Airport to Seoul City Hall?', '인천공항에서 서울 시청까지 택시비가 얼마입니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #307: amk500-03-11-because-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-11-because-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-307'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The airport was paralyzed because of the heavy snow.', '폭설 때문에 공항이 마비되었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #308: amk500-03-11-because-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-11-because-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-308'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The trip was canceled because of the typhoon.', '태풍 때문에 여행이 취소되었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #309: amk500-03-11-because-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-11-because-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-309'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'There was a car accident because of the fog.', '안개 때문에 교통사고가 났습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #310: amk500-03-11-because-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-11-because-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-310'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The road is blocked because of the car accident.', '교통사고 때문에 길이 막힙니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #311: amk500-03-11-because-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-11-because-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-311'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'It is noisy because of the roadwork.', '도로공사 때문에 시끄럽습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #312: amk500-03-11-because-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-11-because-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-312'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He got lung cancer because of smoking.', '그는 담배 때문에 폐암에 걸렸습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #313: amk500-03-11-because-of
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-11-because-of'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-313'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'These days, the economy is bad because of the exchange rate.', '요즘 환율 때문에 경기가 안 좋습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #314: amk500-03-12-thanks-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-12-thanks-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-314'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I passed the TOPIK exam thanks to my teacher.', '저는 선생님 덕분에 토픽 시험에 합격했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #315: amk500-03-12-thanks-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-12-thanks-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-315'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The world became convenient thanks to AI.', '인공지능 덕분에 세상이 편리해졌습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #316: amk500-03-13-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-13-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-316'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Nice to meet you.', '만나서 반갑습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #317: amk500-03-13-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-13-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-317'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I''m sorry for being late.', '늦어서 죄송합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #318: amk500-03-13-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-13-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-318'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi has many friends because she has a good personality.', '수미는 성격이 좋아서 친구가 많습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #319: amk500-03-13-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-13-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-319'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Why are you going to the hospital?', '당신은 왜 병원에 갑니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #320: amk500-03-13-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-13-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-320'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I''m going to the hospital because I have a stomachache.', '저는 배가 아파서 병원에 갑니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #321: amk500-03-13-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-13-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-321'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Why are you studying Korean?', '당신은 왜 한국어를 공부합니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #322: amk500-03-13-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-13-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-322'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I study Korean because I like K-pop.', '저는 케이팝이 좋아서 한국어를 공부합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #323: amk500-03-13-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-13-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-323'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The road is blocked, so I think I will be late.', '(저는) 길이 많이 막혀서 늦을 것 같습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #324: amk500-03-13-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-13-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-324'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I didn''t know the way, so I took a taxi.', '저는 길을 몰라서 택시를 탔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #325: amk500-03-14-since-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-14-since-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-325'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Since it''s raining a lot, shall we leave work early?', '비가 많이 오니까 일찍 퇴근할까요?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #326: amk500-03-14-since-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-14-since-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-326'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Since it''s raining a lot, let''s leave work early.', '비가 많이 오니까 일찍 퇴근합시다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #327: amk500-03-14-since-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-14-since-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-327'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Since it''s raining a lot, please leave work early.', '비가 많이 오니까 일찍 퇴근하세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #328: amk500-03-14-since-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-14-since-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-328'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Since it''s cold, let''s close the window.', '추우니까 창문을 닫읍시다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #329: amk500-03-14-since-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-14-since-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-329'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Since it''s hot, let''s turn on the air conditioner.', '더우니까 에어컨을 켭시다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #330: amk500-03-14-since-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-14-since-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-330'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Since it''s dangerous, be careful.', '위험하니까 조심하세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #331: amk500-03-14-since-because
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-14-since-because'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-331'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'When I opened the bag, the passport was not there.', '가방을 여니까 여권이 없었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #332: amk500-03-15-for-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-15-for-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-332'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He quit smoking for his health.', '그는 건강을 위해서 담배를 끊었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #333: amk500-03-15-for-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-15-for-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-333'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He bought a gift for his friend.', '그는 친구를 위해서 선물을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #334: amk500-03-15-for-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-15-for-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-334'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The husband bought a gift for his wife.', '남편은 아내를 위해서 선물을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #335: amk500-03-15-for-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-15-for-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-335'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He watches Korean dramas to understand Korean culture.', '그는 한국 문화를 이해하기 위해서 한국 드라마를 봅니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #336: amk500-03-15-for-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-15-for-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-336'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He studied hard to achieve his dream.', '그는 꿈을 이루기 위해서 열심히 공부했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #337: amk500-03-15-for-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-15-for-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-337'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'What do you live for?', '당신은 무엇을 위해서 삽니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #338: amk500-03-16-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-16-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-338'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Why do you study Korean?', '당신은 왜 한국어를 공부합니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #339: amk500-03-16-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-16-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-339'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I study Korean to get a job at a Korean company.', '저는 한국 회사에 취직하려고 한국어를 공부합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #340: amk500-03-16-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-16-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-340'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Why do you go to the hospital?', '당신은 왜 병원에 갑니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #341: amk500-03-16-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-16-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-341'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I go to the hospital to get a health check-up.', '저는 건강 검진을 받으려고 병원에 갑니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #342: amk500-03-16-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-16-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-342'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He bought a gift to give to his wife.', '그는 아내에게 주려고 선물을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #343: amk500-03-16-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-16-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-343'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He went to a coffee shop to meet his friend.', '그는 친구를 만나려고 커피숍에 갔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #344: amk500-03-16-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-16-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-344'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She exercises every day to lose weight.', '그녀는 살을 빼려고 매일 운동합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #345: amk500-03-16-in-order-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-16-in-order-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-345'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She always wears a mask not to catch a cold.', '그녀는 감기에 걸리지 않으려고 항상 마스크를 씁니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #346: amk500-03-17-go-come-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-17-go-come-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-346'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I went to a coffee shop to meet a friend.', '저는 친구를 만나러 커피숍에 갔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #347: amk500-03-17-go-come-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-17-go-come-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-347'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She went to the library to borrow a book.', '그녀는 책을 빌리러 도서관에 갔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #348: amk500-03-17-go-come-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-17-go-come-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-348'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She went to the hair salon to get a haircut.', '그녀는 머리를 자르러 미용실에 갔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #349: amk500-03-17-go-come-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-17-go-come-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-349'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She went to the market to buy fruit.', '그녀는 과일을 사러 시장에 갔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #350: amk500-03-17-go-come-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-17-go-come-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-350'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She went to the bank to withdraw money.', '그녀는 돈을 찾으러 은행에 갔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #351: amk500-03-17-go-come-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-17-go-come-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-351'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He came to Korea to learn Korean.', '그는 한국어를 배우러 한국에 왔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #352: amk500-03-18-conditional-if
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-18-conditional-if'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-352'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'If I earn a lot of money, I want to travel the world.', '저는 돈을 많이 벌면 세계 여행을 하고 싶습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #353: amk500-03-18-conditional-if
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-18-conditional-if'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-353'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'If I drink coffee, I can''t sleep.', '저는 커피를 마시면 잠이 안 옵니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #354: amk500-03-18-conditional-if
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-18-conditional-if'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-354'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'If you are a member, you can get a discount.', '회원이면 할인을 받을 수 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #355: amk500-03-18-conditional-if
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-18-conditional-if'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-355'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'If you enter the password, the door opens.', '비밀번호를 입력하면 문이 열립니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #356: amk500-03-19-if-you-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-19-if-you-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-356'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'To be good at Korean, you need to like the language.', '한국어를 잘하려면 한국어를 좋아해야 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #357: amk500-03-19-if-you-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-19-if-you-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-357'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'To drive, you must have a driver''s license.', '운전을 하려면 운전면허증이 있어야 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #358: amk500-03-19-if-you-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-19-if-you-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-358'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'To get up early, you need to go to bed early.', '일찍 일어나려면 일찍 자야 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #359: amk500-03-19-if-you-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-19-if-you-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-359'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'To get a scholarship, your grades must be good.', '장학금을 타려면 성적이 좋아야 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #360: amk500-03-19-if-you-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-19-if-you-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-360'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'To travel abroad, you must have a passport and visa.', '해외여행을 하려면 여권과 비자가 있어야 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #361: amk500-03-19-if-you-want-to
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-19-if-you-want-to'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-361'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'To meet good people, you need to become a good person.', '좋은 사람을 만나려면 좋은 사람이 되어야 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #362: amk500-03-20-even-if
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-20-even-if'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-362'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Even without money, you can still be happy.', '돈이 없어도 행복할 수 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #363: amk500-03-20-even-if
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-20-even-if'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-363'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Even if you have a lot of money, you might not be happy.', '돈이 많아도 행복하지 않을 수 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #364: amk500-03-20-even-if
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-20-even-if'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-364'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'No matter how much medicine I take, my cold doesn''t get better.', '아무리 약을 먹어도 감기가 낫지 않습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #365: amk500-03-20-even-if
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-03-20-even-if'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-365'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'No matter how much I exercise, I don''t lose weight.', '아무리 운동을 해도 살이 빠지지 않습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #366: amk500-04-01-background-context
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-01-background-context'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-366'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'It is raining, but I don''t have an umbrella.', '비가 오는데 저는 우산이 없습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #367: amk500-04-01-background-context
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-01-background-context'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-367'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I played table tennis with Sumi, but I lost.', '수미와 탁구를 쳤는데 제가 졌습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #368: amk500-04-01-background-context
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-01-background-context'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-368'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I got on the bus, but there were no seats.', '(제가) 버스를 탔는데 자리가 없었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #369: amk500-04-01-background-context
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-01-background-context'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-369'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I am busy today, so can you call me tomorrow?', '오늘은 바쁜데 내일 연락 주시겠습니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #370: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-370'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This apple is big.', '이 사과는 큽니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #371: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-371'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This apple is as big as a watermelon.', '이 사과는 수박처럼 큽니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #372: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-372'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This apple is bigger than a watermelon.', '이 사과는 수박보다 더 큽니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #373: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-373'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This apple is less big than a watermelon.', '이 사과는 수박보다 덜 큽니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #374: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-374'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This apple is the biggest in Korea.', '이 사과는 한국에서 가장 큽니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #375: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-375'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This apple is the biggest among the apples.', '이 사과는 사과들 중에서 가장 큽니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #376: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-376'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi sings well.', '수미는 노래를 잘합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #377: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-377'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi sings well like a singer.', '수미는 가수처럼 노래를 잘합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #378: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-378'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi sings better than a singer.', '수미는 가수보다 더 노래를 잘합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #379: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-379'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi sings the best in our class.', '수미는 우리 반에서 가장 노래를 잘합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #380: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 11, 'amk500-sent-380'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi sings the best among the students.', '수미는 우리 반 학생들 중에서 가장 노래를 잘합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #381: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 12, 'amk500-sent-381'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Today is colder than yesterday.', '오늘은 어제보다 더 춥습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #382: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 13, 'amk500-sent-382'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I like fall more than spring.', '저는 봄보다 가을을 더 좋아합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #383: amk500-04-02-comparative-superlative
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-02-comparative-superlative'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 14, 'amk500-sent-383'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Mother''s love is higher than the sky and deeper than the sea.', '어머니의 사랑은 하늘보다 더 높고 바다보다 더 깊습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #384: amk500-04-03-verb-adj-noun
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-03-verb-adj-noun'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-384'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Learning Korean is fun.', '한국어를 배우는 것은 재미있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #385: amk500-04-03-verb-adj-noun
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-03-verb-adj-noun'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-385'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He likes cooking.', '그는 요리하는 것을 좋아합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #386: amk500-04-03-verb-adj-noun
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-03-verb-adj-noun'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-386'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Tomorrow, we have speaking, listening, reading, and writing tests.', '내일 말하기, 듣기, 읽기, 쓰기 시험이 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #387: amk500-04-03-verb-adj-noun
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-03-verb-adj-noun'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-387'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Love is waiting.', '사랑은 기다림입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #388: amk500-04-04-adjective-adverb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-04-adjective-adverb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-388'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Enjoy your meal.', '맛있게 드세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #389: amk500-04-04-adjective-adverb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-04-adjective-adverb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-389'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I enjoyed the meal.', '맛있게 먹었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #390: amk500-04-04-adjective-adverb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-04-adjective-adverb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-390'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She writes beautifully.', '그녀는 글씨를 예쁘게 씁니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #391: amk500-04-04-adjective-adverb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-04-adjective-adverb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-391'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'That store sells clothes cheaply.', '저 가게는 옷을 싸게 팝니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #392: amk500-04-04-adjective-adverb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-04-adjective-adverb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-392'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He explained it to me kindly.', '그는 저에게 친절하게 설명해 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #393: amk500-04-04-adjective-adverb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-04-adjective-adverb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-393'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She washed the apple cleanly.', '그녀는 사과를 깨끗이 씻었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #394: amk500-04-04-adjective-adverb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-04-adjective-adverb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-394'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Please rest comfortably.', '편히 쉬세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #395: amk500-04-04-adjective-adverb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-04-adjective-adverb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-395'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He earned a lot of money.', '그는 돈을 많이 벌었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #396: amk500-04-04-adjective-adverb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-04-adjective-adverb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-396'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He threw the ball far.', '그는 공을 멀리 던졌습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #397: amk500-04-05-adjective-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-05-adjective-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-397'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I like him.', '저는 그를 좋아합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #398: amk500-04-05-adjective-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-05-adjective-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-398'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He dislikes me.', '그는 저를 싫어합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #399: amk500-04-05-adjective-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-05-adjective-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-399'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My grandmother adores me.', '할머니는 저를 예뻐합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #400: amk500-04-05-adjective-verb
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-05-adjective-verb'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-400'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She is afraid of snakes.', '그녀는 뱀을 무서워합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #401: amk500-04-06-only
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-06-only'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-401'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'There is milk in the refrigerator.', '냉장고에 우유가 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #402: amk500-04-06-only
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-06-only'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-402'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'There is only milk in the refrigerator.', '냉장고에 우유만 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #403: amk500-04-06-only
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-06-only'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-403'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'There is nothing except milk in the refrigerator.', '냉장고에 우유밖에 없습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #404: amk500-04-06-only
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-06-only'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-404'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I exercise in the morning.', '저는 아침에 운동을 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #405: amk500-04-06-only
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-06-only'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-405'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Only I exercise in the morning.', '저만 아침에 운동을 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #406: amk500-04-06-only
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-06-only'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-406'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I exercise only in the morning.', '저는 아침에만 운동을 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #407: amk500-04-06-only
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-06-only'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-407'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I only exercise in the morning.', '저는 아침에 운동만 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #408: amk500-04-07-method-direction
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-07-method-direction'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-408'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He went back to his hometown.', '그는 고향으로 돌아갔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #409: amk500-04-07-method-direction
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-07-method-direction'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-409'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He goes to school by bicycle.', '그는 자전거로 학교에 갑니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #410: amk500-04-07-method-direction
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-07-method-direction'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-410'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Kimchi is made with cabbage.', '김치는 배추로 만듭니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #411: amk500-04-07-method-direction
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-07-method-direction'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-411'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Because of COVID-19, the trip was canceled.', '코로나로 여행이 취소되었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #412: amk500-04-07-method-direction
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-07-method-direction'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-412'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He is working as an interpreter.', '그는 통역사로 일하고 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #413: amk500-04-07-method-direction
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-07-method-direction'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-413'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I''d like to exchange dollars for Korean won, please.', '달러를 한국 돈으로 바꿔 주세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #414: amk500-04-08-adverbs-of-frequency
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-08-adverbs-of-frequency'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-414'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He sometimes goes to the beach.', '그는 가끔 해변에 갑니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #415: amk500-04-08-adverbs-of-frequency
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-08-adverbs-of-frequency'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-415'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He often goes to the coffee shop.', '그는 자주 커피숍에 갑니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #416: amk500-04-08-adverbs-of-frequency
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-08-adverbs-of-frequency'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-416'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He always sings.', '그는 항상 노래를 부릅니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #417: amk500-04-08-adverbs-of-frequency
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-08-adverbs-of-frequency'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-417'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He exercises every morning.', '그는 아침마다 운동을 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #418: amk500-04-08-adverbs-of-frequency
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-08-adverbs-of-frequency'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-418'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He goes to a coffee shop every day.', '그는 날마다 커피숍에 갑니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #419: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-419'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'There are four students in the classroom.', '교실에 학생이 네 명 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #420: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-420'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'There are two apples in the refrigerator.', '냉장고에 사과가 두 개 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #421: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-421'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'There is one dog in the house.', '집에 개가 한 마리 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #422: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-422'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He bought two books.', '그는 책 두 권을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #423: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-423'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She booked two movie tickets.', '그녀는 영화표 두 장을 예매했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #424: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-424'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Kyoungjin drank two bottles of cola.', '경진은 콜라를 두 병 마셨습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #425: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-425'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Please give me one cup of water.', '물 한 컵 주세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #426: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-426'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Please give me one cup of coffee.', '커피 한 잔 주세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #427: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-427'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Please give me one sheet of paper.', '종이 한 장 주세요.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #428: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-428'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'There are five cars in the parking lot.', '주차장에 자동차가 다섯 대 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #429: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 11, 'amk500-sent-429'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'There is one ship in the sea.', '바다에 배가 한 척 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #430: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 12, 'amk500-sent-430'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'There are ten trees in the garden.', '정원에 나무가 열 그루 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #431: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 13, 'amk500-sent-431'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He gave me one flower.', '그는 나에게 꽃 한 송이를 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #432: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 14, 'amk500-sent-432'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I gave him a bouquet of flowers.', '저는 그에게 꽃 한 다발을 주었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #433: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 15, 'amk500-sent-433'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I''m going to watch two movies this weekend.', '저는 이번 주말에 영화를 두 편 보려고 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #434: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 16, 'amk500-sent-434'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'There are three pairs of shoes at home.', '집에 구두가 세 켤레 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #435: amk500-04-09-classifiers
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-09-classifiers'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 17, 'amk500-sent-435'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He had one suit tailored.', '그는 양복 한 벌을 맞추었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #436: amk500-04-10-rieul-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-10-rieul-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-436'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I know her.', '저는 그녀를 압니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #437: amk500-04-10-rieul-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-10-rieul-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-437'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'What are you making now?', '당신은 지금 무엇을 만듭니까?' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #438: amk500-04-10-rieul-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-10-rieul-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-438'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He sells vegetables and fruits at the market.', '그는 시장에서 야채와 과일을 팝니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #439: amk500-04-10-rieul-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-10-rieul-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-439'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The children play in the park.', '아이들이 공원에서 놉니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #440: amk500-04-10-rieul-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-10-rieul-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-440'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I live in Seoul.', '저는 서울에서 삽니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #441: amk500-04-11-eu-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-11-eu-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-441'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I was heartbroken.', '저는 마음이 아팠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #442: amk500-04-11-eu-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-11-eu-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-442'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I was very happy to meet my hometown friend.', '저는 고향 친구를 만나서 너무 기뻤습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #443: amk500-04-11-eu-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-11-eu-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-443'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I was busy last week, so I couldn''t exercise.', '저는 지난주에 바빠서 운동을 할 수 없었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #444: amk500-04-11-eu-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-11-eu-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-444'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I wrote a letter to her.', '저는 그녀에게 편지를 썼습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #445: amk500-04-12-reu-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-12-reu-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-445'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He cut the tree.', '그는 나무를 잘랐습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #446: amk500-04-12-reu-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-12-reu-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-446'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He was different from ordinary people.', '그는 보통 사람과 달랐습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #447: amk500-04-12-reu-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-12-reu-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-447'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He sang a song.', '그는 노래를 불렀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #448: amk500-04-12-reu-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-12-reu-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-448'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He was faster than me.', '그는 나보다 빨랐습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #449: amk500-04-13-bieup-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-13-bieup-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-449'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'It was so hot yesterday.', '어제는 너무 더웠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #450: amk500-04-13-bieup-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-13-bieup-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-450'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'It was so cold yesterday.', '어제는 너무 추웠습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #451: amk500-04-13-bieup-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-13-bieup-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-451'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He helped me.', '그는 저를 도왔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #452: amk500-04-13-bieup-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-13-bieup-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-452'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He wore a hanbok.', '그는 한복을 입었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #453: amk500-04-13-bieup-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-13-bieup-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-453'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The road was narrow.', '그 길은 좁았습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #454: amk500-04-14-digeut-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-14-digeut-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-454'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He asked me my name.', '그는 나에게 이름을 물었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #455: amk500-04-14-digeut-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-14-digeut-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-455'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I have heard that song before.', '저는 그 노래를 들은 적이 있습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #456: amk500-04-14-digeut-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-14-digeut-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-456'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He goes to school on foot.', '그는 걸어서 학교에 갑니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #457: amk500-04-14-digeut-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-14-digeut-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-457'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He closed the window.', '그는 창문을 닫았습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #458: amk500-04-14-digeut-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-14-digeut-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-458'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He trusted me.', '그는 저를 믿었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #459: amk500-04-14-digeut-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-14-digeut-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-459'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I received a book as a birthday present.', '저는 생일 선물로 책을 받았습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #460: amk500-04-15-siot-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-15-siot-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-460'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He cooked rice.', '그는 밥을 지었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #461: amk500-04-15-siot-irregular
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-15-siot-irregular'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-461'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He has fully recovered from his illness.', '그는 병이 다 나았습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #462: amk500-04-16-indirect-speech
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-16-indirect-speech'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-462'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He said, ''The weather is too hot.''', '그는 “날씨가 너무 덥습니다.”라고 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #463: amk500-04-16-indirect-speech
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-16-indirect-speech'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-463'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He said the weather was too hot.', '그는 날씨가 너무 덥다고 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #464: amk500-04-16-indirect-speech
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-16-indirect-speech'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-464'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He said, ''I am an office worker.''', '그는 “저는 회사원입니다.”라고 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #465: amk500-04-16-indirect-speech
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-16-indirect-speech'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-465'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He said that he was an office worker.', '그는 자신이 회사원이라고 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #466: amk500-04-16-indirect-speech
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-16-indirect-speech'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-466'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He told me, ''Please come quickly.''', '그는 저에게 “빨리 오세요.”라고 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #467: amk500-04-16-indirect-speech
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-16-indirect-speech'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-467'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He told me to come quickly.', '그는 저에게 빨리 오라고 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #468: amk500-04-16-indirect-speech
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-16-indirect-speech'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-468'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He asked me what time I would come tomorrow.', '그는 저에게 내일 몇 시에 오냐고 물었습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #469: amk500-04-16-indirect-speech
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-16-indirect-speech'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-469'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He told me not to drink alcohol.', '그는 저에게 술을 마시지 말라고 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #470: amk500-04-16-indirect-speech
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-16-indirect-speech'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-470'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He said to me, ''Let''s get married.''', '그는 저에게 “결혼합시다.”라고 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #471: amk500-04-16-indirect-speech
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-16-indirect-speech'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-471'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'He asked me to marry him.', '그는 저에게 결혼하자고 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #472: amk500-04-16-indirect-speech
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-16-indirect-speech'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 11, 'amk500-sent-472'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'She said that she is busy now.', '그녀는 지금 바쁘다고 했습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #473: amk500-04-16-indirect-speech
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-16-indirect-speech'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 12, 'amk500-sent-473'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'They say it will rain tomorrow.', '내일은 비가 온다고 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #474: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-474'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This is the school I attended.', '여기는 제가 다닌 학교입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #475: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-475'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This is the school I attend.', '여기는 제가 다니는 학교입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #476: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-476'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This is the school I will attend.', '여기는 제가 다닐 학교입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #477: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-477'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This is my favorite place.', '여기는 제가 가장 좋아하는 장소입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #478: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-478'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This is the book I am reading.', '이것은 제가 읽는 책입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #479: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-479'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This is the book I will read.', '이것은 제가 읽을 책입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #480: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-480'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'This is the book that I read.', '이것은 제가 읽은 책입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #481: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-481'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi bought clothes at the department store yesterday.', '수미는 어제 백화점에서 옷을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #482: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-482'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi with long hair bought clothes at the department store yesterday.', '머리가 긴 수미는 어제 백화점에서 옷을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #483: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-483'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi bought clothes at the department store yesterday when it rained a lot.', '수미는 비가 많이 온 어제 백화점에서 옷을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #484: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 11, 'amk500-sent-484'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi bought clothes at the department store next to the station yesterday.', '수미는 어제 역 옆에 있는 백화점에서 옷을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #485: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 12, 'amk500-sent-485'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi bought clothes at the department store yesterday to wear at her friend''s wedding.', '수미는 어제 백화점에서 친구의 결혼식에서 입을 옷을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #486: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 13, 'amk500-sent-486'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Sumi with long hair bought clothes at the department store next to the station yesterday, when it rained a lot, to wear at her friend''s wedding.', '머리가 긴 수미는 비가 많이 온 어제 역 옆에 있는 백화점에서 친구의 결혼식에서 입을 옷을 샀습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #487: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 14, 'amk500-sent-487'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The most important moment in life is now.', '인생에서 가장 중요한 순간은 지금입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #488: amk500-04-17-modifier
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-17-modifier'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 15, 'amk500-sent-488'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The most important person in life is the person I am meeting now.', '인생에서 가장 중요한 사람은 지금 내가 만나고 있는 사람입니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #489: amk500-04-18-honorifics
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-18-honorifics'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 1, 'amk500-sent-489'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My mother is cooking.', '어머니께서 요리를 하십니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #490: amk500-04-18-honorifics
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-18-honorifics'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 2, 'amk500-sent-490'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My grandmother is having a meal.', '할머니께서 진지를 드시고 계십니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #491: amk500-04-18-honorifics
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-18-honorifics'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 3, 'amk500-sent-491'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I gave a gift to my mother.', '저는 어머니께 선물을 드렸습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #492: amk500-04-18-honorifics
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-18-honorifics'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 4, 'amk500-sent-492'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My mother gave me a gift.', '어머니께서 저에게 선물을 주셨습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #493: amk500-04-18-honorifics
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-18-honorifics'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 5, 'amk500-sent-493'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'The boss spoke to the employees.', '사장님께서 직원들에게 말씀하셨습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #494: amk500-04-18-honorifics
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-18-honorifics'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 6, 'amk500-sent-494'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My great-grandmother passed away this morning.', '오늘 아침에 증조할머니께서 돌아가셨습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #495: amk500-04-18-honorifics
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-18-honorifics'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 7, 'amk500-sent-495'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My parents are in their hometown.', '제 부모님은 지금 고향에 계십니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #496: amk500-04-18-honorifics
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-18-honorifics'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 8, 'amk500-sent-496'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My grandfather is sleeping.', '할아버지께서 주무시고 계십니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #497: amk500-04-18-honorifics
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-18-honorifics'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 9, 'amk500-sent-497'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I''m going to meet my teacher tomorrow.', '저는 내일 선생님을 뵈려고 합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #498: amk500-04-18-honorifics
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-18-honorifics'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 10, 'amk500-sent-498'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'My grandmother is sick these days.', '요즘 할머니께서 편찮으십니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #499: amk500-04-18-honorifics
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-18-honorifics'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 11, 'amk500-sent-499'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'I took my grandmother to the hospital.', '저는 할머니를 모시고 병원에 갔습니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

-- #500: amk500-04-18-honorifics
WITH new_task AS (
    INSERT INTO study_task (
        study_id, updated_by_user_id, study_task_kind, study_task_seq, study_task_idx
    ) VALUES (
        (SELECT study_id FROM study WHERE study_idx = 'amk500-04-18-honorifics'),
        (SELECT user_id FROM users WHERE user_auth = 'HYMN' LIMIT 1),
        'typing', 12, 'amk500-sent-500'
    )
    ON CONFLICT (study_task_idx) DO UPDATE SET study_task_updated_at = NOW()
    RETURNING study_task_id
)
INSERT INTO study_task_typing (
    study_task_id, study_task_typing_question, study_task_typing_answer
)
SELECT study_task_id, 'Boss, congratulations on your son''s wedding and your daughter''s graduation.', '사장님, 아드님의 결혼과 따님의 졸업을 축하합니다.' FROM new_task
ON CONFLICT (study_task_id) DO UPDATE SET
    study_task_typing_question = EXCLUDED.study_task_typing_question,
    study_task_typing_answer = EXCLUDED.study_task_typing_answer;

