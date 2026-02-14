use sqlx::PgPool;
use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::external::translator::TranslationProvider;
use crate::types::SupportedLanguage;

use super::dto::{
    AutoTranslateBulkItem, AutoTranslateBulkItemResult, AutoTranslateBulkReq, AutoTranslateBulkRes,
    AutoTranslateItemResult, AutoTranslateReq, AutoTranslateRes, ContentRecordsReq,
    ContentRecordsRes, SourceFieldsReq, SourceFieldsRes, TranslationBulkCreateReq,
    TranslationBulkCreateRes, TranslationBulkItemResult, TranslationCreateReq,
    TranslationListMeta, TranslationListReq, TranslationListRes, TranslationRes,
    TranslationSearchReq, TranslationSearchRes, TranslationStatusReq, TranslationUpdateReq,
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

        // content_types(복수)가 있으면 우선, 없으면 content_type(단수) 사용
        let content_types_csv = req.content_types.as_deref();

        let total_count = TranslationRepo::count_all(
            pool,
            req.content_type,
            content_types_csv,
            req.content_id,
            req.lang,
            req.status,
        )
        .await?;

        let items = TranslationRepo::find_all(
            pool,
            req.content_type,
            content_types_csv,
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

    // =========================================================================
    // 콘텐츠 목록 / 원본 텍스트 / 번역 검색 (Step 4, 5, 11)
    // =========================================================================

    /// 콘텐츠 목록 조회 (드롭다운용)
    pub async fn list_content_records(
        pool: &PgPool,
        req: ContentRecordsReq,
    ) -> AppResult<ContentRecordsRes> {
        let items = TranslationRepo::find_content_records(pool, req.content_type).await?;
        Ok(ContentRecordsRes { items })
    }

    /// 원본 텍스트 조회 (한국어 소스 필드)
    pub async fn get_source_fields(
        pool: &PgPool,
        req: SourceFieldsReq,
    ) -> AppResult<SourceFieldsRes> {
        let fields = TranslationRepo::find_source_fields(pool, req.content_type, req.content_id).await?;
        Ok(SourceFieldsRes { fields })
    }

    /// 번역 검색 (언어 + 상태 기반 최근 번역 조회)
    pub async fn search_translations(
        pool: &PgPool,
        req: TranslationSearchReq,
    ) -> AppResult<TranslationSearchRes> {
        let items = TranslationRepo::search_translations(pool, req.lang).await?;
        Ok(TranslationSearchRes { items })
    }

    // =========================================================================
    // 자동 번역 (단건 + 벌크)
    // =========================================================================

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

    /// 벌크 자동 번역 (복수 필드 × 복수 언어)
    pub async fn auto_translate_bulk(
        pool: &PgPool,
        translator: &dyn TranslationProvider,
        req: AutoTranslateBulkReq,
    ) -> AppResult<AutoTranslateBulkRes> {
        req.validate().map_err(AppError::Validation)?;

        let total = req.items.len() * req.target_langs.len();
        let mut results = Vec::with_capacity(total);
        let mut success_count = 0usize;

        for item in &req.items {
            // 순수 숫자이면 번역 스킵 — 번역 필요 없는 값 (예: study_task_choice_answer)
            let is_numeric = item.source_text.trim().parse::<f64>().is_ok();

            for target_lang in &req.target_langs {
                if is_numeric {
                    // 숫자는 그대로 UPSERT
                    match TranslationRepo::upsert_one(
                        pool,
                        item.content_type,
                        item.content_id,
                        &item.field_name,
                        *target_lang,
                        &item.source_text,
                    )
                    .await
                    {
                        Ok(row) => {
                            success_count += 1;
                            results.push(AutoTranslateBulkItemResult {
                                content_type: item.content_type,
                                content_id: item.content_id,
                                field_name: item.field_name.clone(),
                                lang: *target_lang,
                                success: true,
                                translation_id: Some(row.translation_id),
                                translated_text: Some(item.source_text.clone()),
                                error: None,
                            });
                        }
                        Err(e) => {
                            results.push(Self::bulk_error_result(
                                &item, *target_lang, format!("DB save failed: {}", e),
                            ));
                        }
                    }
                    continue;
                }

                let gcp_target = target_lang.to_gcp_code();

                match translator.translate(&item.source_text, "ko", gcp_target).await {
                    Ok(translated_text) => {
                        match TranslationRepo::upsert_one(
                            pool,
                            item.content_type,
                            item.content_id,
                            &item.field_name,
                            *target_lang,
                            &translated_text,
                        )
                        .await
                        {
                            Ok(row) => {
                                success_count += 1;
                                results.push(AutoTranslateBulkItemResult {
                                    content_type: item.content_type,
                                    content_id: item.content_id,
                                    field_name: item.field_name.clone(),
                                    lang: *target_lang,
                                    success: true,
                                    translation_id: Some(row.translation_id),
                                    translated_text: Some(translated_text),
                                    error: None,
                                });
                            }
                            Err(e) => {
                                results.push(Self::bulk_error_result(
                                    &item, *target_lang, format!("DB save failed: {}", e),
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        results.push(Self::bulk_error_result(
                            &item, *target_lang, e.to_string(),
                        ));
                    }
                }
            }
        }

        let fail_count = total - success_count;
        Ok(AutoTranslateBulkRes {
            total,
            success_count,
            fail_count,
            results,
        })
    }

    fn bulk_error_result(
        item: &AutoTranslateBulkItem,
        lang: SupportedLanguage,
        error: String,
    ) -> AutoTranslateBulkItemResult {
        AutoTranslateBulkItemResult {
            content_type: item.content_type,
            content_id: item.content_id,
            field_name: item.field_name.clone(),
            lang,
            success: false,
            translation_id: None,
            translated_text: None,
            error: Some(error),
        }
    }
}
