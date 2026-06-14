import { useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { RotateCw, Volume2 } from "lucide-react";

import { Card } from "@/components/ui/card";

import { isCorrect } from "../lib/guide_grade";
import { isSpeechSupported, speakKorean } from "../lib/guide_speech";
import type { GuideSentence } from "../types";

function stripPrefix(text: string | null | undefined): string {
  return (text ?? "").replace(/^\d+\)\s*/, "");
}

/** ① Read Along — 문장 클릭 시 한국어 듣기 */
function ReadAlong({ sentences }: { sentences: GuideSentence[] }) {
  const { t } = useTranslation();
  if (!isSpeechSupported()) return null;
  return (
    <div>
      <h4 className="text-sm font-semibold text-foreground mb-2">{t("guide.review.readAlong")}</h4>
      <ul className="space-y-1">
        {sentences.map((s) => (
          <li key={s.sentence_no}>
            <button
              type="button"
              onClick={() => speakKorean(s.text_ko ?? "")}
              className="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-sm hover:bg-muted"
            >
              <Volume2 className="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
              <span className="font-medium text-foreground">{s.text_ko}</span>
              <span className="text-xs text-muted-foreground">{stripPrefix(s.text)}</span>
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}

/** ② Look & Speak — 플래시카드 (앞=영어, 탭→뒤=한국어+듣기) */
function Flashcards({ sentences }: { sentences: GuideSentence[] }) {
  const { t } = useTranslation();
  const [flipped, setFlipped] = useState<Set<number>>(new Set());
  const toggle = (n: number) =>
    setFlipped((prev) => {
      const next = new Set(prev);
      if (next.has(n)) {
        next.delete(n);
      } else {
        next.add(n);
      }
      return next;
    });
  return (
    <div>
      <h4 className="text-sm font-semibold text-foreground mb-2">{t("guide.review.lookSpeak")}</h4>
      <div className="grid grid-cols-2 sm:grid-cols-3 gap-2">
        {sentences.map((s) => {
          const isFlipped = flipped.has(s.sentence_no);
          return (
            <div
              key={s.sentence_no}
              role="button"
              tabIndex={0}
              onClick={() => toggle(s.sentence_no)}
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                  e.preventDefault();
                  toggle(s.sentence_no);
                }
              }}
              className="min-h-[72px] cursor-pointer rounded-lg border border-border bg-card p-3 text-left transition-colors hover:border-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
            >
              {isFlipped ? (
                <span className="flex items-center justify-between gap-1">
                  <span className="text-sm font-medium text-foreground">{s.text_ko}</span>
                  {isSpeechSupported() && (
                    <button
                      type="button"
                      onClick={(e) => {
                        e.stopPropagation();
                        speakKorean(s.text_ko ?? "");
                      }}
                      className="rounded-full p-1 text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
                      aria-label={t("guide.listen")}
                    >
                      <Volume2 className="h-3.5 w-3.5 shrink-0" />
                    </button>
                  )}
                </span>
              ) : (
                <span className="text-sm text-muted-foreground">{stripPrefix(s.text)}</span>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}

/** ④ Writing Test — 일괄 채점 */
function WritingTest({ sentences }: { sentences: GuideSentence[] }) {
  const { t } = useTranslation();
  const [answers, setAnswers] = useState<Record<number, string>>({});
  const [graded, setGraded] = useState(false);

  const score = useMemo(
    () =>
      graded
        ? sentences.filter((s) => isCorrect(answers[s.sentence_no] ?? "", s.text_ko)).length
        : 0,
    [answers, sentences, graded]
  );

  return (
    <div>
      <div className="flex items-center justify-between mb-2">
        <h4 className="text-sm font-semibold text-foreground">{t("guide.review.writingTest")}</h4>
        <div className="flex items-center gap-2">
          {graded && (
            <span className="text-sm font-medium text-primary">
              {score} / {sentences.length}
            </span>
          )}
          <button
            type="button"
            onClick={() => {
              setAnswers({});
              setGraded(false);
            }}
            className="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground"
          >
            <RotateCw className="h-3 w-3" /> {t("guide.review.reset")}
          </button>
          <button
            type="button"
            onClick={() => setGraded(true)}
            className="rounded-md bg-primary px-2.5 py-1 text-xs font-medium text-primary-foreground hover:bg-primary/90"
          >
            {t("guide.review.grade")}
          </button>
        </div>
      </div>
      <ul className="space-y-2">
        {sentences.map((s) => {
          const val = answers[s.sentence_no] ?? "";
          const ok = graded && isCorrect(val, s.text_ko);
          const wrong = graded && !ok;
          return (
            <li key={s.sentence_no}>
              <label
                htmlFor={`guide-wt-${s.sentence_no}`}
                className="block text-xs text-muted-foreground mb-0.5"
              >
                {stripPrefix(s.text)}
              </label>
              <input
                id={`guide-wt-${s.sentence_no}`}
                type="text"
                value={val}
                onChange={(e) =>
                  setAnswers((prev) => ({ ...prev, [s.sentence_no]: e.target.value }))
                }
                className={`w-full rounded-md border px-3 py-1.5 text-sm outline-none ${
                  ok
                    ? "border-green-500 bg-green-50"
                    : wrong
                      ? "border-red-400 bg-red-50"
                      : "border-border focus:border-primary"
                }`}
              />
              {wrong && <p className="mt-0.5 text-xs text-red-500">{s.text_ko}</p>}
            </li>
          );
        })}
      </ul>
    </div>
  );
}

/** 단원 복습 섹션 (Read Along / Look&Speak / Writing Test) */
export function GuideReview({ sentences }: { sentences: GuideSentence[] }) {
  const { t } = useTranslation();
  if (!sentences.length) return null;
  return (
    <Card className="p-4 sm:p-5 space-y-6">
      <h3 className="text-base font-semibold text-foreground">{t("guide.review.title")}</h3>
      <ReadAlong sentences={sentences} />
      <Flashcards sentences={sentences} />
      <WritingTest sentences={sentences} />
    </Card>
  );
}
