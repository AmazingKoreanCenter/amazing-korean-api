import type { GuideItem } from "../types";
import { GuideTable } from "./GuideTable";

/**
 * 블록 스트림 렌더 (단원 도입부 = 문장 귀속 없는 블록 + 표).
 * block_type 별 표현. 마커 전용 블록(symbol_only/empty)·빈 텍스트는 스킵.
 */

function BlockView({ item }: { item: GuideItem }) {
  if (item.kind === "table") {
    return item.rows ? <GuideTable rows={item.rows} /> : null;
  }
  // 마커 전용 / 빈 블록 스킵
  if (item.marker === "symbol_only" || item.marker === "empty:짝" || item.marker === "empty:헤더") {
    return null;
  }
  const text = item.text ?? "";
  const ko = item.text_ko;
  const showKo = ko && ko !== text;

  switch (item.block_type) {
    case "title":
      return null; // 제목은 페이지 헤더에서 별도 표시
    case "paragraph":
      return (
        <p className="text-sm font-medium text-primary mt-3 first:mt-0">
          {text}
          {showKo && <span className="block text-xs text-muted-foreground font-normal">{ko}</span>}
        </p>
      );
    case "pattern":
      return text || showKo ? (
        <div className="inline-flex flex-col items-start rounded-md bg-muted px-3 py-1.5 mr-2 mb-2 text-sm">
          {text && <span className="font-medium text-foreground">{text}</span>}
          {showKo && <span className="text-xs text-muted-foreground">{ko}</span>}
        </div>
      ) : null;
    case "note":
    case "blockquote":
      return text ? (
        <div className="rounded-md border-l-2 border-primary/40 bg-muted/40 px-3 py-2 my-2 text-sm text-muted-foreground">
          {text}
          {showKo && <span className="block text-xs">{ko}</span>}
        </div>
      ) : null;
    case "list_item":
      // 플랫 스트림이라 <ul> 부모가 없음 → div+불릿으로 HTML 유효성 유지
      return text ? (
        <div className="my-1 ml-4 flex items-start gap-1.5 text-sm text-muted-foreground">
          <span className="select-none">•</span>
          <span>{text}</span>
        </div>
      ) : null;
    case "other":
    default:
      return text ? (
        <p className="text-sm leading-relaxed text-muted-foreground my-2">
          {text}
          {showKo && <span className="block text-xs">{ko}</span>}
        </p>
      ) : null;
  }
}

/** 문장 귀속 없는 도입부 블록만 렌더 (sentence_no == null) */
export function GuideIntro({ items }: { items: GuideItem[] }) {
  const intro = items.filter((it) => it.sentence_no == null);
  if (!intro.length) return null;
  return (
    <section className="mb-6">
      {intro.map((it) => (
        <BlockView key={`${it.kind}-${it.block_seq}`} item={it} />
      ))}
    </section>
  );
}

/** 특정 문장에 귀속된 블록(어휘·해설표 등) 렌더 */
export function GuideSentenceBlocks({
  items,
  sentenceNo,
}: {
  items: GuideItem[];
  sentenceNo: number;
}) {
  // 해당 문장의 블록 중 section(문장 자체)·ui 라벨은 제외, vocab/표/해설만
  const blocks = items.filter(
    (it) =>
      it.sentence_no === sentenceNo &&
      it.block_type !== "section" &&
      it.block_type !== "ui"
  );
  if (!blocks.length) return null;
  return (
    <div className="mt-2">
      {blocks.map((it) => (
        <BlockView key={`${it.kind}-${it.block_seq}`} item={it} />
      ))}
    </div>
  );
}
