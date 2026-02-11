use super::{dto::CourseListItem, repo};
use crate::api::admin::translation::repo::TranslationRepo;
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::types::{ContentType, SupportedLanguage};

pub struct CourseService;

impl CourseService {
    pub async fn list(state: &AppState, lang: Option<SupportedLanguage>) -> AppResult<Vec<CourseListItem>> {
        let mut items = repo::list(&state.db).await?;

        if let Some(lang) = lang {
            let ids: Vec<i64> = items.iter().map(|c| c.course_id).collect();
            let translations = TranslationRepo::find_translations_for_contents(
                &state.db,
                ContentType::Course,
                &ids,
                lang,
            )
            .await?;

            for item in items.iter_mut() {
                if let Some(t) = translations.get(&(item.course_id, "title".to_string())) {
                    item.course_title = t.text.clone();
                }
                if let Some(t) = translations.get(&(item.course_id, "subtitle".to_string())) {
                    item.course_subtitle = Some(t.text.clone());
                }
            }
        }

        Ok(items)
    }

    pub async fn get_by_id(
        state: &AppState,
        id: i64,
        lang: Option<SupportedLanguage>,
    ) -> AppResult<CourseListItem> {
        let mut item = repo::find_by_id(&state.db, id)
            .await?
            .ok_or(AppError::NotFound)?;

        if let Some(lang) = lang {
            let translations = TranslationRepo::find_translations_for_contents(
                &state.db,
                ContentType::Course,
                &[item.course_id],
                lang,
            )
            .await?;

            if let Some(t) = translations.get(&(item.course_id, "title".to_string())) {
                item.course_title = t.text.clone();
            }
            if let Some(t) = translations.get(&(item.course_id, "subtitle".to_string())) {
                item.course_subtitle = Some(t.text.clone());
            }
        }

        Ok(item)
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
