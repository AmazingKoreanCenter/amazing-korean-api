# AMK 문서화 개선 분석 보고서

> 작성일: 2026-03-15
> 목적: 프로젝트 문서화 현황 진단 + 외부 프로젝트 벤치마킹 → 개선 방향 수립

---

## 1. 현재 프로젝트 문서화 현황 분석

### 1.1 현재 구조

```
CLAUDE.md (118줄)
├── 프로젝트 개요, 기술 스택
├── 필수 참조 문서 테이블 (4개만 안내)
├── 디렉토리 구조
├── 백엔드/프론트엔드 모듈 패턴
├── 핵심 규칙 (코드 변경, 보안, 프로덕션 안전장치)
├── 검증 체크리스트 (cargo check, npm run build)
└── 커밋 컨벤션

docs/AMK_*.md (14개, 총 ~780KB)
├── AMK_API_MASTER.md (269KB) — API 스펙 SSoT
├── AMK_CODE_PATTERNS.md (131KB) — 코드 패턴·컨벤션
├── AMK_SCHEMA_PATCHED.md (76KB) — DB DDL
├── AMK_PIPELINE.md (89KB) — AI 파이프라인
├── AMK_CHANGELOG.md (51KB) — 변경 이력
├── AMK_DEPLOY_OPS.md (30KB) — 배포·운영
├── AMK_DESIGN_SYSTEM.md (34KB) — 디자인 토큰
├── AMK_MACMINI_SETUP.md (24KB) — Mac Mini 설정
├── AMK_MARKET_ANALYSIS.md (22KB) — 시장 분석
├── AMK_FILE_TREE.md (24KB) — 파일 구조 스냅샷
├── AMK_PRINT_GUIDE.md (19KB) — 인쇄 납품 가이드
├── AMK_MEMORY_AUDIT.md (7KB) — 메모리 감사
├── AMK_BOOK_LANDING.md (6KB) — QR 랜딩 페이지
└── AMK_FOOTER_ISSUE.md (6KB) — CSS 버그 수정

~/.claude/projects/.../memory/ (31개, 총 ~220KB)
├── MEMORY.md (인덱스, 298줄 — 200줄 제한 초과)
├── 토픽 파일 30개 (출판, e-book, 리서치, 디버깅 등)
└── docs/textbook/클로드 메모리/ (백업 스냅샷, 2026-03-12)
```

### 1.2 장점

#### 1.2.1 SSoT(Single Source of Truth) 원칙 확립
- `AMK_API_MASTER.md`가 API 스펙의 유일한 진실 소스로 운영되고 있음
- `AMK_SCHEMA_PATCHED.md`가 DB 스키마의 SSoT 역할을 수행
- CLAUDE.md에 "코드 변경 시 AMK_API_MASTER.md도 반드시 함께 업데이트"라는 동기화 규칙이 명시됨
- 이로 인해 코드와 문서 간 괴리가 최소화됨

#### 1.2.2 도메인별 문서 분리
- API, DB, 배포, 코드 패턴, 디자인 등 도메인별로 독립 문서가 존재
- 각 문서가 자체 완결적이므로 특정 도메인 작업 시 해당 문서만 참조하면 됨
- 문서 간 순환 참조 없이 단방향 참조 구조를 유지

#### 1.2.3 풍부한 컨텍스트 기록
- MEMORY.md에 의사결정 배경, 시행착오, 보류 사항이 상세히 기록됨
- 예: "AWS SES 폐기됨 — 프로덕션 승인 3회 거절" → 왜 Resend를 쓰는지 맥락 보존
- 예: "Apple OAuth — 비용 문제로 보류" → 미래 대화에서 불필요한 재논의 방지
- 토픽 파일(30개)이 출판·교재·리서치 등 코드에 없는 도메인 지식을 보존

#### 1.2.4 실용적인 검증 체크리스트
- CLAUDE.md에 `cargo check` + `npm run build` 명시
- 커밋 컨벤션(`Phase V1-2 : <요약>`)이 일관된 이력 관리 가능하게 함
- 프로덕션 안전장치(EMAIL_PROVIDER=none → panic) 같은 운영 규칙이 문서화됨

### 1.3 단점

#### 1.3.1 세 종류 파일의 역할 경계 부재
- **CLAUDE.md, MEMORY.md, AMK 문서** 간 "무엇을 어디에 쓸 것인가"에 대한 원칙이 없음
- MEMORY.md가 문서 역할을 겸함:
  - 법인 행정 작업 30줄 (은행 한도, 세금계산서, 건강보험 — 코드와 무관)
  - 프로젝트 구조 참고 (CLAUDE.md와 동일 내용)
  - 결제 시스템 상세 (AMK_API_MASTER에도 있는 내용)
  - 교재 빌드 명령어·디렉토리 구조 (운영 가이드 성격)
- CLAUDE.md가 불완전:
  - ebook/, textbook/, payment/ 도메인이 디렉토리 구조에서 누락
  - AMK 문서 14개 중 4개만 필수 참조로 안내
  - 메모리 시스템의 존재 자체를 언급하지 않음

#### 1.3.2 MEMORY.md 인덱스 용량 초과
- 현재 298줄로 200줄 제한을 98줄 초과
- 200줄 이후 내용(참고 파일 목록, 피드백 섹션 등)이 대화에 로드되지 않음
- 이는 Claude가 관련 토픽 파일의 존재 자체를 모를 수 있다는 것을 의미
- 인덱스에 완료 작업 이력(20줄+), 법인 행정(30줄+), Mac Mini 상세(12줄+)가 직접 기재되어 공간 낭비

#### 1.3.3 중복 콘텐츠 산재
- `content_protection.md` — AMK_API_MASTER §8.6에 정식 문서화 완료됨에도 메모리에 잔존
- `ebook_web_viewer.md` — `ebook_viewer.md`의 불완전 요약본 (동일 Phase 12.5 내용)
- MEMORY.md "프로젝트 구조 참고" 섹션 ↔ CLAUDE.md 디렉토리 구조
- `ebook_market_research.md` ↔ `AMK_MARKET_ANALYSIS.md` 시장 분석 부분 중복
- 중복이 존재하면 어느 것이 최신인지 판단할 수 없어 잘못된 정보 참조 위험 발생

#### 1.3.4 데이터 정합성 오류
- `i18n_multilingual_plan.md`: "ar"(아랍어)를 22번째 언어로 기재 — 실제 TextbookLanguage enum에 ar 없음. RTL 지원도 계획에 없으므로 잘못된 기대를 유발할 수 있음
- `textbook_tasks.md`: ISBN 매핑이 `isbn_imprint.md`와 충돌 (VN teacher ISBN이 다름). 구버전 데이터가 최신으로 오인될 위험
- 이러한 오류는 메모리 파일에 "최종 갱신일"이나 "폐기 표시"가 없기 때문에 발생

#### 1.3.5 구식 문서 방치
- `AMK_FILE_TREE.md` — 2026-02-18 생성 이후 미갱신. ebook/, textbook/ 도메인 전체가 누락된 상태
- `AMK_FOOTER_ISSUE.md` — 해결 완료된 CSS 버그 기록. 별도 문서로 존재할 가치 없음
- `AMK_MEMORY_AUDIT.md` — 일회성 감사 결과. 메모리 파일에서 참조되지도 않음
- 완료된 계획 파일(login_table_plan.md, db_security_audit.md, db_encryption_phase2_plan.md)이 활성 메모리에 잔존

#### 1.3.6 AMK 문서 크기 비대
- AMK_API_MASTER.md가 269KB(약 6,000줄 추정)로 단일 파일 한계에 근접
- AMK_CODE_PATTERNS.md 131KB, AMK_PIPELINE.md 89KB — Claude가 전체를 읽기에 부담스러운 크기
- 특정 섹션만 필요한데 전체를 읽어야 하는 비효율 발생

### 1.4 개선점

#### 1.4.1 역할 경계 정의 및 문서화
- CLAUDE.md, MEMORY.md, AMK 문서 각각의 역할을 명문화하고 CLAUDE.md에 기재
- 원칙: **CLAUDE.md = 지도** (어디에 무엇이 있는지), **MEMORY = 맥락** (코드에서 읽을 수 없는 것), **AMK = 사실** (구현의 진실)
- "이 정보는 어디에 써야 하는가?" 판단 기준표 작성

#### 1.4.2 MEMORY.md 200줄 이내 축소
- 완료 작업 이력 → 별도 토픽 파일(`completed_milestones.md`)로 이동
- 법인 행정 → 별도 토픽 파일(`corporate_admin.md`)로 이동
- 프로젝트 구조 → 삭제 (CLAUDE.md가 담당)
- Mac Mini 상세 → AMK_MACMINI_SETUP.md 참조 링크만 유지
- 인덱스에는 토픽 파일 링크 + 한줄 설명만 기재

#### 1.4.3 중복 제거
- `content_protection.md` 삭제 (AMK_API_MASTER에 문서화 완료)
- `ebook_web_viewer.md` 삭제 (ebook_viewer.md가 상위 호환)
- MEMORY.md "프로젝트 구조 참고" 섹션 삭제 (CLAUDE.md 중복)
- 데이터 오류 수정 (i18n ar 제거, textbook ISBN 갱신)

#### 1.4.4 CLAUDE.md 현행화
- 디렉토리 구조에 ebook/, textbook/, payment/ 추가
- 필수 참조 문서 테이블에 나머지 AMK 문서들 추가 (역할·참조 시점 포함)
- 메모리 시스템 안내 섹션 추가

#### 1.4.5 구식 문서 정리
- AMK_FILE_TREE.md → 현재 코드 기준 재생성 또는 삭제 후 CLAUDE.md로 통합
- AMK_FOOTER_ISSUE.md → AMK_CHANGELOG.md에 병합 후 삭제
- AMK_MEMORY_AUDIT.md → 일회성이므로 삭제 또는 아카이브

### 1.5 리스크

#### 1.5.1 정리 과정에서의 정보 유실
- 메모리 파일 삭제·병합 시 토픽 파일에만 존재하던 고유 컨텍스트가 사라질 수 있음
- 특히 "왜 이렇게 결정했는지"에 대한 배경 정보는 한번 사라지면 복구 불가
- **완화 방안**: 삭제 전 `docs/textbook/클로드 메모리/`에 최신 백업 갱신, 삭제 대상 파일의 고유 내용을 다른 파일에 병합 확인 후 삭제

#### 1.5.2 MEMORY.md 축소 시 컨텍스트 단절
- 200줄 이내로 축소하면 이전에 인덱스에서 직접 읽히던 정보를 토픽 파일로 이동해야 함
- 토픽 파일은 Claude가 필요할 때 명시적으로 읽어야 하므로, "읽지 않아서 모르는" 상황 발생 가능
- **완화 방안**: 인덱스의 한줄 설명을 충분히 구체적으로 작성하여 관련성 판단이 가능하도록 함

#### 1.5.3 단일 개발자 의존도
- 현재 문서 체계가 단일 개발자(+ Claude) 맥락에 최적화되어 있음
- 협업자 추가 시 MEMORY.md의 의사결정 맥락이 공유되지 않는 구조적 한계
- **완화 방안**: 핵심 의사결정은 AMK 문서에도 기록 (Key Decisions Log 패턴 도입)

#### 1.5.4 AMK_API_MASTER.md 단일 파일 한계
- 269KB 단일 파일은 Claude 컨텍스트 윈도우의 상당 부분을 차지
- 향후 도메인 추가 시 더 커질 수밖에 없음
- **완화 방안**: 도메인별 분할 검토 (당장은 불필요하나 모니터링 필요)

---

## 2. everything-claude-code 프로젝트 분석

> 참조: https://github.com/affaan-m/everything-claude-code

### 2.1 프로젝트 특징

Claude Code 사용을 위한 **운영 매뉴얼형 메타 프로젝트**. 5개 레이어로 관심사를 분리하며, AI가 읽는 문서와 사람이 읽는 문서를 철저히 구분한다.

```
Layer 1 — AI 진입점 (자동 로드)
├── CLAUDE.md         → 최소한의 프로젝트 구조 + 기본 컨벤션
└── AGENTS.md         → 에이전트 운영 매뉴얼 (18개 서브에이전트 정의)

Layer 2 — Rules (항상 따를 규칙, rules/)
├── common/           → 8개 공통 규칙 (security, coding-style, testing...)
└── {language}/       → 언어별 확장 (TypeScript, Python, Go...)

Layer 3 — Contexts (전환 가능한 행동 모드, contexts/)
├── dev.md            → "코드 먼저, 설명은 나중에"
├── research.md       → 리서치 우선
└── review.md         → 코드 리뷰 모드

Layer 4 — Skills & Commands
├── skills/           → 94개 자체 완결 워크플로우
└── commands/         → 40개 슬래시 커맨드

Layer 5 — 사람용 문서 (AI가 읽지 않음)
├── README.md, the-*-guide.md
└── docs/             → 아키텍처, 릴리스 노트
```

### 2.2 장점

#### 2.2.1 CLAUDE.md 최소화 원칙
- CLAUDE.md는 매 대화마다 자동 로드되므로 최소한의 내용만 포함
- 프로젝트 구조, 기본 명령어, 패키지 매니저 감지 규칙 정도만 기재
- 나머지(에이전트 정의, 보안 정책, 코딩 표준 등)는 AGENTS.md와 rules/로 분리
- 이로 인해 컨텍스트 윈도우를 효율적으로 사용

#### 2.2.2 레이어별 관심사 분리
- 규칙(rules/), 행동 모드(contexts/), 스킬(skills/), 사람용 문서(docs/)가 물리적으로 분리
- 각 레이어는 독립적으로 추가·수정·삭제 가능
- 새 언어 지원 시 `rules/{language}/`만 추가하면 됨 — 공통 규칙을 중복하지 않음

#### 2.2.3 AI용 / 사람용 문서 완전 분리
- AI가 읽는 파일: CLAUDE.md, AGENTS.md, rules/, contexts/, skills/
- 사람이 읽는 파일: README.md, the-*-guide.md, docs/
- 두 대상이 같은 정보를 다르게 필요로 한다는 점을 인정하고 별도 관리
- 결과적으로 AI 문서는 구조화·명령형, 사람 문서는 설명형·튜토리얼 형태

#### 2.2.4 규칙의 상속 구조
- `rules/common/` → `rules/typescript/` 같은 상속 관계
- 공통 보안 규칙은 한 번만 정의하고 언어별로 확장만 추가
- DRY 원칙이 문서에도 적용됨

#### 2.2.5 행동 모드(Contexts) 개념
- dev, research, review 같은 모드별로 Claude의 행동 양식을 전환
- 같은 코드베이스에서도 "코드 작성", "리서치", "리뷰" 시 다른 접근법 적용
- 규칙을 반복하지 않고 행동 양식만 바꾸는 경량 구조

#### 2.2.6 실행 가능한 규칙 (Hooks)
- "prettier 항상 실행"을 문서에 쓰는 대신 PostToolUse 훅으로 자동화
- 규칙 준수가 AI의 기억력에 의존하지 않고 시스템적으로 보장됨

### 2.3 단점

#### 2.3.1 메타 프로젝트 편향
- 이 프로젝트 자체가 "Claude Code를 잘 쓰는 방법"을 정리한 메타 프로젝트
- 실제 프로덕션 앱(DB, 결제, 인증 등)의 문서화 패턴과는 거리가 있음
- 94개 스킬, 18개 에이전트 정의는 대부분의 프로젝트에 과도한 규모

#### 2.3.2 AGENTS.md 집중으로 인한 단일 파일 비대
- CLAUDE.md를 최소화한 대신 AGENTS.md에 에이전트 정의, 보안 정책, 코딩 표준, TDD 워크플로우가 모두 들어감
- 결국 "한 파일이 너무 큼" 문제가 CLAUDE.md에서 AGENTS.md로 이동한 것
- AGENTS.md도 자동 로드되므로 컨텍스트 절약 효과가 제한적

#### 2.3.3 메모리 관리 부재
- 대화 간 기억 유지(persistent memory)에 대한 체계가 없음
- 세션 내 컨텍스트는 다루지만, "지난 대화에서 결정한 사항" 같은 장기 기억은 미지원
- 프로덕션 프로젝트에서는 의사결정 이력, 보류 사항, 진행 상태 등의 기억이 필수

#### 2.3.4 도메인 지식 저장소 부재
- API 스펙, DB 스키마, 배포 설정 같은 프로젝트 고유 지식의 저장·참조 패턴이 없음
- rules/는 범용 코딩 규칙이지 프로젝트 특화 스펙이 아님
- 실제 프로덕션 프로젝트에 적용 시 AMK_*.md 같은 도메인 문서 레이어를 별도 설계해야 함

#### 2.3.5 단일 개발자 / 소규모 팀에 과도한 복잡도
- 5개 레이어 × 다수 파일 구조는 대규모 팀이나 오픈소스 프로젝트에 적합
- 단일 개발자 프로젝트에서 94개 스킬 디렉토리, 8개 공통 규칙 파일은 관리 오버헤드
- 규칙이 너무 분산되면 "어디에 뭐가 있는지" 파악 자체가 부담

### 2.4 개선점

#### 2.4.1 레이어 단순화
- 5개 레이어를 프로젝트 규모에 맞게 3개로 축소 가능:
  - Layer 1: AI 진입점 (CLAUDE.md)
  - Layer 2: 프로젝트 문서 (docs/)
  - Layer 3: AI 기억 (memory/)
- rules/, contexts/의 내용은 CLAUDE.md에 인라인 가능 (규칙이 소수일 때)

#### 2.4.2 메모리 레이어 추가
- MEMORY.md + 토픽 파일 구조를 도입하여 대화 간 연속성 확보
- Key Decisions Log를 도입하여 의사결정 이력 구조화

#### 2.4.3 도메인 문서 레이어 정의
- rules/ (범용 규칙)과 별도로 specs/ 또는 docs/ (프로젝트 스펙) 레이어 필요
- 우리 프로젝트의 AMK_*.md가 이 역할을 이미 수행하고 있음

### 2.5 리스크

#### 2.5.1 과도한 구조화의 역효과
- 파일 수가 많아지면 "어디를 읽어야 하는지" 결정 자체가 비용
- Claude가 관련 파일을 찾아 읽는 시간이 실제 작업보다 길어질 수 있음
- 특히 rules/ 8개 + contexts/ 3개 + AGENTS.md를 모두 참조해야 하는 상황은 비효율

#### 2.5.2 유지보수 비용 증가
- 레이어 간 일관성을 유지하려면 하나의 규칙 변경 시 여러 파일 동시 수정 필요
- 예: 보안 규칙 변경 → rules/common/security.md + AGENTS.md + 관련 skills/ 갱신
- 단일 개발자에게는 과도한 관리 부담

#### 2.5.3 컨텍스트 윈도우 비효율
- CLAUDE.md + AGENTS.md가 모두 자동 로드되면 결국 두 파일 합산 크기가 컨텍스트 차지
- 최소화의 이점이 AGENTS.md 비대로 상쇄될 수 있음

### 2.6 우리 프로젝트 도입 방안

#### 도입할 것

| 패턴 | 적용 방법 | 기대 효과 |
|------|-----------|-----------|
| **CLAUDE.md 최소화** | 현재 118줄 → 진입점+인덱스 역할로 축소. 규칙은 유지하되 상세 설명은 AMK 문서 링크로 대체 | 매 대화 컨텍스트 절약 |
| **AI용 / 사실용 분리 원칙** | CLAUDE.md(AI 지침) ↔ AMK(사실 SSoT) ↔ MEMORY(AI 기억) 경계 명문화 | 중복 제거, 역할 혼란 방지 |
| **규칙 상속 개념** | CLAUDE.md에 핵심 규칙만, AMK_CODE_PATTERNS.md에 상세 패턴 → CLAUDE.md가 참조 | DRY 원칙 적용 |
| **훅 자동화** | `cargo check` / `npm run build`를 PostToolUse 훅으로 전환 검토 | 검증 누락 방지 |

#### 도입하지 않을 것

| 패턴 | 미도입 사유 |
|------|-------------|
| **AGENTS.md 분리** | 서브에이전트 18개를 정의할 규모가 아님. CLAUDE.md에 인라인 유지 |
| **rules/ 디렉토리** | 규칙 수가 5~6개로 소규모. 별도 디렉토리 불필요 |
| **contexts/ 모드 전환** | 단일 개발자 + 단일 프로젝트. 모드 전환보다 일관된 작업 흐름이 중요 |
| **skills/ 94개** | 자체 스킬 정의는 프로젝트 규모에 비해 과도. 필요 시 CLAUDE.md에 직접 기재 |

---

## 3. awesome-claude-code-toolkit 프로젝트 분석

> 참조: https://github.com/rohitg00/awesome-claude-code-toolkit

### 3.1 프로젝트 특징

Claude Code를 위한 **조합형 툴킷**. 300+ 파일을 10개 디렉토리로 분류하며, 필요한 조각만 골라 쓰도록 설계되었다. 특히 7단계 CLAUDE.md 템플릿과 19개 라이프사이클 훅이 핵심이다.

```
templates/claude-md/    → 7개 CLAUDE.md 템플릿 (minimal → enterprise)
agents/                 → 135개 에이전트 (10개 카테고리)
commands/               → 42개 슬래시 커맨드
contexts/               → 5개 행동 모드 (dev, debug, review, deploy, research)
rules/                  → 15개 코딩 규칙
skills/                 → 35개 도메인 스킬
plugins/                → 121개 독립 플러그인
hooks/                  → 19개 라이프사이클 훅
mcp-configs/            → 6개 MCP 서버 프리셋
examples/               → 3개 워크플로우 예제
```

### 3.2 장점

#### 3.2.1 CLAUDE.md 템플릿의 단계적 확장
- minimal(소규모) → standard → fullstack → comprehensive → enterprise(대규모)로 프로젝트 규모에 맞게 선택
- 각 템플릿이 공통 구조(스택, 명령어, 구조, 컨벤션)를 공유하면서 점진적으로 항목 추가
- **comprehensive 템플릿**의 구조가 특히 참고 가치 높음:
  1. 프로젝트명 + 한줄 설명
  2. 스택
  3. 명령어
  4. 디렉토리 구조
  5. 아키텍처 패턴
  6. 컨벤션
  7. 환경변수
  8. **서브에이전트 지침** — 위임 작업별 가이드
  9. **Memory Bank** — 세션 간 상태 관리 명시
  10. **Key Decisions Log** — 날짜/결정/근거 테이블

#### 3.2.2 체계적인 메모리 관리
- **Memory Bank 커맨드**: CLAUDE.md에서 세션 인사이트를 추출·병합하는 전용 워크플로우
- **200줄 하드 리밋** 명시 — 오래된 노트는 아카이브
- **프로젝트 메모리 vs 개인 메모리 분리**: 프로젝트별 `./CLAUDE.md` vs 범용 `~/.claude/CLAUDE.md`
- 세션 라이프사이클 훅으로 자동 저장/복원:
  - SessionStart → 이전 컨텍스트 로드
  - SessionEnd → 현재 상태 저장 + 학습 로그
  - PreCompact → 컴팩션 전 보존

#### 3.2.3 강력한 훅 기반 가드레일
- **PreToolUse 훅 6개** — 실행 전 차단:
  - `secret-scanner.js`: AWS 키, GitHub 토큰, 프라이빗 키, API 키 등 패턴 매칭 → 블록
  - `commit-guard.js`: 커밋 메시지 컨벤셔널 포맷 강제
  - `block-md-creation.js`: docs/ 외부에서 불필요한 .md 생성 차단
  - `block-dev-server.js`: 개발 서버 실행 차단
  - `pre-push-check.js`: 푸시 전 검증
- **PostToolUse 훅 10개** — 실행 후 검증:
  - `lint-fix.js`, `type-check.js`: Write/Edit 후 자동 린트·타입체크
  - `auto-test.js`: 코드 변경 후 관련 테스트 자동 실행
  - `bundle-check.js`: 번들 크기 모니터링
  - `suggest-compact.js`: 편집 횟수 추적, 컴팩션 시점 제안
- 규칙 준수가 Claude의 기억력이 아닌 **시스템**에 의해 보장됨

#### 3.2.4 컨텍스트 모드 5종
- `dev.md`: 코드 먼저, 설명은 나중에. 빠른 이터레이션
- `debug.md`: 체계적 진단. 재현 → 가설 → 근본 원인 수정 (증상 수정 금지)
- `review.md`: 전체 diff 읽기. 코멘트 접두사 `blocker:`, `suggestion:`, `question:`, `nit:`
- `deploy.md`: 안전과 가역성. 금요일 배포 금지, 스테이징 건너뛰기 금지, 배포 후 15분 모니터링
- `research.md`: 리서치 우선 모드

#### 3.2.5 MCP 프리셋 구성
- `recommended.json`: 14개 서버 (filesystem, GitHub, PostgreSQL, Redis, Docker, Memory, Fetch 등)
- 프론트엔드/풀스택/DevOps/데이터사이언스/K8s별 프리셋
- 새 프로젝트 시작 시 MCP 구성 시간 대폭 단축

#### 3.2.6 플러그인 마켓플레이스 아키텍처
- 각 플러그인이 `.claude-plugin/` 매니페스트를 포함한 자체 완결 패키지
- `/plugin marketplace add` 명령으로 개별 설치 가능
- 121개 플러그인이 독립적으로 설치·제거·갱신 가능

### 3.3 단점

#### 3.3.1 툴킷 자체의 프로덕션 검증 부재
- 이 프로젝트는 "도구 모음"이지 실제 프로덕션 앱이 아님
- 135개 에이전트, 121개 플러그인이 실제 프로덕션에서 검증되었는지 불확실
- 템플릿의 권장 사항(80% 라인 커버리지, 75% 브랜치 커버리지 등)이 범용적이어서 프로젝트 맥락에 맞지 않을 수 있음

#### 3.3.2 과도한 규모
- 300+ 파일, 10개 디렉토리는 "참고"하기에는 좋지만 전체 도입은 비현실적
- 135개 에이전트 중 실제 필요한 것은 5~10개 수준
- 121개 플러그인은 대부분의 프로젝트에서 불필요

#### 3.3.3 훅의 복잡도와 디버깅 어려움
- 19개 훅이 동시에 작동하면 예상치 못한 상호작용 발생 가능
- PostToolUse에 lint-fix + type-check + auto-test + bundle-check가 모두 걸리면 단순 편집에도 지연
- 훅 실패 시 원인 파악이 어려움 (어떤 훅이 블록했는지 추적 필요)
- 특히 `block-md-creation.js` 같은 훅은 정당한 문서 생성까지 차단할 위험

#### 3.3.4 도메인 스펙 문서 체계 부재
- everything-claude-code와 동일한 한계: API 스펙, DB 스키마 같은 프로젝트 고유 문서의 관리 패턴 미제공
- 템플릿에 "아키텍처 패턴"과 "컨벤션" 섹션이 있지만, 이는 규칙이지 스펙이 아님
- 우리 프로젝트의 AMK_API_MASTER.md(269KB) 같은 대규모 스펙 관리에는 별도 전략 필요

#### 3.3.5 메모리 관리의 CLAUDE.md 의존
- Memory Bank가 CLAUDE.md 자체를 메모리 저장소로 사용하는 접근
- Claude Code의 네이티브 메모리 시스템(~/.claude/projects/.../memory/)과 별개로 동작
- 두 시스템이 공존하면 "어디에 기억이 저장되는가"에 대한 혼란 발생
- 우리 프로젝트는 이미 네이티브 메모리 시스템을 활발히 사용 중이므로 Memory Bank 방식은 충돌 위험

### 3.4 개선점

#### 3.4.1 선택적 도입 가이드 필요
- 프로젝트 규모별 "이것만 쓰세요" 추천 세트 제공 필요
  - 1인 프로젝트: comprehensive 템플릿 + 핵심 훅 3~4개 + 규칙 5개
  - 소규모 팀: + contexts 3개 + 에이전트 10개
  - 대규모 팀: 전체 세트

#### 3.4.2 훅 우선순위/필수·선택 분류
- 19개 훅 중 필수(secret-scanner, commit-guard)와 선택(suggest-compact, bundle-check) 분류
- 프로젝트별 훅 프로필 제공 (예: Rust 프로젝트용, Node.js 프로젝트용)

#### 3.4.3 네이티브 메모리 시스템 통합
- Claude Code의 ~/.claude/memory/와 Memory Bank 커맨드를 통합하는 가이드 필요
- 현재는 두 시스템이 독립적으로 동작하여 메모리 이중 관리 위험

### 3.5 리스크

#### 3.5.1 훅 과부하로 인한 작업 속도 저하
- 19개 훅이 모두 활성화되면 단일 파일 편집에도 lint + type-check + test + bundle-check 실행
- Rust 프로젝트에서 `cargo check`는 수십 초 소요 → 매 편집마다 실행은 비현실적
- **완화 방안**: 핵심 훅만 선별 도입, Bash 커맨드 전용 훅은 PostToolUse 대신 수동 실행

#### 3.5.2 템플릿의 맹목적 적용
- comprehensive 템플릿의 구조를 그대로 복사하면 프로젝트 맥락에 맞지 않는 섹션 발생
- 예: "80% line coverage" 목표는 우리 프로젝트에 테스트가 없는 현재 상태와 괴리
- **완화 방안**: 템플릿의 구조(섹션 분류)만 참고하고 내용은 프로젝트 맥락에 맞게 작성

#### 3.5.3 플러그인 마켓플레이스 의존도
- 플러그인 설치/갱신이 외부 저장소에 의존
- 저장소 비활성화나 호환성 문제 시 툴킷 전체에 영향
- **완화 방안**: 필요한 파일은 프로젝트에 직접 복사하여 사용

#### 3.5.4 오버엔지니어링 유인
- 135개 에이전트, 121개 플러그인의 존재 자체가 "더 많이 쓸수록 좋다"는 인상을 줌
- 실제로는 소수의 잘 선택된 도구가 다수의 어중간한 도구보다 효과적
- **완화 방안**: 도입 시 "이 도구가 없으면 어떤 문제가 발생하는가?" 기준으로 필터링

### 3.6 우리 프로젝트 도입 방안

#### 도입할 것

| 패턴 | 적용 방법 | 기대 효과 |
|------|-----------|-----------|
| **comprehensive 템플릿 구조** | CLAUDE.md를 10개 섹션 구조로 재편: 개요 → 스택 → 명령어 → 구조 → 규칙 → 컨벤션 → 참조 문서 → 메모리 안내 | 체계적인 정보 배치, 빠른 탐색 |
| **Key Decisions Log** | MEMORY.md 또는 별도 토픽 파일에 날짜/결정/근거 테이블 도입 | 의사결정 맥락 구조화 (현재 산재된 정보 통합) |
| **Memory Bank 200줄 제한** | MEMORY.md 인덱스를 200줄 이내로 엄수, 초과 시 아카이브 | 잘리는 정보 없이 전체 인덱스 로드 보장 |
| **secret-scanner 훅** | PreToolUse 훅으로 .env, credentials 등 시크릿 패턴 차단 | 실수로 시크릿이 코드에 포함되는 것 방지 |
| **세션 상태 보존 개념** | 대규모 작업 시 PreCompact 훅 또는 수동 체크포인트로 상태 저장 | 컨텍스트 컴팩션 시 정보 유실 방지 |

#### 도입하지 않을 것

| 패턴 | 미도입 사유 |
|------|-------------|
| **Memory Bank 커맨드** | 네이티브 메모리 시스템(~/.claude/memory/)과 충돌. 기존 시스템 유지 |
| **PostToolUse 자동 lint/test** | Rust `cargo check` 수십 초 소요. 매 편집마다 실행은 비현실적. 커밋 전 수동 실행 유지 |
| **135개 에이전트** | 프로젝트 규모에 과도. 필요 시 CLAUDE.md에 서브에이전트 지침 직접 기재 |
| **플러그인 마켓플레이스** | 외부 의존도 증가. 필요한 파일은 직접 복사 |
| **contexts/ 5개 모드** | 단일 개발자 워크플로우에 모드 전환 불필요 |
| **MCP 프리셋** | 이미 Figma MCP 설정 완료. 추가 프리셋 불필요 |

---

## 부록: 도입 우선순위 총정리

| 순서 | 작업 | 소스 | 난이도 | 영향도 |
|:----:|------|------|:------:|:------:|
| 1 | **CLAUDE.md / MEMORY.md / AMK 역할 경계 명문화** | 자체 분석 | 낮음 | 높음 |
| 2 | **MEMORY.md 200줄 이내 축소** (완료 이력·행정·구조 → 토픽 파일 분리) | toolkit 패턴 | 중간 | 높음 |
| 3 | **CLAUDE.md 재구성** (comprehensive 템플릿 구조 참고) | toolkit 템플릿 | 중간 | 높음 |
| 4 | **메모리 중복 파일 삭제** (content_protection, ebook_web_viewer) | 자체 분석 | 낮음 | 중간 |
| 5 | **데이터 오류 수정** (i18n ar 제거, textbook ISBN 갱신) | 자체 분석 | 낮음 | 중간 |
| 6 | **Key Decisions Log 도입** | toolkit 패턴 | 낮음 | 중간 |
| 7 | **구식 AMK 문서 정리** (FILE_TREE, FOOTER_ISSUE, MEMORY_AUDIT) | 자체 분석 | 낮음 | 낮음 |
| 8 | **secret-scanner 훅 도입** | toolkit 훅 | 중간 | 중간 |
| 9 | **완료된 메모리 계획 파일 아카이브** | 자체 분석 | 낮음 | 낮음 |

---

## 4. 참고 자료: AI 에이전트 문서화 전략

### 4.1 Tier 1 — 철학/전략 수준 (필독)

#### 4.1.1 Context Engineering for Coding Agents — Martin Fowler / ThoughtWorks

- **URL**: https://martinfowler.com/articles/exploring-gen-ai/context-engineering-coding-agents.html
- **유형**: 심층 기술 아티클 (ThoughtWorks / martinfowler.com)
- **핵심 내용**:
  - "컨텍스트 엔지니어링"이 "프롬프트 엔지니어링"의 상위 개념으로 부상 — 대체가 아닌 진화/확장. "모델이 보는 것을 큐레이션하여 더 나은 결과를 얻는 기술"
  - 에이전트에게 **너무 많은 컨텍스트를 주면 오히려 성능이 떨어짐** — 대형 컨텍스트 윈도우가 있어도 균형이 핵심
  - Reusable Prompts(지시+가이드), Context Interfaces(도구, MCP 서버, 스킬), File Access 3가지 범주로 구분. 문서는 Skills(지연 로드 리소스) 내부에 포함되어 간접적으로 컨텍스트 인터페이스 역할 수행
  - **Decision Authority Model**: 컨텍스트 로딩 주체가 LLM(자율적, 불확실) / Human(통제) / Agent Software(결정적) 중 누구인지가 핵심 설계 축
  - **"Illusion of Control" 경고**: LLM 행동은 확률적 — 가드레일에 "보장(ensure)", "방지(prevent)" 같은 확신 용어를 쓰지 말 것
  - **컨텍스트 투명성**: Claude Code의 `/context` 명령이 에이전트가 무엇을 보는지 가시화하는 사례로 소개
- **우리 프로젝트 적용점**:
  - AMK_API_MASTER.md(269KB)를 매번 전체 로드하는 것은 비효율 → 필요한 섹션만 참조하는 패턴 필요
  - CLAUDE.md가 "어떤 문서를 언제 읽을지" 안내하는 역할을 강화해야 함
  - Decision Authority Model 관점: CLAUDE.md(매번 자동 로드 = Agent Software 주도) vs MEMORY 토픽 파일(Claude가 필요 시 로드 = LLM 주도) vs AMK 문서(사용자 지시로 로드 = Human 주도)로 구분 가능

#### 4.1.2 How to Write a Good Spec for AI Agents — Addy Osmani (Google Chrome 리드)

- **URL**: https://addyosmani.com/blog/good-spec/
- **URL (O'Reilly)**: https://www.oreilly.com/radar/how-to-write-a-good-spec-for-ai-agents/
- **유형**: 블로그 포스트 / 아티클
- **핵심 내용**:
  - 스펙을 구조화된 PRD로 취급 — 느슨한 노트가 아닌 명확한 섹션 구분
  - **Self-verification 패턴**: 에이전트가 구현 후 스펙 대비 자체 검증하도록 지시
  - **"LLM-as-a-Judge"** 패턴: 주관적 기준(코드 스타일, 아키텍처 준수)을 LLM이 평가
  - 테스트 계획을 스펙에 직접 포함 — AI 에이전트를 위한 TDD
  - 스펙은 사람과 AI 시스템 간의 **계약서** 역할
  - **Three-Tier Boundary System**: Always-do(항상 수행) / Ask-first(확인 후 수행) / Never-do(절대 금지) 3단계로 에이전트 권한을 정의하는 프레임워크
  - **Spec-Driven 4단계 게이트**: Specify → Plan → Tasks → Implement, 각 단계마다 검증 게이트를 두어 품질 확보
  - **도메인 전문성 주입**: 경험 많은 개발자가 지식을 스펙에 인코딩하여 에이전트가 흔한 함정을 피하도록 유도
- **우리 프로젝트 적용점**:
  - AMK_API_MASTER.md가 이미 "계약서" 역할을 하고 있음 — 이 방향을 유지·강화
  - "코드 변경 시 AMK_API_MASTER.md도 함께 업데이트"라는 규칙이 Self-verification과 유사
  - Three-Tier Boundary 적용 가능: Always-do(문서 동기화, cargo check) / Ask-first(DB 마이그레이션, 프로덕션 배포) / Never-do(시크릿 노출, 프로덕션 직접 수정)

#### 4.1.3 A Complete Guide to AGENTS.md — AI Hero

- **URL**: https://www.aihero.dev/a-complete-guide-to-agents-md
- **유형**: 종합 가이드
- **핵심 내용**:
  - **Progressive disclosure**: 루트 파일은 작고 집중적으로 유지하고, 상세 가이드는 하위 파일로 분산
  - **"지시의 저주"(Curse of Instructions)**: 지시를 쌓을수록 모델의 개별 지시 준수율이 급격히 하락. 프론티어 LLM이 합리적으로 따를 수 있는 한계는 **~150-200개 지시** — 이를 초과하면 준수율 급락
  - 이상적인 AGENTS.md는 "작업을 시작하기에 충분한 컨텍스트 + 더 상세한 가이드로의 이정표"를 제공
  - **지시 예산 효율성**은 설계해야 할 실제 제약 조건. 모든 토큰이 매 요청마다 로드되므로, 파일이 크면 "실제 작업에 쓸 토큰이 줄어들고 에이전트가 혼란"
  - **Stale Documentation Poisoning**: 구식 파일 경로 등 오래된 정보가 에이전트 성능을 **적극적으로 해침**. 파일 구조를 나열하기보다 기능(capability)을 기술하고, "계획 단계에서 just-in-time 문서화"를 권장
- **우리 프로젝트 적용점**:
  - CLAUDE.md 118줄은 적절한 규모이나, 불완전한 내용이 문제 — 줄 수가 아니라 정보 밀도 최적화 필요
  - MEMORY.md 298줄은 "지시의 저주"에 해당 — 200줄 초과분이 로드되지 않을 뿐 아니라, 로드되는 200줄 내에도 불필요한 정보가 혼재
  - Stale Documentation Poisoning이 우리 프로젝트에서 실제 발생 중: AMK_FILE_TREE.md(2026-02-18)의 구식 파일 구조, i18n_multilingual_plan.md의 잘못된 ar 언어 등

### 4.2 Tier 2 — 실증/데이터 기반

#### 4.2.1 How to Write a Great agents.md — GitHub 공식 블로그

- **URL**: https://github.blog/ai-and-ml/github-copilot/how-to-write-a-great-agents-md-lessons-from-over-2500-repositories/
- **유형**: 데이터 기반 블로그 포스트 (2,500+ 저장소 분석)
- **핵심 내용**:
  - 효과적인 파일의 공통점: 구체적 역할/페르소나 부여, 정확한 명령어(플래그·옵션 포함), 명확한 경계 설정, 실제 출력 예시
  - **코드 예시가 산문 설명을 압도** — 스타일을 3문단으로 설명하는 것보다 실제 스니펫 하나가 효과적
  - 명령어는 파일 초반에 배치할 것
  - 역할 기반 에이전트(docs-agent, test-agent, security-agent) 패턴이 부상 중
  - **Three-Tier Boundary**: "always do", "ask first", "never do" 3단계 경계 프레임워크가 최상위 구현의 공통 특징
  - **기술 버전 명시**: "React"가 아닌 "React 18"처럼 구체적 버전을 기재할 것
- **우리 프로젝트 적용점**:
  - CLAUDE.md의 "백엔드 모듈 패턴", "프론트엔드 모듈 패턴"이 이미 코드 구조 예시 → 좋은 패턴
  - 검증 명령어(`cargo check`, `npm run build`)가 파일 후반부에 위치 → 초반으로 이동 검토

#### 4.2.2 AGENTS.md 공식 스펙 — Linux Foundation

- **URL**: https://agents.md/
- **GitHub**: https://github.com/agentsmd/agents.md
- **유형**: 오픈 표준 / 스펙. Agentic AI Foundation (Linux Foundation 산하)이 관리. OpenAI Codex, Amp, Jules(Google), Cursor, Factory 등이 공동 발족
- **핵심 내용**:
  - 의도적으로 최소한의 포맷: 마크다운만, 필수 구조나 필드 없음
  - **"에이전트를 위한 README"** — README.md(사람용)를 보완하는 에이전트 전용 컨텍스트
  - 6만+ 오픈소스 프로젝트 채택. Cursor, Aider, Gemini CLI, Jules, Codex, Zed, VS Code, Devin, Warp, GitHub Copilot, Windsurf, Augment Code 등 광범위 지원
  - 자주 포함되는 영역: 명령어, 테스트, 프로젝트 구조, 코드 스타일, git 워크플로우, 경계 (GitHub 블로그의 2,500개 분석에서 도출된 패턴)
  - **모노레포 중첩 지원**: 디렉토리별 AGENTS.md 배치 가능, 가장 가까운 파일이 우선(closest-file-wins)
- **우리 프로젝트 적용점**:
  - 크로스 도구 호환성을 고려하면 CLAUDE.md + AGENTS.md 병행도 옵션이나, 현재 Claude Code 단일 사용이므로 CLAUDE.md 집중이 효율적
  - 자주 포함되는 영역은 CLAUDE.md 재구성 시 섹션 분류 기준으로 참고 가능

#### 4.2.3 Using CLAUDE.md Files — Anthropic 공식 블로그

- **URL**: https://claude.com/blog/using-claude-md-files
- **유형**: 공식 블로그 포스트 (Anthropic)
- **핵심 내용**:
  - CLAUDE.md는 매 대화마다 시스템 프롬프트에 추가됨 — 간결하게 유지해야 함
  - 가능한 한 적은 지시만 포함. **프로젝트에 특화된** 내용을 담되, "팀이 실제로 소프트웨어를 개발하는 방식"을 반영
  - `/init`으로 시작한 뒤 관찰된 행동을 기반으로 개선 — "시작점이지 완성품이 아님"
  - 가장 효과적인 파일은 실제 문제를 해결: 반복 입력하는 명령어, 설명에 10분 걸리는 아키텍처 컨텍스트, 재작업을 방지하는 워크플로우
  - **보안 경고**: 자격 증명, DB 연결 문자열, 보안 취약점 세부 정보를 CLAUDE.md에 포함하지 말 것
  - `.claude/commands/` 디렉토리로 반복적인 프롬프트를 커스텀 슬래시 커맨드로 정의 가능
- **출처 보정**: "200줄 이하" 목표와 `.claude/rules/` 분리는 이 블로그가 아닌 Claude Code CLI 문서/내부 가이드 출처. 이 블로그의 핵심은 "프로젝트 특화 + 간결 + 실제 문제 해결"
- **우리 프로젝트 적용점**:
  - CLAUDE.md에 "반복되는 실수를 관찰한 후 규칙 추가"라는 반응적 접근이 우리에게도 필요
  - 현재 CLAUDE.md에는 프로젝트 진행 중 추가된 적이 없는 초기 규칙만 있음 — 실제 겪은 문제 기반으로 갱신 필요
  - `.claude/commands/` 활용 검토: 자주 쓰는 워크플로우(문서 동기화 확인, 빌드 검증 등)를 커맨드로 정의 가능

### 4.3 Tier 3 — Spec-Driven Development (스펙 우선 개발)

#### 4.3.1 Kiro — Spec-Driven Development — AWS

- **URL**: https://kiro.dev/blog/from-chat-to-specs-deep-dive/
- **URL**: https://kiro.dev/blog/kiro-and-the-future-of-software-development/
- **유형**: 블로그 포스트 + 도구 문서 (AWS Kiro IDE)
- **핵심 내용**:
  - 스펙 기반 개발 파이프라인: **requirements.md → design.md → tasks.md → 구현**
  - 스펙 수준에서 작업하면 개발자가 중요한 것(아키텍처, 요구사항)에 집중하고 구현은 에이전트에 위임
  - 스펙은 대규모 작업에서 에이전트가 길을 잃지 않게 하는 지속적 참조 역할 (원문에 "북극성" 표현은 없으나 동일한 개념 전달)
  - 반복적 명확화·혼잡한 컨텍스트 문제에서 벗어나 지속 가능한 사람-AI 협업으로 전환
  - 코드베이스 분석 기반 파일(structure.md, tech.md, product.md)을 자동 생성하여 스펙 작성 전 기초 분석 제공
  - 스펙이 **자연스러운 중단점(pause point)**을 생성하여 사람이 검토·승인할 수 있는 체크포인트 역할
  - 스펙은 기술 선택의 이유를 기록하는 **감사 추적(audit trail)** — 기관 지식의 보존 수단
- **우리 프로젝트 적용점**:
  - 우리의 AMK_API_MASTER.md가 이미 "requirements + design" 역할을 겸하고 있음
  - 대규모 기능(e-book 뷰어 같은) 구현 시 tasks.md 패턴을 도입하면 단계별 추적 가능
  - 현재 MEMORY.md의 "다음 작업" 섹션이 tasks.md의 간이 버전 역할 중

#### 4.3.2 Spec-Driven Development with AI — GitHub 공식

- **URL**: https://github.blog/ai-and-ml/generative-ai/spec-driven-development-with-ai-get-started-with-a-new-open-source-toolkit/
- **유형**: 블로그 포스트 + 오픈소스 툴킷 발표
- **핵심 내용**:
  - 스펙을 "살아있는 실행 가능 산출물(living, executable artifacts)"로 정의 — 에이전트에게 "모호하지 않은 지시"를 제공
  - 에이전트가 **컨벤션, 구조화된 스펙, 결정적 프로세스** 안에서 작동할 때 가치 극대화
  - 4단계 프로세스: Specify(고수준 설명) → Plan(아키텍처) → Tasks(검토 가능한 작은 단위) → Implement
  - **"코드가 진실" → "의도가 진실"** 패러다임 전환 — 안정적인 "what"과 유연한 "how"를 분리
  - 3가지 유스케이스: 그린필드 프로젝트, 기존 시스템 기능 추가, 레거시 현대화
  - 엔터프라이즈 고려: 보안 정책, 컴플라이언스 규칙, 디자인 시스템을 스펙에 내장
- **우리 프로젝트 적용점**:
  - 세션 간 지속성은 우리의 MEMORY.md 시스템이 이미 제공 중
  - "스펙 대비 검증" 패턴을 강화: 코드 변경 후 AMK_API_MASTER.md와의 일치 여부를 체계적으로 확인
  - "의도가 진실" 개념은 AMK_API_MASTER.md가 이미 실천 중 — 코드보다 스펙이 우선하는 구조

### 4.4 Tier 4 — 실용 가이드

#### 4.4.1 How to Write a Good CLAUDE.md File — Builder.io

- **URL**: https://www.builder.io/blog/claude-md-guide
- **유형**: 실용 가이드 / 블로그 포스트
- **검증 상태**: ✅ 원문 전체 확인 완료
- **핵심 내용**:
  - "필수 요소(Essentials)": 프로젝트 컨텍스트(한 줄 설명 + 스택), 코드 스타일, 명령어, Gotchas(주의사항). 예시에서는 `## Code Style`, `## Commands`, `## Architecture`, `## Important Notes` 섹션 사용
  - **`@imports` 시스템**: `@path/to/file` 문법으로 CLAUDE.md에서 다른 파일을 참조. 상세 지시를 별도 마크다운에 분리 후 연결
  - **`.claude/rules/` 디렉토리**: 대규모 프로젝트용 별도 메커니즘. `@imports` 없이 자동 로드됨. 위 `@imports`와는 독립적인 두 번째 분할 방법
  - **300줄 이하** 권장 (200줄이 아님)
  - 핵심 실천 사항: 프로젝트 한 줄 설명으로 시작, 코드 스타일은 구체적이고 실행 가능하게, 작업하면서 점진적 추가, 주기적 검토
- **출처 보정**: 초기 분석의 "`--help` 플래그 문서화"와 "plan 모드 사용"은 이 기사에 없음 (다른 builder.io 기사 또는 Anthropic 문서 출처 혼동). `@imports`와 `.claude/rules/`는 별개 메커니즘이나 초기 분석에서 하나로 혼합 기재됨
- **우리 프로젝트 적용점**:
  - `@imports`는 CLAUDE.md에서 AMK 문서 특정 섹션을 직접 참조하는 데 활용 가능
  - `.claude/rules/`는 규칙이 CLAUDE.md 300줄을 초과할 경우 분할 옵션
  - 현재 규모(118줄)에서는 인라인 유지가 적절

#### 4.4.2 Writing a Good CLAUDE.md — HumanLayer

- **URL**: https://www.humanlayer.dev/blog/writing-a-good-claude-md
- **유형**: 블로그 포스트
- **핵심 내용**:
  - 불필요한 지시를 사전에 쌓지 말고, 실제 문제가 발생했을 때 규칙을 추가하는 반응적 접근 권장
  - **"복사 대신 포인터"**: `file:line` 참조를 사용하여 Claude를 권위 있는 컨텍스트로 안내. 복사본은 시간이 지나면 원본과 괴리
  - **~150-200개 지시 용량 한계**: 프론티어 LLM이 합리적으로 따를 수 있는 지시 수. Claude Code의 시스템 프롬프트가 이미 ~50개 지시를 사용하므로, 사용자가 추가할 수 있는 여유분은 제한적
  - **System Reminder Injection**: Claude Code가 CLAUDE.md 내용에 대해 "관련성이 높을 때만 참조하라"는 시스템 리마인더를 주입 — 지시가 무시되는 원인 중 하나
  - **"린터 일에 LLM을 보내지 마라"**: 린터가 더 빠르고 저렴. LLM에게 포매팅·스타일 검사를 맡기지 말 것
  - **Progressive disclosure**: 작업별 문서를 `agent_docs/building_the_project.md` 같은 별도 파일로 분리
  - **CLAUDE.md 자동 생성 금지**: 워크플로우 전체에 영향을 미치므로 반드시 수동 작성
- **출처 보정**: 초기 분석에서 기재한 "강박적 컨텍스트 관리" 표현과 "같은 실수 반복 시 규칙 추가"는 원문에 해당 표현 없음. 위 내용으로 정정
- **우리 프로젝트 적용점**:
  - "복사 대신 포인터" 원칙이 우리의 핵심 개선 방향과 일치
  - MEMORY.md에 AMK 문서 내용을 복사한 부분(결제 시스템 상세, 프로젝트 구조 등)이 정확히 이 안티패턴에 해당
  - ~150-200개 지시 한계를 고려하면 CLAUDE.md + MEMORY.md 합산 지시량 관리가 중요
  - System Reminder Injection을 인지하고, CLAUDE.md의 핵심 규칙은 "관련성이 명확하게 읽히도록" 구체적으로 작성해야 함

#### 4.4.3 The Complete Guide to AI Agent Memory Files — Medium

- **URL**: https://medium.com/data-science-collective/the-complete-guide-to-ai-agent-memory-files-claude-md-agents-md-and-beyond-49ea0df5c5a9
- **미러**: https://hackernoon.com/the-complete-guide-to-ai-agent-memory-files-claudemd-agentsmd-and-beyond (HackerNoon 크로스포스트)
- **유형**: 종합 비교 아티클
- **검증 상태**: ✅ HackerNoon 미러를 통해 원문 확인 완료. Medium URL은 403이나 동일 저자의 크로스포스트로 내용 동일
- **핵심 내용**:
  - CLAUDE.md는 **"기억상실인 신입 팀원에게 주는 브리핑 문서"** — 원문 직접 인용: "Think of it as a briefing document for a new team member who has amnesia"
  - LLM의 **지시 감쇠(instruction decay)**: 중요한 지시가 대화 중에 묻힐 수 있음. 학술적으로는 "Lost in the Middle" 현상 (Stanford, Liu et al. 2023)으로 실증 — LLM이 컨텍스트 시작과 끝은 잘 기억하나 중간부 정확도가 급락하는 U자형 곡선
  - 새 지시가 기존 지시와 충돌하는 **지시 충돌**이 흔한 실패 원인 — 원문: "Instructions accumulate, some become redundant, others conflict." Arbiter 논문(arXiv 2603.08993)에서 Claude Code v2.1.50 시스템 프롬프트에서 실제 21개 간섭 패턴(4개 직접 모순 포함) 발견
  - **메모리 비대**(signal이 noise에 묻힘)로 의사결정 품질 저하 — 원문에서 ~150줄 이내 유지 권장. 독립 연구에서도 "집중된 2K 토큰 컨텍스트가 25K 토큰 덤프보다 우수"하다는 결과
- **우리 프로젝트 적용점**:
  - MEMORY.md 298줄이 정확히 "메모리 비대" 상태
  - 200줄 초과분이 잘리면서 "중요한 지시가 묻히는" 현상이 실제로 발생 중 (참고 파일 목록, 피드백 섹션)
  - 지시 충돌 위험: MEMORY.md의 구버전 정보(ISBN 매핑, ar 언어)가 최신 정보와 충돌

#### 4.4.4 Context Management Best Practices — DigitalOcean

- **URL**: https://docs.digitalocean.com/products/gradient-ai-platform/concepts/context-management/
- **유형**: 공식 문서
- **핵심 내용**:
  - **포괄적 PRD로 시작** → "집중적이면서 철저한(focused but thorough)" 접근. 최소가 아니라 체계적으로 시작
  - **컨텍스트 오버플로우** 시 에이전트가 토큰 한계에 도달하여 초점 없는 응답 생성
  - **세션 격리**: 주요 기능 간 전환 시 새 세션을 시작하여 이전 컨텍스트를 클리어
  - **테스트 주도 컨텍스트**: 테스트 케이스를 먼저 작성하여 에이전트가 기대 동작을 이해하도록 유도
  - **llms.txt 통합**: 표준화된 `llms.txt` 포맷으로 구조화된 플랫폼 문서를 제공하여 환각(hallucination) 감소
  - **이슈 기반 분해**: 한 번에 모든 것을 요청하지 말고, 구체적이고 실행 가능한 이슈로 분해
- **출처 보정**: 초기 분석에서 "최소로 시작"이라고 기재했으나 원문은 반대로 "포괄적 PRD로 시작" 권장. "정기적 감사"와 "프로젝트 특화 스타일 문서화"는 원문에 없음
- **우리 프로젝트 적용점**:
  - 세션 격리 패턴은 `/clear` 활용과 일치 — 대규모 기능 전환 시 적극적으로 사용
  - 이슈 기반 분해는 현재 MEMORY.md "다음 작업" 테이블과 유사 — 유지·강화

### 4.5 소스 간 공통 원칙 종합

검증을 거쳐 확인된 **9가지 핵심 원칙**:

| # | 원칙 | 설명 | 관련 소스 | 검증 |
|---|------|------|-----------|------|
| 1 | **Less is more** | 컨텍스트를 쌓을수록 개별 지시 준수율 하락. ~150-200개 지시가 프론티어 LLM의 합리적 한계 | Fowler, AI Hero, HumanLayer | ✅ 확인 |
| 2 | **Progressive disclosure** | 루트 파일은 작게 유지, 상세는 하위 파일로. 필요할 때만 참조 | AI Hero, Builder.io, HumanLayer | ✅ 확인 |
| 3 | **반응적 규칙 추가** | 사전에 규칙을 만들지 말고, 실제 문제가 발생했을 때 추가 | Anthropic, HumanLayer | ✅ 확인 |
| 4 | **스펙 = 계약** | 문서를 에이전트와의 계약으로 취급. 구현 후 스펙 대비 검증. Three-Tier Boundary로 권한 정의 | Osmani, Kiro, GitHub spec-kit | ✅ 확인 |
| 5 | **코드 예시 > 산문** | 규칙을 설명하기보다 실제 코드 스니펫을 보여줄 것. 기술 버전도 명시 | GitHub 2,500개 분석 | ✅ 확인 |
| 6 | **AI용 ≠ 사람용** | README는 사람용, CLAUDE.md/AGENTS.md는 에이전트용. 겹치지 않게 | AGENTS.md 스펙, Fowler | ✅ 확인 |
| 7 | **구식 문서는 독** | Stale Documentation Poisoning — 구식 정보가 에이전트 성능을 적극적으로 해침. 정기 감사 필수 | AI Hero, HumanLayer | ✅ 확인 |
| 8 | **복사 대신 포인터** | 내용을 복사하지 말고 `file:line` 참조로 연결. 복사본은 시간이 지나면 원본과 괴리 | HumanLayer | ✅ 확인 |
| 9 | **의도가 진실** | "코드가 진실" → "의도(스펙)가 진실" 패러다임. 안정적인 "what"과 유연한 "how"를 분리 | GitHub spec-kit, Kiro | ✅ 확인 |

### 4.6 개발자 특성 기반 문서화 전략

#### 4.6.1 현재 작업 패턴 진단

이 프로젝트는 **강한 실행력 + 강한 비판적 사고를 가진 단일 개발자**가 운영하며, 다음과 같은 작업 패턴을 보임:

| 패턴 | 설명 | 문서화에 미치는 영향 |
|------|------|---------------------|
| **빌드 우선, 정리 나중** | 기능 구현에 집중하고 정리는 뒤로 미룸 (e-book 코드 미커밋, MEMORY 298줄 방치) | 문서/메모리 관리 부채가 누적됨. 정리 시점을 시스템적으로 강제해야 함 |
| **대화 내 교정, 시스템 미반영** | Claude 실수를 그때그때 지적하지만 피드백 메모리에 기록하지 않음 (수개월간 1건) | 같은 교정을 매 대화 반복할 위험. 피드백 루프가 휘발됨 |
| **문서 대량 생성, 유지보수 부재** | AMK 14개(780KB) + 메모리 31개(220KB)를 만들었으나 갱신/삭제 루틴 없음 | 구식 문서가 Stale Documentation Poisoning으로 직결 |
| **CLAUDE.md를 정적 문서로 인식** | 초기 생성 후 한 번도 갱신하지 않음. 3개 도메인(ebook/textbook/payment) 추가에도 미반영 | Claude가 프로젝트 구조의 1/3을 모르는 상태로 매 대화 시작 |
| **시스템 메커니즘 미활용** | `.claude/rules/`, `.claude/commands/`, 훅, `@imports` 미사용. MEMORY 200줄 제한 미인지 | Claude Code의 자동화 기능을 활용하지 못하고 수동으로 보완 |

#### 4.6.2 패턴별 대응 전략

##### 대응 1: "빌드 우선" → 정리 시점을 시스템에 내장

문제: 기능 완료 후 문서 정리를 잊음
해결: 문서 관리를 작업 프로세스의 일부로 만들기

```
기능 구현 완료 시 체크리스트 (CLAUDE.md에 기재):
□ cargo check + npm run build 통과
□ AMK_API_MASTER.md 동기화 확인
□ CLAUDE.md 디렉토리 구조 현행화 확인
□ MEMORY.md 200줄 이내 확인
□ 완료된 메모리 토픽 파일에 "완료" 표기 또는 삭제
□ 코드 커밋
```

향후 검토: PostToolUse 훅으로 `cargo check` 자동화, 커밋 시 문서 동기화 리마인더 훅 도입

##### 대응 2: "대화 내 교정" → 피드백 루프 자동화

문제: Claude 실수 교정이 시스템에 축적되지 않음
해결: 피드백 기록 기준을 명확히 하고 CLAUDE.md에 명시

```
CLAUDE.md에 추가할 규칙:
- Claude에게 같은 교정을 2회 이상 하게 되면, 반드시 피드백 메모리로 기록
- 피드백 메모리에는 "규칙 + Why + How to apply" 구조 사용
```

현재 상태: 피드백 메모리 1건 → 목표: 실제 겪은 문제가 10건 이상 축적될 때 Claude의 행동 품질이 체감 가능하게 향상

##### 대응 3: "대량 생성, 유지보수 부재" → 정기 감사 루틴

문제: 구식 문서와 오류 데이터가 방치됨
해결: 주요 기능 완료 시 or 월 1회 메모리/문서 감사

```
감사 체크리스트:
□ MEMORY.md 200줄 이내인가?
□ 완료된 토픽 파일이 활성 상태로 남아있지 않은가?
□ AMK 문서 중 현재 코드와 불일치하는 것이 있는가?
□ 토픽 파일 간 데이터 충돌이 있는가?
□ docs/textbook/클로드 메모리/ 백업을 갱신할 필요가 있는가?
```

##### 대응 4: "CLAUDE.md 정적 인식" → 살아있는 문서로 전환

문제: CLAUDE.md가 프로젝트 진화를 반영하지 못함
해결: CLAUDE.md를 "매 주요 기능 완료 후 갱신하는 문서"로 재정의

갱신 트리거:
- 새 도메인 추가 시 → 디렉토리 구조 갱신
- 새 AMK 문서 추가 시 → 참조 문서 인덱스 갱신
- 새 환경변수/외부 서비스 추가 시 → 관련 규칙 갱신
- Claude가 반복적으로 틀리는 패턴 발견 시 → 규칙 추가

##### 대응 5: "시스템 메커니즘 미활용" → 단계적 도입

한 번에 모든 기능을 도입하지 않음. 빌드 우선 패턴에 맞게 **실제 불편함을 느낀 것부터** 도입:

| 단계 | 도입 대상 | 트리거 (언제 도입하나) |
|:----:|-----------|----------------------|
| 1 | **MEMORY.md 200줄 축소** | 지금 즉시 (98줄이 매 대화 잘리는 중) |
| 2 | **CLAUDE.md 재구성** | 축소 직후 (Progressive Disclosure 허브) |
| 3 | **피드백 메모리 적극 활용** | 재구성 직후 (규칙 기재) |
| 4 | **`.claude/commands/`** | 반복 워크플로우가 3회 이상 발생할 때 |
| 5 | **`.claude/rules/`** | CLAUDE.md가 200줄에 근접할 때 |
| 6 | **훅 (secret-scanner)** | 시크릿 노출 사고 또는 불안감 발생 시 |
| 7 | **훅 (cargo check 자동화)** | 빌드 검증 누락이 2회 이상 발생할 때 |

### 4.7 최종 적용 방향

위 12개 소스의 원칙 + 개발자 패턴 분석을 종합하면, 3가지 방향 + 운영 규칙으로 수렴됨:

#### 1. CLAUDE.md → "지도" (Progressive Disclosure 허브)
- 매 대화마다 로드되므로 **200줄 이하**, 프로젝트에 특화된 지시만
- **검증 명령어를 파일 초반에 배치** (cargo check, npm run build)
- 프로젝트 개요 + 핵심 규칙 + **전체 AMK 문서 인덱스(역할·참조 시점)** + 메모리 시스템 안내
- 상세 패턴·구조는 AMK 문서 링크로 참조만 걸기
- 코드 예시 위주 (산문 최소화). 기술 버전 명시 (Rust 2021, Axum 0.8, React 18 등)
- Three-Tier Boundary 명시: Always-do / Ask-first / Never-do
- **주요 기능 완료 시 반드시 갱신** (정적 문서 방지)

#### 2. MEMORY.md → "맥락 전용" (코드/문서에서 읽을 수 없는 것만)
- **200줄 엄수** — 초과 시 토픽 파일로 분리
- 역할: 의사결정 배경, 현재 상태, 다음 작업, 피드백, 보류 사항
- AMK 문서에 있는 내용은 **절대 복사하지 않고 참조만** (복사 대신 포인터)
- Key Decisions Log 도입 (날짜/결정/근거 구조)
- **완료된 토픽 파일은 즉시 삭제 또는 아카이브** (활성 메모리에 방치 금지)
- 법인 행정 등 코드 무관 정보는 별도 토픽 파일로 분리 (인덱스에서 제외 가능)

#### 3. AMK_*.md → "SSoT 계약서" (구현의 진실)
- 도메인별 독립 문서 유지 (현행 구조 유지)
- 구식 문서 정리 (FILE_TREE, FOOTER_ISSUE, MEMORY_AUDIT)
- 각 문서의 역할을 CLAUDE.md 인덱스에 명시
- 코드 변경 시 문서 동기화 규칙 유지·강화

#### 4. 운영 규칙 (빌드 우선 패턴 보완)
- **기능 완료 체크리스트**: cargo check → 문서 동기화 → MEMORY 확인 → 커밋 (순서 강제)
- **피드백 루프**: 같은 교정 2회 시 피드백 메모리 기록 필수
- **정기 감사**: 주요 기능 완료 시 메모리/문서 점검 (월 1회 이상)
- **도구 도입**: 반응적으로 — 불편함을 느낀 것부터 단계적으로
