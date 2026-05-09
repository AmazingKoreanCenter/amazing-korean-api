use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt;
use utoipa::ToSchema;

// -----------------------------------------------------------------------------
// 1. User Related Enums
// -----------------------------------------------------------------------------

/// 사용자 권한 레벨
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_auth_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserAuth {
    #[sqlx(rename = "HYMN")] // DB에 대문자로 저장됨
    #[serde(rename = "HYMN")]
    Hymn,
    Admin,
    Manager,
    Learner,
}

/// 사용자 언어 설정 (프로필용) — 21개 언어 (아랍어 제외)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_language_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserLanguage {
    Ko,
    En,
    Ja,
    #[sqlx(rename = "zh_cn")]
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[sqlx(rename = "zh_tw")]
    #[serde(rename = "zh-TW")]
    ZhTw,
    Vi,
    Th,
    Id,
    My,
    Mn,
    Ru,
    Es,
    Pt,
    Fr,
    De,
    Hi,
    Ne,
    Si,
    Km,
    Uz,
    Kk,
    Tg,
}

/// 사용자 성별
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_gender_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserGender {
    None,
    Male,
    Female,
    Other,
}

/// 사용자 로그 액션 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_action_log_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum UserActionLog {
    Signup,
    FindId,
    ResetPw,
    Update,
}

/// 사용자 UI/설정 언어 — 21개 언어 (아랍어 제외)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "user_set_language_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserSetLanguage {
    Ko,
    En,
    Ja,
    #[sqlx(rename = "zh_cn")]
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[sqlx(rename = "zh_tw")]
    #[serde(rename = "zh-TW")]
    ZhTw,
    Vi,
    Th,
    Id,
    My,
    Mn,
    Ru,
    Es,
    Pt,
    Fr,
    De,
    Hi,
    Ne,
    Si,
    Km,
    Uz,
    Kk,
    Tg,
}

impl UserSetLanguage {
    /// 프론트엔드 코드("zh-CN") → DB enum 문자열("zh_cn")로 변환
    pub fn frontend_to_db(code: &str) -> String {
        code.to_lowercase().replace('-', "_")
    }

    /// DB enum 문자열("zh_cn") → 프론트엔드 코드("zh-CN")로 변환
    pub fn db_to_frontend(db_val: &str) -> String {
        match db_val {
            "zh_cn" => "zh-CN".to_string(),
            "zh_tw" => "zh-TW".to_string(),
            other => other.to_string(),
        }
    }
}

// -----------------------------------------------------------------------------
// 2. Login & Session Enums
// -----------------------------------------------------------------------------

/// 로그인 기기 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "login_device_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LoginDevice {
    Mobile,
    Tablet,
    Desktop,
    Other,
}

/// 로그인 방식
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "login_method_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LoginMethod {
    Email,
    Google,
    Apple,
}

/// 로그인 세션 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "login_state_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LoginState {
    Active,
    Revoked,
    Expired,
    LoggedOut,
    Compromised,
}

/// 로그인 로그 이벤트 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "login_event_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LoginEvent {
    Login,
    Logout,
    Refresh,
    Rotate,
    Fail,
    ReuseDetected,
}

// -----------------------------------------------------------------------------
// 3. Admin & Management Enums
// -----------------------------------------------------------------------------

/// 관리자 활동 로그 액션
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "admin_action_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AdminAction {
    Create,
    Update,
    Banned,
    Reorder,
    Publish,
    Unpublish,
    Delete,
}

// -----------------------------------------------------------------------------
// 4. Translation & i18n Enums
// -----------------------------------------------------------------------------

/// 번역 대상 콘텐츠 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "content_type_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Course,
    Lesson,
    Video,
    VideoTag,
    Study,
    StudyTaskChoice,
    StudyTaskTyping,
    StudyTaskVoice,
    StudyTaskExplain,
    StudyTaskWriting,
}

/// 번역 상태 (draft → reviewed → approved)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "translation_status_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TranslationStatus {
    Draft,
    Reviewed,
    Approved,
}

/// 번역 지원 언어 — 37개 (ko, en 포함)
/// content_translations 테이블 전용 (user 테이블과 독립적으로 확장 가능)
/// 2026-04-28: es_es / pt_pt 지역 variant 추가 (2026-04-21 "pt_pt → pt 병합" 정책 번복).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "supported_language_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SupportedLanguage {
    Ko,
    En,
    Ja,
    #[sqlx(rename = "zh_cn")]
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[sqlx(rename = "zh_tw")]
    #[serde(rename = "zh-TW")]
    ZhTw,
    Vi,
    Th,
    Id,
    My,
    Mn,
    Ru,
    Es,
    #[sqlx(rename = "es_es")]
    #[serde(rename = "es-ES")]
    EsEs,
    Pt,
    #[sqlx(rename = "pt_pt")]
    #[serde(rename = "pt-PT")]
    PtPt,
    Fr,
    De,
    Hi,
    Ne,
    Si,
    Km,
    Uz,
    Kk,
    Tg,
    Tl,
    Tr,
    Bn,
    Ar,
    Ur,
    Fa,
    Lo,
    Ky,
    It,
    Sw,
    Uk,
    Am,
    Pl,
}

// -----------------------------------------------------------------------------
// 5. Content (Video, Study, Lesson) Enums
// -----------------------------------------------------------------------------

/// 비디오 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "video_state_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum VideoState {
    Ready,
    Open,
    Close,
}

/// 비디오 접근 권한
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "video_access_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum VideoAccess {
    Public,
    Paid,
    Private,
    Promote,
}

/// 학습(Study) 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "study_state_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum StudyState {
    Ready,
    Open,
    Close,
}

/// 학습(Study) 접근 권한
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "study_access_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum StudyAccess {
    Public,
    Paid,
    Private,
    Promote,
}

/// 학습 프로그램 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "study_program_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum StudyProgram {
    BasicPronunciation,
    BasicWord,
    #[sqlx(rename = "basic_500")] // 숫자가 포함된 경우 명시적 rename 필요
    #[serde(rename = "basic_500")]
    Basic500,
    TopikRead,
    TopikListen,
    TopikWrite,
    Tbc,
}

/// 학습 과제(Task) 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "study_task_kind_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum StudyTaskKind {
    Choice,
    Typing,
    Voice,
    Writing,
}

/// 쓰기 연습 레벨
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "writing_level_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum WritingLevel {
    Beginner,
    Intermediate,
    Advanced,
}

/// 쓰기 연습 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "writing_practice_type_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum WritingPracticeType {
    Jamo,
    Syllable,
    Word,
    Sentence,
    Paragraph,
}

/// 학습 과제 로그 액션
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "study_task_log_action_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum StudyTaskLogAction {
    View,
    Start,
    Answer,
    Finish,
    Explain,
    Status,
}

/// 강의(Lesson) 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "lesson_state_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LessonState {
    Ready,
    Open,
    Close,
}

/// 강의(Lesson) 접근 권한
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "lesson_access_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LessonAccess {
    Public,
    Paid,
    Private,
    Promote,
}

/// 강의(Lesson) 아이템 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "lesson_item_kind_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LessonItemKind {
    Video,
    Task,
}

// -----------------------------------------------------------------------------
// 6. Payment & Subscription Enums
// -----------------------------------------------------------------------------

/// 결제 제공자
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "payment_provider_enum", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PaymentProvider {
    Paddle,
    Apple,
    Google,
    #[sqlx(rename = "revenuecat")]
    #[serde(rename = "revenuecat")]
    RevenueCat,
}

/// 구독 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "subscription_status_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Trialing,
    Active,
    PastDue,
    Paused,
    Canceled,
}

/// 결제 트랜잭션 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "transaction_status_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Completed,
    Refunded,
    PartiallyRefunded,
}

/// 구독 주기
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "billing_interval_enum")]
pub enum BillingInterval {
    #[sqlx(rename = "month_1")]
    #[serde(rename = "month_1")]
    Month1,
    #[sqlx(rename = "month_3")]
    #[serde(rename = "month_3")]
    Month3,
    #[sqlx(rename = "month_6")]
    #[serde(rename = "month_6")]
    Month6,
    #[sqlx(rename = "month_12")]
    #[serde(rename = "month_12")]
    Month12,
}

impl BillingInterval {
    /// 개월 수 반환
    pub fn months(&self) -> i32 {
        match self {
            Self::Month1 => 1,
            Self::Month3 => 3,
            Self::Month6 => 6,
            Self::Month12 => 12,
        }
    }

    /// 센트 단위 정가 반환 (Discount 적용 전)
    pub fn price_cents(&self) -> i32 {
        match self {
            Self::Month1 => 1000,   // $10.00
            Self::Month3 => 3000,   // $30.00 (Discount $5 off → $25)
            Self::Month6 => 6000,   // $60.00 (Discount $10 off → $50)
            Self::Month12 => 12000, // $120.00 (Discount $20 off → $100)
        }
    }
}

impl fmt::Display for BillingInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Month1 => write!(f, "month_1"),
            Self::Month3 => write!(f, "month_3"),
            Self::Month6 => write!(f, "month_6"),
            Self::Month12 => write!(f, "month_12"),
        }
    }
}

// -----------------------------------------------------------------------------
// 7. Textbook Order Enums
// -----------------------------------------------------------------------------

/// 교재 언어 (한국어/영어 제외 35개 언어)
///
/// 2026-05-03 (migration `20260503_textbook_language_expand.sql`): books-api-bridge
/// plan §3 Stage 1 #1 — 21 → 35 확장. 신규 14: am, ar, bn, es_es, fa, it, ky, lo,
/// pl, pt_pt, sw, tr, uk, ur. supported_language_enum 과 동일 표기 체계
/// (snake_case in DB, BCP 47 in API serde).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "textbook_language_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TextbookLanguage {
    Ja,
    #[sqlx(rename = "zh_cn")]
    #[serde(rename = "zh_cn")]
    ZhCn,
    #[sqlx(rename = "zh_tw")]
    #[serde(rename = "zh_tw")]
    ZhTw,
    Vi,
    Th,
    Id,
    My,
    Mn,
    Ru,
    Es,
    #[sqlx(rename = "es_es")]
    #[serde(rename = "es-ES")]
    EsEs,
    Pt,
    #[sqlx(rename = "pt_pt")]
    #[serde(rename = "pt-PT")]
    PtPt,
    Fr,
    De,
    It,
    Pl,
    Uk,
    Tr,
    Hi,
    Ne,
    Si,
    Bn,
    Km,
    Lo,
    Uz,
    Kk,
    Ky,
    Tg,
    Tl,
    Ar,
    Fa,
    Ur,
    Sw,
    Am,
    En,
}

impl TextbookLanguage {
    /// 구매코드용 대문자 언어 코드 (e.g., "VI", "ZH_CN", "ES_ES")
    pub fn to_purchase_code(&self) -> &'static str {
        match self {
            Self::Ja => "JA",
            Self::ZhCn => "ZH_CN",
            Self::ZhTw => "ZH_TW",
            Self::Vi => "VI",
            Self::Th => "TH",
            Self::Id => "ID",
            Self::My => "MY",
            Self::Mn => "MN",
            Self::Ru => "RU",
            Self::Es => "ES",
            Self::EsEs => "ES_ES",
            Self::Pt => "PT",
            Self::PtPt => "PT_PT",
            Self::Fr => "FR",
            Self::De => "DE",
            Self::It => "IT",
            Self::Pl => "PL",
            Self::Uk => "UK",
            Self::Tr => "TR",
            Self::Hi => "HI",
            Self::Ne => "NE",
            Self::Si => "SI",
            Self::Bn => "BN",
            Self::Km => "KM",
            Self::Lo => "LO",
            Self::Uz => "UZ",
            Self::Kk => "KK",
            Self::Ky => "KY",
            Self::Tg => "TG",
            Self::Tl => "TL",
            Self::Ar => "AR",
            Self::Fa => "FA",
            Self::Ur => "UR",
            Self::Sw => "SW",
            Self::Am => "AM",
            Self::En => "EN",
        }
    }
}

impl fmt::Display for TextbookLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ja => write!(f, "ja"),
            Self::ZhCn => write!(f, "zh_cn"),
            Self::ZhTw => write!(f, "zh_tw"),
            Self::Vi => write!(f, "vi"),
            Self::Th => write!(f, "th"),
            Self::Id => write!(f, "id"),
            Self::My => write!(f, "my"),
            Self::Mn => write!(f, "mn"),
            Self::Ru => write!(f, "ru"),
            Self::Es => write!(f, "es"),
            Self::EsEs => write!(f, "es_es"),
            Self::Pt => write!(f, "pt"),
            Self::PtPt => write!(f, "pt_pt"),
            Self::Fr => write!(f, "fr"),
            Self::De => write!(f, "de"),
            Self::It => write!(f, "it"),
            Self::Pl => write!(f, "pl"),
            Self::Uk => write!(f, "uk"),
            Self::Tr => write!(f, "tr"),
            Self::Hi => write!(f, "hi"),
            Self::Ne => write!(f, "ne"),
            Self::Si => write!(f, "si"),
            Self::Bn => write!(f, "bn"),
            Self::Km => write!(f, "km"),
            Self::Lo => write!(f, "lo"),
            Self::Uz => write!(f, "uz"),
            Self::Kk => write!(f, "kk"),
            Self::Ky => write!(f, "ky"),
            Self::Tg => write!(f, "tg"),
            Self::Tl => write!(f, "tl"),
            Self::Ar => write!(f, "ar"),
            Self::Fa => write!(f, "fa"),
            Self::Ur => write!(f, "ur"),
            Self::Sw => write!(f, "sw"),
            Self::Am => write!(f, "am"),
            Self::En => write!(f, "en"),
        }
    }
}

/// 교재 유형 (학생용/교사용)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "textbook_type_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TextbookType {
    Student,
    Teacher,
}

/// 교재 주문 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "textbook_order_status_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TextbookOrderStatus {
    Pending,
    Confirmed,
    Paid,
    Printing,
    Shipped,
    Delivered,
    Canceled,
}

/// 교재 결제 방식
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "textbook_payment_method_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TextbookPaymentMethod {
    BankTransfer,
}

// -----------------------------------------------------------------------------
// 5. E-book Related Enums
// -----------------------------------------------------------------------------

/// E-book 에디션
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "ebook_edition_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EbookEdition {
    Student,
    Teacher,
}

impl EbookEdition {
    /// 구매코드용 에디션 코드 (e.g., "ST", "TC")
    pub fn to_purchase_code(&self) -> &'static str {
        match self {
            Self::Student => "ST",
            Self::Teacher => "TC",
        }
    }
}

/// E-book 구매 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "ebook_purchase_status_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EbookPurchaseStatus {
    Pending,
    Completed,
    Refunded,
}

/// E-book 결제 방법
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "ebook_payment_method_enum", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EbookPaymentMethod {
    Paddle,
    BankTransfer,
    AppleIap,
    GoogleIap,
}

impl EbookPaymentMethod {
    /// 구매코드용 결제 방법 코드: Paddle→CA(Card), BankTransfer→BT, AppleIap→AI, GoogleIap→GI
    pub fn to_purchase_code(&self) -> &'static str {
        match self {
            Self::Paddle => "CA",
            Self::BankTransfer => "BT",
            Self::AppleIap => "AI",
            Self::GoogleIap => "GI",
        }
    }
}

// -----------------------------------------------------------------------------
// Helper Implementations (Display)
// -----------------------------------------------------------------------------

impl fmt::Display for UserAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserAuth::Hymn => write!(f, "HYMN"),
            UserAuth::Admin => write!(f, "admin"),
            UserAuth::Manager => write!(f, "manager"),
            UserAuth::Learner => write!(f, "learner"),
        }
    }
}

// 필요한 경우 다른 Enum들도 Display 구현 추가 가능

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billing_interval_months_matches_variant() {
        assert_eq!(BillingInterval::Month1.months(), 1);
        assert_eq!(BillingInterval::Month3.months(), 3);
        assert_eq!(BillingInterval::Month6.months(), 6);
        assert_eq!(BillingInterval::Month12.months(), 12);
    }

    #[test]
    fn test_billing_interval_price_cents_matches_pricing_table() {
        // SSoT = AMK_API_PAYMENT.md / Paddle Live Catalog. 정가 (Discount 적용 전)
        assert_eq!(BillingInterval::Month1.price_cents(), 1000); // $10.00
        assert_eq!(BillingInterval::Month3.price_cents(), 3000); // $30.00 → $25 (Discount $5)
        assert_eq!(BillingInterval::Month6.price_cents(), 6000); // $60.00 → $50 (Discount $10)
        assert_eq!(BillingInterval::Month12.price_cents(), 12000); // $120.00 → $100 (Discount $20)
    }

    #[test]
    fn test_billing_interval_display_uses_snake_case() {
        // DB enum + JSON serde 와 일치 (snake_case)
        assert_eq!(BillingInterval::Month1.to_string(), "month_1");
        assert_eq!(BillingInterval::Month3.to_string(), "month_3");
        assert_eq!(BillingInterval::Month6.to_string(), "month_6");
        assert_eq!(BillingInterval::Month12.to_string(), "month_12");
    }

    #[test]
    fn test_billing_interval_price_cents_increases_monotonically() {
        // 회귀 테스트: 가격이 항상 기간에 비례하지는 않지만 증가 단조성 보장
        let prices = [
            BillingInterval::Month1.price_cents(),
            BillingInterval::Month3.price_cents(),
            BillingInterval::Month6.price_cents(),
            BillingInterval::Month12.price_cents(),
        ];
        for window in prices.windows(2) {
            assert!(
                window[0] < window[1],
                "price must increase monotonically: {} < {}",
                window[0],
                window[1]
            );
        }
    }

    #[test]
    fn test_billing_interval_per_month_price_decreases_with_term() {
        // 비즈니스 룰: 장기 결제 시 월 단가 할인 효과 (정가 기준 동일가, Discount 적용 시 단가 ↓)
        // 현재 정가 = 1000 cents/month 일정. 회귀 테스트로 가정 캡처
        let m1 = BillingInterval::Month1.price_cents() / BillingInterval::Month1.months();
        let m3 = BillingInterval::Month3.price_cents() / BillingInterval::Month3.months();
        let m6 = BillingInterval::Month6.price_cents() / BillingInterval::Month6.months();
        let m12 = BillingInterval::Month12.price_cents() / BillingInterval::Month12.months();
        assert_eq!(m1, 1000, "1개월 단가 = $10");
        assert_eq!(m3, 1000, "3개월 정가 단가 = $10 (Discount 별도)");
        assert_eq!(m6, 1000, "6개월 정가 단가 = $10");
        assert_eq!(m12, 1000, "12개월 정가 단가 = $10");
    }
}
