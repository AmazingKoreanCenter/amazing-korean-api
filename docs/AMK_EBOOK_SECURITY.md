# AMK_EBOOK_SECURITY — E-book 보안 전략 및 앱 개발 로드맵

> 2026-03-29 작성. E-book 서비스 보안 강화를 위한 조사 결과, 현황 분석, 실행 계획.
> 모든 항목은 도구(WebSearch, Read, Grep)로 검증된 근거 기반.

---

## 1. 현재 E-book 보안 현황

### 1.1 구현 완료 (10층 보안)

| # | 레이어 | 구현 위치 | 상태 |
|---|--------|----------|------|
| 1 | JWT 인증 + AuthUser extractor | `handler.rs` | ✅ |
| 2 | 소유권 검증 (user_id + completed 상태) | `service.rs` | ✅ |
| 3 | Rate Limiting (IP: 구매, User: 페이지/타일) | `handler.rs` | ✅ |
| 4 | Redis 세션 (UUID + TTL 90s + Heartbeat 30s) | `service.rs` | ✅ |
| 5 | 커스텀 헤더 (`X-Ebook-Viewer` + `X-Ebook-Session`) | `handler.rs` | ✅ |
| 6 | Canvas API 무력화 (toDataURL/toBlob/getImageData/captureStream/transferControlToOffscreen/createImageBitmap) | `ebook_viewer_page.tsx` | ✅ |
| 7 | DOM 변조 감지 (MutationObserver + getComputedStyle 폴링) | `ebook_viewer_page.tsx` | ✅ |
| 8 | 포커스/가시성 감지 (blur/visibility/print) | `ebook_viewer_page.tsx` | ✅ |
| 9 | 워터마크 4중 (풋터 + 마이크로도트 + LSB + 접근로그) | `watermark.rs` | ✅ |
| 10 | 응답 헤더 (no-store, nosniff, DENY, no-referrer) | `handler.rs`, `main.rs` | ✅ |

### 1.2 미구현 (검증된 격차)

| # | 항목 | 격차 근거 |
|---|------|----------|
| 1 | 페이지 이미지 암호화 저장 | Fasoo 자동 암호화 (fasoo.com), 리디 AES-128 (GitHub ridi-decrypt) — 현재 WebP 평문 |
| 2 | 요청별 HMAC 서명 | 전자서명법 "변경 여부 확인" 원칙 (law.go.kr) |
| 3 | 진위확인 시스템 | 정부24 발급번호 조회 (gov.kr/confrm) |
| 4 | DevTools 감지 | 마크애니 소스보기 차단 (markany.com) |
| 5 | 저작권 보호 고지 | 저작권법 제104조의2 TPM 보호 (casenote.kr) |
| 6 | OS 레벨 스크린샷 차단 | Android FLAG_SECURE (developer.android.com) — 웹에서 불가, 네이티브 앱 필요 |

---

## 2. 대한민국 공문서/상용 E-book 보안 조사 결과

### 2.1 정부24 문서 발급 보안

| 보안 요소 | 기술 | 출처 |
|-----------|------|------|
| 복사방지마크 | Copy Detection Pattern (CDP) — 인쇄/스캔 복합 열화로 복사 감지 | Wikipedia CDP, HAL 논문 hal-03507376 |
| 2D 고밀도 바코드 | 보이스아이(VoiceEye) — 1.5cm² 바코드에 2페이지 분량 + 전자서명 | voiceye.com |
| 진위확인 3중 | 발급번호 조회 + 바코드 스캐너 + 모바일 스마트검증 | gov.kr/confrm |
| 타임스탬프 | GTSA — SHA-256 해시 + TST(Time-Stamp Token) | gtsa.go.kr |
| 본인인증 | 공동인증서 / 금융인증서 / 간편인증 / PASS 앱 | gov.kr 회원가입 안내 |

### 2.2 한국 E-book DRM 사례

| 서비스 | DRM | 핵심 기술 | 출처 |
|--------|-----|-----------|------|
| 교보문고 | Fasoo DRM | fph.exe 상주 프로세스 + 자동 암호화 + 화면캡처방지 | fasoo.com |
| 리디북스 | 자체 DRM | **username** 기반 AES-128 (ECB: 최신 / CBC: 이전 버전). DRM 주기적 변경 | GitHub ridi-decrypt, Medium John Dykes |
| YES24/알라딘 | MarkAny X-Safe | 한국이퍼브 기반 크레마 앱 | clien.net |
| OPENS DRM | 한국 표준 | KS X 6072, ISO TS 23078 | drminside.com |
| Readium LCP | ISO 표준 | ISO/IEC 23078-2:2024, AES-256, 84개국 1,200만+ E-book | iso.org, EDRLab |

> **수정**: 최초 조사에서 "리디 device_id 기반"으로 기재했으나, 검증 결과 **username 기반** (SHA1('book-{username}')[2:18])으로 확인 (GitHub ridi-decrypt).

### 2.3 알라딘 유출 사건 (2023)

| 항목 | 사실 | 출처 |
|------|------|------|
| 시기 | 2023년 5월 | 한국경제 |
| 주범 | 16세 고등학생 | 보안뉴스 |
| 수법 | DRM 복호화 키 자동 탈취 프로그램 | 보안뉴스 |
| 복호화키 탈취 | ~72만 건 | 보안뉴스 |
| 실제 확인 유출 | ~5,000종 (EPUB/PDF) | 한국저작권보호원 |
| 피해액 | 203억원 (4개 업체 합산) | 한국경제 |
| 요구 | 비트코인 100개 (~360억원) | 보안뉴스 |

**교훈**: 클라이언트에 암호화 원본 파일 전송 → DRM 키 탈취로 대량 복호화. 우리 시스템은 서버에서 워터마크 적용 후 Canvas 렌더링용 이미지만 전송하므로 알라딘형 대량 추출 구조적으로 회피.

### 2.4 법적 근거

| 법령 | 내용 | 출처 |
|------|------|------|
| 저작권법 제104조의2 | 기술적 보호조치(TPM) 무력화 금지 | casenote.kr |
| 저작권법 제136조 2항 | **영리/업 목적** 위반 시 3년 이하 징역 / 3천만원 이하 벌금 | 국가법령정보센터 |
| DRM 의무 여부 | 법적 강제 아님, 업계 관행 | 검색 미확인 (의무화 법률 없음) |

> TPM을 적용하고 이를 고지하면, 영리 목적 우회 시 형사처벌 근거 확보. DRM이 법적 의무는 아니지만 적용 시 법적 보호력 상승.

---

## 3. 플랫폼별 보안 역량 (검증 완료)

### 3.1 모바일

| 플랫폼 | 스크린샷 차단 | 화면녹화 차단 | 비고 | 출처 |
|--------|-------------|-------------|------|------|
| **Android** | `FLAG_SECURE` → 검정/공백/단색 | 동일 | a11y 서비스·가상키보드 우회 가능 (비루팅) | developer.android.com, Ostorlab, Nightwatch |
| **iOS** | **차단 불가** | `UIScreen.isCaptured` 감지만 | `isSecureTextEntry` 해킹: iOS 17/18 작동, `.sublayers?.last` 사용 필요 | Medium khush7068, Swift and Curious 2026.03 |
| **iOS 엔터프라이즈** | MS Intune MAM SDK (2024.11~) | 동일 | 일반 앱 미적용 | Microsoft Tech Community |

### 3.2 데스크탑

| 플랫폼 | 캡처 방지 | 비고 | 출처 |
|--------|----------|------|------|
| **Windows** | `SetWindowDisplayAffinity` → 검정 화면 | Tauri/Electron 모두 가능 | MS 문서 |
| **macOS 15+** | **불가** (Apple 의도적 변경) | ScreenCaptureKit가 `NSWindow.sharingType` 무시 | Tauri #14200, Apple Forums |
| **Linux** | 메커니즘 없음 | — | — |

### 3.3 앱 프레임워크

| 프레임워크 | Android FLAG_SECURE | iOS isSecureTextEntry | Rust 연동 | 출처 |
|-----------|--------------------|-----------------------|-----------|------|
| **Flutter** | `no_screenshot` 등 플러그인 다수 | 지원 (iOS 17+ 대응) | flutter_rust_bridge v2.11.1 (별 5,166) | pub.dev, GitHub fzyzcjy |
| **Tauri Mobile** | 플러그인 존재 (미성숙: 별 1, 2025-03) | 미지원 | 네이티브 Rust | GitHub aiueo13 |
| **React Native** | `react-native-screenguard` | 지원 | HTTP API만 | npm |

| 데스크탑 | Windows 캡처 방지 | macOS 캡처 방지 | 크기 | 출처 |
|---------|------------------|----------------|------|------|
| **Tauri** | 가능 | 불가 (macOS 15+) | 5-10MB | Tauri #14200 |
| **Electron** | 가능 | 불가 (동일) | 150-200MB | Electron #31787 |

### 3.4 WASM 뷰어

| 제품 | 유형 | DRM | 출처 |
|------|------|-----|------|
| Nutrient (구 PSPDFKit) | WASM PDF 뷰어 | Document Engine 페어링 | nutrient.io |
| MuPDF WebViewer | WASM PDF 뷰어 | 없음 | mupdf.com |

> WASM은 리버스 엔지니어링 난이도를 높이나, Canvas 렌더링 한계는 JS와 동일. 스크린 캡처 방지 불가. 현재 투자 대비 효과 낮음.

---

## 4. 실행 계획

### Phase 1: 웹 보안 강화 (즉시)

| # | 작업 | 근거 | 우선순위 |
|---|------|------|---------|
| 1-1 | 페이지 이미지 AES-256 암호화 저장 + 요청 시 복호화 | Fasoo/리디 패턴 | 높음 |
| 1-2 | 요청별 HMAC 서명 (timestamp + purchase_code + page_num) | 전자서명법 원칙 | 높음 |
| 1-3 | DevTools 감지 (debugger, 창 크기 변화) | 마크애니 참고 | 중 |
| 1-4 | 워터마크 진위확인 API (`GET /ebook/verify/{watermark_id}`) | 정부24 참고 | 중 |
| 1-5 | 저작권 보호 고지 + TPM 이용약관 | 저작권법 제104조의2 | 중 |

### Phase 2: 모바일 앱 (Android 우선)

| # | 작업 | 근거 |
|---|------|------|
| 2-1 | 프레임워크 최종 선정 | FLAG_SECURE + Rust 연동 + iOS 대응 |
| 2-2 | Android: FLAG_SECURE + Play Integrity API | OS 레벨 캡처 차단 |
| 2-3 | iOS: isSecureTextEntry + isCaptured 감지 | 플랫폼 한계 내 최선 |

### Phase 3: 데스크탑 앱 (Tauri)

| # | 작업 | 근거 |
|---|------|------|
| 3-1 | Tauri Desktop: Windows SetWindowDisplayAffinity | Windows 캡처 방지 |
| 3-2 | macOS: 워터마크 강화 (캡처 방지 불가 → 추적 집중) | Apple 의도적 변경 |

---

## 5. 보안 한계 인식

- **어떤 DRM도 100% 완벽하지 않음** — 알라딘(Fasoo DRM), 리디(자체 DRM) 모두 우회됨
- **macOS 15+에서 모든 프레임워크의 화면 캡처 방지 무력화** — Apple 의도적 아키텍처 변경, 워크어라운드 없음
- **Android FLAG_SECURE도 루팅 기기에서 우회 가능** (Xposed DisableFlagSecure)
- **iOS는 OS 레벨 스크린샷 차단 API 자체가 없음** — isSecureTextEntry 해킹은 비공식, Apple 업데이트 시 깨질 수 있음
- **핵심 전략**: "완벽한 차단"이 아닌 "비용 대비 효과" — 다층 방어 + 포렌식 추적 + 법적 억제력
