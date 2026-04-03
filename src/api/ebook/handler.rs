use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, Response, StatusCode};
use axum::Json;
use redis::AsyncCommands;

use crate::api::auth::extractor::AuthUser;
use crate::api::util::extract_client_ip;
use crate::error::{AppError, AppResult};
use crate::state::AppState;

use super::dto::{CreatePurchaseReq, EbookCatalogRes, HeartbeatReq, HeartbeatRes, MyPurchasesRes, ViewerMetaRes};
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
    // 매번 EXPIRE 호출 — TOCTOU 경합 방지 (동시 요청 시 첫 요청만 TTL 설정하는 문제 해결)
    let _: () = redis_conn
        .expire(&rl_key, st.cfg.rate_limit_ebook_purchase_window_sec)
        .await?;
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

/// DELETE /ebook/purchase/:code
///
/// 대기 중인 구매 취소 (soft delete). 로그인 + 본인 소유만 가능.
pub async fn cancel_purchase(
    State(st): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(code): Path<String>,
) -> AppResult<StatusCode> {
    EbookService::cancel_pending_purchase(&st, claims.sub, &code).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /ebook/viewer/:code/meta
///
/// 뷰어 메타 정보 (TOC, 총 페이지 수). 로그인 + 소유 확인.
pub async fn get_viewer_meta(
    State(st): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(code): Path<String>,
) -> AppResult<Json<ViewerMetaRes>> {
    let mut res = EbookService::get_viewer_meta(&st, claims.sub, &code).await?;
    let (session_id, hmac_secret) = EbookService::register_session(&st, claims.sub, &code).await?;
    res.session_id = session_id;
    res.hmac_secret = hmac_secret;
    Ok(Json(res))
}

/// POST /ebook/viewer/heartbeat
///
/// 뷰어 세션 heartbeat. 세션 유효 시 TTL 갱신, 무효 시 valid=false 반환.
pub async fn heartbeat(
    State(st): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(req): Json<HeartbeatReq>,
) -> AppResult<Json<HeartbeatRes>> {
    let res = EbookService::heartbeat(&st, claims.sub, &req.session_id).await?;
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

    // 뷰어 세션 검증 (Redis 장애 시 fail closed, session_id 비교)
    let session_id = headers
        .get("x-ebook-session")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    EbookService::verify_session(&st, claims.sub, session_id.as_deref()).await?;

    // HMAC 서명 검증 (요청 무결성 + 리플레이 방지)
    let signature = headers
        .get("x-ebook-signature")
        .and_then(|v| v.to_str().ok());
    let timestamp = headers
        .get("x-ebook-timestamp")
        .and_then(|v| v.to_str().ok());
    match (signature, timestamp) {
        (Some(sig), Some(ts)) => {
            let path = format!("{}/{}", code, page_num);
            EbookService::verify_hmac_signature(&st, claims.sub, &path, sig, ts).await?;
        }
        _ => return Err(AppError::Forbidden("Missing signature headers".into())),
    }

    // User-level Rate Limiting (페이지 크롤링 방지)
    let rl_key = format!("rl:ebook_page:{}", claims.sub);
    let mut redis_conn = st
        .redis
        .get()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
    let _: () = redis_conn
        .expire(&rl_key, st.cfg.rate_limit_ebook_page_window_sec)
        .await?;
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
        .header("Content-Disposition", "inline")
        .header("Cache-Control", "private, no-store")
        .header("X-Content-Type-Options", "nosniff")
        .header("Referrer-Policy", "no-referrer")
        .body(Body::from(image_bytes))
        .map_err(|e| AppError::Internal(format!("Failed to build response: {e}")))?;

    Ok(response)
}

/// GET /ebook/viewer/:code/pages/:page_num/tiles/:row/:col
///
/// 타일 분할 이미지 반환. 로그인 + 소유 확인 + 세션 검증 + 레이트 리밋.
pub async fn get_page_tile(
    State(st): State<AppState>,
    AuthUser(claims): AuthUser,
    Path((code, page_num, tile_row, tile_col)): Path<(String, i32, u32, u32)>,
    headers: HeaderMap,
) -> AppResult<Response<Body>> {
    // 커스텀 헤더 체크
    let is_viewer_request = headers
        .get("x-ebook-viewer")
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "1")
        .unwrap_or(false);
    if !is_viewer_request {
        return Err(AppError::Forbidden("Direct access not allowed".into()));
    }

    // 뷰어 세션 검증 (Redis 장애 시 fail closed, session_id 비교)
    let session_id = headers
        .get("x-ebook-session")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    EbookService::verify_session(&st, claims.sub, session_id.as_deref()).await?;

    // HMAC 서명 검증 (요청 무결성 + 리플레이 방지)
    let signature = headers
        .get("x-ebook-signature")
        .and_then(|v| v.to_str().ok());
    let timestamp = headers
        .get("x-ebook-timestamp")
        .and_then(|v| v.to_str().ok());
    match (signature, timestamp) {
        (Some(sig), Some(ts)) => {
            let path = format!("{}/{}/{}/{}", code, page_num, tile_row, tile_col);
            EbookService::verify_hmac_signature(&st, claims.sub, &path, sig, ts).await?;
        }
        _ => return Err(AppError::Forbidden("Missing signature headers".into())),
    }

    // User-level Rate Limiting (타일 전용)
    let rl_key = format!("rl:ebook_tile:{}", claims.sub);
    let mut redis_conn = st
        .redis
        .get()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let attempts: i64 = redis_conn.incr(&rl_key, 1).await?;
    let _: () = redis_conn
        .expire(&rl_key, st.cfg.rate_limit_ebook_tile_window_sec)
        .await?;
    if attempts > st.cfg.rate_limit_ebook_tile_max {
        return Err(AppError::TooManyRequests(
            "EBOOK_429_TOO_MANY_TILE_REQUESTS".into(),
        ));
    }

    let client_ip = extract_client_ip(&headers);
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let tile_bytes = EbookService::get_page_tile(
        &st,
        claims.sub,
        &code,
        page_num,
        tile_row,
        tile_col,
        Some(&client_ip),
        user_agent.as_deref(),
    )
    .await?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/webp")
        .header("Content-Disposition", "inline")
        .header("Cache-Control", "private, no-store")
        .header("X-Content-Type-Options", "nosniff")
        .header("Referrer-Policy", "no-referrer")
        .body(Body::from(tile_bytes))
        .map_err(|e| AppError::Internal(format!("Failed to build response: {e}")))?;

    Ok(response)
}
