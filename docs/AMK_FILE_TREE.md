# File Tree: amazing-korean-api

**Generated:** 2/18/2026, 2:11:25 PM
**Root Path:** `/home/kkryo/dev/amazing-korean-api`

```
├── 📁 .claude
│   ├── 📁 plans
│   │   └── 📝 EXTERNAL_API_INTEGRATION_PLAN.md
│   ├── ⚙️ settings.json
│   └── ⚙️ settings.local.json
├── 📁 .gemini
│   ├── ⚙️ settings.json
│   └── 📄 settings.json.orig
├── 📁 .github
│   └── 📁 workflows
│       └── ⚙️ deploy.yml
├── 📁 .sqlx
│   ├── ⚙️ query-2b449f1a74915c123db4a0adcf38feb2f0604831d4a36221e47a742b207a072a.json
│   ├── ⚙️ query-5117497ad0c466a0d382ca9fabb5f3e86c9dfd1d262e8eae3b589278a422a220.json
│   ├── ⚙️ query-550475e1e8cb25127bbec91f6ea9210866ff1aa9f550be76bb0574e28278d18a.json
│   ├── ⚙️ query-9c03e14874724877c3bde64ace8b2b1f71945fe67f5cdc51e4dd7d71d254782a.json
│   └── ⚙️ query-a162d2905e0aa110891a76a2995113f3063de7813b2f9e550d7d3e53104da385.json
├── 📁 amazing-korean-client
│   ├── 📁 assets
│   └── 🖼️ Amazing-Korean_LOGO.png
├── 📁 amazing-korean-server
│   ├── 📁 client
│   ├── 📁 public
│   │   └── 📁 images
│   └── 📁 routes
├── 📁 docs
│   ├── 📝 AGENTS.md
│   ├── 📝 AMK_API_MASTER.md
│   ├── 📝 AMK_CHANGELOG.md
│   ├── 📝 AMK_CODE_PATTERNS.md
│   ├── 📝 AMK_DEPLOY_OPS.md
│   ├── 📝 AMK_MARKET_ANALYSIS.md
│   ├── 📝 AMK_PIPELINE.md
│   ├── 📝 AMK_SCHEMA_PATCHED.md
│   ├── 📄 AMK_SCHEMA_PATCHED.sql
│   ├── 📝 GEMINI.md
│   ├── 📝 QA_ADMIN_PAYMENT.md
│   ├── 📝 QA_CHECKOUT_FLOW.md
│   ├── 📝 QA_DASHBOARD_I18N.md
│   ├── 📝 QA_MANUAL_CHECKLIST.md
│   ├── 📝 QA_MFA.md
│   ├── 📝 QA_PHASE2_TRANSLATION.md
│   ├── 📝 QA_PHASE_1B.md
│   ├── 📝 QA_POST_OVERHAUL_ADDITIONS.md
│   ├── 📝 QA_SEO_WEBHOOK.md
│   └── 📝 QA_SUBSCRIPTION_MANAGE.md
├── 📁 frontend
│   ├── 📁 @
│   │   └── 📁 components
│   │       └── 📁 ui
│   │           └── 📄 alert.tsx
│   ├── 📁 public
│   │   ├── 📁 images
│   │   │   ├── 🖼️ Amazing-Korean_LOGO.png
│   │   │   ├── 🖼️ certification_Amazing-Korean-Center.png
│   │   │   └── 🖼️ certification_Online-Marketing_Business.png
│   │   ├── 📄 robots.txt
│   │   ├── ⚙️ sitemap.xml
│   │   └── 🖼️ vite.svg
│   ├── 📁 src
│   │   ├── 📁 api
│   │   │   ├── 📄 client.ts
│   │   │   └── 📄 health_api.ts
│   │   ├── 📁 app
│   │   │   └── 📄 routes.tsx
│   │   ├── 📁 assets
│   │   │   └── 🖼️ react.svg
│   │   ├── 📁 category
│   │   │   ├── 📁 about
│   │   │   │   └── 📁 page
│   │   │   │       └── 📄 about_page.tsx
│   │   │   ├── 📁 admin
│   │   │   │   ├── 📁 components
│   │   │   │   │   └── 📄 vimeo_uploader.tsx
│   │   │   │   ├── 📁 hook
│   │   │   │   │   ├── 📄 use_admin_email.ts
│   │   │   │   │   ├── 📄 use_admin_lessons.ts
│   │   │   │   │   ├── 📄 use_admin_studies.ts
│   │   │   │   │   ├── 📄 use_admin_users.ts
│   │   │   │   │   ├── 📄 use_admin_videos.ts
│   │   │   │   │   ├── 📄 use_translation_mutations.ts
│   │   │   │   │   └── 📄 use_translations.ts
│   │   │   │   ├── 📁 lesson
│   │   │   │   │   └── 📄 types.ts
│   │   │   │   ├── 📁 page
│   │   │   │   │   ├── 📄 admin_dashboard.tsx
│   │   │   │   │   ├── 📄 admin_email_test.tsx
│   │   │   │   │   ├── 📄 admin_layout.tsx
│   │   │   │   │   ├── 📄 admin_lesson_bulk_create.tsx
│   │   │   │   │   ├── 📄 admin_lesson_create.tsx
│   │   │   │   │   ├── 📄 admin_lesson_detail.tsx
│   │   │   │   │   ├── 📄 admin_lessons_page.tsx
│   │   │   │   │   ├── 📄 admin_login_stats_page.tsx
│   │   │   │   │   ├── 📄 admin_mfa_setup_page.tsx
│   │   │   │   │   ├── 📄 admin_studies_page.tsx
│   │   │   │   │   ├── 📄 admin_study_bulk_create.tsx
│   │   │   │   │   ├── 📄 admin_study_create.tsx
│   │   │   │   │   ├── 📄 admin_study_detail.tsx
│   │   │   │   │   ├── 📄 admin_study_stats_page.tsx
│   │   │   │   │   ├── 📄 admin_translation_dashboard.tsx
│   │   │   │   │   ├── 📄 admin_translation_edit.tsx
│   │   │   │   │   ├── 📄 admin_translations_page.tsx
│   │   │   │   │   ├── 📄 admin_upgrade_join.tsx
│   │   │   │   │   ├── 📄 admin_user_bulk_create.tsx
│   │   │   │   │   ├── 📄 admin_user_create.tsx
│   │   │   │   │   ├── 📄 admin_user_detail.tsx
│   │   │   │   │   ├── 📄 admin_user_stats_page.tsx
│   │   │   │   │   ├── 📄 admin_users_page.tsx
│   │   │   │   │   ├── 📄 admin_video_bulk_create.tsx
│   │   │   │   │   ├── 📄 admin_video_create.tsx
│   │   │   │   │   ├── 📄 admin_video_detail.tsx
│   │   │   │   │   ├── 📄 admin_video_stats_page.tsx
│   │   │   │   │   └── 📄 admin_videos_page.tsx
│   │   │   │   ├── 📁 payment
│   │   │   │   │   ├── 📁 hook
│   │   │   │   │   │   └── 📄 use_admin_payment.ts
│   │   │   │   │   ├── 📁 page
│   │   │   │   │   │   ├── 📄 admin_grants_page.tsx
│   │   │   │   │   │   ├── 📄 admin_subscription_detail.tsx
│   │   │   │   │   │   ├── 📄 admin_subscriptions_page.tsx
│   │   │   │   │   │   └── 📄 admin_transactions_page.tsx
│   │   │   │   │   └── 📄 types.ts
│   │   │   │   ├── 📁 study
│   │   │   │   │   └── 📄 types.ts
│   │   │   │   ├── 📁 translation
│   │   │   │   │   └── 📄 types.ts
│   │   │   │   ├── 📁 user
│   │   │   │   │   └── 📄 types.ts
│   │   │   │   ├── 📁 video
│   │   │   │   │   └── 📄 types.ts
│   │   │   │   ├── 📄 admin_api.ts
│   │   │   │   └── 📄 types.ts
│   │   │   ├── 📁 auth
│   │   │   │   ├── 📁 components
│   │   │   │   │   └── 📄 logout_button.tsx
│   │   │   │   ├── 📁 hook
│   │   │   │   │   ├── 📄 use_find_id.ts
│   │   │   │   │   ├── 📄 use_google_login.ts
│   │   │   │   │   ├── 📄 use_login.ts
│   │   │   │   │   ├── 📄 use_logout.ts
│   │   │   │   │   ├── 📄 use_mfa_login.ts
│   │   │   │   │   ├── 📄 use_oauth_callback.ts
│   │   │   │   │   ├── 📄 use_reset_password.ts
│   │   │   │   │   └── 📄 use_signup.ts
│   │   │   │   ├── 📁 page
│   │   │   │   │   ├── 📄 account_recovery_page.tsx
│   │   │   │   │   ├── 📄 login_page.tsx
│   │   │   │   │   ├── 📄 request_reset_password_page.tsx
│   │   │   │   │   ├── 📄 reset_password_page.tsx
│   │   │   │   │   ├── 📄 signup_page.tsx
│   │   │   │   │   └── 📄 verify_email_page.tsx
│   │   │   │   ├── 📄 auth_api.ts
│   │   │   │   └── 📄 types.ts
│   │   │   ├── 📁 error
│   │   │   │   └── 📁 page
│   │   │   │       ├── 📄 access_denied_page.tsx
│   │   │   │       ├── 📄 error_page.tsx
│   │   │   │       ├── 📄 index.ts
│   │   │   │       └── 📄 not_found_page.tsx
│   │   │   ├── 📁 health
│   │   │   │   ├── 📁 hook
│   │   │   │   │   └── 📄 use_health.ts
│   │   │   │   ├── 📁 page
│   │   │   │   │   └── 📄 health_page.tsx
│   │   │   │   └── 📄 types.ts
│   │   │   ├── 📁 home
│   │   │   │   └── 📄 home_page.tsx
│   │   │   ├── 📁 legal
│   │   │   │   └── 📁 page
│   │   │   │       ├── 📄 faq_page.tsx
│   │   │   │       ├── 📄 legal_page.tsx
│   │   │   │       ├── 📄 privacy_page.tsx
│   │   │   │       ├── 📄 refund_policy_page.tsx
│   │   │   │       └── 📄 terms_page.tsx
│   │   │   ├── 📁 lesson
│   │   │   │   ├── 📁 hook
│   │   │   │   │   ├── 📄 use_lesson_detail.ts
│   │   │   │   │   ├── 📄 use_lesson_list.ts
│   │   │   │   │   └── 📄 use_lesson_progress.ts
│   │   │   │   ├── 📁 page
│   │   │   │   │   ├── 📄 lesson_detail_page.tsx
│   │   │   │   │   └── 📄 lesson_list_page.tsx
│   │   │   │   ├── 📄 lesson_api.ts
│   │   │   │   └── 📄 types.ts
│   │   │   ├── 📁 payment
│   │   │   │   ├── 📁 hook
│   │   │   │   │   ├── 📄 use_manage_subscription.ts
│   │   │   │   │   ├── 📄 use_paddle.ts
│   │   │   │   │   ├── 📄 use_payment_plans.ts
│   │   │   │   │   └── 📄 use_subscription.ts
│   │   │   │   ├── 📁 page
│   │   │   │   │   └── 📄 pricing_page.tsx
│   │   │   │   ├── 📄 payment_api.ts
│   │   │   │   └── 📄 types.ts
│   │   │   ├── 📁 study
│   │   │   │   ├── 📁 hook
│   │   │   │   │   ├── 📄 use_study_detail.ts
│   │   │   │   │   ├── 📄 use_study_list.ts
│   │   │   │   │   ├── 📄 use_study_task.ts
│   │   │   │   │   ├── 📄 use_submit_answer.ts
│   │   │   │   │   ├── 📄 use_task_explain.ts
│   │   │   │   │   └── 📄 use_task_status.ts
│   │   │   │   ├── 📁 page
│   │   │   │   │   ├── 📄 study_detail_page.tsx
│   │   │   │   │   ├── 📄 study_list_page.tsx
│   │   │   │   │   └── 📄 study_task_page.tsx
│   │   │   │   ├── 📄 study_api.ts
│   │   │   │   └── 📄 types.ts
│   │   │   ├── 📁 user
│   │   │   │   ├── 📁 hook
│   │   │   │   │   ├── 📄 use_update_settings.ts
│   │   │   │   │   ├── 📄 use_update_user.ts
│   │   │   │   │   ├── 📄 use_user_me.ts
│   │   │   │   │   └── 📄 use_user_settings.ts
│   │   │   │   ├── 📁 page
│   │   │   │   │   ├── 📄 my_page.tsx
│   │   │   │   │   └── 📄 settings_page.tsx
│   │   │   │   ├── 📄 types.ts
│   │   │   │   └── 📄 user_api.ts
│   │   │   └── 📁 video
│   │   │       ├── 📁 components
│   │   │       │   ├── 📄 video_card.tsx
│   │   │       │   └── 📄 video_player.tsx
│   │   │       ├── 📁 hook
│   │   │       │   ├── 📄 use_video_detail.ts
│   │   │       │   ├── 📄 use_video_list.ts
│   │   │       │   └── 📄 use_video_progress.ts
│   │   │       ├── 📁 page
│   │   │       │   ├── 📄 video_detail_page.tsx
│   │   │       │   └── 📄 video_list_page.tsx
│   │   │       ├── 📄 types.ts
│   │   │       └── 📄 video_api.ts
│   │   ├── 📁 components
│   │   │   ├── 📁 layout
│   │   │   │   ├── 📄 footer.tsx
│   │   │   │   ├── 📄 header.tsx
│   │   │   │   └── 📄 root_layout.tsx
│   │   │   ├── 📁 shared
│   │   │   └── 📁 ui
│   │   │       ├── 📄 alert.tsx
│   │   │       ├── 📄 aspect-ratio.tsx
│   │   │       ├── 📄 avatar.tsx
│   │   │       ├── 📄 badge.tsx
│   │   │       ├── 📄 button.tsx
│   │   │       ├── 📄 card.tsx
│   │   │       ├── 📄 checkbox.tsx
│   │   │       ├── 📄 collapsible.tsx
│   │   │       ├── 📄 dialog.tsx
│   │   │       ├── 📄 dropdown-menu.tsx
│   │   │       ├── 📄 form.tsx
│   │   │       ├── 📄 input.tsx
│   │   │       ├── 📄 label.tsx
│   │   │       ├── 📄 pagination.tsx
│   │   │       ├── 📄 progress.tsx
│   │   │       ├── 📄 select.tsx
│   │   │       ├── 📄 separator.tsx
│   │   │       ├── 📄 skeleton.tsx
│   │   │       ├── 📄 sonner.tsx
│   │   │       ├── 📄 switch.tsx
│   │   │       ├── 📄 tabs.tsx
│   │   │       └── 📄 textarea.tsx
│   │   ├── 📁 health
│   │   ├── 📁 hooks
│   │   │   ├── 📄 use_auth_store.ts
│   │   │   └── 📄 use_language_sync.ts
│   │   ├── 📁 i18n
│   │   │   ├── 📁 locales
│   │   │   │   ├── ⚙️ de.json
│   │   │   │   ├── ⚙️ en.json
│   │   │   │   ├── ⚙️ es.json
│   │   │   │   ├── ⚙️ fr.json
│   │   │   │   ├── ⚙️ hi.json
│   │   │   │   ├── ⚙️ id.json
│   │   │   │   ├── ⚙️ ja.json
│   │   │   │   ├── ⚙️ kk.json
│   │   │   │   ├── ⚙️ km.json
│   │   │   │   ├── ⚙️ ko.json
│   │   │   │   ├── ⚙️ mn.json
│   │   │   │   ├── ⚙️ my.json
│   │   │   │   ├── ⚙️ ne.json
│   │   │   │   ├── ⚙️ pt.json
│   │   │   │   ├── ⚙️ ru.json
│   │   │   │   ├── ⚙️ si.json
│   │   │   │   ├── ⚙️ tg.json
│   │   │   │   ├── ⚙️ th.json
│   │   │   │   ├── ⚙️ uz.json
│   │   │   │   ├── ⚙️ vi.json
│   │   │   │   ├── ⚙️ zh-CN.json
│   │   │   │   └── ⚙️ zh-TW.json
│   │   │   └── 📄 index.ts
│   │   ├── 📁 lib
│   │   │   └── 📄 utils.ts
│   │   ├── 📁 routes
│   │   │   ├── 📄 admin_route.tsx
│   │   │   └── 📄 private_route.tsx
│   │   ├── 📁 types
│   │   ├── 📁 utils
│   │   │   ├── 📄 content_lang.ts
│   │   │   └── 📄 font_loader.ts
│   │   ├── 📄 App.tsx
│   │   ├── 🎨 index.css
│   │   └── 📄 main.tsx
│   ├── ⚙️ .gitignore
│   ├── 📝 README.md
│   ├── ⚙️ components.json
│   ├── 📄 eslint.config.js
│   ├── 🌐 index.html
│   ├── ⚙️ package-lock.json
│   ├── ⚙️ package.json
│   ├── 📄 postcss.config.js
│   ├── 📄 qa-browser-test.mjs
│   ├── 📄 qa-debug.mjs
│   ├── 📄 qa-test.mjs
│   ├── 📄 tailwind.config.js
│   ├── ⚙️ tsconfig.app.json
│   ├── ⚙️ tsconfig.json
│   ├── ⚙️ tsconfig.node.json
│   └── 📄 vite.config.ts
├── 📁 migrations
│   ├── 📄 20260208_AMK_V1.sql
│   ├── 📄 20260208_AMK_V1_SEED.sql
│   ├── 📄 20260210_i18n_add_video_content_type.sql
│   ├── 📄 20260210_i18n_content_translations.sql
│   ├── 📄 20260212_add_study_task_explain_content_type.sql
│   ├── 📄 20260214_add_mfa_columns.sql
│   ├── 📄 20260214_video_log_ip_type_fix.sql
│   └── 📄 20260215_payment_system.sql
├── 📁 nginx
│   └── ⚙️ nginx.conf
├── 📁 scripts
│   ├── 📄 dev_preflight.sh
│   └── 📄 mk-support-bundle.sh
├── 📁 src
│   ├── 📁 api
│   │   ├── 📁 admin
│   │   │   ├── 📁 email
│   │   │   │   ├── 🦀 dto.rs
│   │   │   │   ├── 🦀 handler.rs
│   │   │   │   ├── 🦀 mod.rs
│   │   │   │   └── 🦀 router.rs
│   │   │   ├── 📁 lesson
│   │   │   │   ├── 🦀 dto.rs
│   │   │   │   ├── 🦀 handler.rs
│   │   │   │   ├── 🦀 mod.rs
│   │   │   │   ├── 🦀 repo.rs
│   │   │   │   ├── 🦀 router.rs
│   │   │   │   └── 🦀 service.rs
│   │   │   ├── 📁 payment
│   │   │   │   ├── 🦀 dto.rs
│   │   │   │   ├── 🦀 handler.rs
│   │   │   │   ├── 🦀 mod.rs
│   │   │   │   ├── 🦀 repo.rs
│   │   │   │   ├── 🦀 router.rs
│   │   │   │   └── 🦀 service.rs
│   │   │   ├── 📁 study
│   │   │   │   ├── 📁 stats
│   │   │   │   │   ├── 🦀 dto.rs
│   │   │   │   │   ├── 🦀 handler.rs
│   │   │   │   │   ├── 🦀 mod.rs
│   │   │   │   │   ├── 🦀 repo.rs
│   │   │   │   │   ├── 🦀 router.rs
│   │   │   │   │   └── 🦀 service.rs
│   │   │   │   ├── 🦀 dto.rs
│   │   │   │   ├── 🦀 handler.rs
│   │   │   │   ├── 🦀 mod.rs
│   │   │   │   ├── 🦀 repo.rs
│   │   │   │   ├── 🦀 router.rs
│   │   │   │   └── 🦀 service.rs
│   │   │   ├── 📁 translation
│   │   │   │   ├── 🦀 dto.rs
│   │   │   │   ├── 🦀 handler.rs
│   │   │   │   ├── 🦀 mod.rs
│   │   │   │   ├── 🦀 repo.rs
│   │   │   │   ├── 🦀 router.rs
│   │   │   │   └── 🦀 service.rs
│   │   │   ├── 📁 upgrade
│   │   │   │   ├── 🦀 dto.rs
│   │   │   │   ├── 🦀 handler.rs
│   │   │   │   ├── 🦀 mod.rs
│   │   │   │   ├── 🦀 router.rs
│   │   │   │   └── 🦀 service.rs
│   │   │   ├── 📁 user
│   │   │   │   ├── 📁 stats
│   │   │   │   │   ├── 🦀 dto.rs
│   │   │   │   │   ├── 🦀 handler.rs
│   │   │   │   │   ├── 🦀 mod.rs
│   │   │   │   │   ├── 🦀 repo.rs
│   │   │   │   │   ├── 🦀 router.rs
│   │   │   │   │   └── 🦀 service.rs
│   │   │   │   ├── 🦀 dto.rs
│   │   │   │   ├── 🦀 handler.rs
│   │   │   │   ├── 🦀 mod.rs
│   │   │   │   ├── 🦀 repo.rs
│   │   │   │   ├── 🦀 router.rs
│   │   │   │   └── 🦀 service.rs
│   │   │   ├── 📁 video
│   │   │   │   ├── 📁 stats
│   │   │   │   │   ├── 🦀 dto.rs
│   │   │   │   │   ├── 🦀 handler.rs
│   │   │   │   │   ├── 🦀 mod.rs
│   │   │   │   │   ├── 🦀 repo.rs
│   │   │   │   │   ├── 🦀 router.rs
│   │   │   │   │   └── 🦀 service.rs
│   │   │   │   ├── 🦀 dto.rs
│   │   │   │   ├── 🦀 handler.rs
│   │   │   │   ├── 🦀 mod.rs
│   │   │   │   ├── 🦀 repo.rs
│   │   │   │   ├── 🦀 router.rs
│   │   │   │   └── 🦀 service.rs
│   │   │   ├── 🦀 ip_guard.rs
│   │   │   ├── 🦀 mod.rs
│   │   │   ├── 🦀 role_guard.rs
│   │   │   └── 🦀 router.rs
│   │   ├── 📁 auth
│   │   │   ├── 🦀 dto.rs
│   │   │   ├── 🦀 extractor.rs
│   │   │   ├── 🦀 handler.rs
│   │   │   ├── 🦀 jwt.rs
│   │   │   ├── 🦀 mod.rs
│   │   │   ├── 🦀 password.rs
│   │   │   ├── 🦀 repo.rs
│   │   │   ├── 🦀 router.rs
│   │   │   ├── 🦀 service.rs
│   │   │   └── 🦀 token_utils.rs
│   │   ├── 📁 course
│   │   │   ├── 🦀 dto.rs
│   │   │   ├── 🦀 handler.rs
│   │   │   ├── 🦀 mod.rs
│   │   │   ├── 🦀 repo.rs
│   │   │   ├── 🦀 router.rs
│   │   │   └── 🦀 service.rs
│   │   ├── 📁 health
│   │   │   ├── 🦀 dto.rs
│   │   │   ├── 🦀 handler.rs
│   │   │   └── 🦀 mod.rs
│   │   ├── 📁 lesson
│   │   │   ├── 🦀 dto.rs
│   │   │   ├── 🦀 handler.rs
│   │   │   ├── 🦀 mod.rs
│   │   │   ├── 🦀 repo.rs
│   │   │   ├── 🦀 router.rs
│   │   │   └── 🦀 service.rs
│   │   ├── 📁 payment
│   │   │   ├── 🦀 dto.rs
│   │   │   ├── 🦀 handler.rs
│   │   │   ├── 🦀 mod.rs
│   │   │   ├── 🦀 repo.rs
│   │   │   ├── 🦀 router.rs
│   │   │   └── 🦀 service.rs
│   │   ├── 📁 scripts
│   │   │   └── 📄 db_fastcheck.sh
│   │   ├── 📁 study
│   │   │   ├── 🦀 dto.rs
│   │   │   ├── 🦀 handler.rs
│   │   │   ├── 🦀 mod.rs
│   │   │   ├── 🦀 repo.rs
│   │   │   ├── 🦀 router.rs
│   │   │   └── 🦀 service.rs
│   │   ├── 📁 user
│   │   │   ├── 🦀 dto.rs
│   │   │   ├── 🦀 handler.rs
│   │   │   ├── 🦀 mod.rs
│   │   │   ├── 🦀 repo.rs
│   │   │   ├── 🦀 router.rs
│   │   │   └── 🦀 service.rs
│   │   ├── 📁 video
│   │   │   ├── 🦀 dto.rs
│   │   │   ├── 🦀 handler.rs
│   │   │   ├── 🦀 mod.rs
│   │   │   ├── 🦀 repo.rs
│   │   │   ├── 🦀 router.rs
│   │   │   └── 🦀 service.rs
│   │   ├── 🦀 mod.rs
│   │   └── 🦀 util.rs
│   ├── 📁 crypto
│   │   ├── 🦀 blind_index.rs
│   │   ├── 🦀 cipher.rs
│   │   ├── 🦀 mod.rs
│   │   └── 🦀 service.rs
│   ├── 📁 external
│   │   ├── 🦀 email.rs
│   │   ├── 🦀 google.rs
│   │   ├── 🦀 ipgeo.rs
│   │   ├── 🦀 mod.rs
│   │   ├── 🦀 payment.rs
│   │   ├── 🦀 translator.rs
│   │   └── 🦀 vimeo.rs
│   ├── 🦀 config.rs
│   ├── 🦀 docs.rs
│   ├── 🦀 error.rs
│   ├── 🦀 lib.rs
│   ├── 🦀 main.rs
│   ├── 🦀 state.rs
│   └── 🦀 types.rs
├── ⚙️ .dockerignore
├── ⚙️ .gitignore
├── 📝 CLAUDE.md
├── 📄 Cargo.lock
├── ⚙️ Cargo.toml
├── 📄 Cargo.toml.bak
├── 🐳 Dockerfile
├── ⚙️ docker-compose.prod.yml
├── ⚙️ rust-toolchain.toml
└── 📄 verify_refresh.sh
```

---
*Generated by FileTree Pro Extension*