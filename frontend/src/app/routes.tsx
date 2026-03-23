import { Navigate, Route, Routes } from "react-router-dom";

import { RootLayout } from "@/components/layout/root_layout";
import HomePage from "@/category/home/home_page";
import { AboutPage } from "@/category/about/page/about_page";

import { HealthPage } from "@/category/health/page/health_page";
import { SignupPage } from "@/category/auth/page/signup_page";
import { LoginPage } from "@/category/auth/page/login_page";
import { AccountRecoveryPage } from "@/category/auth/page/account_recovery_page";
import { ResetPasswordPage } from "@/category/auth/page/reset_password_page";
import { RequestResetPasswordPage } from "@/category/auth/page/request_reset_password_page";
import { VerifyEmailPage } from "@/category/auth/page/verify_email_page";
import { MyPage } from "@/category/user/page/my_page";
import { SettingsPage } from "@/category/user/page/settings_page";
// Coming Soon — 콘텐츠 준비 중 (video/study/lesson 임시 대체)
import { ComingSoonPage } from "@/category/coming_soon/page/coming_soon_page";
import PrivateRoute from "@/routes/private_route";

// Admin imports
import { AdminRoute } from "@/routes/admin_route";
import { AdminLayout } from "@/category/admin/page/admin_layout";
import { AdminDashboard } from "@/category/admin/page/admin_dashboard";
import { AdminUsersPage } from "@/category/admin/page/admin_users_page";
import { AdminUserDetail } from "@/category/admin/page/admin_user_detail";
import { AdminUserCreate } from "@/category/admin/page/admin_user_create";
import { AdminUserBulkCreate } from "@/category/admin/page/admin_user_bulk_create";
import { AdminVideosPage } from "@/category/admin/page/admin_videos_page";
import { AdminVideoDetail } from "@/category/admin/page/admin_video_detail";
import { AdminVideoCreate } from "@/category/admin/page/admin_video_create";
import { AdminVideoBulkCreate } from "@/category/admin/page/admin_video_bulk_create";
import { AdminVideoStatsPage } from "@/category/admin/page/admin_video_stats_page";
import { AdminUserStatsPage } from "@/category/admin/page/admin_user_stats_page";
import { AdminLoginStatsPage } from "@/category/admin/page/admin_login_stats_page";
import { AdminStudiesPage } from "@/category/admin/page/admin_studies_page";
import { AdminStudyDetail } from "@/category/admin/page/admin_study_detail";
import { AdminStudyCreate } from "@/category/admin/page/admin_study_create";
import { AdminStudyStatsPage } from "@/category/admin/page/admin_study_stats_page";
import { AdminStudyBulkCreate } from "@/category/admin/page/admin_study_bulk_create";
import { AdminLessonsPage } from "@/category/admin/page/admin_lessons_page";
import { AdminLessonDetail } from "@/category/admin/page/admin_lesson_detail";
import { AdminLessonCreate } from "@/category/admin/page/admin_lesson_create";
import { AdminLessonBulkCreate } from "@/category/admin/page/admin_lesson_bulk_create";
import { AdminTranslationsPage } from "@/category/admin/page/admin_translations_page";
import { AdminTranslationDashboard } from "@/category/admin/page/admin_translation_dashboard";
import { AdminTranslationEdit } from "@/category/admin/page/admin_translation_edit";
import { AdminEmailTest } from "@/category/admin/page/admin_email_test";
import { AdminMfaSetupPage } from "@/category/admin/page/admin_mfa_setup_page";
import { AdminUpgradeJoin } from "@/category/admin/page/admin_upgrade_join";
import { AdminSubscriptionsPage } from "@/category/admin/payment/page/admin_subscriptions_page";
import { AdminSubscriptionDetail } from "@/category/admin/payment/page/admin_subscription_detail";
import { AdminTransactionsPage } from "@/category/admin/payment/page/admin_transactions_page";
import { AdminGrantsPage } from "@/category/admin/payment/page/admin_grants_page";

// Legal pages
import { TermsPage } from "@/category/legal/page/terms_page";
import { PrivacyPage } from "@/category/legal/page/privacy_page";
import { RefundPolicyPage } from "@/category/legal/page/refund_policy_page";
import { FaqPage } from "@/category/legal/page/faq_page";

// Payment pages — PricingPage 차단 (콘텐츠 미준비, ComingSoonPage로 대체)

// Textbook pages
import { TextbookCatalogPage } from "@/category/textbook/page/textbook_catalog_page";
import { TextbookOrderPage } from "@/category/textbook/page/textbook_order_page";
import { TextbookOrderStatusPage } from "@/category/textbook/page/textbook_order_status_page";
import { AdminTextbookOrdersPage } from "@/category/admin/textbook/page/admin_textbook_orders_page";
import { AdminTextbookOrderDetail } from "@/category/admin/textbook/page/admin_textbook_order_detail";
import { AdminTextbookOrderPrint } from "@/category/admin/textbook/page/admin_textbook_order_print";

// Book landing page
import { BookLandingPage } from "@/category/book/page/book_landing_page";

// E-book pages
import { EbookCatalogPage } from "@/category/ebook/page/ebook_catalog_page";
import { EbookViewerPage } from "@/category/ebook/page/ebook_viewer_page";
import { EbookMyPurchasesPage } from "@/category/ebook/page/ebook_my_purchases_page";
import { AdminEbookPurchasesPage } from "@/category/admin/ebook/page/admin_ebook_purchases_page";
import { AdminEbookPurchaseDetail } from "@/category/admin/ebook/page/admin_ebook_purchase_detail";

// Error pages
import { AccessDeniedPage, NotFoundPage, ErrorPage } from "@/category/error/page";

export function AppRoutes() {
  return (
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
        <Route path="/textbook" element={<TextbookCatalogPage />} />
        <Route path="/textbook/order" element={<TextbookOrderPage />} />
        <Route path="/textbook/order/:code" element={<TextbookOrderStatusPage />} />
        <Route path="/ebook" element={<EbookCatalogPage />} />
        <Route path="/book/:isbn" element={<BookLandingPage />} />

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
          <Route path="/ebook/viewer/:purchaseCode" element={<EbookViewerPage />} />
          <Route path="/ebook/my" element={<EbookMyPurchasesPage />} />
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
  );
}
