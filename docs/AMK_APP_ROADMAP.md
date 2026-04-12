# AMK_APP_ROADMAP — 모바일/데스크탑 앱 개발 로드맵

> 2026-03-29 작성. E-book Phase 1 웹 보안 5/5 완료 후 Phase 2(모바일) + Phase 3(데스크탑) 실행 계획.
> 플랫폼별 보안 역량: [`AMK_EBOOK_SECURITY.md §3`](./AMK_EBOOK_SECURITY.md)
> 결제 전략: [`AMK_MARKET_ANALYSIS.md §5`](./AMK_MARKET_ANALYSIS.md)
> 공통 규칙: [`AMK_API_MASTER.md`](./AMK_API_MASTER.md)

---

## 1. 프레임워크 최종 결정

### 1.1 모바일: Flutter (확정)

| 기준 | Flutter | Tauri Mobile | React Native |
|------|---------|-------------|-------------|
| Android FLAG_SECURE | 플러그인 다수 (성숙) | 1-star 플러그인 (미성숙) | screenguard |
| iOS isSecureTextEntry | 지원 (iOS 17+) | **미지원** | 지원 |
| Rust 연동 | flutter_rust_bridge v2.11.1 (5,166 stars) | 네이티브 | HTTP API만 |
| 앱 크기 | 15-20MB | 5-10MB | 40-60MB |
| 1인 개발 적합성 | Hot reload, 단일 코드베이스 (iOS+Android) | 모바일 미성숙 | JS 생태계 |

**탈락 사유:**
- **Tauri Mobile**: iOS `isSecureTextEntry` 미지원 → E-book 뷰어 보안 불충분
- **React Native**: Rust 직접 FFI 불가 (HTTP API만) → 암호화/HMAC 로직 공유 불가

### 1.2 데스크탑: Tauri 2.x (확정)

| 기준 | Tauri 2.x | Electron |
|------|-----------|----------|
| Windows 캡처 방지 | SetWindowDisplayAffinity | 동일 |
| macOS 캡처 방지 | 불가 (macOS 15+, Apple 의도적 변경) | 동일 불가 |
| Rust 코드 공유 | 네이티브 (zero FFI) | 불가 |
| 앱 크기 | 5-10MB | 150-200MB |
| UI 재사용 | 기존 React 프론트엔드 WebView 탑재 | 동일 가능 |

**핵심 장점**: 기존 `frontend/` 코드(8중 보안 포함)가 Tauri WebView에서 **수정 없이** 동작. 추가 작업은 플랫폼 보안 래퍼만.

---

## 2. Phase 2: 모바일 앱 (Flutter)

### 2.1 아키텍처

```
amazing-korean-mobile/              # 별도 리포지토리
  lib/
    main.dart
    api/
      client.dart                   # HTTP 클라이언트 (dio), 토큰 저장
      auth_api.dart                 # login-mobile, refresh-mobile, logout
      ebook_api.dart                # 카탈로그, 구매, 뷰어
    models/                         # Dart 데이터 클래스 (백엔드 DTO 대응)
    screens/
      auth/                         # 로그인, 가입, MFA, Google OAuth
      ebook/
        catalog_screen.dart
        viewer_screen.dart          # 핵심: 이미지 렌더링 + 보안
      home/
    services/
      secure_storage.dart           # flutter_secure_storage (Android Keystore / iOS Keychain)
      screenshot_guard.dart         # FLAG_SECURE + isSecureTextEntry
      hmac_service.dart             # Rust FFI 호출
    widgets/
  rust/                             # flutter_rust_bridge로 빌드
    src/lib.rs                      # HMAC-SHA256, AES decrypt (amazing-korean-crypto 의존)
    Cargo.toml
  android/
  ios/
```

### 2.2 백엔드 선행 작업

현재 Refresh Token은 **httpOnly 쿠키** 기반 (`src/api/auth/handler.rs:132`, `service.rs:384-409`). 네이티브 모바일 앱에서는 크로스 오리진 쿠키를 안정적으로 다룰 수 없으므로, body 기반 대안 엔드포인트가 필요.

**B1. `POST /auth/login-mobile`** ✅
- 기존 `AuthService::login()` 재사용
- Refresh token을 쿠키 대신 JSON body로 반환: `LoginMobileRes { user_id, access, session_id, refresh_token, refresh_expires_in }`
- 수정 파일: `src/api/auth/dto.rs`, `handler.rs`, `service.rs`, `router.rs`

**B2. `POST /auth/refresh-mobile`** ✅
- 기존 `POST /auth/refresh`는 쿠키에서 토큰 추출 (`handler.rs:132`)
- 모바일용: `RefreshReq { refresh_token }` JSON body에서 추출
- `X-Platform: mobile` 헤더 필수 (웹 클라이언트가 쿠키 보안 우회 방지)
- 내부적으로 동일한 `AuthService::refresh()` 호출
- 새 refresh token + TTL도 JSON body로 반환

**B3. 라우터/DTO** ✅
- `RefreshReq { refresh_token: String }` DTO (기존 정의 재사용)
- `LoginMobileRes { user_id, access, session_id, refresh_token, refresh_expires_in }` DTO
- `LoginOutcome::Success`에 `refresh_token: String` 필드 추가
- 라우트: `/auth/login-mobile`, `/auth/refresh-mobile`

### 2.3 모바일 보안 구현

| 웹 레이어 | 모바일 적응 |
|-----------|-----------|
| Layer 1: JWT + 소유권 | 동일 (Authorization 헤더) |
| Layer 2: 포렌식 워터마크 | 동일 (서버 사이드 적용) |
| Layer 3: Rate Limit | 동일 (서버 사이드, user_id 기반) |
| Layer 4: Canvas 보호 | 네이티브 GPU 렌더링 (img/Canvas 태그 없음). **주의**: 1587×2245px → 디코딩 ~14MB/장. `cacheWidth`/`cacheHeight`로 디스플레이 크기에 맞춰 리사이즈 필수 |
| Layer 5: 포커스/가시성 | 앱 lifecycle 감지 (AppLifecycleState) |
| Layer 6: DOM 감지 | 해당 없음 → **루팅/탈옥 감지**로 대체 |
| Layer 7: 세션 제한 | 동일 (Redis, x-ebook-session 헤더) |
| Layer 8: HMAC 서명 | **Rust FFI** (flutter_rust_bridge, Web Crypto 대신) |
| 추가: 스크린샷 차단 | **FLAG_SECURE** (Android) + **isSecureTextEntry** (iOS) |

**iOS isSecureTextEntry 리스크 (심각도: 중간)**:
- 비공식 API 활용 — Apple은 "스크린샷 차단 API는 없다"고 명시
- iOS 버전별 sublayer 접근 방식 변경 (iOS 17+ `.sublayers?.last`)
- 은행/금융 앱에서 광범위 사용 중, App Store 리젝션 사례 미확인
- **fallback 전략**: isSecureTextEntry 실패 시 `UIScreen.isCaptured` 감지 + 경고 모달로 대체

**추가 모바일 전용 보안:**
- 루팅/탈옥 감지: `flutter_jailbreak_detection`
- 인증서 피닝: `api.amazingkorean.net` 인증서 고정
- 코드 난독화: Flutter `--obfuscate --split-debug-info` 빌드 플래그
- 토큰 저장: `flutter_secure_storage` (Android Keystore / iOS Keychain)
  - **주의**: iOS에서 앱 삭제 후에도 Keychain 데이터 잔존 → 앱 첫 실행 시 Keychain 클리어 로직 필요 (`first_launch` 플래그)

### 2.4 결제 (IAP)

> 상세: [`AMK_MARKET_ANALYSIS.md §5`](./AMK_MARKET_ANALYSIS.md)

| 플랫폼 | 결제 수단 | 수수료 | 구현 |
|--------|----------|--------|------|
| iOS | StoreKit 2 (IAP 의무) | 15% (Small Business) | Apple ASN V2 웹훅 |
| Android | Play Billing | 15% (구독) | Google RTDN 웹훅 |
| 웹 | Paddle (기존) | 5% + 결제 수수료 | 구현 완료 |

**RevenueCat 도입 검토 (권장)**:
- [`purchases_flutter`](https://pub.dev/packages/purchases_flutter) — StoreKit 2 + Google Play Billing을 단일 SDK로 래핑
- 월 매출 $2,500 미만 **무료**, 이상 $19/월
- Apple ASN V2 / Google RTDN 웹훅 직접 구현 불필요 → RevenueCat 단일 웹훅으로 통합
- 백엔드 `subscriptions` 테이블의 `payment_provider` enum에 `apple`, `google` 추가 필요는 동일

### 2.5 작업 목록

| # | 작업 | 예상 | 의존 | 비고 |
|---|------|------|------|------|
| B1-B3 | 백엔드 모바일 인증 엔드포인트 | 1일 | — | |
| M1 | Flutter 프로젝트 + flutter_rust_bridge 셋업 | 1일 | — | **버전 핀닝 필수** (caret `^` 금지) |
| M2 | Rust 크레이트: HMAC-SHA256 + AES-256-GCM | 0.5일 | C1 | Rust edition 2021 유지 |
| M3 | 인증 화면 (로그인, 가입, Google OAuth) | **3일** | B1-B3 | `google_sign_in` 플러그인. Google Cloud에 앱 클라이언트 ID 별도 등록. Android: SHA-1 + App Link (autoVerify), iOS: Universal Links |
| M4 | flutter_secure_storage 토큰 저장 | 0.5일 | M3 | iOS 첫 실행 시 Keychain 클리어 |
| M5 | E-book 카탈로그 + 구매 화면 | 1.5일 | M4 | |
| M5.5 | **IAP 결제 연동** (RevenueCat 또는 직접) | **2.5일** | M5 | StoreKit 2 + Play Billing + 웹훅 |
| M6 | E-book 뷰어 (이미지 fetch + 네이티브 렌더링) | 2일 | M5 | |
| M7 | 스크린샷 가드 (FLAG_SECURE + isSecureTextEntry) | 1일 | M6 | iOS fallback: isCaptured 감지 |
| M8 | HMAC 서명 Rust FFI 연동 | 0.5일 | M2, M6 | |
| M9 | 루팅/탈옥 감지 + 인증서 피닝 (dio SecurityContext) | 0.5일 | M1 | |
| M9.5 | **CI/CD 파이프라인** (GitHub Actions + Fastlane) | **1.5일** | M1 | `macos-latest` runner (iOS), Fastlane Match (인증서) |
| M10 | 실기기 테스트 (Android + iOS) | 2일 | M7-M9.5 | |
| M11 | 스토어 제출 (아이콘/스플래시/개인정보처리방침/앱 스크린샷) | **3-5일** | M10 | 첫 심사 대기 24-48h + 리젝션 대응 |
| | **합계** | **~21-23일** | | |

### 2.6 필수 계정/비용

| 항목 | 비용 | 비고 |
|------|------|------|
| Apple Developer Account | $99/년 | iOS 빌드 + App Store 제출 필수 |
| Google Play Developer Account | $25 일회 | Play Store 제출 필수 |
| Apple Small Business Program | 무료 (신청) | IAP 수수료 30% → 15% |
| Mac (iOS 빌드) | 보유 (Mac Mini) | Xcode 필수 |

---

## 3. Phase 3: 데스크탑 앱 (Tauri 2.x)

### 3.1 아키텍처

```
amazing-korean-desktop/             # 별도 리포지토리
  src-tauri/
    src/
      main.rs
      lib.rs
      commands/
        crypto.rs                   # amazing-korean-crypto 크레이트 노출
      Cargo.toml                    # amazing-korean-crypto 의존
    tauri.conf.json
  src/                              # frontend/ 코드 복사 또는 심링크
    ...기존 React 코드 그대로...
  package.json
```

### 3.2 공유 Rust 크레이트: `amazing-korean-crypto`

기존 `src/crypto/`에서 추출 → 백엔드 + 모바일 + 데스크탑 3곳 공유. **✅ 구현 완료.**

```
crates/crypto/                      # Cargo 워크스페이스 멤버
  Cargo.toml                        # aes-gcm, hmac, sha2, base64, thiserror
  src/
    lib.rs                          # pub use CryptoService, KeyRing, CryptoError, CryptoResult
    error.rs                        # CryptoError: InvalidFormat | DecryptionFailed | Internal (thiserror)
    cipher.rs                       # AES-256-GCM encrypt/decrypt/encrypt_bytes/decrypt_bytes
    blind_index.rs                  # HMAC-SHA256 블라인드 인덱스
    service.rs                      # KeyRing + CryptoService
```

백엔드: `amazing-korean-crypto = { path = "crates/crypto" }` + `From<CryptoError> for AppError` 변환 (전 variant → `AppError::Internal` 통일, format oracle 방지).
`src/crypto/mod.rs`는 re-export 래퍼로 유지 (기존 `use crate::crypto::*` 호환).

### 3.3 데스크탑 보안

| 플랫폼 | 캡처 방지 | 구현 방법 |
|--------|----------|----------|
| **Windows** | `window.setContentProtected(true)` | **Tauri 내장 API** (v1.2.0+). `SetWindowDisplayAffinity` 자동 적용. 별도 `windows` 크레이트 불필요 |
| **macOS 15+** | **불가** | Apple 의도적 변경 (`ScreenCaptureKit`이 `NSWindow.sharingType` 무시). 워터마크 추적에 집중 |
| **Linux** | 없음 | 웹 수준 보안만 (8중 보안 그대로 동작) |

**인증**: Tauri WebView는 브라우저처럼 쿠키를 정상 지원 → 기존 httpOnly 쿠키 흐름 변경 없음.

### 3.4 작업 목록

| # | 작업 | 예상 | 의존 |
|---|------|------|------|
| C1 | ~~amazing-korean-crypto 크레이트 추출~~ | ✅ | — |
| C2 | ~~백엔드 리팩토링 (공유 크레이트 의존)~~ | ✅ | C1 |
| C3 | Tauri 2.x 프로젝트 스캐폴드 | 0.5일 | — |
| C4 | React 프론트엔드 Tauri 통합 | 1일 | C3 |
| C5 | 캡처 방지 (`setContentProtected`, Tauri 내장) | 0.5일 | C4 |
| C6 | macOS 화면녹화 감지 (best effort) | 0.5일 | C4 |
| C7 | 자동 업데이트 (Tauri 내장 updater) | 0.5일 | C4 |
| C8 | Windows + macOS + Linux 테스트 | 1.5일 | C5-C7 |
| C9 | 코드 서명 + 배포 (Windows EV cert, macOS notarize) | 1.5일 | C8 |
| | **합계** | **~7.5일** | |

---

## 4. 전체 우선순위

### 4.1 의존성 그래프 (2026-04-07 갱신)

```
Paddle Live (#0) ──→ 학습 콘텐츠 시딩 (#0.6)
교재 번역 Wave 2~5 ──→ (전 기간 병행, 기술 작업과 독립)
RDS/ElastiCache (#2) ──→ 동시 세션 제한 (#1)
~~모바일 인증 (B1-B3)~~ ✅ ──→ 모바일 앱 (Phase 2) — 개발자 계정만 남음
~~크레이트 추출 (C1-C2)~~ ✅ ──→ 데스크탑 앱 (Phase 3)
```

### 4.2 실행 순서 (2026-04-07 갱신)

| 순서 | 항목 | 예상 | 근거 |
|------|------|------|------|
| 1 | **Paddle Live 전환** | 1일 | 매출 차단 해소. GitHub Secrets 교체 + 배포 + E2E만 남음. **최우선** |
| 2 | **교재 번역 Wave 2~5** | 병행 | 16개 언어. 기술 작업과 독립, 전 기간 병행 |
| 3 | **학습 콘텐츠 시딩** | 2-3일 | Paddle Live 의존. 플랫폼에 실데이터 투입 |
| — | ~~**모바일 인증 엔드포인트** (B1-B3)~~ | ✅ | login-mobile, refresh-mobile 구현 완료 |
| — | ~~**모바일 OAuth + IAP** (B4-B5)~~ | ✅ | google-mobile, apple-mobile, mfa/login-mobile, ebook/purchase/iap, webhook/revenuecat 구현 완료 |
| — | ~~**공유 크레이트 추출** (C1-C2)~~ | ✅ | amazing-korean-crypto 크레이트 추출 완료 |
| 4 | **모바일 앱 연동** (Phase 2 잔여) | 개발자 계정 후 | 백엔드 API 전부 완료. Apple/Google 개발자 계정 + RevenueCat + 앱 측 연동만 남음 |
| 5 | **데스크탑 앱** (Phase 3, C3-C9) | ~7일 | **지금 착수 가능** — C1-C2 완료, React 프론트 재사용 |
| 6 | **RDS/ElastiCache 이전** | 3-5일 | 앱 개발 이후 |
| 7 | **동시 세션 제한** | 2-3일 | RDS 의존. 앱 출시 시 세션 표면 증가 대비 |

### 4.3 크리티컬 패스 (2026-04-07 갱신)

```
Paddle Live (1일) → 시딩 (2일) → 데스크탑 (7일, 병행 가능)
모바일: 개발자 계정 등록 → 앱 측 OAuth/IAP 연동 → 스토어 제출
RDS (4일) → 세션 제한 (2일) — 앱 개발 이후
번역은 전 기간 병행.
```

### 4.4 병행 가능 구간

- **번역 Wave 2~5**: 전 기간 병행 (기술 작업과 독립)
- **RDS 이전 + 모바일 인증**: 다른 코드 영역이므로 병행 가능
- **모바일 테스트/제출 (M10-M11)** + **데스크탑 스캐폴드 (C3-C5)**: 병행 가능

---

## 5. 기존 문서 참조 현황

> 아래 문서들에 모바일/데스크탑 관련 언급이 산재. 프레임워크 선정이 확정되었으므로 본 문서를 SSoT로 사용.

| 문서 | 관련 섹션 | 내용 |
|------|----------|------|
| `AMK_EBOOK_SECURITY.md` §3 | 플랫폼별 보안 역량 | 조사 데이터 (출처 포함) — 프레임워크 선정 근거 |
| `AMK_EBOOK_SECURITY.md` §4 | Phase 2/3 실행 계획 | 요약 수준 — 본 문서가 상세 |
| `AMK_MARKET_ANALYSIS.md` §5 | 모바일 결제 전략 | Apple IAP / Google Play Billing / 수수료 비교 |
| `AMK_MARKET_ANALYSIS.md` §7 | 실행 로드맵 | 개발 전 준비 + 앱 개발 단계 |
| `AMK_STATUS.md` §8.2 #8-9 | 진행 예정 | Phase 2/3 한줄 요약 |
| `AMK_API_EBOOK.md` | 앱 확장 로드맵 | Phase 1~3 한줄 요약 |
| `AMK_PIPELINE.md` §11 | 온디바이스 AI | 모바일 AI 배포 전략 |
| `AMK_MACMINI_SETUP.md` | iOS 개발 환경 | Xcode, iOS 시뮬레이터, Rust 타겟 |

---

## 6. 검증된 리스크 및 제약사항

> 2026-03-29 WebSearch 기반 실증 검증 결과.

| # | 항목 | 심각도 | 내용 | 대응 |
|---|------|--------|------|------|
| R1 | flutter_rust_bridge 버전 관리 | **높음** | codegen/runtime 버전 불일치 시 빌드 실패. 마이너 업데이트도 호환성 깨짐 | `pubspec.yaml` 정확한 버전 핀닝, caret `^` 금지. Rust edition 2021 유지 |
| R2 | iOS isSecureTextEntry 비공식 | **중간** | Apple 미공식 API. iOS 버전별 sublayer 접근 변경. 향후 Apple 업데이트로 무력화 가능 | fallback: `UIScreen.isCaptured` 감지 + 경고 모달. 은행 앱 사례 모니터링 |
| R3 | flutter_secure_storage iOS 잔존 | **낮음** | iOS 앱 삭제 후 Keychain 데이터 남음 → 재설치 시 구 토큰 잔존 | 앱 첫 실행 시 `first_launch` 플래그 + Keychain 클리어 |
| R4 | IAP 심사 대기 | **중간** | 첫 앱 심사 24-48h 대기 + 리젝션 가능성 (특히 결제 흐름) | 일정 버퍼 3-5일. TestFlight/Internal Testing 충분히 활용 |
| R5 | Tauri macOS 캡처 방지 불가 | **수용** | macOS 15+ Apple 의도적 변경, 모든 프레임워크 동일 | 워터마크 추적 집중, 법적 억제력 활용 |
| R6 | Rust edition 2024 비호환 | **낮음** | flutter_rust_bridge가 Rust edition 2024에서 auto-upgrade 실패 보고 | 프로젝트 Rust edition 2021 유지로 회피 |
| R7 | 페이지 이미지 메모리 | **높음** | 1587×2245px WebP → 디코딩 후 ~14MB/장. Flutter ImageCache 기본 100MB → 7장이면 초과. 프리페치 ±3 포함 시 OOM 위험 | `cacheWidth`/`cacheHeight`로 디스플레이 크기에 맞춰 리사이즈, ImageCache.maximumSizeBytes 조정, LRU 수동 관리 |
| R8 | Google OAuth 네이티브 앱 | **중간** | 웹과 달리 redirect URI가 다름. Custom URI scheme 더 이상 권장 안 됨. App Link (HTTPS) 필수 | Android: `intent-filter` + `autoVerify=true`, iOS: Universal Links. Google Cloud Console에 앱 클라이언트 ID 별도 등록 |
| R9 | iOS 빌드 CI/CD | **중간** | iOS 빌드에 `macos-latest` runner 필수 (GitHub Actions). 코드 서명: Provisioning Profile + Certificate 관리 복잡 | Fastlane Match 도입, 인증서/프로파일을 Git 암호화 저장소로 관리 |
| R10 | 한국 개인정보보호법 (PIPA) | **중간** | 앱 출시 시 개인정보처리방침 필수 (App Store/Play Store 양쪽). 한국 PIPA 추가 요구사항 존재. 2026.01부터 Sign in with Apple 사용 시 서버간 알림 엔드포인트 필수 | 기존 웹 개인정보처리방침 확장 + 앱 내 링크. Apple 서버간 알림 엔드포인트 추가 |
| R11 | Android 네트워크 보안 | **낮음** | Flutter는 기본적으로 cleartext HTTP 차단. `api.amazingkorean.net`은 HTTPS이므로 문제 없음 | `network-security-config.xml`에 debug 전용 예외만 추가 |
