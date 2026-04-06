# AMK 코드 점검 계획

> 2026-04-03 작성. 백엔드 38K줄(Rust) + 프론트엔드 40K줄(TS/TSX) 대상.
> 각 점검은 **독립 세션**에서 실행.

---

## 점검 1: 의존성 취약점 + 업데이트

**목적**: 알려진 보안 취약점, outdated 패키지 식별

**백엔드 (Rust)**:
```bash
# 1. cargo-audit 설치 (최초 1회)
cargo install cargo-audit

# 2. 취약점 스캔
cargo audit

# 3. outdated 확인
cargo install cargo-outdated
cargo outdated -R
```

**프론트엔드 (npm)**:
```bash
cd frontend

# 1. 취약점 스캔
npm audit

# 2. outdated 확인
npm outdated
```

**판단 기준**:
- Critical/High 취약점 → 즉시 수정
- Moderate → 영향 범위 확인 후 판단
- Low / outdated → 기록만, 급하지 않음

**예상 소요**: 30분~1시간

---

## 점검 2: 코드 품질 (정적 분석)

**목적**: 컴파일러/린터 경고, dead code, 미사용 import, 불일치 패턴

**백엔드 (Rust)**:
```bash
# clippy — Rust 린터 (deny warnings로 엄격 모드)
cargo clippy -- -W clippy::all 2>&1 | tee clippy_report.txt

# dead code 탐지
cargo clippy -- -W dead_code 2>&1
```

점검 항목:
- [ ] clippy 경고 0건 확인
- [ ] `#[allow(dead_code)]` 사용처 검토 — 정당한 이유 있는지
- [ ] `unwrap()` 사용처 전수 확인 — 프로덕션 코드에서 panic 가능성
- [ ] `todo!()`, `unimplemented!()` 잔존 여부
- [ ] 미사용 의존성 (`Cargo.toml`에 있지만 코드에서 안 쓰는 크레이트)

**프론트엔드 (TypeScript)**:
```bash
cd frontend

# TypeScript 엄격 모드 빌드
npx tsc --noEmit 2>&1 | tee tsc_report.txt

# ESLint (설정 있는 경우)
npx eslint src/ --ext .ts,.tsx 2>&1 | tee eslint_report.txt
```

점검 항목:
- [ ] TypeScript 에러 0건 확인
- [ ] `any` 타입 사용처 전수 확인
- [ ] `// @ts-ignore`, `// @ts-expect-error` 사용처 검토
- [ ] 미사용 import/export 정리
- [ ] `console.log` 잔존 여부 (개발용 로그)

**예상 소요**: 1~2시간

---

## 점검 3: 보안 리뷰

**목적**: OWASP Top 10 기반 취약점 점검

**백엔드 점검 항목**:
- [ ] **SQL Injection**: SQLx 바인딩 파라미터 사용 여부 (raw SQL 문자열 결합 금지)
- [ ] **인증/인가**: 모든 보호된 엔드포인트에 미들웨어 적용 확인
- [ ] **Anti-enumeration**: 인증 관련 응답에서 존재 여부 노출 없는지
- [ ] **Rate Limiting**: 이메일 발송, 로그인, MFA 시도에 rate limit 적용 확인
- [ ] **암호화**: PII 필드 전부 AES-256-GCM 암호화 확인 (repo → service 흐름 추적)
- [ ] **인증코드**: Redis에 HMAC 해시로 저장 확인 (평문 저장 금지)
- [ ] **CORS**: 허용 origin 목록 적절한지
- [ ] **환경변수**: 하드코딩된 시크릿 없는지
- [ ] **에러 메시지**: 내부 정보 노출 없는지 (스택 트레이스, DB 스키마 등)

**프론트엔드 점검 항목**:
- [ ] **XSS**: `dangerouslySetInnerHTML` 사용처 검토
- [ ] **토큰 노출**: localStorage에 access token 저장 여부 (httpOnly 쿠키 사용 확인)
- [ ] **HMAC 서명**: 요청별 서명 검증 로직 정상 작동 확인
- [ ] **DevTools 감지**: devtools_detect.ts 로직 우회 가능성

**예상 소요**: 2~3시간

---

## 점검 4: 문서 정합성

**목적**: docs/에 적힌 스펙과 실제 코드 불일치 식별

**점검 방법**:
1. `docs/AMK_API_AUTH.md` 엔드포인트 표 → 실제 `src/api/auth/router.rs` 라우트 대조
2. `docs/AMK_API_USER.md` → `src/api/user/router.rs` 대조
3. `docs/AMK_API_LEARNING.md` → 각 도메인 router 대조
4. `docs/AMK_API_PAYMENT.md` → `src/api/payment/router.rs` 대조
5. `docs/AMK_API_TEXTBOOK.md` → `src/api/textbook/router.rs` 대조
6. `docs/AMK_API_EBOOK.md` → `src/api/ebook/router.rs` 대조
7. `docs/AMK_SCHEMA_PATCHED.md` → 실제 마이그레이션 파일 대조
8. `docs/AMK_CODE_PATTERNS.md` → 실제 코드 패턴 대조

**점검 항목 (각 문서별)**:
- [ ] 문서에 있지만 코드에 없는 엔드포인트
- [ ] 코드에 있지만 문서에 없는 엔드포인트
- [ ] 요청/응답 DTO 필드 불일치
- [ ] 상태 코드 불일치
- [ ] 환경변수 목록 불일치 (`config.rs` vs `docs/AMK_DEPLOY_OPS.md`)

**예상 소요**: 2~3시간

---

## 실행 순서 (추천)

| 순서 | 점검 | 이유 |
|------|------|------|
| 1 | 의존성 취약점 | 가장 빠르고, 프로덕션 즉시 영향 |
| 2 | 코드 품질 | clippy/tsc로 자동화 가능, 숨은 버그 발견 |
| 3 | 보안 리뷰 | 가장 중요하지만 시간 소요 큼 |
| 4 | 문서 정합성 | 모바일/데스크탑 개발 시 문서를 참조하므로, 정확해야 함 |

각 점검은 **독립 세션에서 실행**하고, 결과는 `docs/AMK_CODE_AUDIT_RESULT.md`에 통합 기록.
