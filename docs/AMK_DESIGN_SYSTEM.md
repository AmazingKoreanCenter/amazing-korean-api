# Amazing Korean Design System

> tailwind.config.js + index.css = Single Source of Truth.
> 이 문서는 코드에 정의된 토큰/컴포넌트의 **사용법과 금지 규칙**을 설명한다.

---

## 01 Foundations

### Color Tokens

| 역할 | CSS 변수 | Tailwind 클래스 | 용도 |
|------|----------|----------------|------|
| Primary | `--primary` | `bg-primary`, `text-primary` | 메인 액션, 네비게이션, Footer |
| Secondary | `--secondary` | `bg-secondary` | 보조 버튼, 서브 UI |
| Accent | `--accent` | `bg-accent`, `text-accent` | 강조 포인트, 아이콘 |
| Destructive | `--destructive` | `bg-destructive` | 삭제, 에러 (= error) |
| Brand Soft | `--brand-soft` | `bg-brand-soft` | Hero 그라데이션 시작색 |
| Brand Soft Alt | `--brand-soft-alt` | `bg-brand-soft-alt` | Hero 그라데이션 끝색 |
| Success | `--success` | `bg-status-success` | 완료, 활성, 프로모션 |
| Warning | `--warning` | `bg-status-warning` | 유료 콘텐츠, 주의 |
| Info | `--info` | `bg-status-info` | 정보 알림 |

**error = destructive**: 별도 error 토큰 없음. 기존 `destructive`를 에러 용도로 통일 사용.

- 삭제 버튼 등 "강한 fill": `bg-destructive text-destructive-foreground`
- 에러 메시지/배너 "부드러운 surface": `bg-destructive/10 text-destructive border-destructive/20`

**WCAG AA 준수**: 모든 status 색상은 foreground 텍스트와 4.5:1 이상 명암비 확보.

| 토큰 | 라이트 HSL | 다크 HSL | Foreground |
|------|-----------|----------|------------|
| success | `160 84% 28%` | `160 84% 36%` | white |
| warning | `38 92% 50%` | `38 92% 55%` | dark (`20 14% 4%`) |
| info | `217 91% 53%` | `217 91% 58%` | white |
| destructive | `0 84% 50%` | `0 63% 31%` | white |

### Status Color 사용 패턴

```tsx
// Badge (불투명 배경)
<Badge variant="success">완료</Badge>
<Badge variant="warning">유료</Badge>
<Badge variant="info">안내</Badge>

// Badge (투명도 기반 — 인라인 상태 표시)
<Badge className="bg-status-success/10 text-status-success border-0">완료</Badge>

// 아이콘 색상
<CheckCircle2 className="text-status-success" />
<Crown className="text-status-warning" />

// 배경 상태 표시 (Banner 등)
<div className="bg-status-success/10 border-status-success/20">...</div>
<div className="bg-destructive/10 border-destructive/20">...</div>
```

### Gradients

| 이름 | 클래스 | 용도 | 구현 |
|------|--------|------|------|
| Hero Gradient | `bg-hero-gradient` | Hero 섹션 배경 | `from-brand-soft via-background to-brand-soft-alt` |
| Primary Gradient | `gradient-primary` | 버튼, 아이콘 배경 | `hsl(--secondary) → hsl(--accent)` |
| Text Gradient | `text-gradient` | 브랜드 텍스트 강조 | `hsl(--primary) → hsl(--accent)` |

**모든 gradient/shadow는 CSS 변수만 참조** — 직접 HEX 값 사용 금지.

### Section Spacing Scale

| 토큰 | 값 | Tailwind | 용도 |
|------|-----|---------|------|
| section-sm | 2.5rem (40px) | `py-section-sm` | 서브 섹션, 리스트 콘텐츠 |
| section-md | 4rem (64px) | `py-section-md` | 일반 섹션 기본값 (**고정, 반응형 없음**) |
| section-lg | 5rem (80px) | `py-section-lg` | 주요 섹션, Feature 블록 |
| hero-lg | 8rem (128px) | `py-hero-lg` | Hero 전용 (HeroSection 내부) |

**SectionContainer 반응형 규칙**:
- `sm`: `py-section-sm` (고정)
- `md`: `py-section-md` (고정 — 의도 없는 확대 방지)
- `lg`: `py-section-lg lg:py-hero-lg` (Hero/CTA용, 데스크탑에서 확대)

**40px 미만 여백**: SectionContainer 레벨이 아니라 내부 컴포넌트에서 `gap-4`, `my-6` 등으로 처리.

### Typography

- **Font**: Pretendard Variable (한글 최적화)
- **한글 줄바꿈**: `break-keep` 적용 (단어 단위 줄바꿈)
- **Tracking**: 제목에 `tracking-tight` 적용

---

## 02 Layout

### SectionContainer

`components/sections/section_container.tsx` — 섹션 래퍼. padding + container 동시 제공.

```tsx
import { SectionContainer } from "@/components/sections/section_container";

// 기본 사용 (64px 고정 패딩)
<SectionContainer>내용</SectionContainer>

// 크기 변경
<SectionContainer size="sm">서브 섹션</SectionContainer>
<SectionContainer size="lg">메인 섹션 (데스크탑에서 128px)</SectionContainer>

// 배경색 + 좁은 컨테이너 (Legal 페이지)
<SectionContainer container="narrow" className="bg-muted/30">
  법적 고지
</SectionContainer>

// HTML 태그 변경
<SectionContainer as="div">div로 렌더링</SectionContainer>
```

**Props**:
| Prop | 타입 | 기본값 | 설명 |
|------|------|--------|------|
| size | `'sm' \| 'md' \| 'lg'` | `'md'` | 섹션 패딩 |
| container | `'default' \| 'narrow'` | `'default'` | max-w-[1350px] or max-w-3xl |
| as | `React.ElementType` | `'section'` | HTML 태그 (SEO 시맨틱) |
| className | `string` | — | 추가 클래스 (bg 등) |

### Page Templates

- **목록 페이지**: `bg-hero-gradient` 헤더 + `SectionContainer` 콘텐츠
- **상세 페이지**: `bg-hero-gradient` 헤더 + 내용 영역
- **폼 페이지**: 중앙 정렬 Card + max-w-md

---

## 03 Components

### HeroSection

`components/sections/hero_section.tsx` — 메인 페이지 Hero 블록.

```tsx
import { HeroSection } from "@/components/sections/hero_section";

<HeroSection
  badge={<><Sparkles className="h-4 w-4 text-accent" /><span>Badge Text</span></>}
  title="메인 타이틀"
  subtitle="부제목 설명 텍스트"
  size="sm"  // default | sm
>
  {/* CTA 버튼 등 */}
</HeroSection>
```

**Props**:
| Prop | 타입 | 기본값 | 설명 |
|------|------|--------|------|
| badge | `ReactNode` | — | 상단 배지 (아이콘 + 텍스트) |
| title | `ReactNode` | 필수 | 메인 타이틀 |
| subtitle | `ReactNode` | — | 부제목 |
| size | `'default' \| 'sm'` | `'default'` | Hero 크기 |
| children | `ReactNode` | — | CTA, 신뢰 지표 등 |

### Badge Variants

기존 shadcn Badge에 status 확장:

| Variant | 클래스 | 용도 |
|---------|--------|------|
| `default` | bg-primary | 기본 |
| `secondary` | bg-secondary | 보조 |
| `destructive` | bg-destructive | 삭제/에러 |
| `success` | bg-status-success | 완료/활성 |
| `warning` | bg-status-warning | 유료/주의 |
| `info` | bg-status-info | 정보 |
| `outline` | border only | 아웃라인 |

---

## 04 Mobile Checklist

- [ ] Touch target 최소 44px (버튼 `h-11` 이상)
- [ ] Hover-only 인터랙션 금지 (hover와 함께 항상 click/tap도 제공)
- [ ] Input 16px+ (iOS zoom 방지 — `@supports (-webkit-touch-callout: none)` 자동 적용)
- [ ] `break-keep` 한글 줄바꿈 최적화

---

## 05 Anti-Patterns (금지 규칙)

### 하드코딩 색상 (모든 유틸리티 프리픽스)

```
❌ from-[#F0F3FF]          → ✅ from-brand-soft
❌ to-[#E8F4FF]            → ✅ to-brand-soft-alt
❌ bg-[#051D55]            → ✅ bg-primary
❌ bg-[#129DD8]            → ✅ bg-accent
❌ bg-[#4F71EB]            → ✅ bg-secondary
❌ text-[#051D55]          → ✅ text-primary
❌ border-[#129DD8]        → ✅ border-accent
```

### 직접 상태 색상

```
❌ bg-emerald-500          → ✅ bg-status-success
❌ bg-green-500            → ✅ bg-status-success
❌ text-green-600          → ✅ text-status-success
❌ border-green-500        → ✅ border-status-success
❌ bg-amber-500            → ✅ bg-status-warning
❌ bg-red-500 (상태용)     → ✅ bg-destructive
❌ text-red-600            → ✅ text-destructive
❌ text-white (상태 Badge) → ✅ text-status-success-foreground
```

### 매직넘버 여백

```
❌ py-32                   → ✅ py-hero-lg
❌ py-20 lg:py-28          → ✅ py-section-lg lg:py-hero-lg
❌ py-16 lg:py-24          → ✅ SectionContainer size="lg"
❌ py-10 lg:py-14          → ✅ py-section-sm
```

---

## 06 Enforcement

### lint:ui 스크립트

```bash
cd frontend && npm run lint:ui
```

**탐지 범위** (모든 `.tsx` 파일):
- 임의 HEX: `(bg|text|border|ring|from|via|to|stroke|fill)-[#...]`
- 금지 팔레트: `(bg|text|border|ring|from|via|to|stroke|fill)-(emerald|teal|green|amber|rose|red)-*`

위반 시 exit 1 반환 → CI/PR 체크에서 차단 가능.

**현재 상태**: .tsx 파일 전체 0건 위반 (Admin 포함).

### PR 체크리스트

새 코드 작성 시 확인:

- [ ] 색상: semantic 토큰 사용 (직접 HEX/Tailwind 색상 금지)
- [ ] 상태: `variant` prop 사용 (`<Badge variant="success">`)
- [ ] 여백: section spacing 토큰 사용
- [ ] Hero: `bg-hero-gradient` 또는 `<HeroSection>` 사용
- [ ] `npm run lint:ui` 통과

---

## 07 Roadmap (보류 항목)

전문가 리뷰에서 "현재는 불필요, 조건 충족 시 진행" 판정된 항목들.

### Milestone A — 모바일 앱 개발 시

| 항목 | 작업 | 파일 |
|------|------|------|
| Safe-area 유틸 | `pt-safe`/`pb-safe` CSS 유틸 추가, `root_layout`/`footer`에 적용 | `index.css`, `root_layout.tsx`, `footer.tsx` |
| break-keep 조건부 | 한국어에만 `break-keep` 적용 (영어 긴 단어 오버플로우 방지) | `hero_section.tsx` 또는 `index.css` |

### Milestone B — 팀 확장 시 (개발자 3명+)

| 항목 | 작업 | 파일 |
|------|------|------|
| ESLint 커스텀 룰 | `eslint-plugin-tailwindcss` 도입, 금지 패턴 룰 정의, CI 통합 | `eslint.config.js`, `deploy.yml` |
| 디렉토리 재구조화 | `sections/` → `layout/` + `marketing/` 분리 | `section_container.tsx`, `hero_section.tsx` + import 수정 |

### Milestone C — 디자인 고도화 / 다크모드 런칭 시

| 항목 | 작업 | 파일 |
|------|------|------|
| Hero alias 토큰 | `--hero-from`/`--hero-to` alias 추가 (brand-soft 다용도 확장 대비) | `index.css`, `tailwind.config.js` |
| Radius 스케일 문서화 | `rounded-md`/`lg`/`xl`/`2xl`/`full` 용도별 티어 정의 | 이 문서 §01에 추가 |
| 다크모드 QA | Hero gradient 평면감 보정, Badge/chart 색상 재조정 | `index.css`, `hero_section.tsx` |

### On-Demand — 필요 시 즉시

| 항목 | 작업 | 파일 |
|------|------|------|
| `SectionContainer size="none"` | sizeMap에 `none: ""` 추가 (이중 패딩 방지) | `section_container.tsx` |
| `HeroSection layout="split"` | 좌우 분할 레이아웃 variant 추가 | `hero_section.tsx` |
