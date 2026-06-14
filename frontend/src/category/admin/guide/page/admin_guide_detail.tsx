import { useState } from "react";
import { Link, useParams } from "react-router-dom";
import { ChevronLeft } from "lucide-react";

import { Skeleton } from "@/components/ui/skeleton";

import { useAdminGuide, useUpdateGuideBlock, useUpdateGuideMeta } from "../hook/use_admin_guide";
import { GUIDE_STATES, GUIDE_THEMES } from "../types";
import type { AdminGuideBlock } from "../types";

/** 블록 텍스트 인라인 편집 행 (ko/en 병렬, 변경 시 저장) */
function BlockEditor({ block, guideIdx }: { block: AdminGuideBlock; guideIdx: string }) {
  const upd = useUpdateGuideBlock(guideIdx);
  const [ko, setKo] = useState(block.text_ko ?? "");
  const [en, setEn] = useState(block.text_en ?? "");
  const dirty = ko !== (block.text_ko ?? "") || en !== (block.text_en ?? "");

  const save = () => {
    const body: { text_ko?: string | null; text_en?: string | null } = {};
    if (ko !== (block.text_ko ?? "")) body.text_ko = ko === "" ? null : ko;
    if (en !== (block.text_en ?? "")) body.text_en = en === "" ? null : en;
    upd.mutate({ blockId: block.guide_block_id, body });
  };

  const coord =
    block.table_no != null ? `T${block.table_no}_R${block.row_no}_C${block.col_no}` : "";

  return (
    <div className="border-t border-border py-2 first:border-t-0">
      <div className="mb-1 flex items-center gap-2 text-xs text-muted-foreground">
        <span className="font-mono">{block.block_seq}</span>
        <span className="rounded bg-muted px-1.5 py-0.5">{block.block_type}</span>
        {coord && <span className="font-mono">{coord}</span>}
        {block.marker && <span className="text-amber-600">{block.marker}</span>}
        {block.sentence_no != null && <span>문장 {block.sentence_no}</span>}
        <span>v{block.source_version}</span>
        {block.edited && <span className="text-blue-600">edited</span>}
      </div>
      <div className="grid grid-cols-1 gap-2 sm:grid-cols-2">
        <textarea
          value={ko}
          onChange={(e) => setKo(e.target.value)}
          rows={2}
          placeholder="KO (언어불변 학습 콘텐츠)"
          className="w-full rounded border border-border px-2 py-1 text-sm outline-none focus:border-primary"
        />
        <textarea
          value={en}
          onChange={(e) => setEn(e.target.value)}
          rows={2}
          placeholder="EN (번역 원천 — 수정 시 번역 stale)"
          className="w-full rounded border border-border px-2 py-1 text-sm outline-none focus:border-primary"
        />
      </div>
      {dirty && (
        <div className="mt-1 flex items-center gap-2">
          <button
            type="button"
            disabled={upd.isPending}
            onClick={save}
            className="rounded bg-primary px-2.5 py-1 text-xs font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
          >
            저장
          </button>
          <button
            type="button"
            onClick={() => {
              setKo(block.text_ko ?? "");
              setEn(block.text_en ?? "");
            }}
            className="text-xs text-muted-foreground hover:text-foreground"
          >
            취소
          </button>
          <span className="text-xs text-amber-600">EN 수정 시 해당 블록 번역이 stale 처리됩니다</span>
        </div>
      )}
    </div>
  );
}

export function AdminGuideDetail() {
  const { guideIdx } = useParams<{ guideIdx: string }>();
  const { data, isLoading } = useAdminGuide(guideIdx);
  const meta = useUpdateGuideMeta(guideIdx ?? "");

  if (isLoading || !data) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-1/2" />
        <Skeleton className="h-64 rounded-lg" />
      </div>
    );
  }

  return (
    <div className="space-y-5">
      <Link
        to="/admin/guides"
        className="inline-flex items-center gap-1 text-sm text-muted-foreground hover:text-foreground"
      >
        <ChevronLeft className="h-4 w-4" /> 단원 목록
      </Link>

      {/* 단원 메타 편집 */}
      <section className="rounded-lg border border-border p-4">
        <div className="flex flex-wrap items-center gap-3">
          <span className="text-sm font-semibold text-foreground">
            {data.guide_seq}. {data.title_en ?? data.title_ko}
          </span>
          <span className="font-mono text-xs text-muted-foreground">{data.guide_idx}</span>
          {data.sentence_start != null && (
            <span className="text-xs text-muted-foreground">
              문장 {data.sentence_start}–{data.sentence_end}
            </span>
          )}
        </div>
        <div className="mt-3 flex flex-wrap items-center gap-4">
          <label className="flex items-center gap-1 text-sm">
            <span className="text-muted-foreground">상태</span>
            <select
              value={data.guide_state}
              onChange={(e) => meta.mutate({ guide_state: e.target.value })}
              className="rounded border border-border px-2 py-1 text-sm"
            >
              {GUIDE_STATES.map((s) => (
                <option key={s} value={s}>{s}</option>
              ))}
            </select>
          </label>
          <label className="flex items-center gap-1 text-sm">
            <span className="text-muted-foreground">테마</span>
            <select
              value={data.guide_theme}
              onChange={(e) => meta.mutate({ guide_theme: e.target.value })}
              className="rounded border border-border px-2 py-1 text-sm"
            >
              {GUIDE_THEMES.map((t) => (
                <option key={t} value={t}>{t}</option>
              ))}
            </select>
          </label>
        </div>
      </section>

      {/* 블록 편집 */}
      <section className="rounded-lg border border-border p-4">
        <h2 className="mb-2 text-sm font-semibold text-foreground">
          블록 ({data.blocks.length}) — 텍스트 편집
        </h2>
        <div>
          {data.blocks.map((b) => (
            <BlockEditor key={b.guide_block_id} block={b} guideIdx={data.guide_idx} />
          ))}
        </div>
      </section>
    </div>
  );
}
