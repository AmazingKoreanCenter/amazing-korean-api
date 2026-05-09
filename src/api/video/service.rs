use std::collections::HashMap;

use validator::Validate;

use crate::api::admin::translation::dto::{TranslatedField, TranslationMeta};
use crate::api::admin::translation::repo::TranslationRepo;
use crate::api::video::dto::{
    VideoDetailRes, VideoListMeta, VideoListReq, VideoListRes, VideoProgressRes,
    VideoProgressUpdateReq, VideoTagDetail,
};
use crate::api::video::repo::VideoRepo;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::types::{ContentType, SupportedLanguage};

pub struct VideoService;

impl VideoService {
    /// 비디오 목록 조회 (검색 + 페이징 + 필터)
    pub async fn list_videos(st: &AppState, req: VideoListReq) -> AppResult<VideoListRes> {
        // 1. Validation
        if let Err(e) = req.validate() {
            return Err(AppError::BadRequest(e.to_string()));
        }

        // 2. Repo Call (Data + Total Count)
        let (mut data, total_count) = VideoRepo::list_videos(&st.db, &req).await?;

        // 2-1. 번역 주입 + 메타 계산 (Q1c A)
        let translation_meta = match req.lang {
            None => TranslationMeta::not_requested(),
            Some(SupportedLanguage::Ko) => TranslationMeta::ko_full(),
            Some(user_lang) => {
                let ids: Vec<i64> = data.iter().map(|v| v.video_id).collect();
                let translations = TranslationRepo::find_translations_for_contents(
                    &st.db,
                    ContentType::Video,
                    &ids,
                    user_lang,
                )
                .await?;

                // Gemini 3차 리뷰 반영: title/subtitle 모두 Option — source 에 있을 때만 카운트.
                let mut translated = 0usize;
                let mut fallback = 0usize;
                let mut requested = 0usize;
                for item in data.iter_mut() {
                    if item.title.is_some() {
                        requested += 1;
                        if let Some(t) =
                            translations.get(&(item.video_id, "video_title".to_string()))
                        {
                            item.title = Some(t.text.clone());
                            t.count_to(user_lang, &mut translated, &mut fallback);
                        }
                    }
                    if item.subtitle.is_some() {
                        requested += 1;
                        if let Some(t) =
                            translations.get(&(item.video_id, "video_subtitle".to_string()))
                        {
                            item.subtitle = Some(t.text.clone());
                            t.count_to(user_lang, &mut translated, &mut fallback);
                        }
                    }
                }
                TranslationMeta::from_counts(user_lang, requested, translated, fallback)
            }
        };

        // 3. Calc Meta
        let total_pages = if total_count == 0 {
            0
        } else {
            (total_count + req.per_page as i64 - 1) / req.per_page as i64
        };

        Ok(VideoListRes {
            meta: VideoListMeta {
                total_count,
                total_pages,
                current_page: req.page,
                per_page: req.per_page,
            },
            translation_meta,
            data,
        })
    }

    /// 비디오 상세 조회
    pub async fn get_video_detail(
        st: &AppState,
        video_id: i64,
        lang: Option<SupportedLanguage>,
    ) -> AppResult<VideoDetailRes> {
        let mut video = VideoRepo::get_video_detail(&st.db, video_id)
            .await?
            .ok_or(AppError::NotFound)?;

        let translation_meta = match lang {
            None => TranslationMeta::not_requested(),
            Some(SupportedLanguage::Ko) => TranslationMeta::ko_full(),
            Some(user_lang) => {
                // Gemini 3차 리뷰 반영: video title/subtitle + tag title/subtitle 모두
                // Option — source 에 있을 때만 카운트.
                let mut translated = 0usize;
                let mut fallback = 0usize;
                let mut requested = 0usize;

                // Video 레벨 번역 (content_type=video)
                let video_translations = TranslationRepo::find_translations_for_contents(
                    &st.db,
                    ContentType::Video,
                    &[video.video_id],
                    user_lang,
                )
                .await?;

                if video.title.is_some() {
                    requested += 1;
                    if let Some(t) =
                        video_translations.get(&(video.video_id, "video_title".to_string()))
                    {
                        video.title = Some(t.text.clone());
                        t.count_to(user_lang, &mut translated, &mut fallback);
                    }
                }
                if video.subtitle.is_some() {
                    requested += 1;
                    if let Some(t) =
                        video_translations.get(&(video.video_id, "video_subtitle".to_string()))
                    {
                        video.subtitle = Some(t.text.clone());
                        t.count_to(user_lang, &mut translated, &mut fallback);
                    }
                }

                // Video 태그 번역 (Q1c C) — 상세 응답 tags[] 에만 적용
                let tag_ids: Vec<i64> = video.tags.iter().map(|t| t.id).collect();
                if !tag_ids.is_empty() {
                    let tag_translations = TranslationRepo::find_translations_for_contents(
                        &st.db,
                        ContentType::VideoTag,
                        &tag_ids,
                        user_lang,
                    )
                    .await?;

                    apply_tag_translations(
                        &mut video.tags.0,
                        &tag_translations,
                        user_lang,
                        &mut translated,
                        &mut fallback,
                        &mut requested,
                    );
                }

                TranslationMeta::from_counts(user_lang, requested, translated, fallback)
            }
        };

        video.translation_meta = translation_meta;
        Ok(video)
    }

    /// 내 진도율 조회
    pub async fn get_video_progress(
        st: &AppState,
        user_id: i64,
        video_id: i64,
    ) -> AppResult<VideoProgressRes> {
        // 1. 비디오 존재 확인
        let exists = VideoRepo::exists_by_id(&st.db, video_id).await?;
        if !exists {
            return Err(AppError::NotFound);
        }

        // 2. 진도율 조회
        let progress = VideoRepo::get_progress(&st.db, user_id, video_id).await?;

        // 3. 없으면 기본값 반환 (0%)
        Ok(progress.unwrap_or(VideoProgressRes {
            video_id,
            progress_rate: 0,
            is_completed: false,
            last_watched_at: None,
            watch_duration_sec: 0,
        }))
    }

    /// 내 진도율 업데이트
    pub async fn update_video_progress(
        st: &AppState,
        user_id: i64,
        video_id: i64,
        req: VideoProgressUpdateReq,
    ) -> AppResult<VideoProgressRes> {
        // 1. Validation
        if let Err(e) = req.validate() {
            return Err(AppError::Unprocessable(e.to_string()));
        }

        // 2. 비디오 존재 확인
        let exists = VideoRepo::exists_by_id(&st.db, video_id).await?;
        if !exists {
            return Err(AppError::NotFound);
        }

        // 3. 기존 진도 조회 (통계 판단용)
        let existing = VideoRepo::get_progress(&st.db, user_id, video_id).await?;

        // 4. 통계 판단
        let is_new_view = existing.is_none(); // 최초 시청 여부
        let was_completed = existing.as_ref().map(|p| p.is_completed).unwrap_or(false);
        let is_completed = req.progress_rate >= 100;
        let is_new_complete = !was_completed && is_completed; // 이번에 처음 완료

        // 5. Upsert Log (with is_new_view flag)
        let res = VideoRepo::update_progress(
            &st.db,
            user_id,
            video_id,
            req.progress_rate,
            is_completed,
            is_new_view,
            req.watch_duration_sec,
        )
        .await?;

        // 6. 일별 통계 업데이트
        if is_new_view {
            // 최초 시청 시 views 증가
            VideoRepo::increment_daily_views(&st.db, video_id).await?;
        }
        if is_new_complete {
            // 처음 완료 시 completes 증가
            VideoRepo::increment_daily_completes(&st.db, video_id).await?;
        }

        Ok(res)
    }
}

/// VideoTag 번역을 tags[] 에 주입 (Q1c C).
/// Gemini 3차 리뷰 반영: tag.title/subtitle 은 Option — source 에 있을 때만 카운트.
fn apply_tag_translations(
    tags: &mut [VideoTagDetail],
    translations: &HashMap<(i64, String), TranslatedField>,
    user_lang: SupportedLanguage,
    translated: &mut usize,
    fallback: &mut usize,
    requested: &mut usize,
) {
    for tag in tags.iter_mut() {
        if tag.title.is_some() {
            *requested += 1;
            if let Some(t) = translations.get(&(tag.id, "video_tag_title".to_string())) {
                tag.title = Some(t.text.clone());
                t.count_to(user_lang, translated, fallback);
            }
        }
        if tag.subtitle.is_some() {
            *requested += 1;
            if let Some(t) = translations.get(&(tag.id, "video_tag_subtitle".to_string())) {
                tag.subtitle = Some(t.text.clone());
                t.count_to(user_lang, translated, fallback);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tag(id: i64, title: Option<&str>, subtitle: Option<&str>) -> VideoTagDetail {
        VideoTagDetail {
            id,
            key: None,
            title: title.map(String::from),
            subtitle: subtitle.map(String::from),
        }
    }

    fn make_translation(text: &str, actual_lang: SupportedLanguage) -> TranslatedField {
        TranslatedField {
            text: text.to_string(),
            actual_lang,
            fallback_used: actual_lang != SupportedLanguage::Ko,
        }
    }

    #[test]
    fn test_apply_tag_translations_no_translations_keeps_originals() {
        let mut tags = vec![make_tag(1, Some("orig"), Some("orig sub"))];
        let translations = HashMap::new();
        let (mut translated, mut fallback, mut requested) = (0, 0, 0);

        apply_tag_translations(
            &mut tags,
            &translations,
            SupportedLanguage::Ja,
            &mut translated,
            &mut fallback,
            &mut requested,
        );

        assert_eq!(tags[0].title.as_deref(), Some("orig"));
        assert_eq!(tags[0].subtitle.as_deref(), Some("orig sub"));
        assert_eq!(translated, 0, "no translations applied");
        assert_eq!(fallback, 0, "no fallback applied");
        assert_eq!(requested, 2, "title+subtitle requested but not found");
    }

    #[test]
    fn test_apply_tag_translations_replaces_title_when_match() {
        let mut tags = vec![make_tag(1, Some("orig"), None)];
        let mut translations = HashMap::new();
        translations.insert(
            (1, "video_tag_title".to_string()),
            make_translation("번역됨", SupportedLanguage::Ja),
        );
        let (mut translated, mut fallback, mut requested) = (0, 0, 0);

        apply_tag_translations(
            &mut tags,
            &translations,
            SupportedLanguage::Ja,
            &mut translated,
            &mut fallback,
            &mut requested,
        );

        assert_eq!(tags[0].title.as_deref(), Some("번역됨"));
        assert_eq!(translated, 1);
        assert_eq!(fallback, 0);
        assert_eq!(requested, 1);
    }

    #[test]
    fn test_apply_tag_translations_counts_fallback_when_lang_differs() {
        let mut tags = vec![make_tag(1, Some("orig"), None)];
        let mut translations = HashMap::new();
        // user 는 Ja 요청, 실제 actual_lang = En = fallback
        translations.insert(
            (1, "video_tag_title".to_string()),
            make_translation("English fallback", SupportedLanguage::En),
        );
        let (mut translated, mut fallback, mut requested) = (0, 0, 0);

        apply_tag_translations(
            &mut tags,
            &translations,
            SupportedLanguage::Ja,
            &mut translated,
            &mut fallback,
            &mut requested,
        );

        assert_eq!(tags[0].title.as_deref(), Some("English fallback"));
        assert_eq!(translated, 0);
        assert_eq!(fallback, 1);
        assert_eq!(requested, 1);
    }

    #[test]
    fn test_apply_tag_translations_skips_none_fields() {
        // tag.title = None / subtitle = None → requested 카운트 0
        let mut tags = vec![make_tag(1, None, None)];
        let translations = HashMap::new();
        let (mut translated, mut fallback, mut requested) = (0, 0, 0);

        apply_tag_translations(
            &mut tags,
            &translations,
            SupportedLanguage::Ja,
            &mut translated,
            &mut fallback,
            &mut requested,
        );

        assert_eq!(translated, 0);
        assert_eq!(fallback, 0);
        assert_eq!(requested, 0, "None fields must not be requested");
    }

    #[test]
    fn test_apply_tag_translations_handles_subtitle_independently() {
        // title 만 번역되고 subtitle 은 번역 부재 — subtitle 도 requested 카운트는 됨
        let mut tags = vec![make_tag(1, Some("orig"), Some("sub"))];
        let mut translations = HashMap::new();
        translations.insert(
            (1, "video_tag_subtitle".to_string()),
            make_translation("번역 sub", SupportedLanguage::Ja),
        );
        let (mut translated, mut fallback, mut requested) = (0, 0, 0);

        apply_tag_translations(
            &mut tags,
            &translations,
            SupportedLanguage::Ja,
            &mut translated,
            &mut fallback,
            &mut requested,
        );

        assert_eq!(tags[0].title.as_deref(), Some("orig"), "title untouched");
        assert_eq!(tags[0].subtitle.as_deref(), Some("번역 sub"));
        assert_eq!(translated, 1);
        assert_eq!(requested, 2, "both title+subtitle requested");
    }

    #[test]
    fn test_apply_tag_translations_aggregates_multiple_tags() {
        let mut tags = vec![
            make_tag(1, Some("a"), None),
            make_tag(2, Some("b"), Some("b sub")),
        ];
        let mut translations = HashMap::new();
        translations.insert(
            (1, "video_tag_title".to_string()),
            make_translation("a-ja", SupportedLanguage::Ja),
        );
        translations.insert(
            (2, "video_tag_title".to_string()),
            make_translation("b-en", SupportedLanguage::En),
        );
        // tag 2 subtitle 번역 없음 — requested 만 +1
        let (mut translated, mut fallback, mut requested) = (0, 0, 0);

        apply_tag_translations(
            &mut tags,
            &translations,
            SupportedLanguage::Ja,
            &mut translated,
            &mut fallback,
            &mut requested,
        );

        assert_eq!(tags[0].title.as_deref(), Some("a-ja"));
        assert_eq!(tags[1].title.as_deref(), Some("b-en"));
        assert_eq!(
            tags[1].subtitle.as_deref(),
            Some("b sub"),
            "subtitle 원본 유지"
        );
        assert_eq!(translated, 1, "tag 1 ja 매칭 1건");
        assert_eq!(fallback, 1, "tag 2 en fallback 1건");
        assert_eq!(requested, 3, "title 2 + subtitle 1");
    }
}
