# AMK_API_EBOOK — E-book 웹 뷰어 API 스펙

> 회원 전용 E-book 웹 뷰어 시스템 (7중 보안 + 워터마크).
> 공통 규칙(인증, 에러, 페이징): [AMK_API_MASTER.md §3](./AMK_API_MASTER.md)
> DB 스키마: [AMK_SCHEMA_PATCHED.md](./AMK_SCHEMA_PATCHED.md)
> 코드 패턴: [AMK_CODE_PATTERNS.md](./AMK_CODE_PATTERNS.md)

---

### 5.12.5 Phase 12.5 — E-book 웹 뷰어 (E-book Web Viewer) ✅

> 자체 사이트에서 교재(학생용/교사용) e-book을 구매하고 열람할 수 있는 웹 뷰어 시스템.
> **회원 전용** — 로그인 필수 (비회원 구매 불가), `user_id`로 구매 연동.
> **웹 전용** — 페이지 이미지 기반 렌더링 (EPUB/PDF 원본 미노출, 다운로드 없음).
> 향후 모바일 앱(Flutter) 및 데스크탑 앱(Tauri 2.x)에서 오프라인 EPUB 뷰어로 확장. 상세: [`AMK_APP_ROADMAP.md`](./AMK_APP_ROADMAP.md)

**교재 3종 유통 정책**:
| 종류 | 용도 | 유통 채널 |
|------|------|----------|
| 학생용 | 교사가 가르칠 때 학생이 사용 | **자체 사이트만** (인쇄물 + e-book) |
| 교사용 | 교사가 학생을 가르칠 때 사용 | **자체 사이트만** (인쇄물 + e-book) |
| 해설용 | 학생이 혼자 공부 | 자체 사이트 + 외부 플랫폼 (Amazon/Apple/Google/Kobo/교보/Yes24) |

**8중 보안 아키텍처**:
```
Layer 1: 구조적 보안
  • 회원 전용 — AuthUser JWT 필수 (비회원 구매 불가)
  • EPUB/PDF 파일 미노출 (페이지 이미지만 제공)
  • 다운로드 없음 — 웹 뷰어 전용 (오프라인은 추후 앱)
  • user_id로 구매 소유 확인 (타인 구매 접근 차단)

Layer 2: 포렌식 워터마크 (실시간 동적)
  • 풋터 워터마크: purchase_code + page_num
  • 마이크로 도트: user_id 64비트 near-white 도트 인코딩
  • LSB 스테가노그래피: purchase_code + watermark_id SHA-256
  • 감사 로그: ebook_access_log 테이블 (watermark_id, IP, UA)

Layer 3: 플랫폼 보안
  • 브라우저: 우클릭/선택/인쇄/드래그 차단
  • Cache-Control: private, max-age=300 (5분 TTL)
  • blob:// URL 사용 (네트워크 탭에 이미지 URL 미노출)
  • 레이트 리밋: 30페이지/분/user (벌크 크롤링 차단)

Layer 4: Canvas 추출 API 무력화 (프론트)
  • toDataURL, toBlob, getImageData → 1x1 투명 반환
  • captureStream → 빈 MediaStream
  • OffscreenCanvas, createImageBitmap(canvas) 차단
  • 뷰어 mount 시 프로토타입 오버라이드, unmount 시 복원

Layer 5: 포커스/가시성 감지 → 콘텐츠 블러 (프론트)
  • visibilitychange (primary) + blur/focus (secondary)
  • beforeprint/afterprint → Ctrl+P 시도 시 블러
  • CSS filter: blur(30px), will-change: filter (GPU 가속)

Layer 6: DOM 조작 감지 (프론트)
  • MutationObserver: canvas 삭제, style 변경 감지
  • getComputedStyle 주기 검사 (2초): CSS 규칙 추가 감지
  • 탬퍼링 시 canvas 클리어 + 강제 퇴장

Layer 7: 동시 세션 제한 (백엔드 + 프론트)
  • Redis SET EX: user별 단일 세션 (Last Writer Wins)
  • 90초 TTL / 30초 heartbeat (3:1 비율)
  • 새 기기 접속 시 기존 세션 자동 만료
  • Redis 장애 시 fail closed (접근 거부)

Layer 8: 요청별 HMAC 서명 (백엔드 + 프론트)
  • 세션 등록 시 32바이트 랜덤 secret 생성 → Redis 저장 + 클라이언트 전달
  • 페이지/타일 요청: X-Ebook-Signature + X-Ebook-Timestamp 헤더 필수
  • HMAC-SHA256(secret, "{session_id}:{path}:{timestamp}")
  • ±30초 타임스탬프 윈도우 (리플레이 공격 방지)
  • 상수 시간 비교 (타이밍 공격 방지)
  • 프론트: Web Crypto API crypto.subtle.sign("HMAC")
```

**타일 분할 전송** (기능 플래그: `EBOOK_TILE_MODE`):
```
• 3×3 그리드 (9 타일/페이지), 전체 이미지에 워터마크 적용 후 분할
• 기능 플래그로 점진 롤아웃 (기본 false)
• 전용 Rate Limit: 270/분/user (30페이지 × 9타일)
• 프리페치 ±2 페이지 (HTTP/2 동시 100+ 스트림 내)
• 나머지 픽셀: 마지막 행/열은 총크기-offset으로 계산
• imageSmoothingEnabled=false (타일 경계 이음새 방지)
```

**앱 확장 로드맵**:
```
Phase 1 [웹] ✅ 페이지 이미지 뷰어 — 온라인 전용, EPUB 미노출
Phase 1.5 [웹 모바일] ✅ 터치 스와이프 + 반응형 UI + spread 자동 비활성화
Phase 2 [모바일 앱] Flutter + FLAG_SECURE/isSecureTextEntry + Rust FFI — 오프라인 지원
Phase 3 [데스크탑 앱] Tauri 2.x + SetWindowDisplayAffinity (Windows) — 오프라인 지원
```

**DB**: ENUM 3개 (`ebook_edition_enum`, `ebook_purchase_status_enum`, `ebook_payment_method_enum`) + 테이블 3개 (`ebook_purchase`, `ebook_access_log`, `admin_ebook_log`)

**가격**: 교사용 ₩15,000 / 학생용 ₩12,000 (KRW)

<details>
<summary>📋 E-book 웹 뷰어 엔드포인트 상세 (클릭)</summary>

#### 12.5-1 : `GET /ebook/catalog` (e-book 카탈로그)

> 구매 가능한 e-book 목록 (언어별, 에디션별). manifest.json에서 페이지 수 로드.

**인증**: 불필요

**응답 (성공 200)**
```json
{
  "items": [
    {
      "language": "vi",
      "language_name_ko": "베트남어",
      "language_name_en": "Vietnamese",
      "editions": [
        { "edition": "teacher", "price": 15000, "currency": "KRW", "paddle_price_usd": 1000, "total_pages": 124, "available": true },
        { "edition": "student", "price": 12000, "currency": "KRW", "paddle_price_usd": 1000, "total_pages": 90, "available": true }
      ]
    }
  ],
  "paddle_ebook_price_id": "pri_xxx",
  "client_token": "live_xxx",
  "sandbox": false
}
```

- `paddle_price_usd`: Paddle 결제 시 USD 가격 (cents, $10.00 = 1000). null이면 Paddle 미지원.
- `paddle_ebook_price_id`, `client_token`: Paddle 결제 미설정 시 null.
- `sandbox`: true이면 Paddle Sandbox 환경.

#### 12.5-2 : `POST /ebook/purchase` (e-book 구매)

> e-book 구매 생성. 로그인 필수. Paddle 결제 또는 계좌이체.
> 중복 구매 방지 (동일 user + language + edition, completed 상태).

**인증**: AuthUser (JWT)

**Rate Limit**: IP 기반 5회/시간

**요청**
```json
{
  "language": "vi",
  "edition": "teacher",
  "payment_method": "paddle"
}
```

**응답 (성공 200)**
```json
{
  "purchase_code": "VN-TC-20260310-CA-0001",
  "status": "pending",
  "language": "vi",
  "edition": "teacher",
  "payment_method": "paddle",
  "price": 1000,
  "currency": "USD",
  "created_at": "2026-03-09T12:00:00Z"
}
```

**구매코드 형식**: `{LANG}-{ED}-{YYYYMMDD}-{PAY}-{NNNN}`
- LANG: 언어 코드 (VI, JA, ZH_CN, ZH_TW, RU, MN, MY, TH, HI, NE, SI, KM, ES, PT, FR, DE, ID, UZ, KK, TG, TL)
- ED: ST (학생용) / TC (교사용)
- PAY: CA (Paddle 카드) / BT (계좌이체)
- NNNN: 일별 순번 (MAX 기반, Advisory Lock으로 동시성 안전)
- DB: `purchase_code VARCHAR(30)` (최대 25자: ZH_CN-TC-20260310-BT-0001)

#### 12.5-3 : `GET /ebook/my` (내 구매 목록)

> 로그인한 사용자의 e-book 구매 목록 조회.

**인증**: AuthUser (JWT)

**응답 (성공 200)**
```json
{
  "items": [
    {
      "purchase_code": "VN-TC-20260310-CA-0001",
      "status": "completed",
      "language": "vi",
      "edition": "teacher",
      "payment_method": "paddle",
      "price": 1000,
      "currency": "USD",
      "created_at": "2026-03-09T12:00:00Z"
    }
  ]
}
```

#### 12.5-3.5 : `DELETE /ebook/purchase/{code}` (구매 취소)

> pending 상태의 구매를 취소 (soft delete). 본인 소유만 가능.

**인증**: AuthUser (JWT)

**응답**: 204 No Content (성공) / 404 Not Found (없거나 pending 아님)

#### 12.5-4 : `GET /ebook/viewer/{code}/meta` (뷰어 메타 정보)

> 뷰어 초기화 데이터 (TOC, 총 페이지 수). 구매 소유 + completed 상태 확인.

**인증**: AuthUser (JWT) + 구매 소유 확인

**응답 (성공 200)**
```json
{
  "purchase_code": "VI-ST-20260310-CA-0001",
  "language": "vi",
  "edition": "teacher",
  "total_pages": 124,
  "toc": [
    { "title": "Part I. 발음", "page": 1 },
    { "title": "Part II. 어휘", "page": 25 }
  ],
  "session_id": "uuid-v4",
  "hmac_secret": "hex-encoded-32-bytes",
  "tile_mode": false,
  "grid_rows": null,
  "grid_cols": null
}
```

- `session_id`: Redis 동시 세션 관리용 UUID (뷰어 진입 시 발급, heartbeat에 사용)
- `hmac_secret`: 세션별 HMAC-SHA256 서명 키 (32바이트, hex 인코딩). 페이지/타일 요청 시 서명 계산에 사용
- `tile_mode`: true이면 타일 분할 전송 모드 (서버 `EBOOK_TILE_MODE` 설정)
- `grid_rows`, `grid_cols`: 타일 그리드 크기 (tile_mode=true 시 값 존재)

#### 12.5-4.5 : `POST /ebook/viewer/heartbeat` (뷰어 세션 heartbeat)

> 뷰어 세션 유효성 확인 + TTL 갱신. 30초 간격 호출.

**인증**: AuthUser (JWT)

**요청**
```json
{ "session_id": "uuid-v4" }
```

**응답 (성공 200)**
```json
{ "valid": true }
```
- `valid: false` → 다른 기기가 세션 점유 or 세션 만료 → 프론트에서 `/ebook/my`로 이동
- Redis 세션 데이터 파싱 실패 시 500 에러 반환 (fail-closed, 2026-04-07 수정)

#### 12.5-5 : `GET /ebook/viewer/{code}/pages/{page_num}` (페이지 이미지 조회)

> 워터마크 적용된 페이지 이미지 반환. 보안 핵심 엔드포인트.

**인증**: AuthUser (JWT) + 구매 소유 확인 + completed 상태

**Rate Limit**: 30페이지/분/user_id (Redis INCR)

**필수 요청 헤더**:
```
X-Ebook-Viewer: 1
X-Ebook-Session: {session_id}
X-Ebook-Signature: HMAC-SHA256("{session_id}:{code}/{page_num}:{timestamp}")
X-Ebook-Timestamp: {unix_seconds}
```

**응답 보안 헤더**:
```
Content-Type: image/webp
Cache-Control: private, no-store
X-Content-Type-Options: nosniff
```

**워터마크 (4중 다층 보안 — 1 가시 + 3 비가시)**:
1. 풋터 워터마크: `{pageNum} | {purchaseCode} | Amazing Korean` — 페이지 하단 풋터 영역, #999999 회색
2. 마이크로 도트: user_id 64비트를 4 모서리에 16비트씩 near-white(#FEFEFE) 도트 인코딩
3. LSB 스테가노그래피: purchase_code + watermark_id SHA-256 해시 → R 채널 LSB (비트별 고유 시드)
4. 접근 로그: `ebook_access_log` 테이블에 purchase_id, user_id, page_number, watermark_id, IP, UA 기록

**알려진 제약사항**:
- 풋터 텍스트 중앙 정렬은 글자당 ~9px 추정 기반 — 폰트에 따라 약간의 좌우 편차 가능 (기능 영향 없음)
- 커버 페이지(1~4)에도 풋터 워터마크 적용됨 — 커버 디자인에 따라 시각 QA 필요
- `Cache-Control: private, max-age=300` — 5분 내 동일 페이지 재방문 시 캐시 히트로 새 워터마크 미적용 (기존 watermark_id와 이미지 일치하므로 포렌식 추적 정상)
- 감사 로그는 `tokio::spawn` fire-and-forget — DB 일시 장애 시 로그 유실 가능 (이미지 반환 우선)
- HMAC 타임스탬프 ±30초 윈도우 — PC 시계가 30초+ 차이나면 요청 거부 (NTP 기본 활성화 PC는 영향 없음)
- HMAC 검증 시 Redis GET 2회 (verify_session + verify_hmac_signature) — 현 트래픽에서 무시 가능

#### 12.5-5.5 : `GET /ebook/viewer/{code}/pages/{page_num}/tiles/{row}/{col}` (타일 이미지 조회)

> 타일 분할 모드 시 개별 타일 이미지 반환. `EBOOK_TILE_MODE=true` 시 활성화.

**인증**: AuthUser (JWT) + 구매 소유 확인 + completed 상태 + 세션 검증

**Rate Limit**: 270타일/분/user_id (Redis INCR, 전용 키)

**경로 파라미터**: `row` (0-based), `col` (0-based)

**응답**: `image/webp` (워터마크 적용된 타일 이미지)

#### 12.5-6 : `GET /admin/ebook/purchases` (관리자 구매 목록)

> e-book 구매 내역 조회 (검색, 필터, 페이지네이션).

**인증**: Admin (IP Guard + Role Guard)

**쿼리 파라미터**: `page`, `per_page`, `status`, `search`

#### 12.5-7 : `GET /admin/ebook/purchases/{id}` (관리자 구매 상세)

> 구매 상세 정보 + 접근 로그 조회.

**인증**: Admin

#### 12.5-8 : `PATCH /admin/ebook/purchases/{id}/status` (상태 변경)

> 구매 상태 변경. 유효 전환만 허용 (pending→completed, pending→refunded, completed→refunded).

**인증**: Admin

**요청**: `{ "status": "completed" }`

#### 12.5-9 : `DELETE /admin/ebook/purchases/{id}` (구매 삭제)

> 구매 Soft Delete + 관리자 로그 기록.

**인증**: Admin

#### 12.5-10 : `GET /admin/ebook/verify/{watermark_id}` (워터마크 진위확인)

> 유출된 이미지에서 추출한 watermark_id로 구매자 정보를 조회한다. 포렌식 추적용.

**인증**: Admin (JWT + Admin role)

**경로 파라미터**: `watermark_id` (UUID)

---

> **페이지네이션 필드명 참고**: e-book 관리자 목록의 페이지네이션 필드명은 `page` (textbook의 `current_page`와 다름).

</details>

**Paddle 연동 (일회성 결제)**:
- `transaction.completed` 웹훅에서 `custom_data.type == "ebook"` + `custom_data.purchase_code` 확인
- `ebook_purchase.paddle_txn_id` 저장 + `status` → `completed` 업데이트
- **환불**: `adjustment.created`/`adjustment.updated` 이벤트에서 `AdjustmentAction::Refund` + `AdjustmentStatus::Approved` 확인 → `paddle_txn_id`로 조회 → `status` → `refunded`
- **가격 분기**: Paddle 결제 = $10 USD (1000 cents), 계좌이체 = ₩12,000~₩15,000 KRW (에디션별)
- **프론트엔드**: 카탈로그에서 Paddle checkout overlay 호출 (`customData: { type: "ebook", purchase_code }`)
- **Pending 재결제**: `/ebook/my`에서 pending+paddle 구매에 "결제하기" 버튼으로 checkout 재시도
- **주문 취소**: `DELETE /ebook/purchase/{code}`로 pending 구매 soft delete

**빌드 파이프라인 (페이지 이미지 생성)**:
```bash
# 기존 EPUB 빌드 → Puppeteer → 페이지별 WebP 이미지
node scripts/textbook/generate_page_images.js vi teacher   # 단일
node scripts/textbook/generate_page_images.js all all       # 전체
```
- 입력: `docs/textbook/books/{edition}-inner/AMK_{EDITION}_INNER_{LANG}.html`
- 출력: `docs/textbook/page-images/{edition}/{lang}/page-001.webp ~ page-NNN.webp`
- 매니페스트: `docs/textbook/page-images/{edition}/{lang}/manifest.json`

**프론트엔드 페이지**:
- `/ebook` — e-book 카탈로그 (언어/에디션 선택, 가격, 결제방식 선택(계좌이체/Paddle 카드), 샘플 미리보기, 환불정책 링크 — 로그인 필수)
- `/ebook/purchase-complete` — 구매 완료 안내 (구매코드, 요약, 입금안내/Paddle 완료 분기)
- `/ebook/viewer/{purchaseCode}` — 웹 뷰어 (blob:// URL, 키보드/버튼/터치 스와이프 네비, TOC, 줌, 풀스크린, 모바일 최적화)
- `/ebook/my` — 내 구매 목록 (상태 배지, 뷰어 열기 버튼)
- `/admin/ebook/purchases` — 관리자 구매 목록 (필터/검색/페이지네이션)
- `/admin/ebook/purchases/{id}` — 관리자 구매 상세 (상태 변경, 삭제)

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | DB 마이그레이션 (ENUM 3개 + 테이블 3개) | ✅ |
| 2 | 페이지 이미지 생성 스크립트 (generate_page_images.js) | ✅ |
| 3 | 백엔드 API (catalog, purchase, my, viewer meta/pages) | ✅ |
| 4 | 서버사이드 워터마크 (4중 비가시적: 풋터+마이크로도트+LSB+접근로그) | ✅ |
| 5 | 프론트엔드 뷰어 + 카탈로그 + 구매 목록 | ✅ |
| 6 | Paddle 결제 연동 (e-book 일회성 결제) | ✅ |
| 7 | 관리자 페이지 (구매 목록, 상세, 상태 변경, 삭제) | ✅ |

**파일 구조**:
```
# Backend
src/api/ebook/
├── mod.rs, dto.rs, repo.rs, service.rs, handler.rs, router.rs
└── watermark.rs    # 4중 워터마크 (풋터+마이크로도트+LSB+접근로그)

src/api/admin/ebook/
├── mod.rs, dto.rs, handler.rs, service.rs, router.rs

# Frontend
frontend/src/category/ebook/
├── types.ts, ebook_api.ts
├── hook/           # 5 hooks (catalog, purchase, my, viewer_meta, page_image)
└── page/           # 3 pages (catalog, viewer, my_purchases)

frontend/src/category/admin/ebook/page/
├── admin_ebook_purchases_page.tsx, admin_ebook_purchase_detail.tsx
```

**Cargo 의존성**: `image 0.25` (webp+png), `ab_glyph 0.2` (폰트), `imageproc 0.25` (텍스트 오버레이)

**환경변수** (`src/config.rs`):
| 변수 | 기본값 | 설명 |
|------|--------|------|
| `EBOOK_PAGE_IMAGES_DIR` | — | 페이지 이미지 디렉토리 경로 |
| `RATE_LIMIT_EBOOK_PAGE_MAX` | 30 | 페이지 요청 제한 (회/윈도우) |
| `RATE_LIMIT_EBOOK_PAGE_WINDOW_SEC` | 60 | 페이지 제한 윈도우 (초) |
| `RATE_LIMIT_EBOOK_PURCHASE_MAX` | 5 | 구매 요청 제한 (회/윈도우) |
| `RATE_LIMIT_EBOOK_PURCHASE_WINDOW_SEC` | 3600 | 구매 제한 윈도우 (초) |
| `EBOOK_SESSION_TTL_SEC` | 90 | 뷰어 세션 TTL (초, heartbeat 갱신) |
| `EBOOK_TILE_MODE` | false | 타일 분할 전송 활성화 |
| `EBOOK_TILE_GRID_ROWS` | 3 | 타일 그리드 행 수 |
| `EBOOK_TILE_GRID_COLS` | 3 | 타일 그리드 열 수 |
| `RATE_LIMIT_EBOOK_TILE_MAX` | 270 | 타일 요청 제한 (회/윈도우) |
| `RATE_LIMIT_EBOOK_TILE_WINDOW_SEC` | 60 | 타일 제한 윈도우 (초) |

**워터마크 폰트**:
- `OnceLock<Option<FontArc>>` — `watermark.rs`에서 선언, `main.rs`에서 `init_font()` 호출
- 경로: `{EBOOK_PAGE_IMAGES_DIR}/NotoSans-Regular.ttf` (555KB, notofonts)
- 폰트 미발견 시 LSB-only 모드로 graceful degradation

**페이지 이미지 스펙**:
- 크기: 1587×2245px (2x DPR, WebP quality 85)
- 풋터 영역: y≈2124px (바닥에서 ~121px 위), 높이 ~45px
- 총량: 22언어 × 2에디션 = 44세트, 커버 4장 포함 128p/세트, 총 5,632장 (~469MB)
- 모든 이미지 gitignored (`page-images/` 디렉토리)

**추가 보안 (구현 완료)**:
- Canvas 렌더링: `<img>` → `<canvas>` 전환, ArrayBuffer 캐시, blob URL 즉시 revoke
- 커스텀 헤더 체크: `X-Ebook-Viewer: 1` (URL 직접 접근 차단)
- Canvas 추출 API 무력화: toDataURL/toBlob/getImageData/captureStream/OffscreenCanvas/createImageBitmap 프로토타입 오버라이드
- 포커스/가시성 감지: 탭 전환/인쇄 시 blur(30px) 즉시 적용
- DOM 조작 감지: MutationObserver + getComputedStyle 주기 검사 (2초)
- 동시 세션 제한: Redis 기반 user별 단일 세션, 90초 TTL, 30초 heartbeat
- 타일 분할 전송: 3×3 그리드, 기능 플래그로 점진 롤아웃 (`EBOOK_TILE_MODE`)

**주요 구현 노트**:
- TOC 사이드바: state 기반 패널 (프로젝트 shadcn에 Sheet 없음)
- 이미지 로딩: Axios `responseType: 'blob'` → `URL.createObjectURL()`
- 프리페치: 현재 페이지 기준 ±3 페이지 `queryClient.prefetchQuery()`
- `TextbookLanguage` enum 재활용 (20개 언어) + Filipino(TL) 추가 필요
- 중복 구매 방지: pending/completed 모두 차단 (각각 다른 에러 메시지)
- `useMyPurchases` 훅: `enabled: isLoggedIn` 가드
- 관리자 페이지 전체 i18n 적용 (하드코딩 없음)
- TOC 한국어+영어 이중언어 표시 (`to_korean_title()` 매핑, `TocEntry.title_ko`)

**향후 개선 (미구현, 계획만)**:
- Service Worker 캐시, CDN (CloudFront + signed URL), 이미지 해상도 최적화 (2x→1.5x DPR)
- 뷰어 테마 전환 (흰색/세피아/다크), 핀치 줌, Fit 모드 전환, 자동 스프레드

**주의사항**:
- 기존 `EB-YYMMDD-NNNN` 형식 구매코드는 DB에 그대로 유지 (신규 구매부터 새 형식 적용)
- 마이크로 도트는 JPEG 재압축에 취약 — 스크린샷 시 일부 손실 가능 (LSB와 이중 보험)

[⬆️ AMK_API_MASTER.md로 돌아가기](./AMK_API_MASTER.md)
