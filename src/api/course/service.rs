use super::{dto::CourseListItem, repo};
use crate::{error::AppResult, state::AppState};

pub struct CourseService;

impl CourseService {
    pub async fn list(state: &AppState) -> AppResult<Vec<CourseListItem>> {
        repo::list(&state.db).await
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
