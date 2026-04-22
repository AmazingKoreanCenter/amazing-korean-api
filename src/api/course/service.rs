use super::{
    dto::{CourseDetailRes, CourseListRes},
    repo,
};
use crate::api::admin::translation::dto::TranslationMeta;
use crate::api::admin::translation::repo::TranslationRepo;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::types::{ContentType, SupportedLanguage};

pub struct CourseService;

impl CourseService {
    pub async fn list(
        state: &AppState,
        lang: Option<SupportedLanguage>,
    ) -> AppResult<CourseListRes> {
        let mut items = repo::list(&state.db).await?;

        let translation_meta = match lang {
            None => TranslationMeta::not_requested(),
            Some(SupportedLanguage::Ko) => TranslationMeta::ko_full(),
            Some(user_lang) => {
                let ids: Vec<i64> = items.iter().map(|c| c.course_id).collect();
                let translations = TranslationRepo::find_translations_for_contents(
                    &state.db,
                    ContentType::Course,
                    &ids,
                    user_lang,
                )
                .await?;

                // Gemini 3차 리뷰 반영: requested 는 source 에 값이 있는 필드만 카운트
                // (course_subtitle 은 Option 이므로 None 이면 요청 대상 아님). 전체 하드코딩
                // (items.len() * 2) 은 optional null 소스에선 full 에 절대 도달 못함.
                let mut translated = 0usize;
                let mut fallback = 0usize;
                let mut requested = 0usize;
                for item in items.iter_mut() {
                    requested += 1; // course_title 은 필수
                    if let Some(t) = translations
                        .get(&(item.course_id, "course_title".to_string()))
                    {
                        item.course_title = t.text.clone();
                        t.count_to(user_lang, &mut translated, &mut fallback);
                    }
                    if item.course_subtitle.is_some() {
                        requested += 1;
                        if let Some(t) = translations
                            .get(&(item.course_id, "course_subtitle".to_string()))
                        {
                            item.course_subtitle = Some(t.text.clone());
                            t.count_to(user_lang, &mut translated, &mut fallback);
                        }
                    }
                }
                TranslationMeta::from_counts(user_lang, requested, translated, fallback)
            }
        };

        Ok(CourseListRes {
            items,
            translation_meta,
        })
    }

    pub async fn get_by_id(
        state: &AppState,
        id: i64,
        lang: Option<SupportedLanguage>,
    ) -> AppResult<CourseDetailRes> {
        let mut item = repo::find_by_id(&state.db, id)
            .await?
            .ok_or(AppError::NotFound)?;

        let translation_meta = match lang {
            None => TranslationMeta::not_requested(),
            Some(SupportedLanguage::Ko) => TranslationMeta::ko_full(),
            Some(user_lang) => {
                let translations = TranslationRepo::find_translations_for_contents(
                    &state.db,
                    ContentType::Course,
                    &[item.course_id],
                    user_lang,
                )
                .await?;

                let mut translated = 0usize;
                let mut fallback = 0usize;
                let mut requested = 1usize; // course_title 은 필수
                if let Some(t) =
                    translations.get(&(item.course_id, "course_title".to_string()))
                {
                    item.course_title = t.text.clone();
                    t.count_to(user_lang, &mut translated, &mut fallback);
                }
                if item.course_subtitle.is_some() {
                    requested += 1;
                    if let Some(t) =
                        translations.get(&(item.course_id, "course_subtitle".to_string()))
                    {
                        item.course_subtitle = Some(t.text.clone());
                        t.count_to(user_lang, &mut translated, &mut fallback);
                    }
                }
                TranslationMeta::from_counts(user_lang, requested, translated, fallback)
            }
        };

        Ok(CourseDetailRes {
            course: item,
            translation_meta,
        })
    }

    pub async fn create(
        state: &AppState,
        title: &str,
        price: i32,
        ctype: &str,
        subtitle: Option<&str>,
    ) -> AppResult<i64> {
        repo::create(&state.db, title, price, ctype, subtitle).await
    }
}


