use serde::{Deserialize, Serialize};
use validator::Validate;

// SQL 결과를 이 구조체로 바로 매핑할 거라 FromRow 파생
#[derive(Serialize, sqlx::FromRow)]
pub struct CourseListItem {
    pub course_id: i64,
    pub course_title: String,
    pub course_price: i32,
    pub course_type: String,
    pub course_state: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateCourseReq {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    pub price: i32,
    pub course_type: String,
    pub subtitle: Option<String>,
}
