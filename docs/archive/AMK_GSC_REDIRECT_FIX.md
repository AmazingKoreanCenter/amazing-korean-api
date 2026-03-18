# GSC 리디렉션 포함 페이지 문제 해결 계획

> **작성일**: 2026-03-16
> **상태**: 계획 수립 (미구현)
> **관련 도구**: Google Search Console, Cloudflare Pages

---

## 1. 원인

### 1-1. 현상

Google Search Console → 페이지 색인 생성 → "리디렉션이 포함된 페이지" 항목에 3건 접수:

| URL | 최종 크롤링 | 리디렉션 유형 |
|-----|-----------|-------------|
| `http://amazingkorean.net/` | 2026.3.7 | HTTP → HTTPS (Cloudflare) |
| `https://amazingkorean.net/register` | 2026.3.4 | `/register` → `/signup` |
| `https://amazingkorean.net/intro` | 2026.3.4 | `/intro` → `/about` |

### 1-2. 근본 원인: SPA 소프트 리디렉션

Cloudflare Pages의 SPA 모드는 **모든 경로에 `index.html`을 200 OK로 반환**한다. 이후 React Router가 클라이언트에서 `<Navigate to="..." replace />`를 실행하여 리디렉션한다.

**문제**: Google은 이를 "소프트 리디렉션"으로 분류한다. HTTP 301(영구 이동)과 달리 확정적이지 않아:
- 원본 URL 크롤링을 중단하지 않고 **주기적으로 재방문**
- "리디렉션이 포함된 페이지"로 **영구 분류**
- 색인 대상에서 제외 (대상 URL은 별도로 색인될 수 있으나, 원본 URL이 GSC에 지속 노출)

### 1-3. 해당 경로 (routes.tsx)

```tsx
// Public — Google 크롤링 대상 (문제 발생)
<Route path="/intro" element={<Navigate to="/about" replace />} />
<Route path="/register" element={<Navigate to="/signup" replace />} />

// Admin — robots.txt Disallow: /admin/ 으로 차단됨 (문제 없음)
<Route path="payment" element={<Navigate to="subscriptions" replace />} />
<Route path="textbook" element={<Navigate to="textbook/orders" replace />} />
<Route path="ebook" element={<Navigate to="ebook/purchases" replace />} />
```

### 1-4. HTTP→HTTPS (`http://amazingkorean.net/`)

Cloudflare의 "Always Use HTTPS" 설정에 의한 301 리디렉션. 이는 **정상 동작**이며 모든 HTTPS 사이트에 존재한다. GSC에서 "실패함 0"으로 표시되며 별도 조치 불필요.

---

## 2. 경과

| 날짜 | 내용 | 결과 |
|------|------|------|
| 2026-02-26 | **1차 SEO 수정** — `index.html`에 하드코딩된 `<link rel="canonical" href="/">`를 제거하고, `PageMeta` 컴포넌트로 페이지별 동적 canonical 태그 관리 | canonical 중복 문제 해결됨. 그러나 리디렉션 문제는 **별개 레이어**이므로 미해결 |
| 2026-03-04 | GSC에서 `/register`, `/intro` 크롤링 확인 | 소프트 리디렉션으로 분류, 색인 제외 |
| 2026-03-07 | GSC에서 `http://amazingkorean.net/` 크롤링 확인 | HTTP→HTTPS 301 정상 리디렉션 |
| 2026-03-16 | GSC "리디렉션이 포함된 페이지" 유효성 검사 재시작 (3건 접수, 0건 실패) | 문제 지속 확인 → 본 계획 수립 |

**반복 발생 이유**: 1차 수정(PageMeta)은 canonical 태그 레이어의 문제를 해결한 것이며, 리디렉션은 **HTTP 응답 코드 레이어**의 문제로 별도 해결이 필요했다.

---

## 3. 해결 방안

### 3-1. Cloudflare Pages `_redirects` 파일

**공식 문서**: https://developers.cloudflare.com/pages/configuration/redirects/

Cloudflare Pages는 `_redirects` 파일을 **엣지(CDN) 레벨에서 처리**한다. 요청이 SPA의 `index.html`에 도달하기 전에 HTTP 301 응답을 반환하므로, Google이 정식 영구 리디렉션으로 인식한다.

**처리 순서**:
```
클라이언트 요청 → Cloudflare 엣지
  → 1. _redirects 규칙 매칭 → 301 응답 반환 (여기서 끝)
  → 2. 매칭 안 되면 → 정적 파일 서빙 / SPA fallback (index.html)
```

### 3-2. 구현 내용

#### (A) `frontend/public/_redirects` 파일 생성

```
# 레거시 경로 → 정규 경로 (301 영구 리디렉션)
/register /signup 301
/intro /about 301
```

- 빌드 시 `dist/_redirects`로 자동 복사됨 (Vite의 `public/` 디렉토리 정책)
- 상태 코드 생략 시 기본값 302이므로, **반드시 301 명시**
- 제한: 정적 2,000개 + 동적 100개 = 최대 2,100개 (현재 2개, 충분)

#### (B) `routes.tsx`의 `<Navigate>` 유지 (변경 없음)

`_redirects`는 **외부 진입**(Google 크롤러, 브라우저 직접 접속, 외부 링크)을 처리하고, `<Navigate>`는 **SPA 내부 네비게이션**(앱 내에서 프로그래밍 방식으로 이동하는 경우)의 폴백으로 유지한다. 양쪽 모두 있어야 완전한 커버리지.

#### (C) `robots.txt` 변경 없음

301 리디렉션된 URL은 Google이 대상 URL로 전달 처리하므로, Disallow를 추가할 필요 없다. 오히려 Disallow하면 301의 link equity(링크 가치) 전달이 차단될 수 있다.

### 3-3. 향후 리디렉션 추가 시 규칙

새로운 Public 경로 리디렉션 추가 시 **반드시 두 곳에 반영**:

1. `frontend/public/_redirects` — 서버 사이드 301 (Google/외부 진입)
2. `frontend/src/app/routes.tsx` — `<Navigate>` (SPA 내부 폴백)

---

## 4. 검증

### 4-1. 로컬 검증 (구현 직후)

```bash
# 프론트엔드 빌드 확인
cd frontend && npm run build

# _redirects 파일이 dist/에 복사되었는지 확인
cat dist/_redirects
```

### 4-2. 배포 후 검증

```bash
# HTTP 응답 코드 확인 (301이어야 함)
curl -I https://amazingkorean.net/register
# 예상: HTTP/2 301, Location: /signup

curl -I https://amazingkorean.net/intro
# 예상: HTTP/2 301, Location: /about
```

### 4-3. GSC 검증 (배포 후 1~2주)

1. Google Search Console → URL 검사 → `https://amazingkorean.net/register` 입력
2. "리디렉션 페이지" 상태 확인 — 301 영구 리디렉션으로 표시되어야 함
3. "페이지 색인 생성" → "리디렉션이 포함된 페이지" 항목에서 점차 제거됨 (2~4주 소요)
4. 유효성 검사 재시작 → "통과" 상태 전환 확인

---

## 5. 예상 결과

| 항목 | Before (현재) | After (적용 후) |
|------|-------------|-----------------|
| `/register` 응답 | 200 OK + JS 소프트 리디렉션 | **301 Moved Permanently** |
| `/intro` 응답 | 200 OK + JS 소프트 리디렉션 | **301 Moved Permanently** |
| GSC "리디렉션 포함 페이지" | 3건 (지속 재발) | HTTP→HTTPS 1건만 잔존 (정상) |
| Google 재크롤링 빈도 | 주기적 반복 | 301 확인 후 점차 중단 |
| 링크 가치(Link Equity) | 소프트 리디렉션은 전달 불확실 | 301은 대상 URL로 **완전 전달** |

---

## 6. 기타 사항

### 6-1. HTTP→HTTPS 리디렉션 (1건)

`http://amazingkorean.net/` → `https://amazingkorean.net/`은 Cloudflare의 "Always Use HTTPS" 기능에 의한 **정상 301 리디렉션**이다. 이는 모든 HTTPS 사이트에 존재하며, GSC에서 "실패함 0"이므로 조치 불필요. 이 1건은 영구적으로 남을 수 있으나 SEO에 부정적 영향 없음.

### 6-2. 기존 PageMeta와의 관계

| 레이어 | 해결 대상 | 해결 도구 |
|--------|----------|----------|
| HTTP 응답 코드 | 리디렉션이 포함된 페이지 | **`_redirects` (본 계획)** |
| HTML 메타 태그 | 중복 페이지 / canonical | PageMeta 컴포넌트 (2026-02-26 해결됨) |
| 크롤링 접근 제어 | 불필요한 페이지 크롤링 차단 | robots.txt (설정 완료) |
| 색인 대상 목록 | 색인 대상 페이지 명시 | sitemap.xml (설정 완료) |

4개 레이어가 모두 정상 작동해야 GSC 이슈가 발생하지 않는다. 이번 작업으로 HTTP 응답 코드 레이어가 해결되면 **SEO 방어 체계 완성**.

### 6-3. `_redirects` vs `_headers` vs Cloudflare Bulk Redirects

| 방법 | 용도 | 선택 이유 |
|------|------|----------|
| `_redirects` 파일 | 소규모 경로 리디렉션 (≤2,100개) | **채택** — 코드 기반 관리, Git 추적, 배포 자동화 |
| Cloudflare Bulk Redirects | 대규모 리디렉션 (수만 개) | 불필요 — 현재 2개 |
| Cloudflare Page Rules | 도메인/패턴 기반 리디렉션 | 불필요 — 경로별 리디렉션에는 과도 |

### 6-4. 작업 범위

- **변경 파일**: `frontend/public/_redirects` (신규 1개)
- **변경 없는 파일**: `routes.tsx`, `robots.txt`, `sitemap.xml`, `index.html`, `page_meta.tsx`
- **예상 작업 시간**: 파일 생성 + 빌드 확인 + 배포
- **리스크**: 없음 (기존 동작에 영향 없음, 추가 레이어만 적용)
