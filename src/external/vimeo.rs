use reqwest::Client;
use serde::Deserialize;

use crate::error::{AppError, AppResult};

const VIMEO_API_BASE: &str = "https://api.vimeo.com";

/// Vimeo API 클라이언트
pub struct VimeoClient {
    client: Client,
    access_token: String,
}

/// Vimeo 영상 메타데이터
#[derive(Debug, Clone)]
pub struct VimeoVideoMeta {
    pub name: String,
    pub description: Option<String>,
    pub duration: i32,
    pub thumbnail_url: Option<String>,
}

/// Vimeo API 응답 구조
#[derive(Debug, Deserialize)]
struct VimeoVideoResponse {
    name: String,
    description: Option<String>,
    duration: i32,
    pictures: Option<VimeoPictures>,
}

#[derive(Debug, Deserialize)]
struct VimeoPictures {
    sizes: Vec<VimeoPictureSize>,
}

#[derive(Debug, Deserialize)]
struct VimeoPictureSize {
    width: i32,
    link: String,
}

impl VimeoClient {
    /// 새 Vimeo 클라이언트 생성
    pub fn new(access_token: String) -> Self {
        Self {
            client: Client::new(),
            access_token,
        }
    }

    /// Vimeo URL에서 비디오 ID 추출
    /// 지원 형식:
    /// - https://vimeo.com/123456789
    /// - https://player.vimeo.com/video/123456789
    /// - 123456789 (ID만)
    pub fn extract_video_id(url: &str) -> Option<String> {
        let url = url.trim();

        // 이미 숫자만 있는 경우
        if url.chars().all(|c| c.is_ascii_digit()) {
            return Some(url.to_string());
        }

        // URL에서 추출
        let patterns = [
            r"vimeo\.com/(?:.*?/)?(\d+)",
            r"player\.vimeo\.com/video/(\d+)",
        ];

        for pattern in patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                if let Some(caps) = re.captures(url) {
                    if let Some(id) = caps.get(1) {
                        return Some(id.as_str().to_string());
                    }
                }
            }
        }

        None
    }

    /// Vimeo API에서 영상 메타데이터 조회
    pub async fn get_video_meta(&self, video_id: &str) -> AppResult<VimeoVideoMeta> {
        let url = format!("{}/videos/{}", VIMEO_API_BASE, video_id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("bearer {}", self.access_token))
            .header("Accept", "application/vnd.vimeo.*+json;version=3.4")
            .send()
            .await
            .map_err(|e| AppError::External(format!("Vimeo API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::External(format!(
                "Vimeo API error: {} - {}",
                status, body
            )));
        }

        let data: VimeoVideoResponse = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Vimeo API parse error: {}", e)))?;

        // 썸네일 URL 추출 (640px 이상 중 가장 큰 것)
        let thumbnail_url = data.pictures.and_then(|p| {
            p.sizes
                .into_iter()
                .filter(|s| s.width >= 640)
                .max_by_key(|s| s.width)
                .map(|s| s.link)
        });

        Ok(VimeoVideoMeta {
            name: data.name,
            description: data.description,
            duration: data.duration,
            thumbnail_url,
        })
    }

    /// Vimeo에 업로드 티켓 생성 (tus resumable upload)
    /// 반환값: (video_uri, video_id, upload_link)
    pub async fn create_upload_ticket(
        &self,
        file_name: &str,
        file_size: i64,
    ) -> AppResult<(String, String, String)> {
        let url = format!("{}/me/videos", VIMEO_API_BASE);

        let body = serde_json::json!({
            "name": file_name,
            "upload": {
                "approach": "tus",
                "size": file_size
            }
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("bearer {}", self.access_token))
            .header("Accept", "application/vnd.vimeo.*+json;version=3.4")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::External(format!("Vimeo upload ticket request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::External(format!(
                "Vimeo upload ticket error: {} - {}",
                status, body
            )));
        }

        let data: VimeoUploadResponse = response
            .json()
            .await
            .map_err(|e| AppError::External(format!("Vimeo upload response parse error: {}", e)))?;

        // URI에서 video ID 추출: "/videos/123456789" -> "123456789"
        let video_id = data
            .uri
            .split('/')
            .last()
            .unwrap_or_default()
            .to_string();

        Ok((data.uri, video_id, data.upload.upload_link))
    }
}

/// Vimeo 업로드 티켓 응답 구조
#[derive(Debug, Deserialize)]
struct VimeoUploadResponse {
    uri: String,
    upload: VimeoUploadInfo,
}

#[derive(Debug, Deserialize)]
struct VimeoUploadInfo {
    upload_link: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_video_id() {
        assert_eq!(
            VimeoClient::extract_video_id("https://vimeo.com/123456789"),
            Some("123456789".to_string())
        );
        assert_eq!(
            VimeoClient::extract_video_id("https://player.vimeo.com/video/987654321"),
            Some("987654321".to_string())
        );
        assert_eq!(
            VimeoClient::extract_video_id("123456789"),
            Some("123456789".to_string())
        );
        assert_eq!(
            VimeoClient::extract_video_id("https://vimeo.com/channels/staffpicks/123456789"),
            Some("123456789".to_string())
        );
        assert_eq!(VimeoClient::extract_video_id("invalid"), None);
    }
}
