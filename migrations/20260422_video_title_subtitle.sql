-- =============================================================================
-- 20260422: video 테이블에 video_title, video_subtitle 물리 컬럼 추가
-- =============================================================================
-- 배경 (Q1c 결정 B, 2026-04-21 사용자 결정):
--   video 테이블엔 title/subtitle 컬럼이 없어 Consumer `/videos`·`/videos/{id}`
--   가 MAX(video_tag_title)/MAX(video_tag_subtitle) 집계로 파생해 왔음.
--   video_tag 는 "분류" 용도인데 실제 "비디오 제목" 으로 혼용되는 의미 불일치.
--   M05~M08 교재 시딩 본격화 전 정리. Q1b 의 find_source_fields
--   source_text=None stub 부채도 여기서 해소.
--
-- 변경:
--   1. video_title VARCHAR(150) NOT NULL DEFAULT '' 추가
--   2. video_subtitle VARCHAR(250) NULL 추가
--   3. 백필: 기존 video_tag_title MAX / video_tag_subtitle MAX 를 초기값으로
--   4. video_title DEFAULT '' 제거 (백필 후 NOT NULL 유지)
--
-- 참조: plans/translation-field-name-alignment.md §4 Q1c 결정 B
-- =============================================================================

BEGIN;

-- 1. 컬럼 추가 (DEFAULT '' 은 NOT NULL 제약 만족용 임시값)
ALTER TABLE video
    ADD COLUMN video_title    VARCHAR(150) NOT NULL DEFAULT '',
    ADD COLUMN video_subtitle VARCHAR(250);

-- 2. 백필: video_tag_title/subtitle 의 MAX 집계로 기존 영상 제목 복원
UPDATE video v
SET
    video_title = COALESCE(
        (SELECT MAX(vt.video_tag_title)
         FROM video_tag vt
         JOIN video_tag_map vtm ON vtm.video_tag_id = vt.video_tag_id
         WHERE vtm.video_id = v.video_id),
        ''
    ),
    video_subtitle = (
        SELECT MAX(vt.video_tag_subtitle)
        FROM video_tag vt
        JOIN video_tag_map vtm ON vtm.video_tag_id = vt.video_tag_id
        WHERE vtm.video_id = v.video_id
    );

-- 3. 백필 완료 후 DEFAULT 제거 (신규 행은 명시적 입력 요구)
ALTER TABLE video
    ALTER COLUMN video_title DROP DEFAULT;

COMMIT;
