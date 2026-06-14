import { useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { BookOpen, Check, Volume2 } from "lucide-react";

import { Card } from "@/components/ui/card";

import { isCorrect } from "../lib/guide_grade";
import { isSpeechSupported, speakKorean } from "../lib/guide_speech";
import type { GuideItem, GuideLogAction, GuideSentence } from "../types";
import { GuideSentenceBlocks } from "./GuideBlockStream";

/** 문장의 영어 프롬프트에서 "N) " 접두 제거 */
function stripPrefix(text: string | null | undefined): string {
  return (text ?? "").replace(/^\d+\)\s*/, "");
}

/** 한 문장 학습 사이클: 영어 인지 → 어휘 → 작문 시도 → 정답+듣기 → 해설 */
export function GuideSentenceCard({
  sentence,
  items,
  solved = false,
  onLog,
}: {
  sentence: GuideSentence;
  items: GuideItem[];
  /** 서버 진행 = 이미 해결한 문장 (로그인 시) */
  solved?: boolean;
  /** 학습 로그 기록 (로그인 시에만 호출됨; 호출부에서 가드) */
  onLog?: (
    sentenceNo: number,
    action: GuideLogAction,
    answer?: unknown
  ) => void;
}) {
  const { t } = useTranslation();
  const [input, setInput] = useState("");
  const [revealed, setRevealed] = useState(false);
  const [showExplain, setShowExplain] = useState(false);
  // 세션 내 1회만 기록 (키 입력마다 POST 방지)
  const loggedCorrectRef = useRef(false);
  const loggedRevealRef = useRef(false);

  const answer = sentence.text_ko ?? "";
  const correct = isCorrect(input, answer);
  const showSolved = solved || correct;

  // 입력이 처음 정답이 되는 순간 1회 'correct' 기록
  const handleInput = (value: string) => {
    setInput(value);
    if (!loggedCorrectRef.current && isCorrect(value, answer)) {
      loggedCorrectRef.current = true;
      onLog?.(sentence.sentence_no, "correct", { text: value });
    }
  };

  // 미해결 상태에서 정답을 처음 펼치면 'reveal'(포기) 1회 기록
  const handleReveal = () => {
    const next = !revealed;
    setRevealed(next);
    if (next && !loggedRevealRef.current && !loggedCorrectRef.current && !solved) {
      loggedRevealRef.current = true;
      onLog?.(sentence.sentence_no, "reveal");
    }
  };

  // 이 문장의 어휘 블록
  const vocab = items.filter(
    (it) => it.sentence_no === sentence.sentence_no && it.block_type === "vocab"
  );

  return (
    <Card className="p-4 sm:p-5">
      {/* 영어 프롬프트 */}
      <div className="flex items-baseline gap-2">
        <span className="text-xs font-mono text-muted-foreground">{sentence.sentence_no}</span>
        <h3 className="flex-1 text-base font-semibold text-foreground">
          {stripPrefix(sentence.text ?? sentence.text_ko)}
        </h3>
        {showSolved && (
          <span
            className="inline-flex items-center gap-0.5 rounded-full bg-green-100 px-1.5 py-0.5 text-[10px] font-semibold text-green-700"
            title={t("guide.solved")}
          >
            <Check className="h-3 w-3" /> {t("guide.solved")}
          </span>
        )}
      </div>

      {/* 어휘 */}
      {vocab.length > 0 && (
        <div className="mt-2 flex flex-wrap gap-x-3 gap-y-1 text-xs text-muted-foreground">
          {vocab.map((v) => (
            <span key={v.block_seq}>
              <span className="text-foreground">{v.text}</span>
              {v.text_ko && <span> = {v.text_ko}</span>}
            </span>
          ))}
        </div>
      )}

      {/* 작문 시도 (정규화 완전일치 즉시 채점) */}
      <div className="mt-3">
        <input
          type="text"
          value={input}
          onChange={(e) => handleInput(e.target.value)}
          placeholder={t("guide.writePlaceholder")}
          className={`w-full rounded-md border px-3 py-2 text-sm outline-none transition-colors ${
            correct
              ? "border-green-500 bg-green-50 text-green-900"
              : "border-border bg-background focus:border-primary"
          }`}
          aria-label={t("guide.writePlaceholder")}
        />
        {correct && (
          <p className="mt-1 flex items-center gap-1 text-xs text-green-600">
            <Check className="h-3 w-3" /> {t("guide.correct")}
          </p>
        )}
      </div>

      {/* 정답 공개 + 듣기 */}
      <div className="mt-2 flex items-center gap-2">
        <button
          type="button"
          onClick={handleReveal}
          className="text-xs font-medium text-primary hover:underline"
        >
          {revealed ? t("guide.hideAnswer") : t("guide.showAnswer")}
        </button>
        {revealed && (
          <>
            <span className="text-sm font-medium text-foreground">{answer}</span>
            {isSpeechSupported() && (
              <button
                type="button"
                onClick={() => speakKorean(answer)}
                className="text-muted-foreground hover:text-primary"
                aria-label={t("guide.listen")}
              >
                <Volume2 className="h-4 w-4" />
              </button>
            )}
          </>
        )}
      </div>

      {/* 해설 (표/노트) */}
      <div className="mt-2">
        <button
          type="button"
          onClick={() => setShowExplain((v) => !v)}
          className="flex items-center gap-1 text-xs font-medium text-muted-foreground hover:text-foreground"
        >
          <BookOpen className="h-3.5 w-3.5" /> {t("guide.explanation")}
        </button>
        {showExplain && (
          <GuideSentenceBlocks items={items} sentenceNo={sentence.sentence_no} />
        )}
      </div>
    </Card>
  );
}
