#!/usr/bin/env python3
"""guide 도메인 초기 시드 변환기 (1회성 부트스트랩, 2026-06-12).

books 해설집 .txt 67(세그먼트 전체) + HTML 67(표 병합 메타 + 드리프트 대조)
+ sentences.json(교재 10색 테마) + guide_sections.json(카테고리 경계)
→ seeds/guide_seed.json

설계 SoT = docs/AMK_GUIDE_CONTENT_DESIGN.md (D-0~D-8).
산출물은 콘텐츠 포함 — 커밋 금지 (.gitignore), prod 전달 = scp.

사용: python3 scripts/build_guide_seed.py [--books-dir ../amazing-korean-books]
        [--out seeds/guide_seed.json] [--report seeds/guide_seed_report.txt]
"""
import argparse
import json
import re
import sys
from collections import Counter
from pathlib import Path

from bs4 import BeautifulSoup

BLOCK = re.compile(r"(?m)^\[(\d+)_(\d+)\] \[([^\]]+)\]\nKO: (.*)\nEN: (.*)$")
MARKER = re.compile(
    r"^\((symbol_only|table_content_ko_learning|content_ko_learning"
    r"|table_content_en|empty:[^)]*)\)$"
)
COORD = re.compile(r"^T(\d+)_R(\d+)_C(\d+)$")
SENT_EN = re.compile(r"^(\d+)\)\s")
FNAME_RANGE = re.compile(r"^(\d+)~(\d+)\s")
WS = re.compile(r"\s+")

TYPE_MAP = {
    "TITLE": "title", "PARAGRAPH": "paragraph", "PATTERN": "pattern",
    "SECTION": "section", "VOCAB": "vocab", "UI": "ui", "NOTE": "note",
    "BLOCKQUOTE": "blockquote", "OTHER": "other", "TABLE_HEADER": "table_header",
    "TABLE_CELL": "table_cell", "PRACTICE": "practice",
    "LIST_ITEM": "list_item",  # 스펙 12종 외 실측 추가 (47_044/045)
}

# 교재 테마 별칭/순환 해석 (books scripts/textbook/css/themes.css)
THEME_BASE = ["blue", "green", "orange", "purple", "pink",
              "teal", "indigo", "rose", "amber", "slate"]
THEME_ALIAS = {"theme-sky": "teal", "theme-violet": "indigo",
               "theme-g10": "slate", "theme-g11": "blue", "theme-g12": "green",
               "theme-g13": "orange", "theme-g14": "purple"}
CATEGORY_MAP = {"sentence_structure": "sentence_structure", "predicate": "predicate",
                "adverbial": "adverbial", "miscellaneous": "misc"}


def resolve_theme(name: str) -> str:
    if name in THEME_ALIAS:
        return THEME_ALIAS[name]
    m = re.match(r"^theme-(\d+)$", name)
    if m:
        return THEME_BASE[(int(m.group(1)) - 1) % 10]
    return name.removeprefix("theme-")


def norm(s: str) -> str:
    return WS.sub("", s)


def parse_txt(path: Path, sent_range=None):
    """세그먼트 전체 파싱 → 블록 리스트 (마커 분리, 좌표, 문장 귀속).

    sent_range=(start,end): 문장 시작 판정 = EN 'N) ' 접두 + N이 범위 내.
    SECTION 이 정상이나 PARAGRAPH 타이핑 변칙 2건(11_003, 12_003) 실재 —
    범위 제약으로 OTHER 의 노트 번호(1)/2), 범위 밖)와 구분.
    """
    blocks = []
    current_sentence = None
    anomalies = []
    for m in BLOCK.finditer(path.read_text(encoding="utf-8")):
        unit, seq, label, ko, en = m.group(1), int(m.group(2)), m.group(3), \
            m.group(4).strip(), m.group(5).strip()
        if ":" in label:
            type_raw, coord_raw = label.split(":", 1)
            cm = COORD.match(coord_raw)
            if not cm:
                anomalies.append(f"좌표 파싱 실패: {unit}_{seq:03d} [{label}]")
                table_no = row_no = col_no = None
            else:
                table_no, row_no, col_no = int(cm.group(1)), int(cm.group(2)), int(cm.group(3))
        else:
            type_raw, table_no, row_no, col_no = label, None, None, None
        btype = TYPE_MAP.get(type_raw)
        if btype is None:
            anomalies.append(f"미지 타입: {unit}_{seq:03d} [{type_raw}]")
            continue

        ko_marker = MARKER.match(ko)
        en_marker = MARKER.match(en)
        marker = None
        if ko_marker and en_marker:
            if ko_marker.group(1) != en_marker.group(1):
                anomalies.append(
                    f"양측 마커 불일치: {unit}_{seq:03d} KO={ko} EN={en} → KO측 채택")
            marker = ko_marker.group(1)
            ko = en = None
        elif ko_marker:
            marker = ko_marker.group(1)
            ko = None
        elif en_marker:
            marker = en_marker.group(1)
            en = None

        # 문장 귀속: 범위 내 번호 접두 SECTION/PARAGRAPH 가 그룹 시작, 구획 타입이 그룹 종료
        sm = SENT_EN.match(en or "")
        in_range = (sm and sent_range
                    and sent_range[0] <= int(sm.group(1)) <= sent_range[1])
        sentence_no = None
        if btype in ("section", "paragraph") and in_range:
            if btype == "paragraph":
                anomalies.append(
                    f"문장 타이핑 변칙 PARAGRAPH→section 정규화: {unit}_{seq:03d}")
                btype = "section"
            current_sentence = int(sm.group(1))
            sentence_no = current_sentence
        elif btype in ("title", "paragraph", "practice", "section"):
            current_sentence = None  # 섹션 제목/구획 — 그룹 종료
        else:
            sentence_no = current_sentence

        blocks.append({
            "unit": unit, "seq": seq, "block_seq": seq * 10, "block_type": btype,
            "sentence_no": sentence_no, "text_ko": ko, "text_en": en,
            "marker": marker, "table_no": table_no, "row_no": row_no,
            "col_no": col_no, "col_span": None, "row_span": None,
            "legacy_key": f"guidev2:{unit}_{seq:03d}",
        })
    return blocks, anomalies


def extract_html_tables(path: Path):
    """HTML 표 전체 → {1-based 표번호: [[(colspan,rowspan,text), ...행], ...]}."""
    soup = BeautifulSoup(path.read_text(encoding="utf-8"), "lxml")
    tables = {}
    for ti, table in enumerate(soup.find_all("table"), start=1):
        rows = []
        for tr in table.find_all("tr"):
            cells = []
            for cell in tr.find_all(["th", "td"], recursive=False):
                cells.append((
                    int(cell.get("colspan", 1)), int(cell.get("rowspan", 1)),
                    norm(cell.get_text()),
                ))
            rows.append(cells)
        tables[ti] = rows
    return tables, soup


def attach_spans(blocks, html_tables, report):
    """좌표 매칭으로 col_span/row_span 부여 + 텍스트 대조 드리프트 검출."""
    txt_tables = sorted({b["table_no"] for b in blocks if b["table_no"] is not None})
    html_count = len(html_tables)
    if txt_tables and (len(txt_tables) != html_count or txt_tables[-1] != html_count):
        report.append(f"  ⚠️ 표 개수 불일치: txt={txt_tables} html={html_count}")

    matched = mismatched = span_set = 0
    for b in blocks:
        if b["table_no"] is None:
            continue
        rows = html_tables.get(b["table_no"])
        if rows is None or b["row_no"] >= len(rows) or b["col_no"] >= len(rows[b["row_no"]]):
            mismatched += 1
            report.append(f"  ⚠️ HTML 셀 없음: {b['legacy_key']} "
                          f"T{b['table_no']}_R{b['row_no']}_C{b['col_no']}")
            continue
        colspan, rowspan, html_text = rows[b["row_no"]][b["col_no"]]
        own = norm(b["text_ko"] or "") + "|" + norm(b["text_en"] or "")
        ok = True
        for side in (b["text_ko"], b["text_en"]):
            if side and norm(side) not in html_text:
                ok = False
        if ok:
            matched += 1
        else:
            mismatched += 1
            report.append(f"  드리프트(표): {b['legacy_key']} txt={own[:60]!r} "
                          f"html={html_text[:60]!r}")
        if colspan > 1:
            b["col_span"] = colspan
            span_set += 1
        if rowspan > 1:
            b["row_span"] = rowspan
            span_set += 1
    return matched, mismatched, span_set


def check_sentence_drift(blocks, soup, report):
    """문장 KO 정답 vs HTML textarea[data-ans] 전수 대조."""
    answers = {norm(t.get("data-ans", "")) for t in soup.find_all("textarea")}
    answers |= {norm(i.get("data-ans", "")) for i in soup.find_all("input") if i.get("data-ans")}
    drift = 0
    for b in blocks:
        if b["block_type"] == "section" and b["sentence_no"] is not None:
            if norm(b["text_ko"] or "") not in answers:
                drift += 1
                report.append(f"  드리프트(문장): #{b['sentence_no']} "
                              f"{b['legacy_key']} ko={b['text_ko']!r} HTML data-ans에 없음")
    return drift


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--books-dir", default=str(Path(__file__).resolve().parents[2] / "amazing-korean-books"))
    ap.add_argument("--out", default="seeds/guide_seed.json")
    ap.add_argument("--report", default="seeds/guide_seed_report.txt")
    args = ap.parse_args()

    books = Path(args.books_dir)
    txt_dir = books / "해설집"
    html_dir = books / "놀라운 한국어 500문장 해설집 20260606"

    # 테마: sentences.json 섹션 theme → 문장번호별 base 색
    sent_data = json.loads((books / "scripts/textbook/data/sentences.json").read_text(encoding="utf-8"))
    sent_color = {}
    for s in sent_data["sections"]:
        c = resolve_theme(s["theme"])
        assert c in THEME_BASE, f"미지 테마: {s['theme']} → {c}"
        for it in s["items"]:
            sent_color[it["num"]] = c

    # 카테고리: guide_sections.json range
    sections = json.loads((books / "scripts/guide-v2/data/guide_sections.json").read_text(encoding="utf-8"))
    cat_ranges = [(sec["range"][0], sec["range"][1], CATEGORY_MAP[sec["id"]])
                  for sec in sections["sections"]]

    def category_of(n):
        for lo, hi, cat in cat_ranges:
            if lo <= n <= hi:
                return cat
        raise ValueError(f"카테고리 범위 밖: {n}")

    txt_files = sorted(f for f in txt_dir.glob("*.txt") if ".before_" not in f.name)
    assert len(txt_files) == 67, f"해설집 67개 아님: {len(txt_files)}"

    # HTML 짝: 문장 범위 접두("N~M ")로 매칭 (파일명 본문은 드리프트 가능 — 실측 1건: 366~369 기타/부사어)
    html_by_range = {}
    for hf in html_dir.glob("*.html"):
        hm = FNAME_RANGE.match(hf.name)
        if hm:
            html_by_range[(int(hm.group(1)), int(hm.group(2)))] = hf

    report = [f"# guide_seed 변환 리포트", ""]
    guides = []
    all_anomalies = []
    totals = Counter()

    for tf in txt_files:
        fr0 = FNAME_RANGE.match(tf.name)
        hf = html_by_range.get((int(fr0.group(1)), int(fr0.group(2))))
        assert hf is not None, f"HTML 짝 없음: {tf.name}"
        if hf.stem != tf.stem:
            all_anomalies.append(f"파일명 드리프트(범위 일치): txt={tf.name} html={hf.name}")
        blocks, anomalies = parse_txt(tf, (int(fr0.group(1)), int(fr0.group(2))))
        all_anomalies += anomalies
        unit = blocks[0]["unit"]
        report.append(f"[unit {unit}] {tf.stem} — 블록 {len(blocks)}")

        html_tables, soup = extract_html_tables(hf)
        matched, mismatched, span_set = attach_spans(blocks, html_tables, report)
        sent_drift = check_sentence_drift(blocks, soup, report)
        report.append(f"  표 셀 대조: 일치 {matched} / 불일치 {mismatched} / span 부여 {span_set}"
                      f" / 문장 드리프트 {sent_drift}")

        sent_nos = sorted(b["sentence_no"] for b in blocks
                          if b["block_type"] == "section" and b["sentence_no"] is not None)
        fr = FNAME_RANGE.match(tf.name)
        f_start, f_end = int(fr.group(1)), int(fr.group(2))
        if (sent_nos[0], sent_nos[-1]) != (f_start, f_end):
            all_anomalies.append(
                f"문장 범위 불일치: {tf.name} 파일명 {f_start}~{f_end} vs 실측 {sent_nos[0]}~{sent_nos[-1]}")
        if len(sent_nos) != len(set(sent_nos)) or set(sent_nos) != set(range(f_start, f_end + 1)):
            all_anomalies.append(f"문장 번호 비연속/중복: {tf.name} {sent_nos}")

        colors = sorted({sent_color[n] for n in sent_nos})
        assert len(colors) == 1, f"단원 복수 테마: {tf.name} {colors}"
        cats = sorted({category_of(n) for n in sent_nos})
        assert len(cats) == 1, f"단원 복수 카테고리: {tf.name} {cats}"

        # 단원 제목: TITLE 블록, 없으면 첫 블록(PARAGRAPH 변형 3개 단원: 441/449/462~)
        title = next((b for b in blocks if b["block_type"] == "title"), None)
        if title is None:
            title = blocks[0]
            all_anomalies.append(f"TITLE 부재 → 첫 블록({title['block_type']}) 채택: {tf.name}")
        subtitle = next((b for b in blocks
                         if b["block_type"] == "paragraph" and b["seq"] > title["seq"]), None)

        guides.append({
            "guide_idx": f"guidev2-{unit}",
            "guide_seq": 0,  # 아래에서 sentence_start 순으로 부여
            "guide_category": cats[0],
            "guide_theme": colors[0],
            "sentence_start": f_start, "sentence_end": f_end,
            "title_ko": title["text_ko"], "title_en": title["text_en"],
            "subtitle_ko": subtitle["text_ko"] if subtitle else None,
            "subtitle_en": subtitle["text_en"] if subtitle else None,
            "blocks": [{k: b[k] for k in
                        ("block_seq", "block_type", "sentence_no", "text_ko", "text_en",
                         "marker", "table_no", "row_no", "col_no", "col_span", "row_span",
                         "legacy_key")} for b in blocks],
            "sentences": [{"sentence_no": b["sentence_no"], "legacy_key": b["legacy_key"]}
                          for b in blocks
                          if b["block_type"] == "section" and b["sentence_no"] is not None],
        })
        totals["blocks"] += len(blocks)
        totals["sentences"] += len(sent_nos)
        totals["span_set"] += span_set
        totals["table_mismatch"] += mismatched
        totals["sent_drift"] += sent_drift

    guides.sort(key=lambda g: g["sentence_start"])
    for i, g in enumerate(guides, start=1):
        g["guide_seq"] = i

    # 전역 정합: legacy_key 유일, 문장 1~500 완전 커버
    keys = [b["legacy_key"] for g in guides for b in g["blocks"]]
    assert len(keys) == len(set(keys)), "legacy_key 중복"
    all_sents = sorted(s["sentence_no"] for g in guides for s in g["sentences"])
    assert all_sents == list(range(1, 501)), f"문장 커버 불완전: {len(all_sents)}"

    # 납품 참조본 교차검증 (있으면): en 복원본·ko 권위원문 vs .txt
    key_block = {b["legacy_key"]: b for g in guides for b in g["blocks"]}
    for ref_name, field, ref_key in (
            ("seeds/1_en_source_reference.json", "text_en", "translated_text"),
            ("seeds/0_ko_source_reference.json", "text_ko", "ko_text")):
        p = Path(ref_name)
        if not p.exists():
            report.append(f"참조본 없음(스킵): {ref_name}")
            continue
        ref = json.loads(p.read_text(encoding="utf-8"))
        items = ref.get("translations") or ref.get("items")
        miss = diff = 0
        for it in items:
            b = key_block.get(it["id"])
            if b is None:
                miss += 1
                continue
            ref_text = it.get(ref_key)
            if ref_text is not None and b[field] is not None and \
                    norm(ref_text) != norm(b[field]):
                diff += 1
                report.append(f"  참조본 차이({field}): {it['id']} "
                              f"ref={ref_text[:50]!r} txt={b[field][:50]!r}")
        report.append(f"참조본 대조 {ref_name}: 항목 {len(items)} / 키 미존재 {miss} / 텍스트 차이 {diff}")
        totals[f"ref_diff_{field}"] = diff
        totals[f"ref_miss_{field}"] = miss

    if all_anomalies:
        report.append("\n# 변칙 목록")
        report += [f"  {a}" for a in all_anomalies]

    out = {
        "meta": {
            "source": "amazing-korean-books 해설집/*.txt v6 + HTML 20260606",
            "dataset": "guide500",
            "generated_by": "scripts/build_guide_seed.py",
            "guide_count": len(guides),
            "block_count": totals["blocks"],
            "sentence_count": totals["sentences"],
        },
        "guides": guides,
    }
    Path(args.out).write_text(json.dumps(out, ensure_ascii=False, indent=1), encoding="utf-8")
    summary = (f"guides={len(guides)} blocks={totals['blocks']} sentences={totals['sentences']} "
               f"span={totals['span_set']} 표불일치={totals['table_mismatch']} "
               f"문장드리프트={totals['sent_drift']} 변칙={len(all_anomalies)}")
    report.insert(1, summary)
    Path(args.report).write_text("\n".join(report) + "\n", encoding="utf-8")
    print(summary)
    print(f"저장: {args.out} / 리포트: {args.report}")
    if all_anomalies:
        print(f"⚠️ 변칙 {len(all_anomalies)}건 — 리포트 확인")
        sys.exit(2)


if __name__ == "__main__":
    main()
