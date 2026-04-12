# Amazing Korean Design System

> tailwind.config.js + index.css = Single Source of Truth.
> 이 문서는 코드에 정의된 토큰/컴포넌트의 **사용법과 금지 규칙**을 설명한다.
>
> **v4 변경 (2026-03-31):**
> - 3계층 아키텍처: `components/ui/` (Layer 1) → `components/blocks/` (Layer 2) → `category/` (Layer 3)
> - 삭제: `--table-header` CSS 변수, Badge `info` variant, Button `link` variant
> - 추가: `max-w-container-default`(1350px), `max-w-container-narrow`(768px), `max-w-container-form`(448px)
> - 추가: 글로벌 `prefers-reduced-motion` CSS 리셋 (접근성)
> - `max-w-[1350px]` 하드코딩 금지 → `max-w-container-default` 사용
> - 추가: `components/layouts/auth_layout.tsx` (인증 페이지 공통 래퍼)
> - 추가: `components/blocks/cover_card.tsx` (카탈로그 표지 카드)
> - 추가: `components/blocks/feature_grid.tsx` (아이콘+제목+설명 카드 그리드)
> - 전체 `<img>` 태그 `loading="lazy"` 적용 완료
> - V1-9: Tailwind 기본색 → 디자인 시스템 토큰 교체 (7파일, status badge/surface-inverted/coming-soon)
> - V1-9: `text-white` → `text-surface-inverted-foreground` (ebook_viewer fullscreen)
> - 모바일 UX 79건 수정: `@media (pointer: coarse)` 터치 타겟 44px, 고정 그리드 반응형, 모달 뷰포트 제한, 패딩/간격 반응형, header 스크롤 잠금
>
> **v4.1 변경 (2026-04-02) — 다국어 반응형 디자인 규격:**
> - 언어 그룹 유틸리티 (`utils/language_groups.ts`): CJK / Tall Script / Relaxed Tracking 분류
> - `changeLanguage` 시 `<html>`에 `lang-cjk`, `lang-tall-script`, `lang-relaxed-tracking` CSS 클래스 동적 관리
> - `tracking-tight` 조건부 해제: `.lang-relaxed-tracking h1~h6 { letter-spacing: 0 }` (th, my, km, si, hi, ne, mn)
> - Tall script line-height 보정: `.lang-tall-script { line-height: 1.8 }` (th, my, km)
> - `break-keep` → `break-keep-cjk` 전환 (4곳): CJK에서만 `word-break: keep-all` 적용
> - 히어로 제목 `whitespace-nowrap` 제거: 장문 언어(de, ru, pt 등) 컨테이너 넘침 방지

---

## 00 Visual Theme & Atmosphere

### Design Philosophy

학습자가 콘텐츠에 집중할 수 있는 **깨끗하고 따뜻한** 인터페이스. 한국어 학습의 문화적 깊이를 반영하는 **품격 있는** 톤. 정보 과부하 없이 단계적으로 안내하는 **점진적 공개(Progressive Disclosure)**.

### Key Characteristics

- **Warm & Trustworthy**: Dark Navy(`#051D57`) + Cyan(`#13A0D8`) + 따뜻한 gradient로 신뢰감과 친근함을 동시에 전달
- **Content-First**: 학습 콘텐츠가 시각적 장식보다 우선. 여백과 타이포그래피로 정보 계층 표현
- **Culturally Respectful**: 한국 문화의 절제미를 반영한 여백 활용. 22개 언어 타이포그래피 대응
- **Accessible**: WCAG AA 준수 (4.5:1 명암비), 터치 타겟 44px, `prefers-reduced-motion` 지원
- **Dual-Mode Native**: 라이트/다크 모드 완전 대응. CSS 변수 이중 정의로 자동 전환

### Visual Density

중간 밀도. 마케팅 페이지는 넉넉한 여백(80~128px 섹션), 학습/관리 페이지는 컴팩트한 여백(40~64px 섹션). 카드 그리드는 `gap-6`(24px) 표준.

---

## 01 Foundations

### Color Tokens

#### Core Palette

| 역할 | CSS 변수 | 라이트 HSL | Hex | 다크 HSL | Hex | 용도 |
|------|----------|-----------|-----|---------|-----|------|
| Background | `--background` | `0 0% 100%` | `#FFFFFF` | `222 47% 5%` | `#070A13` | 기본 배경 |
| Foreground | `--foreground` | `222 47% 11%` | `#0F1729` | `210 40% 98%` | `#F8FAFC` | 기본 텍스트 (near-black) |
| Primary | `--primary` | `222 90% 18%` | `#051D57` | `210 40% 98%` | `#F8FAFC` | 메인 액션, 네비게이션 |
| Secondary | `--secondary` | `224 81% 61%` | `#4B76EC` | `224 81% 61%` | `#4B76EC` | 보조 버튼, 서브 UI |
| Accent | `--accent` | `197 84% 46%` | `#13A0D8` | `197 84% 46%` | `#13A0D8` | 강조 포인트, 아이콘 |
| Muted | `--muted` | `228 33% 97%` | `#F5F6FA` | `217 33% 17%` | `#1D283A` | 비활성 배경 |
| Muted FG | `--muted-foreground` | `215 16% 47%` | `#65758B` | `215 20% 65%` | `#94A3B8` | 보조 텍스트 |
| Destructive | `--destructive` | `0 84% 50%` | `#EB1414` | `0 63% 31%` | `#811D1D` | 삭제, 에러 |

**`--primary` 다크모드 반전**: 라이트에서 Dark Navy → 다크에서 Near-White. shadcn 표준 패턴. `bg-primary`를 "항상 어두운 배경"으로 사용하면 안 됨 → Surface 토큰 사용.

#### Brand & Gradient

| 역할 | CSS 변수 | 라이트 HSL | Hex | 다크 HSL | Hex | 용도 |
|------|----------|-----------|-----|---------|-----|------|
| Brand Soft | `--brand-soft` | `230 100% 97%` | `#F0F2FF` | `222 47% 10%` | `#0E1525` | Hero 그라데이션 시작 |
| Brand Soft Alt | `--brand-soft-alt` | `206 100% 96%` | `#EBF6FF` | `222 47% 12%` | `#10192D` | Hero 그라데이션 끝 |

#### Status Colors (WCAG AA 4.5:1+)

| 토큰 | 라이트 HSL | Hex | 다크 HSL | Hex | Foreground |
|------|-----------|-----|---------|-----|------------|
| success | `160 84% 28%` | `#0B835B` | `160 84% 36%` | `#0FA976` | white |
| warning | `38 92% 50%` | `#F59F0A` | `38 92% 55%` | `#F6A823` | dark (`20 14% 4%`) |
| info | `217 91% 53%` | `#1A6EF4` | `217 91% 58%` | `#327DF5` | white |
| destructive | `0 84% 50%` | `#EB1414` | `0 63% 31%` | `#811D1D` | white |

**error = destructive**: 별도 error 토큰 없음. 기존 `destructive`를 에러 용도로 통일 사용.

- 삭제 버튼 등 "강한 fill": `bg-destructive text-destructive-foreground`
- 에러 메시지/배너 "부드러운 surface": `bg-destructive/10 text-destructive border-destructive/20`

#### Surface Tokens (항상 어두운 배경)

Footer, CTA 섹션 등 항상 어두운 배경이 필요한 곳에 사용. `bg-primary`는 다크모드에서 반전되므로 직접 사용 금지.

| 토큰 | 라이트 HSL | Hex | 다크 HSL | Hex | 용도 |
|------|-----------|-----|---------|-----|------|
| footer | `222 90% 18%` | `#051D57` | `222 47% 8%` | `#0B111E` | Footer 배경 |
| footer-fg | `210 40% 98%` | `#F8FAFC` | `210 40% 98%` | `#F8FAFC` | Footer 텍스트 |
| surface-inverted | `222 90% 18%` | `#051D57` | `222 47% 10%` | `#0E1525` | CTA, fullscreen 뷰어 배경 |
| surface-inverted-fg | `210 40% 98%` | `#F8FAFC` | `210 40% 98%` | `#F8FAFC` | CTA, fullscreen 뷰어 텍스트 |

#### UI Chrome Tokens

| 토큰 | 라이트 HSL | Hex | 다크 HSL | Hex | 용도 |
|------|-----------|-----|---------|-----|------|
| border | `214 32% 91%` | `#E1E7EF` | `217 33% 17%` | `#1D283A` | 모든 border 기본값 |
| input | `214 32% 91%` | `#E1E7EF` | `217 33% 17%` | `#1D283A` | 폼 input border |
| ring | `222 90% 18%` | `#051D57` | `212 27% 84%` | `#CBD5E1` | Focus ring |
| card | `0 0% 100%` | `#FFFFFF` | `222 47% 8%` | `#0B111E` | 카드 배경 |
| popover | `0 0% 100%` | `#FFFFFF` | `222 47% 8%` | `#0B111E` | 팝오버 배경 |

#### Badge Colors (Enum 표시용, 테마 독립)

| 토큰 | 라이트 HSL | Hex | 다크 HSL | Hex |
|------|-----------|-----|---------|-----|
| badge-blue | `224 76% 48%` | `#1D4FD7` | `224 76% 55%` | `#3564E3` |
| badge-orange | `25 95% 53%` | `#F97415` | `25 95% 55%` | `#F97A1F` |
| badge-purple | `270 60% 55%` | `#8C47D1` | `270 60% 60%` | `#995CD6` |
| badge-yellow | `48 96% 50%` | `#FAC905` | `48 96% 53%` | `#FACC14` |
| badge-sky | `199 89% 48%` | `#0DA2E7` | `199 89% 55%` | `#26B2F2` |
| badge-indigo | `245 58% 50%` | `#4236C9` | `245 58% 58%` | `#6056D2` |

모든 badge foreground: white (`0 0% 100%`). 예외: badge-yellow-foreground = dark (`20 14% 4%`).

#### Chart Colors

| 토큰 | 라이트 HSL | Hex | 다크 HSL | Hex | 역할 |
|------|-----------|-----|---------|-----|------|
| chart-1 | `222 90% 18%` | `#051D57` | `220 70% 50%` | `#2662D9` | Primary |
| chart-2 | `197 84% 46%` | `#13A0D8` | `160 60% 45%` | `#2EB88A` | Accent |
| chart-3 | `224 81% 61%` | `#4B76EC` | `30 80% 55%` | `#E88C30` | Secondary |
| chart-4 | `43 74% 66%` | `#E8C468` | `280 65% 60%` | `#AF57DB` | Warm |
| chart-5 | `340 75% 55%` | `#E23670` | `340 75% 55%` | `#E23670` | Pink |
| chart-6 | `280 65% 60%` | `#AF57DB` | `280 65% 65%` | `#B96CE0` | Purple |

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
| Text Gradient | `text-gradient` | 브랜드 텍스트 강조 | 라이트: `--primary → --accent`, 다크: `--secondary → --accent` |

**모든 gradient/shadow는 CSS 변수만 참조** — 직접 HEX 값 사용 금지.

### Radius Scale

| 티어 | 클래스 | 값 (--radius: 0.625rem = 10px) | 용도 |
|------|--------|------|------|
| Micro | `rounded-sm` | 6px (`--radius - 4px`) | shadcn 내부 프리미티브 (직접 사용 금지) |
| Base | `rounded-md` | 8px (`--radius - 2px`) | 폼 컨트롤 (Input, Button, Select) |
| Panel | `rounded-lg` | 10px (`--radius`) | 패널, 통계 카드, Alert, Dialog |
| Card | `rounded-xl` | 12px | Card default variant |
| Surface | `rounded-2xl` | 16px | Card elevated/interactive, 마케팅 섹션 |
| Pill | `rounded-full` | 9999px | Badge, Avatar, CTA 버튼 |

**Radius Decision Tree**:
```
버튼/인풋인가? → rounded-md
카드인가? → variant에 따라 자동 (default=xl, elevated/interactive=2xl)
Dialog/Alert인가? → rounded-lg
배지/아바타인가? → rounded-full
마케팅 피처 카드인가? → rounded-2xl
```

### Shadow Scale

| 토큰 | CSS 값 (라이트) | CSS 값 (다크) | 용도 |
|------|----------------|--------------|------|
| `shadow-sm` | Tailwind 기본 | Tailwind 기본 | 폼 컨트롤 (resting) |
| `shadow` | Tailwind 기본 | Tailwind 기본 | Card default |
| `shadow-card` | `0 0 26px -5px hsl(var(--primary) / 0.12)` | `0 0 26px -5px hsl(0 0% 0% / 0.3)` | Card elevated (브랜드 기반) |
| `shadow-card-hover` | `0 8px 30px -5px hsl(var(--primary) / 0.18)` | `0 8px 30px -5px hsl(0 0% 0% / 0.4)` | Card interactive hover |
| `shadow-lg` | Tailwind 기본 | Tailwind 기본 | 플로팅 오버레이 (Dropdown, Toast) |
| `shadow-xl` | Tailwind 기본 | Tailwind 기본 | CTA 버튼 hover |

**다크모드 shadow**: `shadow-card` / `shadow-card-hover`는 라이트에서 `--primary` 기반 (브랜드 느낌), 다크에서 순수 검정 기반 (흰색 글로우 방지)으로 자동 전환. `.dark` 셀렉터로 오버라이드.

### Typography

**Font**: Pretendard Variable (한글 최적화), `font-feature-settings: "rlig" 1, "calt" 1` 전역 적용.
**Letter-spacing**: `tracking-tight` (`-0.025em`) 모든 제목(h1~h6) 자동 적용. Relaxed Tracking 언어(th, my, km, si, hi, ne, mn)에서 `letter-spacing: 0`으로 해제.

#### Heading Scale

| 레벨 | 사이즈 | Weight | 용도 |
|------|--------|--------|------|
| Hero (marketing) | `text-4xl md:text-5xl lg:text-6xl` | `font-bold` | HeroSection marketing variant |
| Hero (list) | `text-3xl md:text-4xl` | `font-bold` | HeroSection list variant, 상세 페이지 |
| Section | `text-3xl md:text-4xl` | `font-bold` | Feature 섹션 제목 (home, about) |
| Subsection | `text-xl` | `font-semibold` | 섹션 내 소제목 |
| Card Title | `text-lg` | `font-semibold` (상속) | 리스트 카드 제목 |
| Admin Title | `text-2xl` | `font-bold` | 관리자 페이지 제목 |

#### Weight 규칙

| Weight | 용도 |
|--------|------|
| `font-bold` (700) | h1, h2, KPI 수치, Hero 텍스트 |
| `font-semibold` (600) | h3, CardTitle, 섹션 소제목 |
| `font-medium` (500) | Label, nav 항목, Button 텍스트 |
| `font-normal` (400) | 본문, 설명 텍스트 |

#### Line-height

| 클래스 | 용도 |
|--------|------|
| `leading-none` (1.0) | Hero 제목 (줄간격 최소화) |
| `leading-snug` (1.375) | 카드 제목, 짧은 텍스트 |
| `leading-relaxed` (1.625) | 본문, 설명 (가독성 우선) |

- **한글 줄바꿈**: `break-keep` 적용 (단어 단위 줄바꿈)

### Icon Sizing Scale

| 컨텍스트 | 아이콘 크기 | px | 예시 |
|---------|-----------|-----|------|
| Inline / Button | `h-4 w-4` | 16px | nav 아이콘, 버튼 내부 아이콘 |
| CTA / Hero badge | `h-5 w-5` | 20px | ArrowRight, HeroSection 배지 아이콘 |
| Nav toggle | `h-6 w-6` | 24px | 햄버거 메뉴, 메일 아이콘 |
| Feature card | `h-7 w-7` | 28px | gradient 컨테이너 내부 아이콘 |
| Value card | `h-8 w-8` | 32px | about 페이지 가치 카드 |
| EmptyState | `h-10 w-10` | 40px | EmptyState 컴포넌트 아이콘 |

### Animation & Duration

| 속도 | 클래스 | 용도 |
|------|--------|------|
| Quick | `duration-200` | 호버 색상 변경, 단순 상태 전환 |
| Standard | `duration-300` | 카드 리프트, fade, slide (**기본값**) |
| Slow | `duration-500` | 프로그레스바, 복잡한 전환 |

**기본 transition**: `transition-all duration-300` (카드, 버튼 등).
**모션 감소**: `motion-reduce:transition-none motion-reduce:transform-none` (Card interactive에 적용).
**진입 애니메이션**: `animate-in fade-in duration-300` (EmptyState 등 상태 전환).

### Grid Gap Standard

| Gap | px | 용도 |
|-----|-----|------|
| `gap-1` ~ `gap-2` | 4~8px | 인라인 요소 (아이콘+텍스트, 배지 그룹) |
| `gap-4` | 16px | 폼 필드, compact 리스트 |
| `gap-6` | 24px | 카드 그리드 (**표준**, 대부분의 리스트 페이지) |
| `gap-8` | 32px | Feature/마케팅 그리드 (큰 카드) |

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

---

## 02 Layout

### SectionContainer

`components/blocks/section_container.tsx` — 섹션 래퍼. padding + container 동시 제공.

```tsx
import { SectionContainer } from "@/components/blocks/section_container";

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

### AuthLayout

`components/layouts/auth_layout.tsx` — 인증 페이지 공통 래퍼. 전체 화면 중앙 정렬 + Card(max-w-md).

```tsx
import { AuthLayout } from "@/components/layouts/auth_layout";

<AuthLayout>
  <CardHeader>...</CardHeader>
  <CardContent>...</CardContent>
</AuthLayout>
```

**적용 페이지** (6개): login, signup, verify_email, reset_password, request_reset_password, account_recovery

### Container Sizes

| 토큰 | Tailwind | px | 용도 |
|------|---------|-----|------|
| Default | `max-w-container-default` | 1350px | SectionContainer 기본 (카드 그리드 등) |
| Narrow | `max-w-container-narrow` / `max-w-3xl` | 768px | Legal 페이지, SectionContainer `container="narrow"` |
| Form | `max-w-container-form` / `max-w-md` | 448px | 인증 페이지 (로그인, 회원가입) |
| Text | `max-w-2xl` | 672px | Hero subtitle, 설명문 (가독성 최적화) |

### Page Templates

- **목록 페이지**: `bg-hero-gradient` 헤더 + `SectionContainer` 콘텐츠
- **상세 페이지**: `bg-hero-gradient` 헤더 + 내용 영역
- **폼 페이지**: 중앙 정렬 Card + max-w-md

---

## 03 Components

### HeroSection

`components/blocks/hero_section.tsx` — Hero 블록. `variant` prop으로 마케팅/리스트 레이아웃 전환.

```tsx
import { HeroSection } from "@/components/blocks/hero_section";

// 마케팅 페이지 (기본)
<HeroSection
  badge={<><Sparkles className="h-4 w-4 text-accent" /><span>Badge Text</span></>}
  title="메인 타이틀"
  subtitle="부제목 설명 텍스트"
  size="sm"
>
  {/* CTA 버튼 등 */}
</HeroSection>

// 리스트 페이지
<HeroSection
  variant="list"
  badge={<><GraduationCap className="h-5 w-5 text-secondary" /><span>학습</span></>}
  title="학습 목록"
  subtitle="한국어 학습 프로그램"
>
  {/* 필터 패널 (flex row 배치) */}
</HeroSection>
```

**Props**:
| Prop | 타입 | 기본값 | 설명 |
|------|------|--------|------|
| variant | `'marketing' \| 'list'` | `'marketing'` | 레이아웃 변형 |
| badge | `ReactNode` | — | 상단 배지 (아이콘 + 텍스트) |
| title | `ReactNode` | 필수 | 메인 타이틀 |
| subtitle | `ReactNode` | — | 부제목 |
| size | `'default' \| 'sm'` | `'default'` | Hero 크기 (marketing만 유효) |
| children | `ReactNode` | — | CTA/필터 등 |

**Variant 비교**:
| 속성 | marketing | list |
|------|-----------|------|
| 배경 | `bg-hero-gradient` + 블롭 장식 | `bg-hero-gradient border-b` |
| 정렬 | 중앙 (`text-center`) | 좌측 |
| 패딩 | `py-section-lg lg:py-hero-lg` | `py-section-sm lg:py-section-md` |
| 타이포 | `text-4xl md:text-5xl lg:text-6xl` | `text-3xl md:text-4xl` |
| children | 중앙 배치 | flex row (필터 패널 지원) |

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

### Button Variants & Sizes

shadcn CVA 기반 버튼. `components/ui/button.tsx`.

#### Variants

| Variant | 스타일 | 용도 |
|---------|--------|------|
| `default` | `bg-primary text-primary-foreground` | 기본 액션 |
| `destructive` | `bg-destructive text-destructive-foreground` | 삭제, 위험 액션 |
| `outline` | `border border-input bg-background` | 보조 액션 |
| `secondary` | `bg-secondary text-secondary-foreground` | 2차 액션 |
| `ghost` | 투명, hover시 `bg-accent` | 인라인 액션, nav |

#### Sizes

| Size | 높이 | 용도 |
|------|------|------|
| `sm` | `h-8 px-3 text-xs` | 테이블 내부, 보조 버튼 |
| `default` | `h-9 px-4 py-2` | 일반 액션 |
| `lg` | `h-10 px-8` | 강조 액션 |
| `icon` | `h-9 w-9` | 아이콘 전용 버튼 |

#### CTA 패턴

3가지 변형이 존재하며, `gradient-primary text-white rounded-full`이 공통 베이스:

| 변형 | 클래스 | 용도 |
|------|--------|------|
| **Full CTA** | `gradient-primary hover:opacity-90 text-white shadow-lg hover:shadow-xl transition-all rounded-full px-8 h-14 text-base` | Hero, 마케팅 섹션 |
| **Nav CTA** | `gradient-primary hover:opacity-90 text-white shadow-md hover:shadow-lg transition-all rounded-full px-6` | 헤더 네비게이션 (Button size="sm") |
| **Inline CTA** | `gradient-primary text-white rounded-full` | 모바일 메뉴, 콘텐츠 내 소형 |

```tsx
{/* Full CTA */}
<Button className="gradient-primary hover:opacity-90 text-white shadow-lg hover:shadow-xl transition-all rounded-full px-8 h-14 text-base">
  시작하기 <ArrowRight className="ml-2 h-5 w-5" />
</Button>
```

**`text-white` 예외**: gradient 배경 위 텍스트는 `text-white` 직접 사용 허용. CSS 변수 기반 gradient는 다크모드에서도 채도 있는 배경을 유지하므로 `text-white`가 양쪽 모드에서 안전. 동일 이유로 gradient 아이콘 컨테이너(`gradient-primary` 위 아이콘)도 `text-white` 허용.

### Card CVA Variants

`components/ui/card.tsx` — CVA 기반 카드 변형. `cardVariants` export.

| Variant | 스타일 | 용도 |
|---------|--------|------|
| `default` | `rounded-xl border shadow` | 일반 카드 (기존 shadcn 기본) |
| `elevated` | `border-0 shadow-card rounded-2xl` | 시각적 강조 (skeleton, 통계 등) |
| `interactive` | elevated + hover/focus/active 인터랙션 | 클릭 가능한 카드 (리스트 아이템) |

**interactive 포함 상태**:
- `hover:-translate-y-1 hover:shadow-card-hover` — 호버 리프트
- `focus-visible:ring-2 ring-ring ring-offset-2 ring-offset-background` — 키보드 포커스
- `active:translate-y-0` — 클릭/탭 피드백
- `motion-reduce:transition-none motion-reduce:transform-none` — 모션 감소 접근성

**카드 링크 패턴**: 리스트 카드는 `<Link>` → `<Card variant="interactive">` 구조 사용.

```tsx
<Link to={`/items/${id}`}>
  <Card variant="interactive" className="h-full group">
    <CardHeader>...</CardHeader>
    <CardContent>
      <CardTitle className="group-hover:text-primary transition-colors">
        {title}
      </CardTitle>
    </CardContent>
  </Card>
</Link>
```

### PaginationBar

`components/blocks/pagination_bar.tsx` — 페이지네이션 로직+UI 통합 컴포넌트.

```tsx
import { PaginationBar } from "@/components/blocks/pagination_bar";

<PaginationBar
  currentPage={currentPage}
  totalPages={totalPages}
  onPageChange={setPage}
/>
```

**Props**:
| Prop | 타입 | 설명 |
|------|------|------|
| currentPage | `number` | 현재 페이지 |
| totalPages | `number` | 총 페이지 수 |
| onPageChange | `(page: number) => void` | 페이지 변경 핸들러 |
| className | `string?` | 추가 클래스 |

- `totalPages <= 1`이면 렌더링 안 함
- 내부에서 `getPageItems()` 호출 (ELLIPSIS 포함)
- 접근성: `aria-current="page"` (활성), `aria-disabled` (비활성 prev/next)

**페이지네이션 유틸**: `lib/pagination.ts` — `getPageItems(current, total, siblingCount?)` + `ELLIPSIS` Symbol 상수.

### EmptyState

`components/blocks/empty_state.tsx` — 데이터 없음 상태 표시.

```tsx
import { EmptyState } from "@/components/blocks/empty_state";

<EmptyState
  icon={<BookOpen className="h-10 w-10 text-muted-foreground" />}
  title="데이터가 없습니다"
  description="아직 등록된 항목이 없습니다."
  action={<Button>추가하기</Button>}
/>
```

**Props**:
| Prop | 타입 | 설명 |
|------|------|------|
| icon | `ReactNode` | 아이콘 (h-10 w-10 권장) |
| title | `string` | 제목 |
| description | `string?` | 부가 설명 |
| action | `ReactNode?` | CTA 버튼 등 |
| className | `string?` | 추가 클래스 (기본 py-20) |

- 접근성: `role="status"`, 아이콘 컨테이너 `aria-hidden="true"`
- 진입 애니메이션: `animate-in fade-in duration-300`

### SkeletonGrid

`components/blocks/skeleton_grid.tsx` — 로딩 스켈레톤 그리드.

```tsx
import { SkeletonGrid } from "@/components/blocks/skeleton_grid";

<SkeletonGrid count={10} variant="video-card" columns={3} />
```

**Props**:
| Prop | 타입 | 설명 |
|------|------|------|
| count | `number` | 스켈레톤 카드 수 |
| variant | `"video-card" \| "content-card" \| "study-card"` | 카드 형태 |
| columns | `2 \| 3 \| 4` | 그리드 열 수 (기본 3) |
| className | `string?` | 추가 클래스 |

- 모바일 항상 1열 (`grid-cols-1`), md부터 columns 적용
- Tailwind 동적 클래스는 매핑 객체 사용 (인터폴레이션 금지)

### ListStatsBar

`components/blocks/list_stats_bar.tsx` — 리스트 페이지 통계 바.

```tsx
import { ListStatsBar } from "@/components/blocks/list_stats_bar";

<ListStatsBar
  icon={Film}
  totalLabel={t("video.totalVideos", { count: 42 })}
  total={42}
  currentPage={1}
  totalPages={5}
  isFetching={false}
/>
```

**Props**:
| Prop | 타입 | 설명 |
|------|------|------|
| icon | `LucideIcon` | 좌측 아이콘 |
| totalLabel | `string` | 총 개수 라벨 |
| total | `number` | 총 개수 |
| currentPage | `number` | 현재 페이지 |
| totalPages | `number` | 총 페이지 수 |
| isFetching | `boolean?` | 로딩 인디케이터 표시 |
| className | `string?` | 추가 클래스 |

### StatCard

`components/blocks/stat_card.tsx` — **대시보드 KPI 카드** 전용.

```tsx
import { StatCard } from "@/components/blocks/stat_card";

<StatCard icon={Users} label="총 사용자" value={1234} loading={false} />
```

**Props**:
| Prop | 타입 | 설명 |
|------|------|------|
| icon | `LucideIcon` | 아이콘 |
| label | `string` | 라벨 |
| value | `number \| string?` | 표시값 |
| loading | `boolean?` | 스켈레톤 표시 |

> **용도 제한**: Admin Dashboard KPI 카드 목적. `blocks/` 폴더의 범용 컴포넌트화 방지.

### CoverCard

`components/blocks/cover_card.tsx` — 카탈로그 표지 카드 (교재/E-book 공유).

```tsx
import { CoverCard } from "@/components/blocks/cover_card";

<CoverCard
  imageSrc="/covers/student-en.webp"
  imageAlt="English student"
  title="영어 학생용 교재"
  subtitle="1권당 ₩25,000"
  actionLabel="상세보기"
  onClick={() => openModal(item)}
/>
```

**Props**:
| Prop | 타입 | 설명 |
|------|------|------|
| imageSrc | `string` | 표지 이미지 경로 |
| imageAlt | `string` | alt 텍스트 |
| title | `string` | 카드 제목 |
| subtitle | `string` | 부가 텍스트 (가격 등) |
| actionLabel | `string` | 하단 액션 버튼 텍스트 |
| onClick | `() => void` | 클릭 핸들러 |

**적용 페이지**: textbook_catalog_page (GridSection), ebook_catalog_page (EbookGridSection)

### FeatureGrid

`components/blocks/feature_grid.tsx` — 아이콘 + 제목 + 설명 카드 그리드 (3열).

```tsx
import { FeatureGrid } from "@/components/blocks/feature_grid";

<FeatureGrid
  items={[
    { icon: <Lightbulb className="h-8 w-8 text-white" />, title: "습득", description: "..." },
    { icon: <Timer className="h-8 w-8 text-white" />, title: "효율", description: "..." },
    { icon: <Heart className="h-8 w-8 text-white" />, title: "이해", description: "..." },
  ]}
/>
```

**Props**:
| Prop | 타입 | 설명 |
|------|------|------|
| items | `{ icon: ReactNode, title: string, description: string }[]` | 카드 배열 |

**적용 페이지**: home_page (핵심가치 섹션), about_page (차별점 섹션)

### DataTable

`components/blocks/data_table.tsx` — 관리자 테이블 공통 블록 (검색 + 정렬 + 선택 + 페이지네이션).

```tsx
import { DataTable, useDataTable, type DataTableColumn } from "@/components/blocks/data_table";

const columns: DataTableColumn<Item>[] = [
  { key: "id", header: "ID", sortField: "id", skeletonWidth: "w-8", render: (item) => item.id },
  { key: "actions", header: "Actions", skeletonWidth: "w-16", render: (item) => <Button>Edit</Button> },
];

const table = useDataTable({ defaultSortField: "id" });

<DataTable
  columns={columns}
  data={data?.items}
  isLoading={isLoading}
  isError={isError}
  entityName="items"
  getId={(item) => item.id}
  searchPlaceholder="Search..."
  searchInput={table.searchInput}
  onSearchInputChange={table.setSearchInput}
  onSearch={table.handleSearch}
  sortField={table.sortField}
  sortOrder={table.sortOrder}
  onSort={table.handleSort}
  selectedIds={table.selectedIds}
  onSelectAll={table.setSelectedIds}
  onSelectOne={table.handleSelectOne}
  bulkActionSlot={<Button>Edit Selected</Button>}
  page={table.params.page}
  totalPages={totalPages}
  totalCount={totalCount}
  onPageChange={table.handlePageChange}
/>
```

**useDataTable Hook**: 검색/정렬/페이지네이션/선택 상태 + 핸들러 일괄 관리.

**DataTableColumn Props**:
| Prop | 타입 | 설명 |
|------|------|------|
| key | `string` | 고유 키 |
| header | `string` | 헤더 텍스트 |
| sortField? | `string` | 정렬 필드 (없으면 비정렬 컬럼) |
| skeletonWidth | `string` | 로딩 스켈레톤 너비 (`w-8`, `w-40` 등) |
| render | `(item: T) => ReactNode` | 셀 렌더링 함수 |

**적용 페이지**: admin_users_page, admin_lessons_page, admin_videos_page (각 ~200줄 감소)

### Dark Mode (Theme Toggle)

`components/ui/theme_toggle.tsx` — 테마 전환 토글. next-themes 기반.

```tsx
import { ThemeToggle } from "@/components/ui/theme_toggle";

<ThemeToggle />  // Sun/Moon 아이콘 + 드롭다운 (Light/Dark/System)
```

**구현 구조**:
- `next-themes` `ThemeProvider` → `App.tsx`에서 최상위 래핑
- `attribute="class"` → Tailwind `darkMode: ["class"]`와 일치
- `defaultTheme="system"` → OS 테마 자동 추종
- `disableTransitionOnChange` → 테마 전환 시 깜빡임 방지

**다크모드 자동 전환 원리**:
1. CSS 변수가 `:root` (라이트) / `.dark` (다크) 이중 정의
2. `next-themes`가 `<html class="dark">` 토글
3. Tailwind 클래스가 CSS 변수 참조 → 자동 전환

**`--primary` 반전 문제와 해결**:

라이트에서 `--primary: 222 90% 18%` (dark navy) → 다크에서 `--primary: 210 40% 98%` (near-white). shadcn 표준 패턴이지만, `bg-primary`를 "항상 어두운 배경"으로 사용한 곳에서 문제 발생.

```
❌ Footer/CTA: bg-primary text-white → 다크에서 흰 배경 + 흰 텍스트
✅ Footer: bg-footer text-footer-foreground → 항상 어두운 톤 유지
✅ CTA: bg-surface-inverted text-surface-inverted-foreground → 항상 어두운 톤 유지
```

**다크모드 shadow 오버라이드**:

라이트에서 `shadow-card`는 `--primary` 기반 (브랜드 느낌). 다크에서 `--primary`가 밝아지면 흰색 글로우가 발생하므로, `.dark .shadow-card`에서 검정 기반으로 오버라이드.

---

## 04 Mobile Checklist

- [x] Touch target 최소 44px — `@media (pointer: coarse)` 글로벌 CSS로 강제 (`index.css`)
- [x] Hover-only 인터랙션 금지 (hover와 함께 항상 click/tap도 제공)
- [x] Input 16px+ (iOS zoom 방지 — `@supports (-webkit-touch-callout: none)` 자동 적용)
- [x] `break-keep` 한글 줄바꿈 최적화
- [x] 모바일 메뉴 body 스크롤 잠금 (`header.tsx`)
- [x] 모달 뷰포트 초과 방지 (`max-w-[calc(100vw-2rem)]`)
- [x] 고정 그리드 반응형 전환 (`grid-cols-1 sm:grid-cols-*`)
- [x] 콘텐츠/섹션 패딩 반응형 (`py-section-sm md:py-section-lg`)

**Touch Target 구현**:
```css
/* index.css — 터치 기기에서만 적용, 데스크탑 UI 유지 */
@media (pointer: coarse) {
  button, [role="button"] {
    min-height: 44px;
    min-width: 44px;
  }
}
```
- shadcn DropdownMenuItem(`role="menuitem"`), SelectItem(`role="option"`) 미영향
- Dialog 닫기 버튼: `h-5 w-5` 아이콘 + `p-1` 패딩 (20px 시각 + 44px 터치)

### Responsive Behavior

#### Breakpoints

| Name | Width | Tailwind | Primary Change |
|------|-------|----------|---------------|
| Mobile | <640px | (default) | 1열 레이아웃, 축소된 패딩 |
| Tablet | ≥640px | `sm:` | 2열 그리드 시작 |
| Desktop | ≥768px | `md:` | Hero 타이포 확대, 네비게이션 전환 |
| Wide | ≥1024px | `lg:` | 3열 그리드, 사이드바 노출, 섹션 패딩 확대 |
| Ultra | ≥1280px | `xl:` | 4열 그리드 (관리자) |

#### Collapsing Strategy

| 요소 | 축소 패턴 |
|------|----------|
| Header | `lg` 이하에서 햄버거 메뉴로 축소 |
| Footer | 4열 → 2열 → 1열 |
| Card Grid | `xl`=4열, `lg`=3열, `sm`=2열, default=1열 |
| Hero | 텍스트 단계적 축소 (`6xl` → `5xl` → `4xl` → `3xl`) |
| SectionContainer | `sm`: 40px, `md`: 64px, `lg`: 80~128px |

#### Touch Targets

- 최소 44×44px (WCAG 2.5.8) — `@media (pointer: coarse)` 감지
- iOS: 인풋 `font-size: 16px` (auto-zoom 방지)

---

## 05 Anti-Patterns (금지 규칙)

### Do

- CSS 변수로만 색상 참조 (HEX 직접 사용 금지)
- 컨테이너 토큰 사용 (`max-w-container-default`, not `max-w-[1350px]`)
- Status Badge는 `variant` prop 사용 (`bg-status-*` 직접 적용 금지)
- 다크모드에서 고정 어두운 배경: `bg-footer`, `bg-surface-inverted`
- img 태그에 `loading="lazy"` 필수
- 터치 타겟 최소 44×44px (`@media (pointer: coarse)` 글로벌 적용)
- CJK 텍스트에 `break-keep-cjk` 적용 (`.lang-cjk` 하위에서만 동작)
- 반응형은 mobile-first: `sm:` → `md:` → `lg:` → `xl:` 순서
- 다크모드 항상 고려: `bg-background`, `text-foreground` 등 시맨틱 토큰 사용

### Don't

- Tailwind 기본색 직접 사용 (`bg-white`, `bg-gray-*`, `text-gray-*` 등)
- `rounded-sm` 직접 사용 (shadcn 내부 프리미티브 전용)
- `bg-primary`를 고정 어두운 배경에 사용 (다크모드에서 반전됨)
- `font-bold`를 본문에 사용 (제목/KPI 전용)
- 40px 미만 여백을 SectionContainer 레벨에서 처리
- `whitespace-nowrap`을 다국어 타이틀에 적용 (장문 언어 넘침)
- 커스텀 `box-shadow` 값 사용 (`shadow-card` 등 유틸 클래스 사용)

### 허용 예외

| 패턴 | 허용 위치 | 이유 |
|------|----------|------|
| `text-white` | `gradient-primary` 배경 위 | gradient는 다크모드에서도 채도 유지 → white 안전 |
| `text-white` | FeatureGrid 아이콘 컨테이너 | gradient 배경 위 아이콘 |
| `bg-black/50~80` | Dialog overlay, ebook overlay | 오버레이 표준. 시맨틱 토큰 불필요 |
| `print:text-gray-*` | 인쇄 전용 스타일 | 화면에 영향 없음 |
| 장식용 Tailwind 색상 | `book_hub_page` SLIDE_COLORS, `textbook_order_page` | 순수 장식 (status/semantic 의미 없음) |

---

### 금지 패턴 상세

#### 하드코딩 색상 (모든 유틸리티 프리픽스)

```
❌ from-[#F0F3FF]          → ✅ from-brand-soft
❌ to-[#E8F4FF]            → ✅ to-brand-soft-alt
❌ bg-[#051D55]            → ✅ bg-primary
❌ bg-[#129DD8]            → ✅ bg-accent
❌ bg-[#4F71EB]            → ✅ bg-secondary
❌ text-[#051D55]          → ✅ text-primary
❌ border-[#129DD8]        → ✅ border-accent
```

#### Tailwind Named Colors (다크모드 미지원)

```
❌ bg-white                → ✅ bg-background 또는 bg-card
❌ text-black              → ✅ text-foreground
❌ bg-gray-50, bg-gray-100 → ✅ bg-muted
❌ text-gray-900           → ✅ text-foreground
❌ text-gray-600           → ✅ text-muted-foreground
❌ border-gray-200         → ✅ border-border
❌ text-blue-600           → ✅ text-primary
❌ bg-blue-50              → ✅ bg-primary/5 또는 bg-primary/10
```

#### 직접 상태 색상

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

#### Footer/CTA 하드코딩

```
❌ bg-primary text-white   → ✅ bg-footer text-footer-foreground (Footer)
❌ bg-primary text-white   → ✅ bg-surface-inverted text-surface-inverted-foreground (CTA)
```

#### Radius 혼용

```
❌ rounded-lg (일반 Card)  → ✅ Card variant="default" (자동 rounded-xl)
❌ rounded-xl (Dialog)     → ✅ rounded-lg (Dialog/Alert 표준)
❌ rounded-md (마케팅 카드) → ✅ rounded-2xl (Surface 티어)
```

#### Shadow 혼용

```
❌ shadow-md (카드 강조)   → ✅ shadow-card (브랜드 shadow, 다크모드 대응)
❌ shadow-lg (카드 hover)  → ✅ shadow-card-hover (다크모드 대응)
❌ [custom box-shadow]     → ✅ CSS 유틸 클래스 사용 (shadow-card 등)
```

#### Typography 혼용

```
❌ text-5xl (리스트 Hero)  → ✅ text-3xl md:text-4xl (list variant 표준)
❌ font-bold (CardTitle)   → ✅ font-semibold (CardTitle 표준)
❌ font-normal (라벨)      → ✅ font-medium (라벨 표준)
```

#### 매직넘버 여백

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

**현재 상태**: 8건 잔여 — 전부 장식용 색상 팔레트 (의도적 예외).

**의도적 예외 (장식용 색상)**:
- `book_hub_page.tsx` SLIDE_COLORS: 6색 순환 팔레트 (blue/emerald/amber/violet/rose/teal). 개별 슬라이드 시각 구분 목적.
- `textbook_order_page.tsx` 주문 안내 아이콘: 4색 (blue/emerald/violet/amber). 정보 카드 시각 구분 목적.

이 색상들은 status/semantic 의미 없이 순수 장식용이므로 토큰 교체 불필요.

### PR 체크리스트

새 코드 작성 시 확인:

- [ ] 색상: semantic 토큰 사용 (직접 HEX/Tailwind named 색상 금지)
- [ ] 상태: `variant` prop 사용 (`<Badge variant="success">`)
- [ ] 여백: section spacing 토큰 사용
- [ ] Hero: `bg-hero-gradient` 또는 `<HeroSection>` 사용
- [ ] Radius: Scale 준수 (Card=xl, Dialog=lg, Button=md, Badge=full)
- [ ] Typography: Heading Scale 준수 (marketing Hero=4xl~6xl, list=3xl~4xl)
- [ ] Shadow: `shadow-card` / `shadow-card-hover` 사용 (커스텀 box-shadow 금지)
- [ ] Icon: 컨텍스트별 사이즈 준수 (Button=4, Hero badge=5, EmptyState=10)
- [ ] 다크모드: `bg-white` 금지 → `bg-background` / `bg-card`, Footer/CTA는 전용 토큰 사용
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

### Milestone C — 디자인 고도화 시

| 항목 | 작업 | 파일 |
|------|------|------|
| Hero alias 토큰 | `--hero-from`/`--hero-to` alias 추가 (brand-soft 다용도 확장 대비) | `index.css`, `tailwind.config.js` |

### On-Demand — 필요 시 즉시

| 항목 | 작업 | 파일 |
|------|------|------|
| `SectionContainer size="none"` | sizeMap에 `none: ""` 추가 (이중 패딩 방지) | `section_container.tsx` |
| ~~`HeroSection layout="split"`~~ | ~~좌우 분할 레이아웃 variant 추가~~ | ~~`hero_section.tsx`~~ |

> `HeroSection layout="split"` → `variant="list"` (children flex row)로 대체 완료 (DS v2).

---

## 07-B Agent Prompt Guide

AI 에이전트가 Amazing Korean의 디자인 시스템을 준수하며 UI를 생성할 때 참조하는 퀵 레퍼런스.

### Quick Color Reference

```
Primary (Dark Navy):  hsl(222 90% 18%)  #051D57  — 메인 액션, 네비게이션
Secondary (Blue):     hsl(224 81% 61%)  #4B76EC  — 보조 버튼, 서브 UI
Accent (Cyan):        hsl(197 84% 46%)  #13A0D8  — 강조 포인트, 아이콘
Background:           hsl(0 0% 100%)    #FFFFFF  — 기본 배경
Foreground:           hsl(222 47% 11%)  #0F1729  — 기본 텍스트 (near-black)
Muted:                hsl(228 33% 97%)  #F5F6FA  — 비활성 배경
Destructive:          hsl(0 84% 50%)    #EB1414  — 에러, 삭제
```

### Example Component Prompts

**1. Hero Section (마케팅)**
```
Pretendard Variable 폰트, text-4xl md:text-5xl lg:text-6xl font-bold tracking-tight.
bg-hero-gradient 배경. text-center 정렬, max-w-container-default.
부제목은 text-lg text-muted-foreground max-w-2xl mx-auto.
CTA: gradient-primary hover:opacity-90 text-white rounded-full px-8 h-14 shadow-lg hover:shadow-xl.
```

**2. 카드 그리드 (리스트)**
```
Card variant="interactive" — rounded-2xl shadow-card, hover:-translate-y-1 shadow-card-hover.
그리드: grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6.
제목: text-lg font-semibold, group-hover:text-primary.
Badge variant="warning"으로 유료 표시. variant="success"으로 완료 표시.
```

**3. 인증 폼**
```
AuthLayout (전체 화면 중앙 정렬, max-w-container-form 448px).
Card + CardHeader + CardContent. Input은 rounded-md.
Button variant="default" (bg-primary) 전체 너비. 에러: text-destructive.
```

**4. 관리자 테이블**
```
DataTable + useDataTable 훅. 검색+정렬+선택+페이지네이션 내장.
StatCard로 KPI 표시 (Card variant="elevated").
Badge 색상: blue/orange/purple/yellow/sky/indigo (enum 표시용).
```

**5. Footer/CTA 어두운 섹션**
```
bg-surface-inverted text-surface-inverted-foreground (항상 어두운 톤).
Footer: bg-footer text-footer-foreground.
절대로 bg-primary text-white 사용 금지 (다크모드에서 반전됨).
```

### Iteration Guide

1. 모든 색상은 CSS 변수(HSL)로만 참조 — HEX 하드코딩 금지
2. 컴포넌트는 shadcn/ui 기반 — 커스텀 컴포넌트 전에 기존 `components/ui/` 확인
3. 반응형은 mobile-first — `sm:` → `md:` → `lg:` → `xl:` 순서
4. 다크모드 항상 고려 — `bg-primary` 대신 Surface 토큰 (고정 어두운 배경)
5. 카드 variant 자동 radius — default=xl, elevated/interactive=2xl
6. 섹션 여백은 SectionContainer — `size="sm"(40px)` / `"md"(64px)` / `"lg"(80~128px)`
7. gradient 위 텍스트는 `text-white` 허용 — 유일한 예외

---

## 08 Figma Design System

> **2026-04-10 결정**: Figma 도입 보류. **이 문서(AMK_DESIGN_SYSTEM.md) + Phase A의 32 PNG 시각 레퍼런스 + 코드** 3계층을 디자인 SSoT로 운영.
> Phase B/C(Figma 정리·임포트·네이티브 생성)는 재평가 트리거 도달 시까지 보류.

### 결정 배경 (2026-04-10)

원래 Phase A→B→C 하이브리드 계획으로 Figma 컴포넌트 라이브러리를 구축하려 했으나, Phase B 착수 시 Figma MCP Starter 플랜 한도 초과를 트리거로 도입 자체를 재검토. 다음 5가지 이유로 보류 결정:

1. **CEO 원래 4가지 목적이 이미 대부분 달성됨**
   - ① 페이지별 UI/UX 구성 확정 → Phase A의 32 PNG가 정답지 역할
   - ② 디자인 개선 → v4.2 보강 19건 + 코드 수정 5건으로 진행 중, 추가 개선은 코드 직접 수정으로 가능
   - ③ 디자인 요소 확정 → §03 컴포넌트 권장 + §07-B Agent Prompt Guide
   - ④ 유지보수 효율 → §07-B Quick Color Ref + 예제로 Claude Code 작업 효율화 달성

2. **1인 풀스택 환경의 SSoT 단순화**
   - 현재: 코드 ⇄ AMK_DESIGN_SYSTEM.md (2-way 동기화)
   - Figma 도입 시: 코드 ⇄ 문서 ⇄ Figma (3-way 동기화) → 1인 운영자에게 트리플 동기화 부담 추가
   - 결과: 4번(유지보수 효율) 목표에 오히려 역행

3. **Phase C의 진짜 가치 의문**
   - Button variants는 이미 `class-variance-authority`로 코드에 정의됨 → Figma 중복 정의는 동기화 부담만 증가
   - 페이지 레이아웃은 Phase A PNG 보면서 코드 수정이 더 빠름
   - "디자이너에게 설명" 시나리오 자체가 없음 (1인 운영)

4. **비용/효익 비대칭**
   - 비용: Figma 유료 플랜 연 $144~$540 + MCP 한도 관리 + 트리플 동기화 시간
   - 효익: 시각화 자산 + 향후 디자이너 협업 시 자산 (디자이너 영입 계획 없음)

5. **Phase A 산출물이 이미 Figma 가치의 80%를 무료로 달성**
   - 32 PNG는 시각 레퍼런스 가치를 코드 + 도구만으로 제공
   - Playwright 도구가 영구 자산이라 언제든 재캡처 가능 (Figma처럼 별도 동기화 의무 없음)

### 현재 운영 모델 (3계층 SSoT)

| 계층 | 위치 | 역할 |
|------|------|------|
| **디자인 의도 (SSoT)** | `docs/AMK_DESIGN_SYSTEM.md` (이 문서) | 토큰 정의, 컴포넌트 권장, Do/Don't, Agent Prompt Guide |
| **시각 레퍼런스 (현재 모습)** | `frontend/figma-capture/artifacts/screenshots/` (32 PNG) | 코드 변경 시 시각 비교, 디자인 회귀 감지 |
| **실제 구현** | `frontend/src/` + `tailwind.config.js` + `index.css` | 토큰/컴포넌트의 진짜 진실 |

**원칙**: 1인 풀스택 + Claude Code 협업 환경에서는 **코드가 가장 빠르고 정확한 디자인 표현**. 중간 레이어(Figma) 추가는 트리플 동기화 비용을 만들어 ④ 유지보수 효율 목표에 역행.

### Phase A — Playwright 캡처 도구 ✅ **상시 운영 도구로 전환 (2026-04-10)**

원래 Figma 임포트 준비용으로 만들었으나, 그 자체로 충분한 가치가 있어 **영구 자산**으로 위치 재정의.

**위치**: `frontend/figma-capture/`

| 파일 | 역할 |
|------|------|
| `playwright.config.ts` | 1440×900 viewport · deviceScaleFactor 2 (Retina) · Vite webServer 자동 기동 · ko-KR 로캘 |
| `tests/capture.spec.ts` | `document.fonts.ready` + 점진 스크롤 + img decoded 대기 + next-themes localStorage 주입 |
| `pages.ts` | 16개 페이지 정의 (P1 공개 4 + P2 Book 4 + P3 Auth 5 + P4 Legal 3) |
| `fixtures.ts` | textbook/ebook catalog API 모의 응답 (백엔드 부재 시에도 카탈로그 렌더링 보장) |
| `README.md` | 사용법, 안정화 장치, 출력 구조 |

**산출물**: `figma-capture/artifacts/screenshots/{group}/{slug}--{theme}.png` — 16 페이지 × Light/Dark = **32 PNG** (약 8.8MB, .gitignore 제외)

**활용 시나리오**:
1. **디자인 작업 전** — 현재 모습 확인용 시각 레퍼런스
2. **디자인 작업 후** — 재캡처 후 변경 영향 확인 (시각 회귀 감지)
3. **Claude Code 협업** — 작업 대상 페이지 PNG 첨부로 컨텍스트 전달

**재캡처 명령**:
```bash
cd frontend/figma-capture && npx playwright test
```

### Phase B/C — 보류 (2026-04-10)

| Phase | 원래 계획 | 보류 사유 | 재평가 트리거 |
|-------|----------|----------|-------------|
| **B** — Figma 정리 + 임포트 | 기존 34프레임 삭제 + 32 PNG 임포트 | Figma MCP Starter 한도 초과 + Phase A 산출물 자체로 시각 레퍼런스 충분 | 디자이너 영입 또는 외부 디자인 협업 시작 |
| **C** — Figma 네이티브 생성 (C-F1 Variables / C-F2 컴포넌트 6종 / C-F3 페이지 4개) | §01 토큰을 Variables로 등록, Button/Badge/Card/HeroSection/Input/SectionContainer 컴포넌트, Home/About/FAQ/ComingSoon 페이지 재구축 | 1인 풀스택 환경에서 Figma는 트리플 동기화 부담만 추가, 코드가 이미 SSoT 역할 수행 | 사용자 수 확대 → 다수 stakeholder 디자인 결정 개입 / 모바일·데스크탑 멀티 플랫폼 일관성 관리 필요성 발생 |

**기존 Figma 파일** (`AUYoLTYOsDWipKoNGfD3Fv`): 그대로 유지 (삭제 X). 향후 재개 시 시작점으로 활용 가능.

### 재개 시 시작점

향후 Phase B/C 재개가 필요해진 경우:
1. 이 §08의 보류 결정 재검토 (재평가 트리거가 실제로 도달했는지 확인)
2. `~/.claude/projects/-home-kkryo-dev-amazing-korean-api/memory/project_figma_plan.md` — 원본 하이브리드 계획 + Phase B 착수 절차 보관됨
3. Figma 유료 플랜 활성화 (Pro 이상) + MCP 한도 확인
4. Phase A 도구로 최신 PNG 재캡처 (영구 자산이라 언제든 사용 가능)

---

## 09 Changelog

### v3 — 다크모드 + 디자인 가이드라인 (2026-02-19)

**다크모드**:
- CSS 변수 이중 정의 (`:root` + `.dark`) — 전체 토큰 대응
- `next-themes` ThemeProvider 연결 (`attribute="class"`, `defaultTheme="system"`)
- `theme_toggle.tsx` 신규 — Sun/Moon 토글 + 드롭다운 (Light/Dark/System)
- 전용 Surface 토큰: `--footer`, `--surface-inverted` — `--primary` 반전 문제 해결
- 다크 shadow 오버라이드: `shadow-card` / `shadow-card-hover` → 검정 기반
- 다크 text-gradient 오버라이드: `--secondary → --accent` (라이트: `--primary → --accent`)
- Header/Footer + 공개 페이지 6개 + Admin 페이지 10개 하드코딩 색상 전면 교체
- 22개 로케일 테마 i18n 키 추가

**디자인 가이드라인 (이 문서 §01~§06 확장)**:
- Radius Scale 문서화 (Micro~Pill 6단계 + Decision Tree)
- Typography Scale 확장 (Heading Scale, Weight 규칙, Line-height)
- Shadow Scale 문서화 (sm~xl 6단계 + 다크모드 오버라이드)
- Icon Sizing Scale 문서화 (h-4~h-10 6단계)
- Container Sizes 문서화 (Default/Narrow/Form/Text)
- Button Variants & Sizes 문서화 (6 variants + 4 sizes + CTA 패턴)
- Animation & Duration 문서화 (Quick/Standard/Slow + 모션 감소)
- Grid Gap Standard 문서화 (gap-1~gap-8)
- Anti-Pattern 6개 추가 (Named colors, Footer/CTA, Radius, Shadow, Typography 혼용)
- PR 체크리스트 5개 항목 추가 (Radius, Typography, Shadow, Icon, 다크모드)

**Roadmap 정리**:
- ~~Radius 스케일 문서화~~ → v3에서 완료
- ~~다크모드 QA~~ → v3에서 완료

### v2 — 공유 컴포넌트 추출 (2026-02-19)

**신규 컴포넌트 (6개)**:
- `lib/pagination.ts` — getPageItems 유틸 + ELLIPSIS Symbol
- `sections/pagination_bar.tsx` — PaginationBar (로직+UI 통합)
- `sections/empty_state.tsx` — EmptyState (접근성: role="status", aria-hidden)
- `sections/skeleton_grid.tsx` — SkeletonGrid (variant별 스켈레톤 카드)
- `sections/list_stats_bar.tsx` — ListStatsBar (리스트 통계 바)
- `sections/stat_card.tsx` — StatCard (대시보드 KPI 전용)

**수정 (9개 파일)**:
- `ui/card.tsx` — CVA cardVariants (default/elevated/interactive + ring-offset + motion-reduce)
- `sections/hero_section.tsx` — variant prop (marketing/list), 리스트 패딩 토큰화
- `video_list_page.tsx` — Hero + Empty + Skeleton + PaginationBar + ListStatsBar 전체 교체
- `lesson_list_page.tsx` — 동일 + Card interactive
- `study_list_page.tsx` — 동일 + Card interactive + HeroSection children 필터 슬롯
- `study_detail_page.tsx` — Empty + Skeleton + PaginationBar + Card interactive
- `lesson_detail_page.tsx` — EmptyState 교체
- `admin_dashboard.tsx` — StatCard 교체
- `AMK_DESIGN_SYSTEM.md` — 이 문서 업데이트
