# DESIGN.md 도입 분석 — Amazing Korean 적용 방안

> **작성일**: 2026-04-08
> **참조**: [VoltAgent/awesome-design-md](https://github.com/VoltAgent/awesome-design-md) (Google Stitch DESIGN.md 컨셉)
> **관련 문서**: `docs/AMK_DESIGN_SYSTEM.md`, `docs/AMK_FRONTEND.md`, `docs/AMK_DESIGN_SYSTEM.md`

---

## 1. DESIGN.md란 무엇인가

### 1.1 개념

Google Stitch가 도입한 **DESIGN.md**는 프로젝트의 시각적 디자인 시스템 전체를 마크다운으로 기술한 파일이다. AI 코딩 에이전트(Claude, Cursor, Copilot, Google Stitch 등)가 이 파일을 읽고 **일관된 UI를 자동 생성**할 수 있도록 설계되었다.

```
AGENTS.md → AI에게 "어떻게 빌드하는지" 알려줌
DESIGN.md → AI에게 "어떻게 보여야 하는지" 알려줌
```

### 1.2 기존 디자인 시스템 문서와의 차이

| 구분 | 기존 디자인 시스템 문서 | DESIGN.md |
|------|----------------------|-----------|
| **대상** | 개발자 (사람) | AI 에이전트 + 개발자 |
| **형식** | 자유 형식, 프로젝트마다 다름 | 표준 9개 섹션 구조 |
| **정밀도** | "primary 색상은 네이비" | `#051D55`, HSL `222 90% 18%`, 용도별 역할 명시 |
| **컴포넌트** | 사용법 중심 | CSS 수치(padding, radius, shadow) 포함 |
| **프롬프트** | 없음 | Agent Prompt Guide 섹션 포함 |
| **Do/Don't** | 간헐적 | 필수 섹션으로 구조화 |

### 1.3 동작 방식

1. 프로젝트 루트(또는 docs)에 `DESIGN.md` 배치
2. AI 에이전트에게 "DESIGN.md 따라서 페이지 만들어줘" 지시
3. 에이전트가 마크다운을 파싱하여 디자인 토큰, 컴포넌트 스타일, 레이아웃 규칙을 준수하며 코드 생성

Figma 익스포트나 JSON 스키마 없이 **마크다운만으로** 디자인 시스템을 전달할 수 있다. LLM이 가장 잘 이해하는 형식이 마크다운이기 때문이다.

---

## 2. awesome-design-md 레포지토리 분석

### 2.1 레포 개요

| 항목 | 내용 |
|------|------|
| URL | https://github.com/VoltAgent/awesome-design-md |
| 수록 사이트 | **58개** 유명 웹사이트의 DESIGN.md |
| 카테고리 | AI/ML (12), Developer Tools (14), Infrastructure (6), Design/Productivity (10), Fintech (4), Enterprise/Consumer (7), Car Brands (5) |
| 파일 구조 | `design-md/{site-name}/DESIGN.md` + `preview.html` + `preview-dark.html` |

### 2.2 표준 9개 섹션 구조

모든 DESIGN.md 파일이 동일한 구조를 따른다:

| # | 섹션 | 목적 | 포함 내용 |
|---|------|------|-----------|
| 1 | **Visual Theme & Atmosphere** | 전체 분위기 정의 | 디자인 철학, 밀도, 무드, 핵심 특성 bullet list |
| 2 | **Color Palette & Roles** | 색상 시스템 | 시맨틱 그룹별 (Primary, Brand, Neutral, Interactive, Shadows) hex + 기능 설명 |
| 3 | **Typography Rules** | 타이포그래피 | 폰트 패밀리, 전체 계층 테이블 (사이즈/웨이트/행간/자간), 원칙 |
| 4 | **Component Stylings** | 컴포넌트 스타일 | 버튼 variants, 카드, 인풋, 네비게이션, 뱃지 — CSS 수치 포함 |
| 5 | **Layout Principles** | 레이아웃 | 스페이싱 스케일, 그리드, 여백 철학, Border Radius 스케일 |
| 6 | **Depth & Elevation** | 깊이/그림자 | Shadow 레벨 테이블, Shadow 철학 |
| 7 | **Do's and Don'ts** | 디자인 가드레일 | 브랜드별 허용/금지 사항 bullet list |
| 8 | **Responsive Behavior** | 반응형 | 브레이크포인트, 터치 타겟, 축소 전략, 이미지 동작 |
| 9 | **Agent Prompt Guide** | AI 에이전트 가이드 | Quick Color Reference, 예제 프롬프트, 반복 가이드 |

### 2.3 주요 사이트별 시그니처 기법

교육 플랫폼인 Amazing Korean에 참고할 만한 6개 사이트를 상세 분석했다.

#### Notion — 따뜻한 미니멀리즘, 콘텐츠 플랫폼

| 항목 | 상세 |
|------|------|
| **디자인 철학** | 콘텐츠가 주인공. 인터페이스는 투명하게 |
| **배경** | `#ffffff` (순백) / Alt: `#f6f5f4` (황갈 언더톤의 따뜻한 화이트) |
| **텍스트** | `rgba(0,0,0,0.95)` — 순수 검정이 아닌 near-black |
| **액센트** | `#0075de` (Notion Blue) — UI 전체에서 유일한 채도 높은 색상 |
| **폰트** | NotionInter (수정 Inter), OpenType `"lnum"`, `"locl"` |
| **웨이트** | 400/500/600/700 (4단계) |
| **자간** | 64px에서 -2.125px, 16px에서 0 (크기에 비례하여 축소) |
| **보더** | `1px solid rgba(0,0,0,0.1)` ("whisper borders") |
| **그림자** | 4~5 레이어 스택, 개별 opacity 0.05 이하 |
| **유용한 점** | 콘텐츠 중심 레이아웃, 학습 플랫폼과 유사한 정보 구조 |

#### Stripe — 핀테크 럭셔리, 가벼운 무게감

| 항목 | 상세 |
|------|------|
| **디자인 철학** | "가벼움이 럭셔리" — 관습을 뒤집는 weight 300 헤드라인 |
| **배경** | `#ffffff` / 다크 섹션: `#1c1e54` (브랜드 인디고) |
| **텍스트** | `#061b31` (깊은 네이비, 순수 검정 아님) |
| **액센트** | `#533afd` (Stripe Purple) |
| **폰트** | `sohne-var` + OpenType `"ss01"` 전역 적용 |
| **시그니처** | **헤드라인에 weight 300 사용** — 통상적 700과 정반대 |
| **자간** | 56px에서 -1.4px |
| **그림자** | 블루 틴트 `rgba(50,50,93,0.25)` — 브랜드 컬러 반영 |
| **Radius** | 보수적 4~8px (pill 없음) |
| **유용한 점** | 신뢰감/프리미엄 느낌, 유료 교육 서비스에 적합 |

#### Linear — 다크모드 네이티브, 정밀 엔지니어링

| 항목 | 상세 |
|------|------|
| **디자인 철학** | 다크모드 우선, 빛으로 깊이 표현 |
| **배경** | `#08090a` (마케팅), `#0f1011` (패널), `#191a1b` (상승 표면) |
| **텍스트** | `#f7f8f8` (순수 화이트 아님), Secondary: `#d0d6e0` |
| **액센트** | `#5e6ad2` (브랜드 인디고) / `#7170ff` (인터랙티브 바이올렛) |
| **폰트** | Inter Variable + OpenType `"cv01"`, `"ss03"` |
| **시그니처** | **weight 510** (regular 400과 medium 500 사이), 최대 590 (700 사용 안 함) |
| **보더** | 반투명 화이트 `rgba(255,255,255,0.05~0.08)` |
| **깊이** | 그림자가 아닌 **배경 밝기 단계(luminance stepping)** 로 깊이 표현 |
| **유용한 점** | 다크모드 구현 시 참조, 반투명 보더 기법 |

#### Vercel — 극단적 미니멀리즘

| 항목 | 상세 |
|------|------|
| **디자인 철학** | 급진적 단순함, 그림자로 보더 대체 |
| **배경** | `#ffffff` / 텍스트: `#171717` |
| **워크플로우 색상** | Ship Red `#ff5b4f`, Preview Pink `#de1d8d`, Develop Blue `#0a72ef` |
| **폰트** | Geist / Geist Mono, OpenType `"liga"` |
| **자간** | 48px에서 **-2.4px ~ -2.88px** (가장 공격적) |
| **시그니처** | **Shadow-as-border** — `box-shadow: 0 0 0 1px rgba(0,0,0,0.08)`로 CSS border 대체 |
| **카드 그림자** | 4레이어: border ring + elevation + ambient + inner `#fafafa` glow ring |
| **유용한 점** | shadow-as-border 기법, 산만함 없는 학습 인터페이스에 적합 |

#### Figma — 흑백 갤러리, 가변 폰트 마스터리

| 항목 | 상세 |
|------|------|
| **디자인 철학** | 인터페이스는 흑백만. 색상은 콘텐츠에만 존재 |
| **색상** | 오직 `#000000`과 `#ffffff`. Glass: `rgba(0,0,0,0.08)` |
| **폰트** | figmaSans (variable), **비표준 웨이트: 320, 330, 340, 450, 480, 540, 700** |
| **시그니처** | 본문 웨이트 320~340 (일반적 400보다 가벼움), 미세 웨이트 조절 |
| **포커스** | `dashed 2px` — Figma 에디터의 선택 핸들과 동일한 스타일 |
| **유용한 점** | 가변 폰트 활용법 (Pretendard Variable에 적용 가능) |

#### Airbnb — 따뜻한 소비자 마켓플레이스

| 항목 | 상세 |
|------|------|
| **디자인 철학** | 사진 중심, 따뜻한 소비자 경험 |
| **배경** | `#ffffff` / 텍스트: `#222222` (따뜻한 near-black) |
| **액센트** | Rausch Red `#ff385c` — CTA에만 사용 (단일 액센트) |
| **폰트** | Airbnb Cereal VF, OpenType `"salt"` (배지/캡션) |
| **웨이트** | 500~700만 사용 (얇은 웨이트 헤드라인 금지) |
| **그림자** | 3레이어: border ring(0.02) + soft blur(0.04) + stronger blur(0.1) |
| **Radius** | 8px 버튼, 14px 배지, 20px 카드, 32px 대형, 50% 컨트롤 |
| **반응형** | **61개 브레이크포인트** — 가장 세밀한 반응형 시스템 |
| **유용한 점** | 코스/레슨 카드에 적용 가능한 사진 중심 카드 디자인, 소비자 친화적 따뜻함 |

### 2.4 6개 사이트의 공통 패턴

분석한 모든 사이트에서 공통적으로 발견된 패턴:

| # | 패턴 | 상세 |
|---|------|------|
| 1 | **Near-black 텍스트** | 순수 `#000000` 사용하는 사이트 없음. 모두 소프트닝된 near-black 사용 |
| 2 | **디스플레이 사이즈 음수 자간** | -0.44px(Airbnb) ~ -2.88px(Vercel) 범위 |
| 3 | **다층 그림자 스택** | 단일 box-shadow 사용 사이트 없음. 모두 3~5개 레이어 |
| 4 | **OpenType 피처 활용** | 각 사이트가 고유한 OT 피처로 브랜드 차별화 (ss01, cv01, lnum 등) |
| 5 | **Do/Don't 가드레일** | 명령형 동사로 시작하는 두 bullet list |
| 6 | **Agent Prompt Guide** | Quick Color Reference + 예제 프롬프트 + 반복 가이드 |

---

## 3. Amazing Korean 현재 디자인 시스템 진단

### 3.1 현재 보유 자산

| 자산 | 위치 | 상태 |
|------|------|------|
| 디자인 시스템 문서 | `docs/AMK_DESIGN_SYSTEM.md` | v4.1 (2026-04-02), 상세하고 체계적 |
| Tailwind 설정 | `frontend/tailwind.config.js` | CSS 변수 기반 전체 토큰 정의 |
| CSS 토큰 | `frontend/src/index.css` | 라이트/다크 모드, 상태 색상, 그라데이션, 그림자 |
| shadcn/ui 컴포넌트 | `frontend/src/components/ui/` | 20+ 컴포넌트 (New York 스타일) |
| 블록 컴포넌트 | `frontend/src/components/blocks/` | HeroSection, SectionContainer, CoverCard 등 |
| Figma 디자인 파일 | 외부 (Figma) | 프레임 구조 정의됨 |
| 다국어 타이포그래피 | `utils/language_groups.ts` + CSS | CJK/Tall Script/Relaxed Tracking 분류 |

### 3.2 DESIGN.md 표준 대비 갭 분석

현재 `AMK_DESIGN_SYSTEM.md`를 DESIGN.md 9개 섹션 기준으로 비교:

| DESIGN.md 섹션 | AMK 현재 상태 | 갭 |
|----------------|-------------|-----|
| 1. Visual Theme & Atmosphere | ❌ **없음** | 디자인 철학/분위기 서술 없음 |
| 2. Color Palette & Roles | ✅ **있음** | HSL 값 + 역할 정의 완비. hex 값 병기 필요 |
| 3. Typography Rules | ✅ **있음** | 계층 테이블 존재. 자간(letter-spacing) 수치 상세화 필요 |
| 4. Component Stylings | ✅ **있음** | variant 정의 완비. CSS 수치(px) 명시 강화 필요 |
| 5. Layout Principles | ✅ **있음** | 스페이싱/그리드/컨테이너 정의 완비 |
| 6. Depth & Elevation | ⚠️ **부분** | Shadow Scale 존재하나 레이어 상세 없음 |
| 7. Do's and Don'ts | ⚠️ **산재** | 규칙이 각 섹션에 흩어져 있음. 통합 섹션 필요 |
| 8. Responsive Behavior | ⚠️ **부분** | 다국어 반응형 있으나 브레이크포인트 테이블 없음 |
| 9. Agent Prompt Guide | ❌ **없음** | AI 에이전트용 퀵 레퍼런스/프롬프트 없음 |

---

## 4. 적용 방안

### 방안 1: DESIGN.md 신규 생성 (권장)

프로젝트 루트에 `DESIGN.md` 파일을 생성하여 AI 에이전트가 자동으로 참조하도록 한다.

**대상**: Claude Code, Cursor, GitHub Copilot, Google Stitch 등 모든 AI 에이전트

**구현 방법**:
- 기존 `AMK_DESIGN_SYSTEM.md`의 토큰/컴포넌트 정보를 DESIGN.md 9개 섹션 형식으로 재구성
- `AMK_DESIGN_SYSTEM.md`는 개발자용 상세 문서로 유지 (코드 예시, Props 테이블 등)
- `DESIGN.md`는 AI 에이전트 최적화 문서로 별도 관리

**역할 분리**:

| 문서 | 대상 | 내용 |
|------|------|------|
| `DESIGN.md` | AI 에이전트 | 시각적 토큰, CSS 수치, Do/Don't, Agent Prompt Guide |
| `AMK_DESIGN_SYSTEM.md` | 개발자 | 컴포넌트 Props, 사용 예시, 코드 패턴, 아키텍처 |

**예상 효과**:
- 새 페이지 개발 시 "DESIGN.md 따라서 만들어줘" 한 줄로 일관된 UI 생성
- 모바일(Flutter)/데스크탑(Tauri) 확장 시 동일 DESIGN.md로 크로스플랫폼 디자인 일관성
- 디자인 리뷰 자동화: AI가 DESIGN.md 대비 코드의 디자인 준수 여부 검증 가능

### 방안 2: Visual Theme & Atmosphere 섹션 추가

현재 디자인 시스템에 가장 부재한 것은 **디자인 철학 서술**이다.

**Amazing Korean이 정의해야 할 항목들**:

```markdown
## Visual Theme & Atmosphere

### Design Philosophy
- 학습자가 콘텐츠에 집중할 수 있는 **깨끗하고 따뜻한** 인터페이스
- 한국어 학습의 문화적 깊이를 반영하는 **품격 있는** 톤
- 정보 과부하 없이 단계적으로 안내하는 **점진적 공개**

### Key Characteristics
- Warm & Trustworthy: 네이비 + 시안 + 따뜻한 그라데이션
- Content-First: 학습 콘텐츠가 시각적 장식보다 우선
- Culturally Respectful: 한국 문화의 절제미를 반영한 여백 활용
- Accessible: WCAG AA 준수, 22개 언어 타이포그래피 대응
```

### 방안 3: Agent Prompt Guide 섹션 추가

AI 에이전트가 즉시 활용할 수 있는 퀵 레퍼런스를 추가한다.

**구성 요소**:

1. **Quick Color Reference** — 핵심 색상 key/value 목록
2. **Example Component Prompts** — 복사-붙여넣기 가능한 3~5개 프롬프트
3. **Iteration Guide** — 번호 매긴 규칙 목록

**예시**:

```markdown
## Agent Prompt Guide

### Quick Color Reference
- Primary (Navy): hsl(222, 90%, 18%) — 메인 액션, 네비게이션
- Secondary (Blue): hsl(224, 81%, 61%) — 보조 버튼, 서브 UI
- Accent (Cyan): hsl(197, 84%, 46%) — 강조 포인트, 아이콘
- Background: hsl(0, 0%, 100%) — 기본 배경
- Text: hsl(222, 47%, 11%) — 기본 텍스트 (near-black)

### Example Component Prompts
- "Pretendard Variable 폰트, text-4xl font-bold tracking-tight으로
  Hero 타이틀을 만들어. bg-hero-gradient 배경 위에 text-center 정렬."
- "rounded-xl shadow-card 카드에 코스 정보를 표시해.
  상단에 커버 이미지, 하단에 제목(text-lg font-semibold)과
  Badge variant=warning으로 유료 표시."

### Iteration Guide
1. 모든 색상은 CSS 변수(HSL)로만 참조 — HEX 하드코딩 금지
2. 컴포넌트는 shadcn/ui 기반 — 커스텀 컴포넌트 전에 기존 ui/ 확인
3. 반응형은 mobile-first — sm → md → lg → xl 순서
4. 다크모드 항상 고려 — bg-primary 대신 bg-surface-inverted (고정 어두운 배경)
5. 카드 variant 자동 radius — default=xl, elevated/interactive=2xl
```

### 방안 4: Do's and Don'ts 통합 섹션

현재 각 섹션에 산재된 규칙들을 하나의 섹션으로 통합한다.

**예시 구조**:

```markdown
## Do's and Don'ts

### Do
- ✅ CSS 변수로만 색상 참조 (HEX 직접 사용 금지)
- ✅ 컨테이너 토큰 사용 (max-w-container-default, not max-w-[1350px])
- ✅ Status Badge는 variant prop 사용 (bg-status-* 직접 적용 금지)
- ✅ 다크모드에서 고정 어두운 배경: bg-footer, bg-surface-inverted
- ✅ img 태그에 loading="lazy" 필수
- ✅ 터치 타겟 최소 44x44px (@media (pointer: coarse))
- ✅ CJK 텍스트에 word-break: keep-all 적용

### Don't
- ❌ Tailwind 기본색 직접 사용 (bg-slate-*, text-gray-* 등)
- ❌ rounded-sm 직접 사용 (shadcn 내부 프리미티브 전용)
- ❌ bg-primary를 고정 어두운 배경에 사용 (다크모드에서 반전됨)
- ❌ font-bold를 본문에 사용 (제목/KPI 전용)
- ❌ 40px 미만 여백을 SectionContainer 레벨에서 처리
- ❌ whitespace-nowrap을 다국어 타이틀에 적용 (장문 언어 넘침)
```

### 방안 5: Depth & Elevation 상세화

현재 Shadow Scale을 awesome-design-md 수준으로 상세화한다.

**현재**: 토큰명 + 용도만 기술
**개선**: 각 레벨의 CSS 값, 레이어 구성, 다크모드 동작까지 명시

```markdown
## Depth & Elevation

### Shadow Levels
| Level | CSS Value | Purpose |
|-------|-----------|---------|
| Flat | none | 기본 텍스트, 플랫 요소 |
| Resting | shadow-sm | 폼 컨트롤 (Input, Select) |
| Card | shadow + border | Card default (보더 + 미세 그림자) |
| Elevated | shadow-card (0 1px 3px primary/12%, 0 4px 12px primary/8%) | Card elevated |
| Hover | shadow-card-hover (0 2px 6px primary/18%, 0 8px 24px primary/12%) | Card interactive hover |
| Overlay | shadow-lg | Dropdown, Toast, Dialog |
| CTA | shadow-xl | CTA 버튼 hover |

### Dark Mode Shadow Strategy
- Light: primary 기반 (브랜드 느낌)
- Dark: 순수 검정 기반 (white glow 방지)
- 절대로 다크모드에서 밝은 색상 그림자를 사용하지 않는다
```

### 방안 6: Responsive Behavior 체계화

현재 다국어 반응형은 있으나, 기본 반응형 브레이크포인트 테이블이 없다.

```markdown
## Responsive Behavior

### Breakpoints
| Name | Width | Tailwind | Primary Change |
|------|-------|----------|---------------|
| Mobile | <640px | (default) | 1열 레이아웃, 축소된 패딩 |
| Tablet | ≥640px | sm: | 2열 그리드 시작 |
| Desktop | ≥768px | md: | Hero 타이포 확대, 네비게이션 전환 |
| Wide | ≥1024px | lg: | 3열 그리드, 사이드바 노출 |
| Ultra | ≥1280px | xl: | 4열 그리드 (관리자) |

### Touch Targets
- 최소 44x44px (WCAG 2.5.8)
- `@media (pointer: coarse)` 감지로 터치 디바이스 대응
- iOS: 인풋 font-size 16px (auto-zoom 방지)

### Collapsing Strategy
- Header: lg 이하에서 햄버거 메뉴로 축소
- Footer: 4열 → 2열 → 1열 순서로 축소
- Card Grid: xl=4열, lg=3열, sm=2열, default=1열
- Hero: 텍스트 사이즈 단계적 축소 (6xl → 5xl → 4xl → 3xl)
```

### 방안 7: 크로스플랫폼 디자인 일관성 (모바일/데스크탑)

DESIGN.md의 가장 큰 장점은 **플랫폼 독립적**이라는 것이다.

| 플랫폼 | 기술 스택 | DESIGN.md 활용 방식 |
|--------|----------|-------------------|
| 웹 (현재) | React + Tailwind + shadcn/ui | Tailwind 토큰으로 직접 매핑 |
| 모바일 (예정) | Flutter | Material Design 3 테마에 DESIGN.md 색상/타이포 매핑 |
| 데스크탑 (예정) | Tauri 2.x + 웹뷰 | 웹과 동일한 CSS 토큰 재사용 |

DESIGN.md에 플랫폼별 매핑 가이드를 포함하면:
- Flutter 개발 시 "DESIGN.md의 Primary Navy를 Material ColorScheme.primary에 매핑"
- Tauri 개발 시 동일한 CSS 변수 재사용

---

## 5. 구현 로드맵

### Phase 1: DESIGN.md 기본 생성 (즉시)

| 작업 | 소스 | 예상 |
|------|------|------|
| Visual Theme & Atmosphere 작성 | 신규 (브랜드 철학 정의) | 신규 작성 |
| Color Palette & Roles | `AMK_DESIGN_SYSTEM.md` §01 + `index.css` | 형식 변환 |
| Typography Rules | `AMK_DESIGN_SYSTEM.md` §01 Typography | 형식 변환 + 자간 수치 보완 |
| Component Stylings | `AMK_DESIGN_SYSTEM.md` §03 | 형식 변환 + CSS 수치 추가 |
| Layout Principles | `AMK_DESIGN_SYSTEM.md` §02 | 형식 변환 |

### Phase 2: 고급 섹션 추가

| 작업 | 소스 |
|------|------|
| Depth & Elevation 상세화 | `index.css` shadow 정의 분석 |
| Do's and Don'ts 통합 | `AMK_DESIGN_SYSTEM.md` 전체에서 규칙 수집 |
| Responsive Behavior | `tailwind.config.js` + 컴포넌트 분석 |
| Agent Prompt Guide | 신규 작성 |

### Phase 3: 크로스플랫폼 확장

| 작업 | 시점 |
|------|------|
| Flutter 테마 매핑 가이드 | 모바일 앱 개발 착수 시 |
| Tauri 토큰 매핑 가이드 | 데스크탑 앱 개발 착수 시 |
| preview.html 생성 | DESIGN.md 완성 후 검증용 |

---

## 6. 참고할 디자인 기법 요약

awesome-design-md에서 발견된 기법 중 Amazing Korean에 적용 가능한 것들:

### 6.1 즉시 적용 가능

| 기법 | 출처 | Amazing Korean 적용 |
|------|------|-------------------|
| **다층 그림자 스택** | 전체 | 현재 `shadow-card` 2레이어 → 3레이어로 확장 검토 |
| **Do/Don't 통합 섹션** | 전체 | 산재된 규칙을 한 곳으로 |
| **Agent Prompt Guide** | 전체 | 신규 추가 (AI 활용 효율 극대화) |
| **Near-black 텍스트** | 전체 | ✅ 이미 적용 (`hsl(222, 47%, 11%)`) |
| **음수 자간** | 전체 | ✅ 이미 적용 (`tracking-tight`) |

### 6.2 검토 후 적용 가능

| 기법 | 출처 | 검토 사항 |
|------|------|----------|
| **Shadow-as-border** | Vercel | 현재 CSS border 사용 중. 전환 시 전체 컴포넌트 수정 필요 |
| **Luminance stepping** (다크모드) | Linear | 현재 그림자 기반. 다크모드 품질 향상 가능 |
| **가변 폰트 미세 웨이트** | Figma | Pretendard Variable이 지원하는 웨이트 범위 확인 필요 |
| **Brand-tinted shadows** | Stripe | 현재 라이트모드에서 primary 기반 그림자 사용 중. 더 강화 가능 |
| **OpenType 피처 활용** | 전체 | Pretendard Variable의 지원 OT 피처 조사 필요 |

### 6.3 장기 참고

| 기법 | 출처 | 메모 |
|------|------|------|
| **61개 브레이크포인트** | Airbnb | 현재 5단계로 충분. 향후 세밀한 반응형 필요 시 참고 |
| **흑백 인터페이스** | Figma | 교육 앱과 맞지 않음 (따뜻함 필요). 참고만 |
| **Weight 300 헤드라인** | Stripe | 브랜드 차별화로는 흥미하나 한글 가독성 검증 필요 |

---

## 7. 결론

### DESIGN.md 도입의 핵심 가치

1. **AI 에이전트 생산성**: 새 페이지/컴포넌트 개발 시 일관된 디자인 자동 적용
2. **크로스플랫폼 SSoT**: 웹/모바일/데스크탑 모두 하나의 디자인 명세 참조
3. **디자인 리뷰 자동화**: AI가 DESIGN.md 대비 디자인 준수 여부 검증
4. **온보딩 효율**: 새 AI 에이전트/도구 도입 시 즉시 디자인 컨텍스트 전달

### 권장 우선순위

1. **즉시**: `DESIGN.md` 파일 생성 (9개 섹션 중 5개는 기존 문서에서 변환)
2. **단기**: Agent Prompt Guide + Do's/Don'ts 통합 섹션 추가
3. **중기**: preview.html 생성, 크로스플랫폼 매핑 가이드
4. **장기**: 디자인 준수 자동 검증 파이프라인 구축
