import { useState } from "react";
import { useTranslation } from "react-i18next";
import { BookText, ChevronDown } from "lucide-react";

import { Card } from "@/components/ui/card";
import { conceptCardItemSchema } from "../types";
import type { ExplanationBlock, ExplanationUnit } from "../types";

/**
 * 해설(문법·문장) 컨텍스트 통합 섹션 (관리자 세션과 무관, 학습 콘텐츠).
 * study 상세 → pattern_guide / task 화면 → sentence_explain 을 접이식으로 표시.
 * 콘텐츠는 서버에서 user_lang 으로 해소돼 내려오고(structured 골격 + i18n 맵),
 * 여기서 block_type 별로 재조립한다(§5.10 index 불변식).
 *
 * HTML 주입(dangerouslySetInnerHTML)은 우리 시드 콘텐츠(books 파이프라인 생성)에만 적용 —
 * 사용자 입력 아님(신뢰 출처).
 */

type StructuredRows = { rows?: Array<{ form?: string; role?: string }> };
type ConceptItems = { items?: Array<{ raw?: string }> };

function BlockView({ block }: { block: ExplanationBlock }) {
  const i18n = block.i18n ?? {};

  switch (block.block_type) {
    case "heading": {
      const text = block.text ?? i18n.explanation_block_text ?? "";
      if (!text) return null;
      const Tag = (block.level ?? 1) >= 2 ? "h4" : "h3";
      return (
        <Tag className="font-semibold text-foreground mt-4 first:mt-0">{text}</Tag>
      );
    }
    case "subtitle": {
      const text = block.text ?? i18n.explanation_block_text ?? "";
      if (!text) return null;
      return <p className="text-sm font-medium text-primary">{text}</p>;
    }
    case "paragraph":
    case "step": {
      const text = block.text ?? i18n.explanation_block_text ?? "";
      if (!text) return null;
      return <p className="text-sm leading-relaxed text-muted-foreground">{text}</p>;
    }
    case "structured_explain": {
      const rows = (block.structured as StructuredRows)?.rows ?? [];
      const header = i18n.explanation_block_header;
      return (
        <div className="rounded-lg border border-border overflow-hidden">
          {header && (
            <div className="bg-muted px-3 py-2 text-xs font-medium text-muted-foreground">
              {header}
            </div>
          )}
          <table className="w-full text-sm">
            <tbody>
              {rows.map((row, i) => (
                <tr key={i} className="border-t border-border first:border-t-0">
                  <td className="px-3 py-2 font-medium text-foreground whitespace-nowrap align-top">
                    {row.form}
                  </td>
                  <td className="px-3 py-2 text-muted-foreground whitespace-nowrap align-top">
                    {row.role}
                    {i18n[`explanation_block_row_${i}_en`] && (
                      <span className="block text-xs text-muted-foreground/70">
                        {i18n[`explanation_block_row_${i}_en`]}
                      </span>
                    )}
                  </td>
                  <td className="px-3 py-2 text-muted-foreground align-top">
                    {i18n[`explanation_block_row_${i}_explanation`]}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      );
    }
    case "concept_card": {
      const items = (block.structured as ConceptItems)?.items ?? [];
      const cards = items
        .map((item) => {
          try {
            return conceptCardItemSchema.parse(JSON.parse(item.raw ?? "{}"));
          } catch {
            return null;
          }
        })
        .filter((c): c is NonNullable<typeof c> => c !== null);
      if (cards.length === 0) return null;
      return (
        <div className="grid gap-3 sm:grid-cols-2">
          {cards.map((card, i) => (
            <div key={i} className="rounded-lg border border-border p-3 space-y-1.5">
              {(card.icon || card.tag_html) && (
                <div className="flex items-center gap-1.5 text-xs font-semibold text-primary">
                  {card.icon && <span>{card.icon}</span>}
                  {card.tag_html && (
                    <span dangerouslySetInnerHTML={{ __html: card.tag_html }} />
                  )}
                </div>
              )}
              {card.pattern_html && (
                <div
                  className="text-sm font-medium text-foreground"
                  dangerouslySetInnerHTML={{ __html: card.pattern_html }}
                />
              )}
              {card.desc_html && (
                <div
                  className="text-sm text-muted-foreground leading-relaxed [&_strong]:text-foreground [&_strong]:font-semibold"
                  dangerouslySetInnerHTML={{ __html: card.desc_html }}
                />
              )}
            </div>
          ))}
        </div>
      );
    }
    default: {
      // raw HTML 블록 (table/diagram/example/qword_card 등 lang-invariant) 또는 폴백.
      if (block.raw) {
        return (
          <div
            className="text-sm text-muted-foreground [&_table]:w-full [&_td]:border [&_td]:border-border [&_td]:px-2 [&_td]:py-1"
            dangerouslySetInnerHTML={{ __html: block.raw }}
          />
        );
      }
      const text = block.text ?? i18n.explanation_block_text;
      return text ? (
        <p className="text-sm leading-relaxed text-muted-foreground">{text}</p>
      ) : null;
    }
  }
}

function UnitView({ unit }: { unit: ExplanationUnit }) {
  const blocks = [...unit.blocks].sort((a, b) => a.block_seq - b.block_seq);
  return (
    <div className="space-y-2">
      {unit.title && (
        <h3 className="text-base font-bold text-foreground">{unit.title}</h3>
      )}
      {unit.subtitle && (
        <p className="text-sm text-muted-foreground">{unit.subtitle}</p>
      )}
      {blocks.map((block) => (
        <BlockView key={block.block_seq} block={block} />
      ))}
    </div>
  );
}

export function ExplanationSection({
  units,
  defaultOpen = false,
}: {
  units: ExplanationUnit[];
  defaultOpen?: boolean;
}) {
  const { t } = useTranslation();
  const [open, setOpen] = useState(defaultOpen);

  if (!units || units.length === 0) return null;

  return (
    <Card className="overflow-hidden">
      <button
        type="button"
        onClick={() => setOpen((v) => !v)}
        className="flex w-full items-center justify-between gap-2 px-4 py-3 text-left hover:bg-muted/50 transition-colors"
        aria-expanded={open}
      >
        <span className="flex items-center gap-2 text-sm font-semibold text-foreground">
          <BookText className="h-4 w-4 text-primary" />
          {t("explanation.grammarTitle", { defaultValue: "문법 해설" })}
        </span>
        <ChevronDown
          className={`h-4 w-4 text-muted-foreground transition-transform ${open ? "rotate-180" : ""}`}
        />
      </button>
      {open && (
        <div className="space-y-6 border-t border-border px-4 py-4">
          {units.map((unit) => (
            <UnitView key={unit.unit_idx} unit={unit} />
          ))}
        </div>
      )}
    </Card>
  );
}
