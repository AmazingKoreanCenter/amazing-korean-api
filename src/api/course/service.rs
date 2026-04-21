use super::{
    dto::{CourseDetailRes, CourseListRes},
    repo,
};
use crate::api::admin::translation::dto::{TranslatedField, TranslationMeta};
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

                let mut translated = 0usize;
                let mut fallback = 0usize;
                for item in items.iter_mut() {
                    if let Some(t) = translations
                        .get(&(item.course_id, "course_title".to_string()))
                    {
                        item.course_title = t.text.clone();
                        count_field(t, user_lang, &mut translated, &mut fallback);
                    }
                    if let Some(t) = translations
                        .get(&(item.course_id, "course_subtitle".to_string()))
                    {
                        item.course_subtitle = Some(t.text.clone());
                        count_field(t, user_lang, &mut translated, &mut fallback);
                    }
                }
                let requested = items.len().saturating_mul(2);
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
                if let Some(t) =
                    translations.get(&(item.course_id, "course_title".to_string()))
                {
                    item.course_title = t.text.clone();
                    count_field(t, user_lang, &mut translated, &mut fallback);
                }
                if let Some(t) =
                    translations.get(&(item.course_id, "course_subtitle".to_string()))
                {
                    item.course_subtitle = Some(t.text.clone());
                    count_field(t, user_lang, &mut translated, &mut fallback);
                }
                TranslationMeta::from_counts(user_lang, 2, translated, fallback)
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

/// 번역 1건 집계 (user_lang 일치 vs fallback)
fn count_field(
    t: &TranslatedField,
    user_lang: SupportedLanguage,
    translated: &mut usize,
    fallback: &mut usize,
) {
    if t.actual_lang == user_lang {
        *translated += 1;
    } else {
        *fallback += 1;
    }
}

