use sqlx::PgPool;
use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::external::translator::TranslationProvider;

use super::dto::{
    AutoTranslateItemResult, AutoTranslateReq, AutoTranslateRes, TranslationBulkCreateReq,
    TranslationBulkCreateRes, TranslationBulkItemResult, TranslationCreateReq,
    TranslationListMeta, TranslationListReq, TranslationListRes, TranslationRes,
    TranslationStatusReq, TranslationUpdateReq,
};
use super::repo::TranslationRepo;

pub struct TranslationService;

impl TranslationService {
    /// 번역 목록 조회
    pub async fn list_translations(
        pool: &PgPool,
        req: TranslationListReq,
    ) -> AppResult<TranslationListRes> {
        req.validate().map_err(AppError::Validation)?;

        let page = req.page.unwrap_or(1).max(1);
        let per_page = req.per_page.unwrap_or(20).clamp(1, 100);
        let offset = (page - 1) * per_page;

        let total_count = TranslationRepo::count_all(
            pool,
            req.content_type,
            req.content_id,
            req.lang,
            req.status,
        )
        .await?;

        let items = TranslationRepo::find_all(
            pool,
            req.content_type,
            req.content_id,
            req.lang,
            req.status,
            per_page,
            offset,
        )
        .await?;

        let total_pages = (total_count + per_page - 1) / per_page;

        Ok(TranslationListRes {
            items,
            meta: TranslationListMeta {
                total_count,
                total_pages,
                current_page: page,
                per_page,
            },
        })
    }

    /// 단건 번역 생성 (UPSERT)
    pub async fn create_translation(
        pool: &PgPool,
        req: TranslationCreateReq,
    ) -> AppResult<TranslationRes> {
        req.validate().map_err(AppError::Validation)?;

        let res = TranslationRepo::upsert_one(
            pool,
            req.content_type,
            req.content_id,
            &req.field_name,
            req.lang,
            &req.translated_text,
        )
        .await?;

        Ok(res)
    }

    /// 벌크 번역 생성
    pub async fn bulk_create_translations(
        pool: &PgPool,
        req: TranslationBulkCreateReq,
    ) -> AppResult<TranslationBulkCreateRes> {
        req.validate().map_err(AppError::Validation)?;

        let total = req.items.len();
        let mut results = Vec::with_capacity(total);
        let mut success_count = 0usize;

        for (i, item) in req.items.into_iter().enumerate() {
            match TranslationRepo::upsert_one(
                pool,
                item.content_type,
                item.content_id,
                &item.field_name,
                item.lang,
                &item.translated_text,
            )
            .await
            {
                Ok(row) => {
                    success_count += 1;
                    results.push(TranslationBulkItemResult {
                        index: i,
                        success: true,
                        translation_id: Some(row.translation_id),
                        error: None,
                    });
                }
                Err(e) => {
                    results.push(TranslationBulkItemResult {
                        index: i,
                        success: false,
                        translation_id: None,
                        error: Some(e.to_string()),
                    });
                }
            }
        }

        Ok(TranslationBulkCreateRes {
            total,
            success_count,
            fail_count: total - success_count,
            results,
        })
    }

    /// 번역 상세 조회
    pub async fn get_translation(
        pool: &PgPool,
        translation_id: i64,
    ) -> AppResult<TranslationRes> {
        TranslationRepo::find_by_id(pool, translation_id)
            .await?
            .ok_or(AppError::NotFound)
    }

    /// 번역 수정
    pub async fn update_translation(
        pool: &PgPool,
        translation_id: i64,
        req: TranslationUpdateReq,
    ) -> AppResult<TranslationRes> {
        req.validate().map_err(AppError::Validation)?;

        TranslationRepo::update_one(
            pool,
            translation_id,
            req.translated_text.as_deref(),
            req.status,
        )
        .await?
        .ok_or(AppError::NotFound)
    }

    /// 번역 상태 변경
    pub async fn update_translation_status(
        pool: &PgPool,
        translation_id: i64,
        req: TranslationStatusReq,
    ) -> AppResult<TranslationRes> {
        TranslationRepo::update_status(pool, translation_id, req.status)
            .await?
            .ok_or(AppError::NotFound)
    }

    /// 번역 삭제
    pub async fn delete_translation(
        pool: &PgPool,
        translation_id: i64,
    ) -> AppResult<()> {
        let deleted = TranslationRepo::delete_one(pool, translation_id).await?;
        if !deleted {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    /// 자동 번역 (TranslationProvider를 통해 원본 → 타겟 언어 자동 번역 후 DB 저장)
    pub async fn auto_translate(
        pool: &PgPool,
        translator: &dyn TranslationProvider,
        req: AutoTranslateReq,
    ) -> AppResult<AutoTranslateRes> {
        req.validate().map_err(AppError::Validation)?;

        let total = req.target_langs.len();
        let mut results = Vec::with_capacity(total);
        let mut success_count = 0usize;

        // 각 타겟 언어별로 번역 + DB 저장
        for target_lang in &req.target_langs {
            let gcp_target = target_lang.to_gcp_code();

            // 1) 번역 API 호출
            match translator
                .translate(&req.source_text, "ko", gcp_target)
                .await
            {
                Ok(translated_text) => {
                    // 2) DB UPSERT (텍스트 변경 시 자동 draft)
                    match TranslationRepo::upsert_one(
                        pool,
                        req.content_type,
                        req.content_id,
                        &req.field_name,
                        *target_lang,
                        &translated_text,
                    )
                    .await
                    {
                        Ok(row) => {
                            success_count += 1;
                            results.push(AutoTranslateItemResult {
                                lang: *target_lang,
                                success: true,
                                translation_id: Some(row.translation_id),
                                translated_text: Some(translated_text),
                                error: None,
                            });
                        }
                        Err(e) => {
                            results.push(AutoTranslateItemResult {
                                lang: *target_lang,
                                success: false,
                                translation_id: None,
                                translated_text: Some(translated_text),
                                error: Some(format!("DB save failed: {}", e)),
                            });
                        }
                    }
                }
                Err(e) => {
                    results.push(AutoTranslateItemResult {
                        lang: *target_lang,
                        success: false,
                        translation_id: None,
                        translated_text: None,
                        error: Some(e.to_string()),
                    });
                }
            }
        }

        Ok(AutoTranslateRes {
            total,
            success_count,
            results,
        })
    }
}
