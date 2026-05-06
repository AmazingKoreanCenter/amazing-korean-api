use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert(Default::default());
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
        components.add_security_scheme(
            "refreshCookie",
            SecurityScheme::ApiKey(utoipa::openapi::security::ApiKey::Cookie(
                utoipa::openapi::security::ApiKeyValue::new("ak_refresh".to_string()),
            )),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Amazing Korean API", version = "1.0.0"),
    paths(
        // health
        crate::api::health::handler::health,
        crate::api::health::handler::ready,

        // auth
        crate::api::auth::handler::login,
        crate::api::auth::handler::login_mobile,
        crate::api::auth::handler::logout,
        crate::api::auth::handler::logout_all,
        crate::api::auth::handler::refresh,
        crate::api::auth::handler::refresh_mobile,
        crate::api::auth::handler::find_id,
        crate::api::auth::handler::find_password,
        crate::api::auth::handler::reset_password,
        crate::api::auth::handler::request_reset,
        crate::api::auth::handler::verify_reset,
        crate::api::auth::handler::verify_email,
        crate::api::auth::handler::resend_verification,
        crate::api::auth::handler::google_auth_start,
        crate::api::auth::handler::google_auth_callback,
        crate::api::auth::handler::google_mobile_login,
        crate::api::auth::handler::apple_mobile_login,
        crate::api::auth::handler::mfa_setup,
        crate::api::auth::handler::mfa_verify_setup,
        crate::api::auth::handler::mfa_login,
        crate::api::auth::handler::mfa_login_mobile,
        crate::api::auth::handler::mfa_disable,

        // user (me/settings)
        crate::api::user::handler::signup,
        crate::api::user::handler::get_me,
        crate::api::user::handler::update_me,
        crate::api::user::handler::get_settings,
        crate::api::user::handler::update_settings,

        // videos (user)
        crate::api::video::handler::list_videos,
        crate::api::video::handler::get_video_detail,
        crate::api::video::handler::get_video_progress,
        crate::api::video::handler::update_video_progress,

        // lessons (user)
        crate::api::lesson::handler::list_lessons,
        crate::api::lesson::handler::get_lesson_detail,
        crate::api::lesson::handler::get_lesson_items,
        crate::api::lesson::handler::get_lesson_progress,
        crate::api::lesson::handler::update_lesson_progress,

        // study (user)
        crate::api::study::handler::list_studies,
        crate::api::study::handler::get_study_detail,
        crate::api::study::handler::get_study_task,
        crate::api::study::handler::submit_answer,
        crate::api::study::handler::get_task_status,
        crate::api::study::handler::get_task_explain,
        crate::api::study::handler::start_writing_session,
        crate::api::study::handler::finish_writing_session,
        crate::api::study::handler::list_writing_sessions,
        crate::api::study::handler::get_writing_stats,
        crate::api::study::handler::list_writing_practice_seed,

        // admin - users
        crate::api::admin::user::handler::admin_list_users,
        crate::api::admin::user::handler::admin_create_user,
        crate::api::admin::user::handler::admin_create_users_bulk,
        crate::api::admin::user::handler::admin_get_user,
        crate::api::admin::user::handler::admin_update_user,
        crate::api::admin::user::handler::admin_update_users_bulk,

        // admin - videos
        crate::api::admin::video::handler::admin_list_videos,
        crate::api::admin::video::handler::admin_create_video,
        crate::api::admin::video::handler::admin_bulk_create_videos,
        crate::api::admin::video::handler::admin_update_video,
        crate::api::admin::video::handler::admin_bulk_update_videos,
        crate::api::admin::video::handler::admin_update_video_tags,
        crate::api::admin::video::handler::admin_bulk_update_video_tags,

        // admin - lessons
        crate::api::admin::lesson::handler::admin_list_lessons,
        crate::api::admin::lesson::handler::admin_list_lesson_items,
        crate::api::admin::lesson::handler::admin_list_lesson_progress,
        crate::api::admin::lesson::handler::admin_create_lesson_item,
        crate::api::admin::lesson::handler::admin_update_lesson_progress,
        crate::api::admin::lesson::handler::admin_bulk_update_lesson_progress,
        crate::api::admin::lesson::handler::admin_bulk_create_lesson_items,
        crate::api::admin::lesson::handler::admin_bulk_update_lesson_items,
        crate::api::admin::lesson::handler::admin_update_lesson_item,
        crate::api::admin::lesson::handler::admin_create_lesson,
        crate::api::admin::lesson::handler::admin_bulk_create_lessons,
        crate::api::admin::lesson::handler::admin_bulk_update_lessons,
        crate::api::admin::lesson::handler::admin_update_lesson,

        // admin - studies
        crate::api::admin::study::handler::admin_list_studies,
        crate::api::admin::study::handler::admin_create_study,
        crate::api::admin::study::handler::admin_bulk_create_studies,
        crate::api::admin::study::handler::admin_update_study,
        crate::api::admin::study::handler::admin_bulk_update_studies,
        crate::api::admin::study::handler::admin_list_study_tasks,
        crate::api::admin::study::handler::admin_list_task_explains,
        crate::api::admin::study::handler::admin_list_task_status,
        crate::api::admin::study::handler::admin_update_task_status,
        crate::api::admin::study::handler::admin_bulk_update_task_status,
        crate::api::admin::study::handler::admin_create_task_explain,
        crate::api::admin::study::handler::admin_update_task_explain,
        crate::api::admin::study::handler::admin_bulk_create_task_explains,
        crate::api::admin::study::handler::admin_bulk_update_task_explains,
        crate::api::admin::study::handler::admin_update_study_task,
        crate::api::admin::study::handler::admin_create_study_task,
        crate::api::admin::study::handler::admin_bulk_create_study_tasks,
        crate::api::admin::study::handler::admin_bulk_update_study_tasks,


        // admin - video stats
        crate::api::admin::video::stats::handler::admin_get_video_daily_stats,

        // admin - translations (번역 관리)
        crate::api::admin::translation::handler::admin_list_translations,
        crate::api::admin::translation::handler::admin_create_translation,
        crate::api::admin::translation::handler::admin_bulk_create_translations,
        crate::api::admin::translation::handler::admin_get_translation,
        crate::api::admin::translation::handler::admin_update_translation,
        crate::api::admin::translation::handler::admin_update_translation_status,
        crate::api::admin::translation::handler::admin_delete_translation,
        crate::api::admin::translation::handler::admin_list_content_records,
        crate::api::admin::translation::handler::admin_get_source_fields,
        crate::api::admin::translation::handler::admin_search_translations,
        crate::api::admin::translation::handler::admin_get_translation_stats,

        // admin - upgrade (관리자 초대)
        crate::api::admin::upgrade::handler::create_invite,
        crate::api::admin::upgrade::handler::verify_invite,
        crate::api::admin::upgrade::handler::accept_invite,

        // admin - email (테스트 이메일)
        crate::api::admin::email::handler::send_test_email,

        // payment (user-facing)
        // 주의: /payment/webhook (Paddle) 와 /payment/webhook/revenuecat 는 외부 webhook
        // 호출 전용이며 OpenAPI 노출에서 의도적으로 제외함 (handler 의 doc comment 참조).
        crate::api::payment::handler::get_plans,
        crate::api::payment::handler::get_subscription,
        crate::api::payment::handler::cancel_subscription,

        // textbook (user-facing)
        crate::api::textbook::handler::get_catalog,
        crate::api::textbook::handler::create_order,
        crate::api::textbook::handler::get_order_by_code,
        crate::api::textbook::handler::get_my_orders,

        // admin - payment
        crate::api::admin::payment::handler::list_subscriptions,
        crate::api::admin::payment::handler::get_subscription,
        crate::api::admin::payment::handler::cancel_subscription,
        crate::api::admin::payment::handler::list_transactions,
        crate::api::admin::payment::handler::create_grant,
        crate::api::admin::payment::handler::list_grants,
        crate::api::admin::payment::handler::revoke_grant,

        // admin - textbook
        crate::api::admin::textbook::handler::list_orders,
        crate::api::admin::textbook::handler::get_order,
        crate::api::admin::textbook::handler::update_status,
        crate::api::admin::textbook::handler::update_discount,
        crate::api::admin::textbook::handler::update_tracking,
        crate::api::admin::textbook::handler::admin_create_order,
        crate::api::admin::textbook::handler::delete_order,
        crate::api::admin::textbook::handler::list_admin_logs,

        // course (user-facing)
        crate::api::course::handler::list,
        crate::api::course::handler::create,
        crate::api::course::handler::get_by_id,

        // admin - ebook
        crate::api::admin::ebook::handler::list_purchases,
        crate::api::admin::ebook::handler::get_purchase,
        crate::api::admin::ebook::handler::update_status,
        crate::api::admin::ebook::handler::verify_watermark,
        crate::api::admin::ebook::handler::delete_purchase,
    ),
    components(
        schemas(
            // base types
            crate::types::UserGender,
            crate::types::UserAuth,
            crate::types::UserSetLanguage,
            crate::types::ContentType,
            crate::types::TranslationStatus,
            crate::types::SupportedLanguage,
            crate::error::ErrorBody,
            crate::types::LoginDevice,

            // health dto 
            crate::api::health::dto::HealthRes,
            crate::api::health::dto::ReadyRes,

            // auth dto
            crate::api::auth::dto::LoginReq,
            crate::api::auth::dto::LoginRes,
            crate::api::auth::dto::LoginMobileRes,
            crate::api::auth::dto::RefreshReq,
            crate::api::auth::dto::RefreshRes,
            crate::api::auth::dto::LogoutAllReq,
            crate::api::auth::dto::LogoutRes,
            crate::api::auth::dto::FindIdReq,
            crate::api::auth::dto::FindIdRes,
            crate::api::auth::dto::FindPasswordReq,
            crate::api::auth::dto::FindPasswordRes,
            crate::api::auth::dto::ResetPwReq,
            crate::api::auth::dto::ResetPwRes,
            crate::api::auth::dto::RequestResetReq,
            crate::api::auth::dto::RequestResetRes,
            crate::api::auth::dto::VerifyResetReq,
            crate::api::auth::dto::VerifyResetRes,
            crate::api::auth::dto::VerifyEmailReq,
            crate::api::auth::dto::VerifyEmailRes,
            crate::api::auth::dto::ResendVerificationReq,
            crate::api::auth::dto::ResendVerificationRes,
            crate::api::auth::dto::GoogleAuthUrlRes,
            crate::api::auth::dto::GoogleMobileLoginReq,
            crate::api::auth::dto::AppleMobileLoginReq,
            crate::api::auth::dto::MfaChallengeRes,
            crate::api::auth::dto::MfaLoginReq,
            crate::api::auth::dto::MfaSetupRes,
            crate::api::auth::dto::MfaVerifySetupReq,
            crate::api::auth::dto::MfaVerifySetupRes,
            crate::api::auth::dto::MfaDisableReq,
            crate::api::auth::dto::MfaDisableRes,

            // user dto
            crate::api::user::dto::SignupReq,
            crate::api::user::dto::SignupRes,
            crate::api::user::dto::ProfileRes,
            crate::api::user::dto::ProfileUpdateReq,
            crate::api::user::dto::SettingsRes,
            crate::api::user::dto::SettingsUpdateReq,
            /*crate::api::user::dto::StudyLangItem, // 향후 추가할 내용*/

            // course dto
            crate::api::course::dto::CourseListItem,
            crate::api::course::dto::CourseListRes,
            crate::api::course::dto::CourseDetailRes,
            crate::api::course::dto::CreateCourseReq,
            crate::api::course::dto::CreateCourseRes,

            // videos dto
            crate::api::video::dto::VideoListReq,
            crate::api::video::dto::VideoListItem,
            crate::api::video::dto::VideoTagDetail,
            crate::api::video::dto::VideoDetailRes,
            crate::api::video::dto::VideoProgressRes,
            crate::api::video::dto::VideoProgressUpdateReq,

            // lessons dto
            crate::api::lesson::dto::LessonListReq,
            crate::api::lesson::dto::LessonRes,
            crate::api::lesson::dto::LessonListMeta,
            crate::api::lesson::dto::LessonListRes,
            crate::api::lesson::dto::LessonDetailReq,
            crate::api::lesson::dto::LessonItemRes,
            crate::api::lesson::dto::LessonDetailRes,
            crate::api::lesson::dto::LessonItemsReq,
            crate::api::lesson::dto::LessonItemDetailRes,
            crate::api::lesson::dto::LessonItemsRes,
            crate::api::lesson::dto::LessonProgressRes,
            crate::api::lesson::dto::LessonProgressUpdateReq,

            // studies dto
            crate::api::study::dto::StudyListResp,
            crate::api::study::dto::StudyListMeta,
            crate::api::study::dto::StudySummaryDto,
            crate::api::study::dto::StudyDetailReq,
            crate::api::study::dto::StudyDetailRes,
            crate::api::study::dto::StudyTaskSummaryDto,
            crate::api::study::dto::StudyTaskDetailRes,
            crate::api::study::dto::TaskPayload,
            crate::api::study::dto::SubmitAnswerReq,
            crate::api::study::dto::SubmitAnswerRes,
            crate::api::study::dto::TaskStatusRes,
            crate::api::study::dto::TaskExplainRes,
            crate::api::study::dto::StartWritingSessionReq,
            crate::api::study::dto::FinishWritingSessionReq,
            crate::api::study::dto::WritingMistake,
            crate::api::study::dto::WritingSessionListReq,
            crate::api::study::dto::WritingSessionRes,
            crate::api::study::dto::WritingSessionListRes,
            crate::api::study::dto::WritingStatsReq,
            crate::api::study::dto::WritingLevelStat,
            crate::api::study::dto::WritingDailyStat,
            crate::api::study::dto::WritingWeakChar,
            crate::api::study::dto::WritingStatsRes,
            crate::api::study::dto::WritingPracticeSeedReq,
            crate::api::study::dto::WritingPracticeSeedItem,
            crate::api::study::dto::WritingPracticeSeedRes,

            // admin - users dto
            crate::api::admin::user::dto::AdminUserRes,
            crate::api::admin::user::dto::AdminUserListReq,
            crate::api::admin::user::dto::AdminUserSummary,
            crate::api::admin::user::dto::AdminUserListMeta,
            crate::api::admin::user::dto::AdminUserListRes,
            crate::api::admin::user::dto::AdminCreateUserReq,
            crate::api::admin::user::dto::AdminBulkCreateReq,
            crate::api::admin::user::dto::BulkSummary,
            crate::api::admin::user::dto::BulkItemError,
            crate::api::admin::user::dto::BulkItemResult,
            crate::api::admin::user::dto::AdminBulkCreateRes,
            crate::api::admin::user::dto::AdminUpdateUserReq,
            crate::api::admin::user::dto::AdminBulkUpdateItemReq,
            crate::api::admin::user::dto::AdminBulkUpdateReq,
            crate::api::admin::user::dto::BulkUpdateItemResult,
            crate::api::admin::user::dto::AdminBulkUpdateRes,

            // admin - videos dto
            crate::api::admin::video::dto::AdminVideoListReq,
            crate::api::admin::video::dto::AdminVideoRes,
            crate::api::admin::video::dto::AdminVideoListRes,
            crate::api::admin::video::dto::Pagination,
            crate::api::admin::video::dto::VideoBulkCreateReq,
            crate::api::admin::video::dto::VideoBulkCreateRes,
            crate::api::admin::video::dto::VideoBulkItemError,
            crate::api::admin::video::dto::VideoBulkItemResult,
            crate::api::admin::video::dto::VideoBulkSummary,
            crate::api::admin::video::dto::VideoBulkUpdateItem,
            crate::api::admin::video::dto::VideoBulkUpdateReq,
            crate::api::admin::video::dto::VideoBulkUpdateItemResult,
            crate::api::admin::video::dto::VideoBulkUpdateRes,
            crate::api::admin::video::dto::VideoCreateReq,
            crate::api::admin::video::dto::VideoRes,
            crate::api::admin::video::dto::VideoTagBulkUpdateItem,
            crate::api::admin::video::dto::VideoTagBulkUpdateReq,
            crate::api::admin::video::dto::VideoTagUpdateReq,
            crate::api::admin::video::dto::VideoUpdateReq,

            // admin - lessons dto
            crate::api::admin::lesson::dto::LessonListReq,
            crate::api::admin::lesson::dto::LessonCreateReq,
            crate::api::admin::lesson::dto::LessonCreateItem,
            crate::api::admin::lesson::dto::LessonBulkCreateReq,
            crate::api::admin::lesson::dto::LessonBulkCreateRes,
            crate::api::admin::lesson::dto::LessonBulkResult,
            crate::api::admin::lesson::dto::LessonUpdateItem,
            crate::api::admin::lesson::dto::LessonBulkUpdateReq,
            crate::api::admin::lesson::dto::LessonBulkUpdateRes,
            crate::api::admin::lesson::dto::LessonBulkUpdateResult,
            crate::api::admin::lesson::dto::LessonUpdateReq,
            crate::api::admin::lesson::dto::LessonItemListReq,
            crate::api::admin::lesson::dto::LessonProgressListReq,
            crate::api::admin::lesson::dto::LessonProgressUpdateReq,
            crate::api::admin::lesson::dto::LessonProgressUpdateItem,
            crate::api::admin::lesson::dto::LessonProgressBulkUpdateReq,
            crate::api::admin::lesson::dto::LessonProgressBulkUpdateRes,
            crate::api::admin::lesson::dto::LessonProgressBulkUpdateResult,
            crate::api::admin::lesson::dto::LessonItemCreateReq,
            crate::api::admin::lesson::dto::LessonItemUpdateReq,
            crate::api::admin::lesson::dto::LessonItemCreateItem,
            crate::api::admin::lesson::dto::LessonItemBulkCreateReq,
            crate::api::admin::lesson::dto::LessonItemBulkCreateRes,
            crate::api::admin::lesson::dto::LessonItemBulkCreateResult,
            crate::api::admin::lesson::dto::LessonItemUpdateItem,
            crate::api::admin::lesson::dto::LessonItemBulkUpdateReq,
            crate::api::admin::lesson::dto::LessonItemBulkUpdateRes,
            crate::api::admin::lesson::dto::LessonItemBulkUpdateResult,
            crate::api::admin::lesson::dto::AdminLessonItemRes,
            crate::api::admin::lesson::dto::AdminLessonItemListRes,
            crate::api::admin::lesson::dto::AdminLessonProgressRes,
            crate::api::admin::lesson::dto::AdminLessonProgressListRes,
            crate::api::admin::lesson::dto::AdminLessonRes,
            crate::api::admin::lesson::dto::AdminLessonListRes,

            // admin - studies dto
            crate::api::admin::study::dto::StudyListReq,
            crate::api::admin::study::dto::AdminStudyRes,
            crate::api::admin::study::dto::AdminStudyListRes,
            crate::api::admin::study::dto::StudyCreateReq,
            crate::api::admin::study::dto::StudyBulkCreateReq,
            crate::api::admin::study::dto::StudyBulkCreateRes,
            crate::api::admin::study::dto::StudyBulkResult,
            crate::api::admin::study::dto::StudyUpdateReq,
            crate::api::admin::study::dto::StudyBulkUpdateReq,
            crate::api::admin::study::dto::StudyBulkUpdateItem,
            crate::api::admin::study::dto::StudyBulkUpdateRes,
            crate::api::admin::study::dto::StudyBulkUpdateResult,
            crate::api::admin::study::dto::StudyTaskListReq,
            crate::api::admin::study::dto::TaskExplainListReq,
            crate::api::admin::study::dto::TaskStatusListReq,
            crate::api::admin::study::dto::AdminStudyTaskRes,
            crate::api::admin::study::dto::AdminStudyTaskListRes,
            crate::api::admin::study::dto::StudyTaskUpdateReq,
            crate::api::admin::study::dto::AdminStudyTaskDetailRes,
            crate::api::admin::study::dto::AdminTaskStatusRes,
            crate::api::admin::study::dto::AdminTaskStatusListRes,
            crate::api::admin::study::dto::TaskStatusUpdateReq,
            crate::api::admin::study::dto::TaskStatusUpdateItem,
            crate::api::admin::study::dto::TaskStatusBulkUpdateReq,
            crate::api::admin::study::dto::TaskStatusBulkUpdateResult,
            crate::api::admin::study::dto::TaskStatusBulkUpdateRes,
            crate::api::admin::study::dto::TaskExplainCreateReq,
            crate::api::admin::study::dto::TaskExplainUpdateReq,
            crate::api::admin::study::dto::TaskExplainCreateItem,
            crate::api::admin::study::dto::TaskExplainBulkCreateReq,
            crate::api::admin::study::dto::TaskExplainBulkCreateRes,
            crate::api::admin::study::dto::TaskExplainBulkResult,
            crate::api::admin::study::dto::TaskExplainBulkUpdateReq,
            crate::api::admin::study::dto::TaskExplainUpdateItem,
            crate::api::admin::study::dto::TaskExplainBulkUpdateResult,
            crate::api::admin::study::dto::TaskExplainBulkUpdateRes,
            crate::api::admin::study::dto::StudyTaskCreateReq,
            crate::api::admin::study::dto::StudyTaskBulkCreateReq,
            crate::api::admin::study::dto::StudyTaskBulkCreateRes,
            crate::api::admin::study::dto::StudyTaskBulkResult,
            crate::api::admin::study::dto::AdminTaskExplainRes,
            crate::api::admin::study::dto::AdminTaskExplainListRes,
            crate::api::admin::study::dto::StudyTaskUpdateItem,
            crate::api::admin::study::dto::StudyTaskBulkUpdateReq,
            crate::api::admin::study::dto::StudyTaskBulkUpdateRes,
            crate::api::admin::study::dto::StudyTaskBulkUpdateResult,


            // admin - translations dto
            crate::api::admin::translation::dto::TranslationListReq,
            crate::api::admin::translation::dto::TranslationCreateReq,
            crate::api::admin::translation::dto::TranslationBulkCreateReq,
            crate::api::admin::translation::dto::TranslationUpdateReq,
            crate::api::admin::translation::dto::TranslationStatusReq,
            crate::api::admin::translation::dto::TranslationRes,
            crate::api::admin::translation::dto::TranslationListRes,
            crate::api::admin::translation::dto::TranslationListMeta,
            crate::api::admin::translation::dto::TranslationBulkItemResult,
            crate::api::admin::translation::dto::TranslationBulkCreateRes,
            crate::api::admin::translation::dto::ContentRecordsReq,
            crate::api::admin::translation::dto::ContentRecordItem,
            crate::api::admin::translation::dto::ContentRecordsRes,
            crate::api::admin::translation::dto::SourceFieldsReq,
            crate::api::admin::translation::dto::SourceFieldItem,
            crate::api::admin::translation::dto::SourceFieldsRes,
            crate::api::admin::translation::dto::TranslationSearchReq,
            crate::api::admin::translation::dto::TranslationSearchItem,
            crate::api::admin::translation::dto::TranslationSearchRes,
            crate::api::admin::translation::dto::TranslationStatItem,
            crate::api::admin::translation::dto::TranslationStatsRes,

            // admin - video stats dto
            crate::api::admin::video::stats::dto::DailyStatsQuery,
            crate::api::admin::video::stats::dto::DailyStatItem,
            crate::api::admin::video::stats::dto::DailyStatsRes,

            // admin - upgrade dto (관리자 초대)
            crate::api::admin::upgrade::dto::UpgradeInviteReq,
            crate::api::admin::upgrade::dto::UpgradeInviteRes,
            crate::api::admin::upgrade::dto::UpgradeVerifyReq,
            crate::api::admin::upgrade::dto::UpgradeVerifyRes,
            crate::api::admin::upgrade::dto::UpgradeAcceptReq,
            crate::api::admin::upgrade::dto::UpgradeAcceptRes,

            // admin - email dto
            crate::api::admin::email::dto::TestEmailReq,
            crate::api::admin::email::dto::TestEmailRes,
            crate::api::admin::email::dto::EmailTemplateType,

            // payment dto
            crate::api::payment::dto::PlanInfo,
            crate::api::payment::dto::PlansRes,
            crate::api::payment::dto::SubscriptionInfo,
            crate::api::payment::dto::SubscriptionRes,
            crate::api::payment::dto::CancelSubscriptionReq,

            // textbook dto
            crate::api::textbook::dto::CatalogItem,
            crate::api::textbook::dto::CatalogRes,
            crate::api::textbook::dto::CreateOrderItemReq,
            crate::api::textbook::dto::CreateOrderReq,
            crate::api::textbook::dto::OrderItemRes,
            crate::api::textbook::dto::MyOrdersRes,
            crate::api::textbook::dto::OrderRes,

            // admin - payment dto
            crate::api::admin::payment::dto::AdminPaymentMeta,
            crate::api::admin::payment::dto::AdminSubListReq,
            crate::api::admin::payment::dto::AdminSubSummary,
            crate::api::admin::payment::dto::AdminSubListRes,
            crate::api::admin::payment::dto::AdminSubDetailRes,
            crate::api::admin::payment::dto::AdminSubDetail,
            crate::api::admin::payment::dto::AdminSubUser,
            crate::api::admin::payment::dto::AdminTxnListReq,
            crate::api::admin::payment::dto::AdminTxnSummary,
            crate::api::admin::payment::dto::AdminTxnListRes,
            crate::api::admin::payment::dto::AdminGrantReq,
            crate::api::admin::payment::dto::AdminGrantRes,
            crate::api::admin::payment::dto::AdminGrantListReq,
            crate::api::admin::payment::dto::AdminGrantSummary,
            crate::api::admin::payment::dto::AdminGrantListRes,
            crate::api::admin::payment::dto::AdminCancelSubReq,

            // admin - textbook dto
            crate::api::admin::textbook::dto::AdminTextbookListReq,
            crate::api::admin::textbook::dto::AdminTextbookMeta,
            crate::api::admin::textbook::dto::AdminTextbookListRes,
            crate::api::admin::textbook::dto::AdminTextbookLogQuery,
            crate::api::admin::textbook::dto::AdminTextbookLogItem,
            crate::api::admin::textbook::dto::AdminTextbookLogMeta,
            crate::api::admin::textbook::dto::AdminTextbookLogListRes,
            crate::api::admin::textbook::dto::AdminUpdateStatusReq,
            crate::api::admin::textbook::dto::AdminUpdateDiscountReq,
            crate::api::admin::textbook::dto::AdminUpdateTrackingReq,
            crate::api::admin::textbook::dto::AdminCreateOrderReq,

            // admin - ebook dto
            crate::api::admin::ebook::dto::AdminEbookMeta,
            crate::api::admin::ebook::dto::AdminEbookPurchaseItem,
            crate::api::admin::ebook::dto::AdminEbookListRes,
            crate::api::admin::ebook::dto::AdminUpdateEbookStatusReq,
            crate::api::admin::ebook::dto::WatermarkVerifyRes,
            crate::api::admin::ebook::dto::AdminEbookDeleteRes,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "health", description = "Health and Readiness checks"),
        (name = "auth", description = "Authentication management"),
        (name = "user", description = "User management"),
        (name = "videos", description = "Video APIs"),
        (name = "study", description = "Study APIs"),
        (name = "lesson", description = "Lesson APIs"),
        (name = "admin", description = "Admin user & content management"),
        (name = "admin_translation", description = "Admin translation management"),
        (name = "Payment", description = "Subscription and payment APIs (webhooks intentionally excluded)"),
        (name = "Textbook", description = "Textbook catalog and orders (user-facing)"),
        (name = "admin_payment", description = "Admin subscription/transaction/grant management"),
        (name = "Admin Textbook", description = "Admin textbook order management"),
        (name = "Course", description = "Course catalog (user-facing)"),
        (name = "Admin Ebook", description = "Admin ebook purchase management + watermark verification")
    )
)]
pub struct ApiDoc;
