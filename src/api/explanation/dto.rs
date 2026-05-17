//! 해설(설명) 콘텐츠 조회 DTO
//!
//! 서빙 모델 = structured 골격(skeleton) + i18n 해소 맵.
//! 단순 텍스트 블록은 `text` 해소, structured/concept/qword 는 `structured` 골격 +
//! `i18n`(field_name→해소 텍스트, inherit explanation 계승 적용) 으로 반환한다.
//! 프론트는 §5.10 의 index 불변식으로 골격+i18n 을 재조립한다.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::types::SupportedLanguage;

/// `GET /explanations/{unit_idx}` 쿼리
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ExplanationDetailReq {
    /// 번역 언어 (없으면 한국어 원본 / structured 는 en 기준)
    pub lang: Option<SupportedLanguage>,
}

/// `GET /explanations` 쿼리 (연결키로 조회)
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct ExplanationListReq {
    /// study.study_idx 로 연결된 pattern_guide 조회
    pub study_idx: Option<String>,
    /// study_task.study_task_idx (amk500-sent-NNN) 로 연결된 sentence_explain 조회
    pub study_task_idx: Option<String>,
    pub lang: Option<SupportedLanguage>,
}

/// 블록 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct ExplanationBlockRes {
    pub block_seq: i32,
    pub block_type: String,
    pub level: Option<i16>,
    /// 단순 텍스트 블록 해소 결과 (paragraph/heading/subtitle/step)
    pub text: Option<String>,
    /// lang-invariant 원형 (table/diagram/example HTML 등)
    pub raw: Option<String>,
    /// lang-invariant 골격 (structured_explain rows / concept_card items / qword_card table)
    #[schema(value_type = Object, nullable)]
    pub structured: Option<serde_json::Value>,
    /// field_name → 해소 텍스트 (user_lang→en 폴백, inherit explanation 계승 적용)
    #[schema(value_type = Object)]
    pub i18n: BTreeMap<String, String>,
}

/// 단위 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct ExplanationUnitRes {
    pub unit_idx: String,
    pub unit_kind: String,
    pub unit_source: String,
    pub study_idx: Option<String>,
    pub study_task_idx: Option<String>,
    pub sentence_num: Option<i32>,
    pub section_id: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    /// 응답에 적용된 언어 라벨 (요청 lang 또는 "ko")
    pub lang: String,
    pub blocks: Vec<ExplanationBlockRes>,
}

/// 목록 응답
#[derive(Debug, Serialize, ToSchema)]
pub struct ExplanationListRes {
    pub items: Vec<ExplanationUnitRes>,
}
