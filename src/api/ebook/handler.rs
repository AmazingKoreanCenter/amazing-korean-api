use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, Response, StatusCode};
use axum::Json;
use redis::AsyncCommands;

use crate::api::auth::extractor::AuthUser;
use crate::api::util::extract_client_ip;
use crate::error::{AppError, AppResult};
use crate::state::AppState;

use super::dto::{CreatePurchaseReq, EbookCatalogRes, MyPurchasesRes, ViewerMetaRes};
use super::service::EbookService;

/// GET /ebook/catalog
///
/// E-book 카탈로그. 인증 불필요.
pub async fn get_catalog(State(st): State<AppState>) -> AppResult<Json<EbookCatalogRes>> {
    let res = EbookService::get_catalog(&st).await?;
    Ok(Json(res))
}

/// POST /ebook/purchase
///
/// E-book 구매 생성. 로그인 필수.
pub async fn create_purchase(
    State(st): State<AppState>,
    AuthUser(claims): AuthUser,
    headers: HeaderMap,
    Json(req): Json<CreatePurchaseReq>,
) -> AppResult<Json<super::dto::PurchaseRes>> {
    // IP 기반 Rate Limiting (구매 스팸 방지)
    let client_ip = extract_client_ip(&headers);
    let rl_key = format!("rl:ebook_purchase:{}", client_ip);
    let mut redis_conn = st
        .redis
        .get()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
    if attempts == 1 {
        let _: () = redis_conn
            .expire(&rl_key, st.cfg.rate_limit_ebook_purchase_window_sec)
            .await?;
    }
    if attempts > st.cfg.rate_limit_ebook_purchase_max {
        return Err(AppError::TooManyRequests(
            "EBOOK_429_TOO_MANY_PURCHASES".into(),
        ));
    }

    let res = EbookService::create_purchase(&st, claims.sub, req).await?;
    Ok(Json(res))
}

/// GET /ebook/my
///
/// 내 구매 목록. 로그인 필수.
pub async fn get_my_purchases(
    State(st): State<AppState>,
    AuthUser(claims): AuthUser,
) -> AppResult<Json<MyPurchasesRes>> {
    let res = EbookService::get_my_purchases(&st, claims.sub).await?;
    Ok(Json(res))
}

/// GET /ebook/viewer/:code/meta
///
/// 뷰어 메타 정보 (TOC, 총 페이지 수). 로그인 + 소유 확인.
pub async fn get_viewer_meta(
    State(st): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(code): Path<String>,
) -> AppResult<Json<ViewerMetaRes>> {
    let res = EbookService::get_viewer_meta(&st, claims.sub, &code).await?;
    Ok(Json(res))
}

/// GET /ebook/viewer/:code/pages/:page_num
///
/// 워터마크 적용된 페이지 이미지 반환. 로그인 + 소유 확인 + 레이트 리밋.
pub async fn get_page_image(
    State(st): State<AppState>,
    AuthUser(claims): AuthUser,
    Path((code, page_num)): Path<(String, i32)>,
    headers: HeaderMap,
) -> AppResult<Response<Body>> {
    // 커스텀 헤더 체크 — 뷰어 JS에서의 XHR만 허용 (URL 직접 접근 차단)
    // 브라우저 직접 네비게이션은 커스텀 헤더를 보낼 수 없음
    let is_viewer_request = headers
        .get("x-ebook-viewer")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "1")
        .unwrap_or(false);
    if !is_viewer_request {
        return Err(AppError::Forbidden("Direct access not allowed".into()));
    }

    // User-level Rate Limiting (페이지 크롤링 방지)
    let rl_key = format!("rl:ebook_page:{}", claims.sub);
    let mut redis_conn = st
        .redis
        .get()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
    if attempts == 1 {
        let _: () = redis_conn
            .expire(&rl_key, st.cfg.rate_limit_ebook_page_window_sec)
            .await?;
    }
    if attempts > st.cfg.rate_limit_ebook_page_max {
        return Err(AppError::TooManyRequests(
            "EBOOK_429_TOO_MANY_PAGE_REQUESTS".into(),
        ));
    }

    let client_ip = extract_client_ip(&headers);
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let image_bytes = EbookService::get_page_image(
        &st,
        claims.sub,
        &code,
        page_num,
        Some(&client_ip),
        user_agent.as_deref(),
    )
    .await?;

    // 보안 헤더와 함께 이미지 반환
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/webp")
        .header("Cache-Control", "private, max-age=300")
        .header("X-Content-Type-Options", "nosniff")
        .body(Body::from(image_bytes))
        .map_err(|e| AppError::Internal(format!("Failed to build response: {e}").into()))?;

    Ok(response)
}
