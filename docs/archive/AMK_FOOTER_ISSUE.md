# 내지 풋터 미러 마진 이슈 분석 (2026-03-13)

## 발단

KM 학생용 내지 PDF 최종 검수 중 **3페이지 풋터**가 다른 페이지와 다르게 보이는 현상 발견:
- 페이지 번호 '3'의 위치가 다른 홀수 페이지와 다름
- 풋터 구분선(border-top)이 다른 페이지보다 더 김

## 풋터 미러 마진 시스템

### page.css 규칙 (SSoT)

```css
/* 홀수 페이지: 왼쪽(제본측) 25mm, 오른쪽 18mm */
[data-mirror="odd"] .page-footer {
  left: 25mm !important;
  right: 18mm !important;
}

/* 짝수 페이지: 왼쪽 18mm, 오른쪽(제본측) 25mm */
[data-mirror="even"] .page-footer {
  left: 18mm !important;
  right: 25mm !important;
}
```

이 규칙은 일반 CSS와 `@media print` 양쪽 모두에 `!important`로 선언되어 있음.

### per-page CSS 생성 경로

```
page_styles.json → css_builder.js → <style> 블록 (빌드된 HTML)
```

- `footer` 카테고리 → **일반 CSS**로 출력 (per-page 셀렉터 `#pgXX .page-footer`)
- `print` 카테고리 → **@media print** 블록으로 출력
- css_builder.js는 `!important`를 자동 추가하지 않음 — JSON 값 그대로 출력

## 조사 결과

### 1. page_styles.json의 `left`/`right` 하드코딩 (44개 페이지)

page_styles.json의 거의 모든 페이지(44/45)에 `.page-footer`의 `left`/`right`가 하드코딩되어 있음:

| 값 | 해당 페이지 | 개수 |
|---|---|---|
| `left:18mm, right:18mm` | pg00~pg26, pg35~pg43, pgPreface | 35개 |
| `left:15mm, right:15mm` | pg27, pg29, pg31, pg33 | 4개 |
| `left:20mm, right:20mm` | pg28, pg30, pg32 | 3개 |
| `left:18mm, right:18mm` (print) | pg01 | 1개 |
| 하드코딩 없음 | pg34 | 1개 |

**결론: 이 `left`/`right` 하드코딩은 무해함.**

이유: per-page CSS의 `left`/`right`에는 `!important`가 없고, page.css의 미러 마진 규칙에는 `!important`가 있음. CSS 규칙상 `!important`가 항상 이기므로, 이 하드코딩된 값들은 실질적으로 무시됨 (dead CSS).

### 2. 진짜 원인: pg01의 `width: 174mm` (유일한 문제)

`page_styles.json` → `pg01` → `print` → `.page-footer`:

```json
".page-footer": {
    "position": "absolute",
    "bottom": "10mm",
    "left": "18mm",
    "right": "18mm",
    "width": "174mm"
}
```

**전체 page_styles.json에서 `.page-footer`에 `width`를 지정한 페이지는 pg01이 유일함.**

빌드된 HTML(KM, RU 모두 확인)에서도 동일:

```css
@media print {
  #pg01 .page-footer {
    position: absolute;
    bottom: 10mm;
    left: 18mm;
    right: 18mm;
    width: 174mm;
  }
}
```

### 3. 문제 메커니즘

pg01 = 물리적 **3페이지** (`data-mirror="odd"`, `data-page-num="3"`)

CSS absolute positioning에서 `left`, `right`, `width`가 모두 지정되면 over-constrained 상태:

```
적용되는 값:
  left  = 25mm  (page.css !important → 승리)
  right = 18mm  (page.css !important → 승리)
  width = 174mm (pg01 print CSS, 경쟁 규칙 없음 → 그대로 적용)

LTR 레이아웃에서 over-constrained 해결:
  left + width = 25mm + 174mm = 199mm
  right가 무시됨 → 실제 right = 210mm - 199mm = 11mm

결과:
  풋터 폭 = 174mm (25mm ~ 199mm)
```

정상이어야 할 값:

```
  left = 25mm, right = 18mm, width = auto
  풋터 폭 = 210mm - 25mm - 18mm = 167mm (25mm ~ 192mm)
```

**차이: 174mm - 167mm = 7mm 더 넓음 → 풋터 구분선이 다른 페이지보다 길게 보임**

### 4. 다른 페이지와의 비교

| wrapper | 물리 페이지 | normal CSS footer | @media print footer | width |
|---------|------------|-------------------|-------------------|-------|
| pg01 | 3 | margin-top:auto (flex) | position:absolute, left/right/width | **174mm** |
| pg02 | 4 | position:absolute, left:18mm, right:18mm | border-top, color (코스메틱만) | 없음 |
| pg03 | 5 | position:absolute, left:18mm, right:18mm | border-top, color (코스메틱만) | 없음 |

pg01만 유일하게 @media print에서 `width`를 지정하고, 다른 모든 페이지는 `width: auto`로 `left`/`right`에 의해 자연스럽게 폭이 결정됨.

## Wrapper → 물리 페이지 매핑

| Wrapper | 물리 페이지 |
|---------|------------|
| pgPreface | 1 |
| pg00 | 2 |
| **pg01** | **3** |
| pg02 | 4 |
| pg03 | 5 |
| pg04 | 6 |
| pg05 | 7 |
| pg06 | 8 |
| pg07 | 9 |
| pg08 | 10 |
| pg09 | 11 |
| pg10 | 12 |
| pg11 | 13 |
| pg12 | 14 |
| pg13 | 15 |
| pg14 | 16 |
| pg15 | 17 |
| pg16 | 18 |
| pg17 | 19 |
| pg18 | 20 |
| pg19 | 21 |
| pg20 | 22 |
| pg21 | 23 |
| pg22 | 24 |
| pg23 | 25 |
| pg24 | 26~35 |
| pg25 | 36~38 |
| pg26 | 39 |
| pg27 | 40 |
| pg28 | 41~49 |
| pg29 | 50 |
| pg30 | 51~71 |
| pg31 | 72 |
| pg32 | 73~93 |
| pg33 | 94 |
| pg34 | 95~115 |
| pg35 | 116 |
| pg36 | 117 |
| pg37 | 118 |
| pg38 | 119 |
| pg39 | 120 |
| pg40 | 121 |
| pg41 | 122 |
| pg42 | 123 |
| pg43 | 124 |

## 수정 방안

### 변경 사항

- **파일**: `scripts/textbook/data/page_styles.json`
- **위치**: `pg01` → `print` → `.page-footer`
- **내용**: `"width": "174mm"` 제거
- **영향 범위**: 물리적 3페이지 (전 22개 언어 × 2에디션 공통)
- **다른 페이지 영향**: 없음

### 수정 후 결과

```
수정 후:
  left  = 25mm  (page.css !important)
  right = 18mm  (page.css !important)
  width = auto  (left/right에 의해 자동 계산)
  풋터 폭 = 210mm - 25mm - 18mm = 167mm ← 다른 홀수 페이지와 동일
```

### 검증 체크리스트

수정 후 아래 사항 확인 필요:

- [ ] 단일 언어(예: KM) 학생용 내지 PDF 재빌드
- [ ] 3페이지 풋터 구분선 길이가 5페이지(pg03)와 동일한지 비교
- [ ] 3페이지 페이지 번호 위치가 다른 홀수 페이지와 동일한지 확인
- [ ] 전체 22언어 × 2에디션 재빌드
- [ ] 무작위 3~5개 언어에서 3페이지 풋터 시각 확인
