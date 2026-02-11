use async_trait::async_trait;
use serde::Deserialize;

use crate::error::{AppError, AppResult};

// =============================================================================
// TranslationProvider trait
// =============================================================================

/// 번역 서비스 추상화 trait
///
/// `TRANSLATE_PROVIDER` 환경변수로 구현체 전환:
/// - `google`: Google Cloud Translation v2 Basic (GoogleCloudTranslator)
/// - `none`: 비활성 (번역 자동화 미사용)
#[async_trait]
pub trait TranslationProvider: Send + Sync {
    /// 단일 텍스트를 source_lang → target_lang으로 번역
    async fn translate(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> AppResult<String>;

    /// 여러 텍스트를 한 번에 번역 (배치)
    async fn translate_batch(
        &self,
        texts: &[&str],
        source_lang: &str,
        target_lang: &str,
    ) -> AppResult<Vec<String>>;
}

// =============================================================================
// Google Cloud Translation v2 (Basic) 구현
// =============================================================================

/// Google Cloud Translation v2 (Basic) REST API 클라이언트
///
/// https://cloud.google.com/translate/docs/reference/rest/v2/translations/translate
/// v2는 API Key 인증을 지원 (v3는 OAuth2/서비스 계정만 지원)
#[derive(Clone)]
pub struct GoogleCloudTranslator {
    http: reqwest::Client,
    api_key: String,
    /// v2에서는 사용하지 않으나, 향후 v3 전환 시 활용 가능
    #[allow(dead_code)]
    project_id: String,
}

impl GoogleCloudTranslator {
    pub fn new(api_key: String, project_id: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key,
            project_id,
        }
    }
}

/// GCP Translation v2 응답 구조
#[derive(Debug, Deserialize)]
struct GcpTranslateResponse {
    data: GcpTranslateData,
}

#[derive(Debug, Deserialize)]
struct GcpTranslateData {
    translations: Vec<GcpTranslation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GcpTranslation {
    translated_text: String,
}

/// GCP 에러 응답 구조
#[derive(Debug, Deserialize)]
struct GcpErrorResponse {
    error: Option<GcpError>,
}

#[derive(Debug, Deserialize)]
struct GcpError {
    message: String,
    #[allow(dead_code)]
    code: Option<u16>,
}

#[async_trait]
impl TranslationProvider for GoogleCloudTranslator {
    async fn translate(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> AppResult<String> {
        let results = self.translate_batch(&[text], source_lang, target_lang).await?;
        results.into_iter().next().ok_or_else(|| {
            AppError::External("Google Translate returned empty response".to_string())
        })
    }

    async fn translate_batch(
        &self,
        texts: &[&str],
        source_lang: &str,
        target_lang: &str,
    ) -> AppResult<Vec<String>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let url = format!(
            "https://translation.googleapis.com/language/translate/v2?key={}",
            self.api_key
        );

        let body = serde_json::json!({
            "q": texts,
            "source": source_lang,
            "target": target_lang,
            "format": "text"
        });

        let resp = self
            .http
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "Failed to call Google Translate API");
                AppError::External(format!("Failed to call Google Translate API: {}", e))
            })?;

        let status = resp.status();
        if !status.is_success() {
            let error_body = resp.text().await.unwrap_or_default();
            tracing::warn!(
                status = %status,
                error_body = %error_body,
                source = %source_lang,
                target = %target_lang,
                "Google Translate API error"
            );

            // GCP 에러 메시지 파싱 시도
            let msg = if let Ok(err_resp) = serde_json::from_str::<GcpErrorResponse>(&error_body) {
                err_resp
                    .error
                    .map(|e| e.message)
                    .unwrap_or_else(|| format!("Google Translate API error: {}", status))
            } else {
                match status.as_u16() {
                    401 => "Google Translate API key is invalid".to_string(),
                    403 => "Google Translate API quota exceeded or access denied".to_string(),
                    429 => "Google Translate API rate limit exceeded".to_string(),
                    _ => format!("Google Translate API error: {} {}", status, error_body),
                }
            };

            return Err(AppError::External(msg));
        }

        let gcp_resp: GcpTranslateResponse = resp.json().await.map_err(|e| {
            tracing::error!(error = %e, "Failed to parse Google Translate response");
            AppError::External(format!("Failed to parse Google Translate response: {}", e))
        })?;

        let translations = gcp_resp
            .data
            .translations
            .into_iter()
            .map(|t| t.translated_text)
            .collect();

        tracing::debug!(
            source = %source_lang,
            target = %target_lang,
            count = texts.len(),
            "Google Translate batch completed"
        );

        Ok(translations)
    }
}
