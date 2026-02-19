# Amazing Korean Design System

> tailwind.config.js + index.css = Single Source of Truth.
> 이 문서는 코드에 정의된 토큰/컴포넌트의 **사용법과 금지 규칙**을 설명한다.

---

## 01 Foundations

### Color Tokens

| 역할 | CSS 변수 | Tailwind 클래스 | 용도 |
|------|----------|----------------|------|
| Primary | `--primary` | `bg-primary`, `text-primary` | 메인 액션, 네비게이션 |
| Secondary | `--secondary` | `bg-secondary` | 보조 버튼, 서브 UI |
| Accent | `--accent` | `bg-accent`, `text-accent` | 강조 포인트, 아이콘 |
| Destructive | `--destructive` | `bg-destructive` | 삭제, 에러 (= error) |
| Brand Soft | `--brand-soft` | `bg-brand-soft` | Hero 그라데이션 시작색 |
| Brand Soft Alt | `--brand-soft-alt` | `bg-brand-soft-alt` | Hero 그라데이션 끝색 |
| Success | `--success` | `bg-status-success` | 완료, 활성, 프로모션 |
| Warning | `--warning` | `bg-status-warning` | 유료 콘텐츠, 주의 |
| Info | `--info` | `bg-status-info` | 정보 알림 |
| Footer | `--footer` | `bg-footer` | Footer (항상 어두운 톤) |
| Surface Inverted | `--surface-inverted` | `bg-surface-inverted` | CTA 섹션 (항상 어두운 톤) |

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

**Surface Tokens (항상 어두운 배경)**:

Footer, CTA 섹션 등 항상 어두운 배경이 필요한 곳에 사용. `bg-primary`는 다크모드에서 반전되므로 직접 사용 금지.

| 토큰 | 라이트 HSL | 다크 HSL | 용도 |
|------|-----------|----------|------|
| footer | `222 90% 18%` | `222 47% 8%` | Footer 배경 |
| footer-foreground | `210 40% 98%` | `210 40% 98%` | Footer 텍스트 |
| surface-inverted | `222 90% 18%` | `222 47% 10%` | CTA 섹션 배경 |
| surface-inverted-foreground | `210 40% 98%` | `210 40% 98%` | CTA 섹션 텍스트 |

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

| 티어 | 클래스 | 값 (--radius: 0.625rem 기준) | 용도 |
|------|--------|-----|------|
| Micro | `rounded-sm` | ~6px | shadcn 내부 프리미티브 (직접 사용 금지) |
| Base | `rounded-md` | ~8px | 폼 컨트롤 (Input, Button, Select) |
| Panel | `rounded-lg` | 10px | 패널, 통계 카드, Alert, Dialog |
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

| 토큰 | 용도 | 다크모드 비고 |
|------|------|-------------|
| `shadow-sm` | 폼 컨트롤 (resting) | Tailwind 기본 |
| `shadow` (Tailwind 기본) | Card default | Tailwind 기본 |
| `shadow-card` | Card elevated (브랜드 컬러 기반) | 검정 기반으로 오버라이드 (흰색 글로우 방지) |
| `shadow-card-hover` | Card interactive hover | 검정 기반으로 오버라이드 |
| `shadow-lg` | 플로팅 오버레이 (Dropdown, Toast) | Tailwind 기본 |
| `shadow-xl` | CTA 버튼 hover | Tailwind 기본 |

**다크모드 shadow**: `shadow-card` / `shadow-card-hover`는 라이트에서 `--primary` 기반 (브랜드 느낌), 다크에서 `hsl(0 0% 0%)` 기반 (자연스러운 어둠)으로 자동 전환. `.dark` 셀렉터로 오버라이드.

### Typography

**Font**: Pretendard Variable (한글 최적화), `tracking-tight` 제목 자동 적용.

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

| 컨텍스트 | 아이콘 크기 | 예시 |
|---------|-----------|------|
| Inline / Button | `h-4 w-4` | nav 아이콘, 버튼 내부 아이콘 |
| CTA / Hero badge | `h-5 w-5` | ArrowRight, HeroSection 배지 아이콘 |
| Nav toggle | `h-6 w-6` | 햄버거 메뉴, 메일 아이콘 |
| Feature card | `h-7 w-7` | gradient 컨테이너 내부 아이콘 |
| Value card | `h-8 w-8` | about 페이지 가치 카드 |
| EmptyState | `h-10 w-10` | EmptyState 컴포넌트 아이콘 |

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

| Gap | 용도 |
|-----|------|
| `gap-1` ~ `gap-2` | 인라인 요소 (아이콘+텍스트, 배지 그룹) |
| `gap-4` | 폼 필드, compact 리스트 |
| `gap-6` | 카드 그리드 (**표준**, 대부분의 리스트 페이지) |
| `gap-8` | Feature/마케팅 그리드 (큰 카드) |

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

### Container Sizes

| 토큰 | 값 | 용도 |
|------|-----|------|
| Default | `max-w-[1350px]` | SectionContainer 기본 (카드 그리드 등) |
| Narrow | `max-w-3xl` | Legal 페이지, SectionContainer `container="narrow"` |
| Form | `max-w-md` | 인증 페이지 (로그인, 회원가입) |
| Text | `max-w-2xl` | Hero subtitle, 설명문 (가독성 최적화) |

### Page Templates

- **목록 페이지**: `bg-hero-gradient` 헤더 + `SectionContainer` 콘텐츠
- **상세 페이지**: `bg-hero-gradient` 헤더 + 내용 영역
- **폼 페이지**: 중앙 정렬 Card + max-w-md

---

## 03 Components

### HeroSection

`components/sections/hero_section.tsx` — Hero 블록. `variant` prop으로 마케팅/리스트 레이아웃 전환.

```tsx
import { HeroSection } from "@/components/sections/hero_section";

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
| `link` | 텍스트만, 밑줄 | 텍스트 링크 |

#### Sizes

| Size | 높이 | 용도 |
|------|------|------|
| `sm` | `h-8 px-3 text-xs` | 테이블 내부, 보조 버튼 |
| `default` | `h-9 px-4 py-2` | 일반 액션 |
| `lg` | `h-10 px-8` | 강조 액션 |
| `icon` | `h-9 w-9` | 아이콘 전용 버튼 |

#### CTA 패턴

Hero, 마케팅 섹션의 주요 CTA 버튼:

```tsx
<Button className="gradient-primary rounded-full h-14 px-8 text-white shadow-lg hover:shadow-xl transition-all duration-300">
  시작하기 <ArrowRight className="ml-2 h-5 w-5" />
</Button>
```

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

`components/sections/pagination_bar.tsx` — 페이지네이션 로직+UI 통합 컴포넌트.

```tsx
import { PaginationBar } from "@/components/sections/pagination_bar";

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

`components/sections/empty_state.tsx` — 데이터 없음 상태 표시.

```tsx
import { EmptyState } from "@/components/sections/empty_state";

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

`components/sections/skeleton_grid.tsx` — 로딩 스켈레톤 그리드.

```tsx
import { SkeletonGrid } from "@/components/sections/skeleton_grid";

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

`components/sections/list_stats_bar.tsx` — 리스트 페이지 통계 바.

```tsx
import { ListStatsBar } from "@/components/sections/list_stats_bar";

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

`components/sections/stat_card.tsx` — **대시보드 KPI 카드** 전용.

```tsx
import { StatCard } from "@/components/sections/stat_card";

<StatCard icon={Users} label="총 사용자" value={1234} loading={false} />
```

**Props**:
| Prop | 타입 | 설명 |
|------|------|------|
| icon | `LucideIcon` | 아이콘 |
| label | `string` | 라벨 |
| value | `number \| string?` | 표시값 |
| loading | `boolean?` | 스켈레톤 표시 |

> **용도 제한**: Admin Dashboard KPI 카드 목적. `sections/` 폴더의 범용 컴포넌트화 방지.

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

### Tailwind Named Colors (다크모드 미지원)

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

### Footer/CTA 하드코딩

```
❌ bg-primary text-white   → ✅ bg-footer text-footer-foreground (Footer)
❌ bg-primary text-white   → ✅ bg-surface-inverted text-surface-inverted-foreground (CTA)
```

### Radius 혼용

```
❌ rounded-lg (일반 Card)  → ✅ Card variant="default" (자동 rounded-xl)
❌ rounded-xl (Dialog)     → ✅ rounded-lg (Dialog/Alert 표준)
❌ rounded-md (마케팅 카드) → ✅ rounded-2xl (Surface 티어)
```

### Shadow 혼용

```
❌ shadow-md (카드 강조)   → ✅ shadow-card (브랜드 shadow, 다크모드 대응)
❌ shadow-lg (카드 hover)  → ✅ shadow-card-hover (다크모드 대응)
❌ [custom box-shadow]     → ✅ CSS 유틸 클래스 사용 (shadow-card 등)
```

### Typography 혼용

```
❌ text-5xl (리스트 Hero)  → ✅ text-3xl md:text-4xl (list variant 표준)
❌ font-bold (CardTitle)   → ✅ font-semibold (CardTitle 표준)
❌ font-normal (라벨)      → ✅ font-medium (라벨 표준)
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

## 08 Changelog

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
