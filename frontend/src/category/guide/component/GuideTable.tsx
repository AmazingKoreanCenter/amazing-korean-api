import type { GuideCell } from "../types";

/**
 * 서버 재조립 격자(rows: GuideCell[][])를 HTML 표로 렌더 (D-7).
 * R0 = colspan 배너(어순 라벨), 이후 헤더/셀. 마커 셀(symbol_only 등)은 빈 칸 처리.
 * 셀은 표시 언어 text + 한국어 학습칸 text_ko 를 함께 보여줌(이중언어).
 */

function cellContent(cell: GuideCell) {
  if (cell.marker === "symbol_only") {
    return <span className="text-muted-foreground/50">·</span>;
  }
  const top = cell.text ?? "";
  // 한국어 학습칸: text_ko 가 text 와 다르면 보조로 병기
  const showKo = cell.text_ko && cell.text_ko !== cell.text;
  return (
    <>
      {top && <span>{top}</span>}
      {showKo && (
        <span className={`block text-xs text-muted-foreground ${top ? "mt-0.5" : ""}`}>
          {cell.text_ko}
        </span>
      )}
    </>
  );
}

export function GuideTable({ rows }: { rows: GuideCell[][] }) {
  if (!rows.length) return null;
  return (
    <div className="overflow-x-auto rounded-lg border border-border my-3">
      <table className="w-full text-sm border-collapse">
        <tbody>
          {rows.map((row, ri) => (
            <tr key={ri} className="border-t border-border first:border-t-0">
              {row.map((cell, ci) => {
                const Tag = cell.header ? "th" : "td";
                return (
                  <Tag
                    key={ci}
                    colSpan={cell.col_span ?? undefined}
                    rowSpan={cell.row_span ?? undefined}
                    className={
                      cell.header
                        ? "px-3 py-2 bg-muted text-left font-medium text-foreground align-top border-l border-border first:border-l-0"
                        : "px-3 py-2 text-muted-foreground align-top border-l border-border first:border-l-0"
                    }
                  >
                    {cellContent(cell)}
                  </Tag>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
