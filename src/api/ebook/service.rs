use std::path::Path;

use image::GenericImageView;

use hmac::{Hmac, Mac};
use redis::AsyncCommands;
use sha2::Sha256;

use crate::crypto::CryptoService;
use crate::error::{AppError, AppResult};
use crate::external::email::{send_templated, EmailTemplate};
use crate::state::AppState;
use crate::types::{EbookEdition, EbookPaymentMethod, EbookPurchaseStatus, TextbookLanguage};

use super::dto::{
    CreateIapPurchaseReq, CreatePurchaseReq, EbookCatalogItem, EbookCatalogRes,
    EbookEditionInfo, HeartbeatRes, IapPlatform, MyPurchasesRes, PurchaseRes,
    TocEntry, ViewerMetaRes,
};
use super::repo;
use super::watermark;

/// E-book 가격 (KRW, 계좌이체용) — 학생/교사 동일
const EBOOK_PRICE_KRW: i32 = 15_000;
/// E-book 가격 (USD cents, Paddle용 — $9.99)
const EBOOK_PRICE_USD_CENTS: i32 = 9_99;

/// 타일 요청 파라미터
pub struct TileRequest<'a> {
    pub user_id: i64,
    pub purchase_code: &'a str,
    pub page_num: i32,
    pub tile_row: u32,
    pub tile_col: u32,
    pub ip_address: Option<&'a str>,
    pub user_agent: Option<&'a str>,
}

pub struct EbookService;

impl EbookService {
    // ─────────────────────── Catalog ───────────────────────

    pub async fn get_catalog(st: &AppState) -> AppResult<EbookCatalogRes> {
        let languages = catalog_languages();
        let mut items = Vec::new();

        for (lang, name_ko, name_en) in languages {
            let mut editions = Vec::new();

            for edition in [EbookEdition::Teacher, EbookEdition::Student] {
                let edition_dir = match edition {
                    EbookEdition::Teacher => "teacher",
                    EbookEdition::Student => "student",
                };
                let lang_code = lang.to_code();
                let manifest_path = Path::new(&st.cfg.ebook_page_images_dir)
                    .join(edition_dir)
                    .join(lang_code)
                    .join("manifest.json");

                let (total_pages, available) = if manifest_path.exists() {
                    match tokio::fs::read_to_string(&manifest_path).await {
                        Ok(content) => {
                            match serde_json::from_str::<serde_json::Value>(&content) {
                                Ok(manifest) => {
                                    let pages = manifest["total_pages"].as_i64().unwrap_or(0) as i32;
                                    (pages, pages > 0)
                                }
                                Err(e) => {
                                    tracing::error!("Invalid manifest JSON at {}: {e}", manifest_path.display());
                                    (0, false)
                                }
                            }
                        }
                        Err(e) => {
                            tracing::warn!("Failed to read manifest at {}: {e}", manifest_path.display());
                            (0, false)
                        }
                    }
                } else {
                    (0, false)
                };

                let price = EBOOK_PRICE_KRW;

                editions.push(EbookEditionInfo {
                    edition,
                    price,
                    currency: "KRW".to_string(),
                    paddle_price_usd: Some(EBOOK_PRICE_USD_CENTS),
                    total_pages,
                    available,
                });
            }

            items.push(EbookCatalogItem {
                language: lang,
                language_name_ko: name_ko.to_string(),
                language_name_en: name_en.to_string(),
                editions,
            });
        }

        let (paddle_ebook_price_id, client_token, sandbox) =
            if let Some(ref payment) = st.payment {
                (
                    st.cfg.paddle_ebook_price.clone(),
                    Some(payment.client_token().to_string()),
                    payment.is_sandbox(),
                )
            } else {
                (None, None, false)
            };

        Ok(EbookCatalogRes {
            items,
            paddle_ebook_price_id,
            client_token,
            sandbox,
        })
    }

    // ─────────────────────── Purchase ───────────────────────

    pub async fn create_purchase(
        st: &AppState,
        user_id: i64,
        req: CreatePurchaseReq,
    ) -> AppResult<PurchaseRes> {
        // 중복 구매 체크 (같은 언어+에디션으로 pending/completed 상태가 있으면 거절)
        let existing =
            repo::find_existing_purchase(&st.db, user_id, req.language, req.edition).await?;
        if let Some(row) = &existing {
            let msg = if row.status == EbookPurchaseStatus::Completed {
                "이미 해당 교재를 구매하셨습니다."
            } else {
                "해당 교재의 결제 대기 중인 주문이 있습니다."
            };
            return Err(AppError::Conflict(msg.into()));
        }

        let (price, currency) = match req.payment_method {
            EbookPaymentMethod::Paddle => (EBOOK_PRICE_USD_CENTS, "USD"),
            EbookPaymentMethod::BankTransfer => (EBOOK_PRICE_KRW, "KRW"),
            EbookPaymentMethod::AppleIap | EbookPaymentMethod::GoogleIap => {
                return Err(AppError::BadRequest("IAP 결제는 /ebook/purchase/iap 엔드포인트를 사용하세요".into()));
            }
        };

        // 트랜잭션으로 주문코드 생성 + INSERT
        let mut tx = st.db.begin().await?;

        let code = repo::generate_purchase_code(&mut tx, req.language, req.edition, req.payment_method).await?;

        let row = repo::insert_purchase(
            &mut tx,
            &repo::InsertPurchaseParams {
                purchase_code: &code,
                user_id,
                language: req.language,
                edition: req.edition,
                payment_method: req.payment_method,
                price,
                currency,
            },
        )
        .await?;

        tx.commit().await?;

        // 구매 접수 확인 이메일 발송 (fire-and-forget)
        if let Some(ref email_sender) = st.email {
            let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
            if let Ok(Some(encrypted_email)) = repo::find_user_encrypted_email(&st.db, user_id).await {
                if let Ok(user_email) = crypto.decrypt(&encrypted_email, "users.user_email") {
                    let lang_name = language_name_ko(req.language);
                    let edition_label = edition_label_ko(req.edition);
                    let template = EmailTemplate::EbookPurchaseConfirmation {
                        purchase_code: row.purchase_code.clone(),
                        language_name: lang_name.to_string(),
                        edition_label: edition_label.to_string(),
                        price: row.price.to_string(),
                        currency: row.currency.clone(),
                    };
                    if let Err(e) = send_templated(email_sender.as_ref(), &user_email, template).await {
                        tracing::warn!(
                            purchase_code = %row.purchase_code,
                            error = %e,
                            "Failed to send ebook purchase confirmation email"
                        );
                    }
                }
            }
        }

        Ok(PurchaseRes {
            purchase_code: row.purchase_code,
            status: row.status,
            language: row.language,
            edition: row.edition,
            payment_method: row.payment_method,
            price: row.price,
            currency: row.currency,
            created_at: row.created_at,
        })
    }

    // ─────────────────────── IAP Purchase ───────────────────────

    /// 모바일 IAP 구매 확정 (RevenueCat 영수증 검증 → 구매 생성, status=completed)
    pub async fn create_iap_purchase(
        st: &AppState,
        user_id: i64,
        req: CreateIapPurchaseReq,
    ) -> AppResult<PurchaseRes> {
        // 중복 구매 검사
        let existing = repo::find_existing_purchase(&st.db, user_id, req.language, req.edition).await?;
        if let Some(row) = &existing {
            let msg = if row.status == EbookPurchaseStatus::Completed {
                "이미 해당 교재를 구매하셨습니다."
            } else {
                "해당 교재의 결제 대기 중인 주문이 있습니다."
            };
            return Err(AppError::Conflict(msg.into()));
        }

        // RevenueCat 영수증 검증 (설정된 경우)
        if let Some(ref rc_client) = st.revenuecat {
            let subscriber = rc_client.get_subscriber(&user_id.to_string()).await?;
            // transaction_id가 non_subscriptions 또는 entitlements에 존재하는지 확인
            let has_valid_purchase = subscriber.non_subscriptions.values()
                .any(|purchases| purchases.iter().any(|p| p.id == req.transaction_id))
                || subscriber.entitlements.values()
                    .any(|e| e.is_active && e.product_identifier == req.product_id);

            if !has_valid_purchase {
                return Err(AppError::BadRequest("IAP 영수증 검증 실패: 유효한 구매를 찾을 수 없습니다".into()));
            }
        }

        let payment_method = match req.platform {
            IapPlatform::Apple => EbookPaymentMethod::AppleIap,
            IapPlatform::Google => EbookPaymentMethod::GoogleIap,
        };

        let platform_str = match req.platform {
            IapPlatform::Apple => "apple",
            IapPlatform::Google => "google",
        };

        // IAP 가격 (USD cents — 스토어 가격과 동일하게 기록)
        let (price, currency) = (EBOOK_PRICE_USD_CENTS, "USD");

        // 트랜잭션으로 주문코드 생성 + INSERT (status=completed)
        let mut tx = st.db.begin().await?;
        let code = repo::generate_purchase_code(&mut tx, req.language, req.edition, payment_method).await?;
        let row = repo::insert_iap_purchase(
            &mut tx,
            &repo::InsertIapPurchaseParams {
                purchase_code: &code,
                user_id,
                language: req.language,
                edition: req.edition,
                payment_method,
                price,
                currency,
                iap_platform: platform_str,
                iap_transaction_id: &req.transaction_id,
                iap_product_id: &req.product_id,
            },
        ).await?;
        tx.commit().await?;

        // 이메일 (fire-and-forget)
        if let Some(ref email_sender) = st.email {
            let crypto = CryptoService::new(&st.cfg.encryption_ring, &st.cfg.hmac_key);
            if let Ok(Some(encrypted_email)) = repo::find_user_encrypted_email(&st.db, user_id).await {
                if let Ok(user_email) = crypto.decrypt(&encrypted_email, "users.user_email") {
                    let lang_name = language_name_ko(req.language);
                    let edition_label = edition_label_ko(req.edition);
                    let template = EmailTemplate::EbookPurchaseConfirmation {
                        purchase_code: row.purchase_code.clone(),
                        language_name: lang_name.to_string(),
                        edition_label: edition_label.to_string(),
                        price: row.price.to_string(),
                        currency: row.currency.clone(),
                    };
                    if let Err(e) = send_templated(email_sender.as_ref(), &user_email, template).await {
                        tracing::warn!(
                            purchase_code = %row.purchase_code,
                            error = %e,
                            "Failed to send IAP purchase confirmation email"
                        );
                    }
                }
            }
        }

        Ok(PurchaseRes {
            purchase_code: row.purchase_code,
            status: row.status,
            language: row.language,
            edition: row.edition,
            payment_method: row.payment_method,
            price: row.price,
            currency: row.currency,
            created_at: row.created_at,
        })
    }

    // ─────────────────────── My Purchases ───────────────────────

    pub async fn get_my_purchases(st: &AppState, user_id: i64) -> AppResult<MyPurchasesRes> {
        let rows = repo::find_by_user(&st.db, user_id).await?;

        let items = rows
            .into_iter()
            .map(|r| PurchaseRes {
                purchase_code: r.purchase_code,
                status: r.status,
                language: r.language,
                edition: r.edition,
                payment_method: r.payment_method,
                price: r.price,
                currency: r.currency,
                created_at: r.created_at,
            })
            .collect();

        Ok(MyPurchasesRes { items })
    }

    // ─────────────────────── Cancel Pending ───────────────────────

    pub async fn cancel_pending_purchase(
        st: &AppState,
        user_id: i64,
        purchase_code: &str,
    ) -> AppResult<()> {
        let deleted = repo::cancel_pending_purchase(&st.db, user_id, purchase_code).await?;
        if !deleted {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    // ─────────────────────── Viewer Meta ───────────────────────

    pub async fn get_viewer_meta(
        st: &AppState,
        user_id: i64,
        purchase_code: &str,
    ) -> AppResult<ViewerMetaRes> {
        let row = repo::find_by_code(&st.db, purchase_code)
            .await?
            .ok_or(AppError::NotFound)?;

        // 소유 확인
        if row.user_id != user_id {
            return Err(AppError::NotFound);
        }

        // completed 상태만 뷰어 접근 허용
        if row.status != EbookPurchaseStatus::Completed {
            return Err(AppError::Forbidden(
                "결제가 완료되지 않았습니다.".into(),
            ));
        }

        let edition_dir = match row.edition {
            EbookEdition::Teacher => "teacher",
            EbookEdition::Student => "student",
        };
        let lang_code = row.language.to_code();
        let manifest_path = Path::new(&st.cfg.ebook_page_images_dir)
            .join(edition_dir)
            .join(lang_code)
            .join("manifest.json");

        let manifest_content = tokio::fs::read_to_string(&manifest_path)
            .await
            .map_err(|e| AppError::Internal(format!("Manifest not found: {e}")))?;

        let manifest: serde_json::Value = serde_json::from_str(&manifest_content)
            .map_err(|e| AppError::Internal(format!("Invalid manifest JSON: {e}")))?;

        let total_pages = manifest["total_pages"].as_i64().unwrap_or(0) as i32;

        let toc: Vec<TocEntry> = manifest["toc"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|entry| {
                        let title = entry["title"].as_str()?.to_string();
                        let title_ko = to_korean_title(&title);
                        Some(TocEntry {
                            title,
                            title_ko,
                            page: entry["page"].as_i64()? as i32,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let tile_mode = st.cfg.ebook_tile_enabled;
        let (grid_rows, grid_cols) = if tile_mode {
            (Some(st.cfg.ebook_tile_grid_rows), Some(st.cfg.ebook_tile_grid_cols))
        } else {
            (None, None)
        };

        Ok(ViewerMetaRes {
            purchase_code: row.purchase_code,
            language: row.language,
            edition: row.edition,
            total_pages,
            toc,
            session_id: String::new(), // handler에서 Redis 세션 등록 후 설정
            hmac_secret: String::new(), // handler에서 Redis 세션 등록 후 설정
            tile_mode,
            grid_rows,
            grid_cols,
        })
    }

    // ─────────────────────── Session ───────────────────────

    /// 뷰어 세션 등록 (Redis SET EX, 새 기기가 기존 세션 덮어쓰기 = Last Writer Wins)
    /// 32바이트 HMAC secret을 함께 생성하여 Redis에 저장, 클라이언트에 hex로 반환.
    /// 반환: (session_id, hmac_secret_hex)
    pub async fn register_session(
        st: &AppState,
        user_id: i64,
        purchase_code: &str,
    ) -> AppResult<(String, String)> {
        use rand::RngCore;
        let session_id = uuid::Uuid::new_v4().to_string();
        let mut hmac_secret = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut hmac_secret);
        let hmac_secret_hex = hex::encode(hmac_secret);

        let session_key = format!("ebook_viewer:{}", user_id);
        let session_data = serde_json::json!({
            "session_id": &session_id,
            "purchase_code": purchase_code,
            "hmac_secret": &hmac_secret_hex,
        });
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let _: () = redis::cmd("SET")
            .arg(&session_key)
            .arg(session_data.to_string())
            .arg("EX")
            .arg(st.cfg.ebook_session_ttl_sec)
            .query_async(&mut redis_conn)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok((session_id, hmac_secret_hex))
    }

    /// 뷰어 세션 heartbeat (세션 유효 시 TTL 갱신, 무효 시 valid=false)
    pub async fn heartbeat(
        st: &AppState,
        user_id: i64,
        session_id: &str,
    ) -> AppResult<HeartbeatRes> {
        let session_key = format!("ebook_viewer:{}", user_id);
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let stored: Option<String> = redis_conn.get(&session_key).await?;
        match stored {
            Some(data) => {
                let parsed: serde_json::Value = serde_json::from_str(&data)
                    .map_err(|_| AppError::Internal("ebook session data corrupted".into()))?;

                let stored_sid = parsed.get("session_id").and_then(|v| v.as_str())
                    .ok_or_else(|| AppError::Internal("ebook session missing session_id".into()))?;

                if stored_sid != session_id {
                    return Ok(HeartbeatRes { valid: false });
                }

                let _: () = redis_conn
                    .expire(&session_key, st.cfg.ebook_session_ttl_sec)
                    .await?;
                Ok(HeartbeatRes { valid: true })
            }
            None => Ok(HeartbeatRes { valid: false }),
        }
    }

    /// 뷰어 세션 검증 (페이지/타일 요청 시, Redis 장애 = fail closed)
    /// session_id가 제공되면 저장된 값과 비교, 미제공 시 존재만 확인 (하위 호환)
    /// TODO(2026-04-24): session_id 필수화 — Phase 1 관측(2026-04-16~) 완료 후 None → Forbidden 전환
    pub async fn verify_session(st: &AppState, user_id: i64, session_id: Option<&str>) -> AppResult<()> {
        if session_id.is_none() {
            tracing::warn!(
                user_id = user_id,
                "EBOOK_SESSION_AUDIT: verify_session called without x-ebook-session header"
            );
        }
        let session_key = format!("ebook_viewer:{}", user_id);
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let stored: Option<String> = redis_conn.get(&session_key).await?;
        match stored {
            Some(data) => {
                if let Some(sid) = session_id {
                    let parsed: serde_json::Value = serde_json::from_str(&data)
                        .map_err(|_| AppError::Forbidden("Viewer session invalid".into()))?;

                    let stored_sid = parsed.get("session_id").and_then(|v| v.as_str())
                        .ok_or_else(|| AppError::Forbidden("Viewer session invalid".into()))?;

                    if stored_sid != sid {
                        return Err(AppError::Forbidden("Viewer session invalid".into()));
                    }
                }
                Ok(())
            }
            None => Err(AppError::Forbidden("Viewer session expired".into())),
        }
    }

    // ─────────────────────── HMAC Signature ───────────────────────

    /// 요청별 HMAC 서명 검증.
    /// payload = "{session_id}:{path}:{timestamp}" (path 예: "VN-ST-.../3" 또는 "VN-ST-.../3/1/2")
    /// timestamp: Unix epoch 초, ±30초 허용.
    pub async fn verify_hmac_signature(
        st: &AppState,
        user_id: i64,
        path: &str,
        signature: &str,
        timestamp: &str,
    ) -> AppResult<()> {
        // 1. timestamp 파싱 + 윈도우 검증 (±30초)
        let ts: i64 = timestamp
            .parse()
            .map_err(|_| AppError::BadRequest("Invalid timestamp".into()))?;
        let now = chrono::Utc::now().timestamp();
        if (now - ts).abs() > 30 {
            return Err(AppError::Forbidden("Signature expired".into()));
        }

        // 2. Redis에서 세션 데이터 조회 → hmac_secret 추출
        let session_key = format!("ebook_viewer:{}", user_id);
        let mut redis_conn = st
            .redis
            .get()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let stored: Option<String> = redis_conn.get(&session_key).await?;
        let data = stored.ok_or_else(|| AppError::Forbidden("Viewer session expired".into()))?;
        let parsed: serde_json::Value = serde_json::from_str(&data)
            .map_err(|_| AppError::Internal("Invalid session data".into()))?;
        let secret_hex = parsed["hmac_secret"]
            .as_str()
            .ok_or_else(|| AppError::Internal("Missing hmac_secret in session".into()))?;
        let secret_bytes = hex::decode(secret_hex)
            .map_err(|_| AppError::Internal("Invalid hmac_secret hex".into()))?;

        // 3. 서명 계산 + 비교
        let session_id = parsed["session_id"]
            .as_str()
            .ok_or_else(|| AppError::Internal("Missing session_id in session".into()))?;
        let payload = format!("{session_id}:{path}:{timestamp}");

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(&secret_bytes)
            .map_err(|e| AppError::Internal(format!("HMAC init failed: {e}")))?;
        mac.update(payload.as_bytes());
        let expected = hex::encode(mac.finalize().into_bytes());

        if !constant_time_eq(signature.as_bytes(), expected.as_bytes()) {
            return Err(AppError::Forbidden("Invalid signature".into()));
        }

        Ok(())
    }

    // ─────────────────────── Page Image ───────────────────────

    /// 페이지 이미지 반환 (보안 핵심: 인증 + 소유 확인 + 레이트리밋 + 워터마크)
    pub async fn get_page_image(
        st: &AppState,
        user_id: i64,
        purchase_code: &str,
        page_num: i32,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
    ) -> AppResult<Vec<u8>> {
        // 1. 구매 확인 + 소유 확인
        let row = repo::find_by_code(&st.db, purchase_code)
            .await?
            .ok_or(AppError::NotFound)?;

        if row.user_id != user_id {
            return Err(AppError::NotFound);
        }

        // 2. completed 상태만 허용
        if row.status != EbookPurchaseStatus::Completed {
            return Err(AppError::Forbidden(
                "결제가 완료되지 않았습니다.".into(),
            ));
        }

        // 3. 페이지 번호 유효성 검증
        if page_num < 1 {
            return Err(AppError::BadRequest("Invalid page number".into()));
        }

        // 4. 이미지 파일 로드
        let edition_dir = match row.edition {
            EbookEdition::Teacher => "teacher",
            EbookEdition::Student => "student",
        };
        let lang_code = row.language.to_code();

        // manifest에서 total_pages 확인 → 상한 검증
        let manifest_path = Path::new(&st.cfg.ebook_page_images_dir)
            .join(edition_dir)
            .join(lang_code)
            .join("manifest.json");
        if let Ok(content) = tokio::fs::read_to_string(&manifest_path).await {
            if let Ok(manifest) = serde_json::from_str::<serde_json::Value>(&content) {
                let total = manifest["total_pages"].as_i64().unwrap_or(0) as i32;
                if total > 0 && page_num > total {
                    return Err(AppError::BadRequest("Page number out of range".into()));
                }
            }
        }

        let image_bytes = if st.cfg.ebook_images_encrypted {
            let enc_path = Path::new(&st.cfg.ebook_page_images_dir)
                .join(edition_dir)
                .join(lang_code)
                .join(format!("page-{:03}.webp.enc", page_num));
            let encrypted = tokio::fs::read(&enc_path).await.map_err(|_| AppError::NotFound)?;
            let key = st.cfg.ebook_image_key.as_ref()
                .unwrap_or_else(|| st.cfg.encryption_ring.current_key());
            crate::crypto::cipher::decrypt_bytes(key, &encrypted, "ebook.page_image")?
        } else {
            let image_path = Path::new(&st.cfg.ebook_page_images_dir)
                .join(edition_dir)
                .join(lang_code)
                .join(format!("page-{:03}.webp", page_num));
            tokio::fs::read(&image_path).await.map_err(|_| AppError::NotFound)?
        };

        // 5. 워터마크 적용 (4중 비가시적 보안: 풋터+마이크로도트+LSB+접근로그)
        let watermark_id = uuid::Uuid::new_v4().to_string();
        let watermarked =
            watermark::apply_watermark(&image_bytes, purchase_code, &watermark_id, user_id, page_num)?;

        // 6. 감사 로그 (비동기, 실패해도 이미지 반환)
        let db = st.db.clone();
        let wm_id = watermark_id.clone();
        let ip = ip_address.map(|s| s.to_string());
        let ua = user_agent.map(|s| s.to_string());
        tokio::spawn(async move {
            let _ = repo::insert_access_log(
                &db,
                row.purchase_id,
                user_id,
                page_num,
                &wm_id,
                ip.as_deref(),
                ua.as_deref(),
            )
            .await;
        });

        Ok(watermarked)
    }

    // ─────────────────────── Page Tile ───────────────────────

    /// 타일 분할 이미지 반환 (3×3 그리드 → 9개 타일)
    pub async fn get_page_tile(
        st: &AppState,
        req: &TileRequest<'_>,
    ) -> AppResult<Vec<u8>> {
        let user_id = req.user_id;
        let purchase_code = req.purchase_code;
        let page_num = req.page_num;
        let tile_row = req.tile_row;
        let tile_col = req.tile_col;
        let ip_address = req.ip_address;
        let user_agent = req.user_agent;
        let grid_rows = st.cfg.ebook_tile_grid_rows;
        let grid_cols = st.cfg.ebook_tile_grid_cols;

        // 타일 좌표 유효성 검증
        if tile_row >= grid_rows || tile_col >= grid_cols {
            return Err(AppError::BadRequest("Invalid tile coordinates".into()));
        }

        // 1. 구매 확인 + 소유 확인
        let row = repo::find_by_code(&st.db, purchase_code)
            .await?
            .ok_or(AppError::NotFound)?;

        if row.user_id != user_id {
            return Err(AppError::NotFound);
        }

        if row.status != EbookPurchaseStatus::Completed {
            return Err(AppError::Forbidden(
                "결제가 완료되지 않았습니다.".into(),
            ));
        }

        if page_num < 1 {
            return Err(AppError::BadRequest("Invalid page number".into()));
        }

        let edition_dir = match row.edition {
            EbookEdition::Teacher => "teacher",
            EbookEdition::Student => "student",
        };
        let lang_code = row.language.to_code();

        // manifest 페이지 범위 검증
        let manifest_path = Path::new(&st.cfg.ebook_page_images_dir)
            .join(edition_dir)
            .join(lang_code)
            .join("manifest.json");
        if let Ok(content) = tokio::fs::read_to_string(&manifest_path).await {
            if let Ok(manifest) = serde_json::from_str::<serde_json::Value>(&content) {
                let total = manifest["total_pages"].as_i64().unwrap_or(0) as i32;
                if total > 0 && page_num > total {
                    return Err(AppError::BadRequest("Page number out of range".into()));
                }
            }
        }

        // 2. 이미지 로드 (암호화 모드 시 .webp.enc → AES-256-GCM 복호화)
        let image_bytes = if st.cfg.ebook_images_encrypted {
            let enc_path = Path::new(&st.cfg.ebook_page_images_dir)
                .join(edition_dir)
                .join(lang_code)
                .join(format!("page-{:03}.webp.enc", page_num));
            let encrypted = tokio::fs::read(&enc_path).await.map_err(|_| AppError::NotFound)?;
            let key = st.cfg.ebook_image_key.as_ref()
                .unwrap_or_else(|| st.cfg.encryption_ring.current_key());
            crate::crypto::cipher::decrypt_bytes(key, &encrypted, "ebook.page_image")?
        } else {
            let image_path = Path::new(&st.cfg.ebook_page_images_dir)
                .join(edition_dir)
                .join(lang_code)
                .join(format!("page-{:03}.webp", page_num));
            tokio::fs::read(&image_path).await.map_err(|_| AppError::NotFound)?
        };

        // 3. 워터마크 적용 (전체 이미지에 먼저 적용 후 분할)
        let watermark_id = uuid::Uuid::new_v4().to_string();
        let watermarked =
            watermark::apply_watermark(&image_bytes, purchase_code, &watermark_id, user_id, page_num)?;

        // 4. 워터마크된 이미지 → 타일 추출
        let img = image::load_from_memory(&watermarked)
            .map_err(|e| AppError::Internal(format!("Failed to decode image: {e}")))?;
        let (w, h) = img.dimensions();
        let tile_w = w / grid_cols;
        let tile_h = h / grid_rows;
        let x = tile_col * tile_w;
        let y = tile_row * tile_h;
        let actual_w = if tile_col == grid_cols - 1 { w - x } else { tile_w };
        let actual_h = if tile_row == grid_rows - 1 { h - y } else { tile_h };
        let tile = img.crop_imm(x, y, actual_w, actual_h);

        // 5. WebP 인코딩 (quality 90+)
        let mut buf = std::io::Cursor::new(Vec::new());
        tile.write_to(&mut buf, image::ImageFormat::WebP)
            .map_err(|e| AppError::Internal(format!("Failed to encode tile: {e}")))?;

        // 6. 감사 로그 (비동기)
        let db = st.db.clone();
        let wm_id = watermark_id.clone();
        let ip = ip_address.map(|s| s.to_string());
        let ua = user_agent.map(|s| s.to_string());
        tokio::spawn(async move {
            let _ = repo::insert_access_log(
                &db,
                row.purchase_id,
                user_id,
                page_num,
                &wm_id,
                ip.as_deref(),
                ua.as_deref(),
            )
            .await;
        });

        Ok(buf.into_inner())
    }
}

// ─────────────────────── Helpers ───────────────────────

/// TextbookLanguage → 언어 코드 문자열 변환
trait LanguageCode {
    fn to_code(&self) -> &'static str;
}

impl LanguageCode for TextbookLanguage {
    fn to_code(&self) -> &'static str {
        match self {
            TextbookLanguage::Vi => "vi",
            TextbookLanguage::Ru => "ru",
            TextbookLanguage::Mn => "mn",
            TextbookLanguage::My => "my",
            TextbookLanguage::Ja => "ja",
            TextbookLanguage::ZhCn => "zh_cn",
            TextbookLanguage::ZhTw => "zh_tw",
            TextbookLanguage::Th => "th",
            TextbookLanguage::Hi => "hi",
            TextbookLanguage::Ne => "ne",
            TextbookLanguage::Si => "si",
            TextbookLanguage::Km => "km",
            TextbookLanguage::Es => "es",
            TextbookLanguage::Pt => "pt",
            TextbookLanguage::Fr => "fr",
            TextbookLanguage::De => "de",
            TextbookLanguage::Id => "id",
            TextbookLanguage::Uz => "uz",
            TextbookLanguage::Kk => "kk",
            TextbookLanguage::Tg => "tg",
            TextbookLanguage::Tl => "tl",
        }
    }
}

/// 영어 TOC 제목 → 한국어 변환
/// "Pronunciation 1" → "발음 1", "Contents" → "목차"
fn to_korean_title(en_title: &str) -> String {
    // 정확한 매칭 (특수 제목)
    match en_title {
        "Introduction" => return "머리말".to_string(),
        "Contents" => return "목차".to_string(),
        _ => {}
    }

    // "SectionName N" 패턴: 섹션 접두사를 한국어로 변환
    let section_map: &[(&str, &str)] = &[
        ("Pronunciation", "발음"),
        ("Grammar Basics", "문법 기초"),
        ("Structure", "조사"),
        ("Predicate", "용언 활용"),
        ("Adverbial", "연결 어미"),
        ("Miscellaneous", "기타"),
    ];

    for (en_prefix, ko_prefix) in section_map {
        if let Some(suffix) = en_title.strip_prefix(en_prefix) {
            let suffix = suffix.trim();
            if suffix.is_empty() {
                return ko_prefix.to_string();
            }
            return format!("{ko_prefix} {suffix}");
        }
    }

    // 매칭 없으면 영어 그대로
    en_title.to_string()
}

fn catalog_languages() -> Vec<(TextbookLanguage, &'static str, &'static str)> {
    vec![
        (TextbookLanguage::Vi, "베트남어", "Vietnamese"),
        (TextbookLanguage::Ru, "러시아어", "Russian"),
        (TextbookLanguage::Mn, "몽골어", "Mongolian"),
        (TextbookLanguage::My, "미얀마어", "Burmese"),
        (TextbookLanguage::Ja, "일본어", "Japanese"),
        (TextbookLanguage::ZhCn, "중국어(간체)", "Chinese (Simplified)"),
        (TextbookLanguage::ZhTw, "중국어(번체)", "Chinese (Traditional)"),
        (TextbookLanguage::Th, "태국어", "Thai"),
        (TextbookLanguage::Hi, "힌디어", "Hindi"),
        (TextbookLanguage::Ne, "네팔어", "Nepali"),
        (TextbookLanguage::Si, "싱할라어", "Sinhala"),
        (TextbookLanguage::Km, "크메르어", "Khmer"),
        (TextbookLanguage::Es, "스페인어", "Spanish"),
        (TextbookLanguage::Pt, "포르투갈어", "Portuguese"),
        (TextbookLanguage::Fr, "프랑스어", "French"),
        (TextbookLanguage::De, "독일어", "German"),
        (TextbookLanguage::Id, "인도네시아어", "Indonesian"),
        (TextbookLanguage::Uz, "우즈베크어", "Uzbek"),
        (TextbookLanguage::Kk, "카자흐어", "Kazakh"),
        (TextbookLanguage::Tg, "타지크어", "Tajik"),
        (TextbookLanguage::Tl, "필리핀어", "Filipino"),
    ]
}

/// 언어 → 한국어 이름 (이메일용)
pub fn language_name_ko(lang: TextbookLanguage) -> &'static str {
    catalog_languages()
        .iter()
        .find(|(l, _, _)| *l == lang)
        .map(|(_, ko, _)| *ko)
        .unwrap_or("알 수 없음")
}

/// 에디션 → 한국어 라벨 (이메일용)
pub fn edition_label_ko(edition: EbookEdition) -> &'static str {
    match edition {
        EbookEdition::Teacher => "교사용",
        EbookEdition::Student => "학생용",
    }
}

/// 상수 시간 바이트 비교 (타이밍 공격 방지)
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y))
        == 0
}
