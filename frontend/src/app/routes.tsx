import { lazy, Suspense } from "react";
import { Navigate, Route, Routes, useParams } from "react-router-dom";

import { RootLayout } from "@/components/layout/root_layout";

// Public 핵심 — 첫 페인트 경로라 eager. 메인 번들에 유지.
import HomePage from "@/category/home/home_page";
import { AboutPage } from "@/category/about/page/about_page";
import { LoginPage } from "@/category/auth/page/login_page";
import { SignupPage } from "@/category/auth/page/signup_page";
import { BookHubPage } from "@/category/book/page/book_hub_page";
import { BookLandingPage } from "@/category/book/page/book_landing_page";

// React.lazy는 default export만 지원. named export 모듈은 .then() 어댑터로
// { default: m.ExportName } 형태로 변환해야 함 (React 공식 문서 참조).
// Coming Soon — 7개 라우트에서 사용하지만 임시 페이지라 lazy 처리
const ComingSoonPage = lazy(() => import("@/category/coming_soon/page/coming_soon_page").then((m) => ({ default: m.ComingSoonPage })));
const FaqPage = lazy(() => import("@/category/legal/page/faq_page").then((m) => ({ default: m.FaqPage })));

// Auth 보조 — 사용 빈도 낮음, lazy
const AccountRecoveryPage = lazy(() => import("@/category/auth/page/account_recovery_page").then((m) => ({ default: m.AccountRecoveryPage })));
const ResetPasswordPage = lazy(() => import("@/category/auth/page/reset_password_page").then((m) => ({ default: m.ResetPasswordPage })));
const RequestResetPasswordPage = lazy(() => import("@/category/auth/page/request_reset_password_page").then((m) => ({ default: m.RequestResetPasswordPage })));
const VerifyEmailPage = lazy(() => import("@/category/auth/page/verify_email_page").then((m) => ({ default: m.VerifyEmailPage })));
const HealthPage = lazy(() => import("@/category/health/page/health_page").then((m) => ({ default: m.HealthPage })));

// User — 로그인 후에만 접근, lazy
const MyPage = lazy(() => import("@/category/user/page/my_page").then((m) => ({ default: m.MyPage })));
const SettingsPage = lazy(() => import("@/category/user/page/settings_page").then((m) => ({ default: m.SettingsPage })));

// Writing practice (한글 자판 연습) — 로그인 후 접근, lazy
const WritingLevelSelectPage = lazy(() => import("@/category/study/page/writing_level_select_page").then((m) => ({ default: m.WritingLevelSelectPage })));
const WritingPracticePage = lazy(() => import("@/category/study/page/writing_practice_page").then((m) => ({ default: m.WritingPracticePage })));

import PrivateRoute from "@/routes/private_route";

// Admin — admin/HYMN만 접근, 일반 사용자 번들에서 완전 제거. 30+ 페이지.
import { AdminRoute } from "@/routes/admin_route";
const AdminLayout = lazy(() => import("@/category/admin/page/admin_layout").then((m) => ({ default: m.AdminLayout })));
const AdminDashboard = lazy(() => import("@/category/admin/page/admin_dashboard").then((m) => ({ default: m.AdminDashboard })));
const AdminUsersPage = lazy(() => import("@/category/admin/page/admin_users_page").then((m) => ({ default: m.AdminUsersPage })));
const AdminUserDetail = lazy(() => import("@/category/admin/page/admin_user_detail").then((m) => ({ default: m.AdminUserDetail })));
const AdminUserCreate = lazy(() => import("@/category/admin/page/admin_user_create").then((m) => ({ default: m.AdminUserCreate })));
const AdminUserBulkCreate = lazy(() => import("@/category/admin/page/admin_user_bulk_create").then((m) => ({ default: m.AdminUserBulkCreate })));
const AdminVideosPage = lazy(() => import("@/category/admin/page/admin_videos_page").then((m) => ({ default: m.AdminVideosPage })));
const AdminVideoDetail = lazy(() => import("@/category/admin/page/admin_video_detail").then((m) => ({ default: m.AdminVideoDetail })));
const AdminVideoCreate = lazy(() => import("@/category/admin/page/admin_video_create").then((m) => ({ default: m.AdminVideoCreate })));
const AdminVideoBulkCreate = lazy(() => import("@/category/admin/page/admin_video_bulk_create").then((m) => ({ default: m.AdminVideoBulkCreate })));
const AdminVideoStatsPage = lazy(() => import("@/category/admin/page/admin_video_stats_page").then((m) => ({ default: m.AdminVideoStatsPage })));
const AdminUserStatsPage = lazy(() => import("@/category/admin/page/admin_user_stats_page").then((m) => ({ default: m.AdminUserStatsPage })));
const AdminLoginStatsPage = lazy(() => import("@/category/admin/page/admin_login_stats_page").then((m) => ({ default: m.AdminLoginStatsPage })));
const AdminStudiesPage = lazy(() => import("@/category/admin/page/admin_studies_page").then((m) => ({ default: m.AdminStudiesPage })));
const AdminStudyDetail = lazy(() => import("@/category/admin/page/admin_study_detail").then((m) => ({ default: m.AdminStudyDetail })));
const AdminStudyCreate = lazy(() => import("@/category/admin/page/admin_study_create").then((m) => ({ default: m.AdminStudyCreate })));
const AdminStudyStatsPage = lazy(() => import("@/category/admin/page/admin_study_stats_page").then((m) => ({ default: m.AdminStudyStatsPage })));
const AdminStudyBulkCreate = lazy(() => import("@/category/admin/page/admin_study_bulk_create").then((m) => ({ default: m.AdminStudyBulkCreate })));
const AdminLessonsPage = lazy(() => import("@/category/admin/page/admin_lessons_page").then((m) => ({ default: m.AdminLessonsPage })));
const AdminLessonDetail = lazy(() => import("@/category/admin/page/admin_lesson_detail").then((m) => ({ default: m.AdminLessonDetail })));
const AdminLessonCreate = lazy(() => import("@/category/admin/page/admin_lesson_create").then((m) => ({ default: m.AdminLessonCreate })));
const AdminLessonBulkCreate = lazy(() => import("@/category/admin/page/admin_lesson_bulk_create").then((m) => ({ default: m.AdminLessonBulkCreate })));
const AdminTranslationsPage = lazy(() => import("@/category/admin/page/admin_translations_page").then((m) => ({ default: m.AdminTranslationsPage })));
const AdminTranslationDashboard = lazy(() => import("@/category/admin/page/admin_translation_dashboard").then((m) => ({ default: m.AdminTranslationDashboard })));
const AdminTranslationEdit = lazy(() => import("@/category/admin/page/admin_translation_edit").then((m) => ({ default: m.AdminTranslationEdit })));
const AdminEmailTest = lazy(() => import("@/category/admin/page/admin_email_test").then((m) => ({ default: m.AdminEmailTest })));
const AdminMfaSetupPage = lazy(() => import("@/category/admin/page/admin_mfa_setup_page").then((m) => ({ default: m.AdminMfaSetupPage })));
const AdminUpgradeJoin = lazy(() => import("@/category/admin/page/admin_upgrade_join").then((m) => ({ default: m.AdminUpgradeJoin })));
const AdminSubscriptionsPage = lazy(() => import("@/category/admin/payment/page/admin_subscriptions_page").then((m) => ({ default: m.AdminSubscriptionsPage })));
const AdminSubscriptionDetail = lazy(() => import("@/category/admin/payment/page/admin_subscription_detail").then((m) => ({ default: m.AdminSubscriptionDetail })));
const AdminTransactionsPage = lazy(() => import("@/category/admin/payment/page/admin_transactions_page").then((m) => ({ default: m.AdminTransactionsPage })));
const AdminGrantsPage = lazy(() => import("@/category/admin/payment/page/admin_grants_page").then((m) => ({ default: m.AdminGrantsPage })));
const AdminTextbookOrdersPage = lazy(() => import("@/category/admin/textbook/page/admin_textbook_orders_page").then((m) => ({ default: m.AdminTextbookOrdersPage })));
const AdminTextbookOrderDetail = lazy(() => import("@/category/admin/textbook/page/admin_textbook_order_detail").then((m) => ({ default: m.AdminTextbookOrderDetail })));
const AdminTextbookOrderPrint = lazy(() => import("@/category/admin/textbook/page/admin_textbook_order_print").then((m) => ({ default: m.AdminTextbookOrderPrint })));
const AdminEbookPurchasesPage = lazy(() => import("@/category/admin/ebook/page/admin_ebook_purchases_page").then((m) => ({ default: m.AdminEbookPurchasesPage })));
const AdminEbookPurchaseDetail = lazy(() => import("@/category/admin/ebook/page/admin_ebook_purchase_detail").then((m) => ({ default: m.AdminEbookPurchaseDetail })));

// Legal — 가벼움, 일부는 사용 빈도 낮음. lazy로 ↓
const TermsPage = lazy(() => import("@/category/legal/page/terms_page").then((m) => ({ default: m.TermsPage })));
const PrivacyPage = lazy(() => import("@/category/legal/page/privacy_page").then((m) => ({ default: m.PrivacyPage })));
const RefundPolicyPage = lazy(() => import("@/category/legal/page/refund_policy_page").then((m) => ({ default: m.RefundPolicyPage })));

// Textbook 카탈로그 — Public, eager
import { TextbookCatalogPage } from "@/category/textbook/page/textbook_catalog_page";
// Textbook 후속 — 주문/상태/마이/인쇄, lazy
const TextbookOrderPage = lazy(() => import("@/category/textbook/page/textbook_order_page").then((m) => ({ default: m.TextbookOrderPage })));
const TextbookOrderStatusPage = lazy(() => import("@/category/textbook/page/textbook_order_status_page").then((m) => ({ default: m.TextbookOrderStatusPage })));
const TextbookMyOrdersPage = lazy(() => import("@/category/textbook/page/textbook_my_orders_page").then((m) => ({ default: m.TextbookMyOrdersPage })));
const TextbookOrderPrint = lazy(() => import("@/category/textbook/page/textbook_order_print").then((m) => ({ default: m.TextbookOrderPrint })));

// E-book 카탈로그 — Public, eager
import { EbookCatalogPage } from "@/category/ebook/page/ebook_catalog_page";
// E-book 후속 — 무거운 뷰어/구매/마이, lazy
const EbookViewerPage = lazy(() => import("@/category/ebook/page/ebook_viewer_page").then((m) => ({ default: m.EbookViewerPage })));
const EbookMyPurchasesPage = lazy(() => import("@/category/ebook/page/ebook_my_purchases_page").then((m) => ({ default: m.EbookMyPurchasesPage })));
const EbookPurchaseCompletePage = lazy(() => import("@/category/ebook/page/ebook_purchase_complete_page").then((m) => ({ default: m.EbookPurchaseCompletePage })));

// Error pages — 가벼움, 자주 안 보임. lazy로 메인에서 분리
const AccessDeniedPage = lazy(() => import("@/category/error/page").then((m) => ({ default: m.AccessDeniedPage })));
const NotFoundPage = lazy(() => import("@/category/error/page").then((m) => ({ default: m.NotFoundPage })));
const ErrorPage = lazy(() => import("@/category/error/page").then((m) => ({ default: m.ErrorPage })));

// 라우트 단위 lazy fallback
function RouteFallback() {
  return (
    <div className="flex items-center justify-center min-h-[40vh]">
      <div className="h-8 w-8 animate-spin rounded-full border-2 border-primary border-t-transparent" aria-label="Loading" />
    </div>
  );
}

// Redirect helpers for parameterized old routes
function RedirectTextbookOrder() {
  const { code } = useParams();
  return <Navigate to={`/book/textbook/order/${code}`} replace />;
}
function RedirectTextbookOrderPrint() {
  const { code } = useParams();
  return <Navigate to={`/book/textbook/order/${code}/print`} replace />;
}
function RedirectEbookViewer() {
  const { purchaseCode } = useParams();
  return <Navigate to={`/book/ebook/viewer/${purchaseCode}`} replace />;
}

export function AppRoutes() {
  return (
    <Suspense fallback={<RouteFallback />}>
      <Routes>
      {/* RootLayout으로 모든 페이지 감싸기 (Header + Footer) */}
      <Route element={<RootLayout />}>
        {/* 누구나 접근 가능 (Public) */}
        <Route path="/" element={<HomePage />} />
        <Route path="/about" element={<AboutPage />} />
        <Route path="/intro" element={<Navigate to="/about" replace />} />
        <Route path="/find-id" element={<AccountRecoveryPage />} />
        <Route path="/login" element={<LoginPage />} />
        <Route path="/reset-password" element={<ResetPasswordPage />} />
        <Route path="/signup" element={<SignupPage />} />
        <Route path="/register" element={<Navigate to="/signup" replace />} />
        <Route path="/verify-email" element={<VerifyEmailPage />} />
        <Route path="/health" element={<HealthPage />} />
        <Route path="/request-reset-password" element={<RequestResetPasswordPage />} />
        {/* 콘텐츠 준비 중 — 오픈 시 원래 컴포넌트로 복원 */}
        <Route path="/videos" element={<ComingSoonPage />} />
        <Route path="/videos/:videoId" element={<ComingSoonPage />} />
        <Route path="/studies" element={<ComingSoonPage />} />
        <Route path="/studies/:studyId" element={<ComingSoonPage />} />
        <Route path="/studies/tasks/:taskId" element={<ComingSoonPage />} />
        <Route path="/lessons" element={<ComingSoonPage />} />
        <Route path="/lessons/:lessonId" element={<ComingSoonPage />} />
        <Route path="/pricing" element={<ComingSoonPage />} />

        {/* Book 허브 + 카탈로그 (Public) */}
        <Route path="/book" element={<BookHubPage />} />
        <Route path="/book/textbook" element={<TextbookCatalogPage />} />
        <Route path="/book/textbook/order/:code" element={<TextbookOrderStatusPage />} />
        <Route path="/book/textbook/order/:code/print" element={<TextbookOrderPrint />} />
        <Route path="/book/ebook" element={<EbookCatalogPage />} />
        <Route path="/book/:isbn" element={<BookLandingPage />} />

        {/* 기존 경로 리다이렉트 (하위 호환) */}
        <Route path="/textbook" element={<Navigate to="/book/textbook" replace />} />
        <Route path="/textbook/order/:code/print" element={<RedirectTextbookOrderPrint />} />
        <Route path="/textbook/order/:code" element={<RedirectTextbookOrder />} />
        <Route path="/ebook" element={<Navigate to="/book/ebook" replace />} />

        {/* 법적/정책 페이지 (Public) */}
        <Route path="/terms" element={<TermsPage />} />
        <Route path="/privacy" element={<PrivacyPage />} />
        <Route path="/refund-policy" element={<RefundPolicyPage />} />
        <Route path="/faq" element={<FaqPage />} />

        {/* 관리자 초대 페이지 (Public - 초대 코드로 접근) */}
        <Route path="/admin/upgrade/join" element={<AdminUpgradeJoin />} />

        {/* 로그인한 사람만 접근 가능 (Private) */}
        <Route element={<PrivateRoute />}>
          <Route path="/user/me" element={<MyPage />} />
          <Route path="/user/settings" element={<SettingsPage />} />
          <Route path="/book/textbook/order" element={<TextbookOrderPage />} />
          <Route path="/book/textbook/my" element={<TextbookMyOrdersPage />} />
          <Route path="/book/ebook/purchase-complete" element={<EbookPurchaseCompletePage />} />
          <Route path="/book/ebook/viewer/:purchaseCode" element={<EbookViewerPage />} />
          <Route path="/book/ebook/my" element={<EbookMyPurchasesPage />} />
          {/* 한글 자판 연습 (Writing practice) — 로그인 필요 */}
          <Route path="/studies/writing" element={<WritingLevelSelectPage />} />
          <Route path="/studies/writing/:level" element={<WritingPracticePage />} />
          <Route path="/studies/writing/:level/:practiceType" element={<WritingPracticePage />} />
          {/* 기존 Private 경로 리다이렉트 */}
          <Route path="/textbook/order" element={<Navigate to="/book/textbook/order" replace />} />
          <Route path="/textbook/my" element={<Navigate to="/book/textbook/my" replace />} />
          <Route path="/ebook/purchase-complete" element={<Navigate to="/book/ebook/purchase-complete" replace />} />
          <Route path="/ebook/viewer/:purchaseCode" element={<RedirectEbookViewer />} />
          <Route path="/ebook/my" element={<Navigate to="/book/ebook/my" replace />} />
        </Route>

        {/* 에러 페이지 (RootLayout 내부 — Header/Footer 유지) */}
        <Route path="/403" element={<AccessDeniedPage />} />
        <Route path="/error" element={<ErrorPage />} />
        <Route path="*" element={<NotFoundPage />} />
      </Route>

      {/* Admin 라우트 (admin/HYMN 권한 필요) */}
      <Route element={<AdminRoute />}>
        {/* MFA 설정 페이지 (AdminLayout 밖 — MFA 미설정 시 강제 이동) */}
        <Route path="/admin/mfa/setup" element={<AdminMfaSetupPage />} />
        <Route path="/admin" element={<AdminLayout />}>
          <Route index element={<AdminDashboard />} />
          <Route path="users" element={<AdminUsersPage />} />
          <Route path="users/stats" element={<AdminUserStatsPage />} />
          <Route path="users/new" element={<AdminUserCreate />} />
          <Route path="users/bulk-create" element={<AdminUserBulkCreate />} />
          <Route path="users/:userId" element={<AdminUserDetail />} />
          <Route path="logins/stats" element={<AdminLoginStatsPage />} />
          <Route path="videos" element={<AdminVideosPage />} />
          <Route path="videos/stats" element={<AdminVideoStatsPage />} />
          <Route path="videos/new" element={<AdminVideoCreate />} />
          <Route path="videos/bulk-create" element={<AdminVideoBulkCreate />} />
          <Route path="videos/:videoId" element={<AdminVideoDetail />} />
          <Route path="studies" element={<AdminStudiesPage />} />
          <Route path="studies/stats" element={<AdminStudyStatsPage />} />
          <Route path="studies/new" element={<AdminStudyCreate />} />
          <Route path="studies/bulk-create" element={<AdminStudyBulkCreate />} />
          <Route path="studies/:studyId" element={<AdminStudyDetail />} />
          <Route path="lessons" element={<AdminLessonsPage />} />
          <Route path="lessons/new" element={<AdminLessonCreate />} />
          <Route path="lessons/bulk-create" element={<AdminLessonBulkCreate />} />
          <Route path="lessons/:lessonId" element={<AdminLessonDetail />} />
          <Route path="translations" element={<AdminTranslationsPage />} />
          <Route path="translations/dashboard" element={<AdminTranslationDashboard />} />
          <Route path="translations/new" element={<AdminTranslationEdit />} />
          <Route path="translations/:id/edit" element={<AdminTranslationEdit />} />
          <Route path="payment" element={<Navigate to="subscriptions" replace />} />
          <Route path="payment/subscriptions" element={<AdminSubscriptionsPage />} />
          <Route path="payment/subscriptions/:id" element={<AdminSubscriptionDetail />} />
          <Route path="payment/transactions" element={<AdminTransactionsPage />} />
          <Route path="payment/grants" element={<AdminGrantsPage />} />
          <Route path="textbook" element={<Navigate to="textbook/orders" replace />} />
          <Route path="textbook/orders" element={<AdminTextbookOrdersPage />} />
          <Route path="textbook/orders/:orderId" element={<AdminTextbookOrderDetail />} />
          <Route path="textbook/orders/:orderId/print" element={<AdminTextbookOrderPrint />} />
          <Route path="ebook" element={<Navigate to="ebook/purchases" replace />} />
          <Route path="ebook/purchases" element={<AdminEbookPurchasesPage />} />
          <Route path="ebook/purchases/:id" element={<AdminEbookPurchaseDetail />} />
          <Route path="email" element={<AdminEmailTest />} />
        </Route>
      </Route>

      </Routes>
    </Suspense>
  );
}
