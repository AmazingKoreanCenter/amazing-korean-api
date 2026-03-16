use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::state::AppState;
use crate::types::{EbookEdition, EbookPurchaseStatus, TextbookLanguage};

use super::dto::{
    CreatePurchaseReq, EbookCatalogItem, EbookCatalogRes, EbookEditionInfo, MyPurchasesRes,
    PurchaseRes, TocEntry, ViewerMetaRes,
};
use super::repo;
use super::watermark;

/// E-book 가격 (KRW)
const TEACHER_PRICE: i32 = 15_000;
const STUDENT_PRICE: i32 = 12_000;

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

                let price = match edition {
                    EbookEdition::Teacher => TEACHER_PRICE,
                    EbookEdition::Student => STUDENT_PRICE,
                };

                editions.push(EbookEditionInfo {
                    edition,
                    price,
                    currency: "KRW".to_string(),
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

        Ok(EbookCatalogRes { items })
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

        let price = match req.edition {
            EbookEdition::Teacher => TEACHER_PRICE,
            EbookEdition::Student => STUDENT_PRICE,
        };

        // 트랜잭션으로 주문코드 생성 + INSERT
        let mut tx = st.db.begin().await?;

        let code = repo::generate_purchase_code(&mut tx, req.language, req.edition, req.payment_method).await?;

        let row = repo::insert_purchase(
            &mut tx,
            &code,
            user_id,
            req.language,
            req.edition,
            req.payment_method,
            price,
            "KRW",
        )
        .await?;

        tx.commit().await?;

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
            .map_err(|e| AppError::Internal(format!("Manifest not found: {e}").into()))?;

        let manifest: serde_json::Value = serde_json::from_str(&manifest_content)
            .map_err(|e| AppError::Internal(format!("Invalid manifest JSON: {e}").into()))?;

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

        Ok(ViewerMetaRes {
            purchase_code: row.purchase_code,
            language: row.language,
            edition: row.edition,
            total_pages,
            toc,
        })
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

        let image_path = Path::new(&st.cfg.ebook_page_images_dir)
            .join(edition_dir)
            .join(lang_code)
            .join(format!("page-{:03}.webp", page_num));

        let image_bytes = tokio::fs::read(&image_path).await.map_err(|_| {
            AppError::NotFound
        })?;

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
