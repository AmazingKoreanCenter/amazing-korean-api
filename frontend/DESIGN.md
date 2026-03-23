# Design System: Amazing Korean

> Source of truth for prompting Stitch (and other AI design tools) to generate screens
> consistent with the Amazing Korean brand. Import this file into Stitch projects
> or feed it to coding agents via the MCP server.

---

## 1. Visual Theme & Atmosphere

Amazing Korean is a **premium yet approachable** online Korean-language learning platform
for global learners aged 20-40. The visual language sits between Duolingo's playfulness
and Coursera's professionalism — confident, clean, and warm.

**Mood keywords**: Trustworthy, scholarly, modern, warm, inviting.

The interface uses **expansive whitespace** with a deep navy foundation that evokes
academic authority, softened by cyan accents and gentle gradient washes that feel
welcoming rather than corporate. Dark mode inverts gracefully — the navy deepens
while accent colors brighten slightly for readability.

**Density**: Medium — generous section spacing (64-128px) with compact card interiors.
Content breathes, but pages don't feel empty.

---

## 2. Color Palette & Roles

### Core Brand

| Role | Name | Light Hex | Dark Hex | When to Use |
|------|------|-----------|----------|-------------|
| Primary | Deep Navy Blue | `#041D57` | `#F7F9FB` (inverted) | Main navigation, primary buttons, headings, brand anchoring |
| Secondary | Bright Cornflower | `#4A75EC` | `#4A75EC` (stable) | Secondary buttons, gradient start color, supporting UI |
| Accent | Vivid Cyan | `#129FD7` | `#129FD7` (stable) | Emphasis icons, links, gradient end color, interactive highlights |

### Surfaces

| Role | Name | Light Hex | Dark Hex | When to Use |
|------|------|-----------|----------|-------------|
| Background | Pure White | `#FFFFFF` | `#060A12` | Page background |
| Card | Pure White | `#FFFFFF` | `#0A101D` | Card surfaces, popover panels |
| Muted | Whisper Blue-Gray | `#F4F5F9` | `#1D2839` | Disabled states, subtle section fills, input backgrounds |
| Brand Soft | Barely-There Lavender | `#EFF2FF` | `#0D1425` | Hero gradient start — gentle wash, never solid fill |
| Brand Soft Alt | Barely-There Sky | `#EAF6FF` | `#10182C` | Hero gradient end — pairs with Brand Soft |

### Status & Semantic

| Role | Name | Light Hex | Dark Hex | When to Use |
|------|------|-----------|----------|-------------|
| Success | Deep Emerald | `#0B835B` | `#0EA875` | Completion, active states, positive actions |
| Warning | Vivid Amber | `#F49E0A` | `#F5A822` | Premium content markers, caution alerts |
| Info | Royal Blue | `#1A6DF4` | `#327DF5` | Informational badges, neutral notifications |
| Destructive | Crimson Red | `#EA1414` | `#801D1D` | Delete actions, error states, validation failures |

### Always-Dark Surfaces

These tokens **never invert** — they stay dark in both light and dark mode,
ensuring consistent contrast for footer and CTA sections.

| Role | Name | Light Hex | Dark Hex | When to Use |
|------|------|-----------|----------|-------------|
| Footer | Deep Navy | `#041D57` | `#0A101D` | Footer background — always dark tone |
| Surface Inverted | Deep Navy | `#041D57` | `#0D1425` | CTA banners, promotional sections — always dark tone |

### Gradients

| Name | Composition | When to Use |
|------|-------------|-------------|
| Hero Gradient | Brand Soft → Background → Brand Soft Alt (diagonal) | Hero section backgrounds — a delicate wash, not a bold stripe |
| Primary Gradient | Secondary (#4A75EC) → Accent (#129FD7) (horizontal) | CTA buttons, icon containers, promotional badges |
| Text Gradient | Light: Primary → Accent / Dark: Secondary → Accent | Brand text emphasis — sparingly, only hero or marketing headings |

### Do / Don't

- **Do** use the Primary color sparingly — it's the anchor, not the flood.
- **Do** pair Secondary and Accent in gradients — they harmonize naturally.
- **Don't** hardcode hex values in code — always reference CSS variables or Tailwind tokens.
- **Don't** use Primary (navy) as a solid background in dark mode — it inverts to near-white. Use `footer` or `surface-inverted` tokens instead.
- **Don't** mix warm and cool status colors in the same component — pick one semantic family.

---

## 3. Typography Rules

### Font Family

**Pretendard Variable** — a Korean-optimized variable font with clean geometric forms
and excellent CJK rendering. Falls back through the system font stack:
`Pretendard Variable, Pretendard, -apple-system, BlinkMacSystemFont, system-ui, Roboto, Helvetica Neue, sans-serif`

### Heading Hierarchy

| Level | Size (responsive) | Weight | Letter-spacing | When to Use |
|-------|-------------------|--------|----------------|-------------|
| Hero (marketing) | 36px → 48px → 60px | Bold (700) | Tight (-0.025em) | Landing page hero, one per page maximum |
| Hero (list) | 30px → 36px | Bold (700) | Tight | List page headers, detail page titles |
| Section | 30px → 36px | Bold (700) | Tight | Feature section headings on marketing pages |
| Subsection | 20px | Semibold (600) | Tight | Section subheadings, group labels |
| Card Title | 18px | Semibold (600) | Tight | List card titles, dialog headers |
| Admin Title | 24px | Bold (700) | Tight | Admin panel page titles |

### Body Text

| Variant | Size | Weight | Line-height | When to Use |
|---------|------|--------|-------------|-------------|
| Body | 16px (base) | Normal (400) | 1.625 (relaxed) | Paragraphs, descriptions, long-form content |
| Label | 14px (sm) | Medium (500) | 1.5 | Form labels, navigation items, button text |
| Caption | 12px (xs) | Normal (400) | 1.5 | Timestamps, metadata, helper text |

### Rules

- All headings get `tracking-tight` (-0.025em letter-spacing) automatically.
- Korean text uses `word-break: keep-all` (`break-keep`) — words never split mid-syllable.
- Hero headlines use `leading-none` (line-height: 1.0) for dramatic compact stacking.
- Body text uses `leading-relaxed` (1.625) for comfortable reading.
- **Never** use more than 3 weight levels on a single screen.

---

## 4. Component Stylings

### Buttons

| Variant | Style | When to Use |
|---------|-------|-------------|
| Default | Deep Navy fill, white text | Primary actions — submit, confirm, navigate |
| Secondary | Cornflower Blue fill, white text | Supporting actions alongside a primary button |
| Outline | Transparent with border, dark text | Cancel, back, secondary choices |
| Ghost | Transparent, hover reveals accent tint | Inline actions, navigation items, icon-only buttons |
| Destructive | Crimson fill, white text | Delete, remove — always requires confirmation |
| Link | Text-only with underline | Inline text navigation |

**Sizes**: Small (h-32px, px-12, text-xs), Default (h-36px, px-16), Large (h-40px, px-32), Icon (36x36px square).

**CTA Pattern**: Hero and marketing CTAs use `gradient-primary` fill with `rounded-full` (pill shape),
h-56px, px-32, white text, and a subtle shadow that deepens on hover. Always include a trailing arrow icon.

**Transitions**: All buttons use `transition-all duration-300` — never instant state changes.

### Cards

| Variant | Style | When to Use |
|---------|-------|-------------|
| Default | Generously rounded corners (12px), thin border, standard shadow | Static content display, information panels |
| Elevated | No border, brand-tinted diffused shadow, extra-rounded (16px) | Visual emphasis — stats, featured content, skeleton containers |
| Interactive | Elevated + lift on hover (-4px translate), deeper shadow, keyboard focus ring | Clickable list items — always wrapped in a `<Link>` |

**Interactive behaviors**:
- Hover: gentle lift (-4px), shadow deepens, title color shifts to primary.
- Focus: 2px ring in ring color with 2px offset.
- Active: returns to baseline (0px translate) for tactile click feedback.
- Reduced motion: all transforms and transitions disabled.

### Badges

Seven semantic variants: Default (navy), Secondary (cornflower), Destructive (crimson),
Success (emerald), Warning (amber), Info (royal blue), Outline (border-only).

All badges use `rounded-full` (pill shape). For subtle inline status, use 10% opacity
background with full-strength text color: `bg-status-success/10 text-status-success`.

### Inputs & Forms

- All form controls use `rounded-md` (8px radius).
- Border color matches the `border` token — subtle blue-gray.
- Focus state: 2px ring in primary color.
- Error state: `border-destructive` with red error message below.
- On iOS, all inputs render at 16px minimum to prevent zoom.

### Dialogs & Overlays

- `rounded-lg` (10px radius), standard shadow.
- Semi-transparent backdrop overlay.
- Content uses the same card surface color.

---

## 5. Layout Principles

### Container Widths

| Token | Max Width | When to Use |
|-------|-----------|-------------|
| Default | 1350px | Standard page content — card grids, sections, navigation |
| Narrow | 768px (max-w-3xl) | Legal pages, long-form text — optimized line length |
| Form | 448px (max-w-md) | Authentication pages — login, signup, password reset |
| Text | 672px (max-w-2xl) | Hero subtitles, description paragraphs — readability sweet spot |

All containers are horizontally centered with `px-24` (desktop) / `px-16` (mobile) side padding.

### Section Spacing

| Token | Value | When to Use |
|-------|-------|-------------|
| Section SM | 40px | Sub-sections, list content areas |
| Section MD | 64px | Standard section padding — the default for most sections |
| Section LG | 80px | Major sections, feature blocks |
| Hero LG | 128px | Hero sections only — creates dramatic vertical breathing room |

**Rule**: Section spacing is **fixed, not responsive** — 64px stays 64px on all breakpoints.
Only Hero LG scales (80px mobile → 128px desktop).

### Grid System

- **Card grids**: 1 column (mobile) → 2 columns (md: 768px) → 3 columns (lg: 1024px),
  with `gap-24` (1.5rem) standard spacing.
- **Feature grids**: Same breakpoints but with `gap-32` (2rem) for larger cards.
- **Admin tables**: Full-width within the content area, no max-width constraint.

### Responsive Breakpoints

| Name | Width | Key Changes |
|------|-------|-------------|
| Base | 0px | Single column, hamburger nav, compact spacing |
| SM | 640px | Minor layout adjustments |
| MD | 768px | 2-column grids, expanded navigation visible |
| LG | 1024px | 3-column grids, full desktop navigation, hero enlarges |
| XL | 1280px | Maximum content expansion |

### Page Templates

| Template | Structure | Used By |
|----------|-----------|---------|
| Marketing | Hero (gradient bg, centered text, CTA) → Feature sections → CTA banner | Home, About, Pricing |
| List | Hero (gradient bg, left-aligned, filter row) → Card grid → Pagination | Videos, Studies, Lessons |
| Detail | Hero (gradient bg, breadcrumb) → Content area | Video detail, Lesson detail |
| Form | Centered card (max-w-md), no hero | Login, Signup, Password reset |
| Admin | Fixed sidebar (w-256px) + scrollable main content | All admin pages |

---

## 6. Iconography

**Icon library**: Lucide React — consistent 24px stroke icons with 1.5px stroke width.

| Context | Size | Example |
|---------|------|---------|
| Inline / Button | 16x16px | Navigation arrows, button trailing icons |
| CTA / Hero badge | 20x20px | Arrow-right in CTA buttons, hero badge icons |
| Navigation toggle | 24x24px | Hamburger menu, mail icon |
| Feature card | 28x28px | Inside gradient icon containers |
| Value card | 32x32px | About page value proposition icons |
| Empty state | 40x40px | Large status icons in empty state displays |

**Icon containers**: Feature card icons sit inside a 48x48px container with `gradient-primary`
background, `rounded-xl` (12px), and scale up on card group-hover (`scale-110`).

---

## 7. Animation & Motion

| Speed | Duration | When to Use |
|-------|----------|-------------|
| Quick | 200ms | Hover color shifts, simple state toggles |
| Standard | 300ms | Card lifts, fade-ins, slide transitions — **the default** |
| Slow | 500ms | Progress bars, complex multi-step transitions |

**Default transition**: `transition-all duration-300` on all interactive elements.

**Enter animations**: `animate-in fade-in duration-300` for state changes (empty state appearance, content reveal).

**Reduced motion**: Interactive cards respect `prefers-reduced-motion` —
all transforms and transitions become instant.

**Scroll effects**: Header gains `backdrop-blur` and shadow on scroll (sticky, z-50).

---

## 8. Dark Mode Behavior

The design system supports **full dark mode** via class-based toggling (`class="dark"` on root).

### Key Inversions

| Element | Light | Dark | Notes |
|---------|-------|------|-------|
| Background | Pure White | Near-Black Navy (#0D1321) | Deep, not pure black — less eye strain |
| Primary | Deep Navy → white text | Near-White → navy text | Full inversion for buttons/headings |
| Cards | White surface | Dark Navy (#121B2E) | Subtle distinction from background |
| Shadows | Primary-tinted (brand feel) | Pure black-based | Prevents white glow artifact |
| Status colors | Standard saturation | +8% lightness boost | Maintains WCAG AA on dark backgrounds |
| Text gradient | Primary → Accent | Secondary → Accent | Avoids invisible near-white-on-white |
| Footer / CTA surfaces | Deep Navy | Slightly lighter dark | Always dark — never fully inverts |

### Don't

- **Don't** use `bg-primary` for always-dark sections — it inverts. Use `footer` or `surface-inverted`.
- **Don't** assume shadows look the same — dark mode overrides brand shadows with black.
- **Don't** reduce contrast below WCAG AA (4.5:1) in dark mode.

---

## 9. Accessibility Standards

- **Color contrast**: All text/background pairs meet **WCAG AA (4.5:1)** minimum.
- **Focus indicators**: 2px ring with offset on all interactive elements.
- **Keyboard navigation**: Tab order follows visual layout. Cards with interactive variant support Enter/Space activation.
- **Reduced motion**: `motion-reduce:transition-none motion-reduce:transform-none` on animated elements.
- **Screen readers**: `sr-only` labels on icon-only buttons. `role="status"` on empty states. `aria-current="page"` on active pagination.
- **Korean text**: `word-break: keep-all` prevents mid-syllable line breaks.
- **iOS input zoom**: All inputs render at 16px minimum font size.

---

## 10. Internationalization

The platform supports **21 languages** with dynamic font loading for CJK and special scripts.
All user-facing text is externalized via i18n keys — never hardcode display strings.

**Supported languages**: Korean, English, Japanese, Chinese (Simplified/Traditional), Vietnamese,
Thai, Indonesian, Myanmar, Mongolian, Russian, Spanish, Portuguese, French, German, Hindi,
Nepali, Sinhala, Khmer, Uzbek, Kazakh, Tajik.

**Admin pages** are Korean-only (intentional — internal tool).

---

## Stitch Prompting Notes

When generating screens for Amazing Korean in Stitch, keep these rules in mind:

1. **Always specify "web, desktop, 1350px max-width"** — Stitch defaults to mobile.
2. **Reference the Deep Navy + Cyan accent palette** — don't let Stitch pick its own colors.
3. **Use Pretendard font** — specify explicitly or Stitch may default to Inter/Roboto.
4. **Hero sections need the soft gradient wash** — not a solid color fill. Describe as "barely-there lavender-to-sky diagonal gradient."
5. **Cards should lift on hover** — mention "interactive card with gentle hover lift and shadow deepening."
6. **CTA buttons are pill-shaped with gradient** — say "rounded-full gradient button from cornflower blue to cyan."
7. **Footer is always dark navy** regardless of the page theme.
8. **Korean text should not break mid-word** — if showing Korean content, mention `word-break: keep-all`.
