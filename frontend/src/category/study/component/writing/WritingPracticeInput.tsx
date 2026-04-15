import { forwardRef, useCallback, useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";

import { cn } from "@/lib/utils";
import type { WritingLevel, WritingMistake } from "@/category/study/types";

import { decomposeSyllable } from "./keyboard_layout";

export interface CharResult {
  index: number;
  expected: string;
  actual: string;
  status: "pending" | "correct" | "wrong";
}

export interface WritingStats {
  total_chars: number;
  correct_chars: number;
  mistakes: WritingMistake[];
  duration_ms: number;
}

interface WritingPracticeInputProps {
  prompt: string;
  answer?: string | null;
  hint?: string | null;
  level: WritingLevel;
  disabled?: boolean;
  onChange?: (text: string) => void;
  onStatsChange?: (stats: WritingStats) => void;
  onNextExpectedJamo?: (jamo: string | null) => void;
  text: string;
}

export const WritingPracticeInput = forwardRef<HTMLTextAreaElement, WritingPracticeInputProps>(
  function WritingPracticeInput(
    {
      prompt,
      answer,
      hint,
      level,
      disabled = false,
      onChange,
      onStatsChange,
      onNextExpectedJamo,
      text,
    },
    ref,
  ) {
    const { t } = useTranslation();

    // 초급은 answer가 내려오면 실시간 피드백. 중급/고급은 prompt 자체가 answer 역할.
    const compareTarget = useMemo(() => {
      if (level === "beginner") return answer ?? prompt;
      return prompt;
    }, [answer, level, prompt]);

    const showLiveFeedback = level === "beginner" && !!answer;

    const startedAtRef = useRef<number | null>(null);
    const mistakesRef = useRef<Map<number, WritingMistake>>(new Map());
    const [isComposing, setIsComposing] = useState(false);

    const expectedChars = useMemo(() => Array.from(compareTarget), [compareTarget]);
    const actualChars = useMemo(() => Array.from(text), [text]);

    // 마지막 입력 글자가 한글 IME 조합 중이면 그 위치를 pending 으로 유지해
    // '가' 를 목표로 치는 도중 'ㄱ' 만 본 상태에서 wrong 으로 반짝이는 현상을 제거한다.
    // 조합이 끝난 시점(onCompositionEnd)에 isComposing=false 로 돌아오면 정상 비교로 복귀.
    const charResults: CharResult[] = useMemo(() => {
      const max = Math.max(expectedChars.length, actualChars.length);
      const results: CharResult[] = [];
      const lastActualIdx = actualChars.length - 1;
      for (let i = 0; i < max; i += 1) {
        const expected = expectedChars[i] ?? "";
        const actual = actualChars[i] ?? "";
        const isComposingHere = isComposing && i === lastActualIdx;
        if (actual === "") {
          results.push({ index: i, expected, actual, status: "pending" });
        } else if (isComposingHere) {
          results.push({ index: i, expected, actual, status: "pending" });
        } else if (actual === expected) {
          results.push({ index: i, expected, actual, status: "correct" });
        } else {
          results.push({ index: i, expected, actual, status: "wrong" });
        }
      }
      return results;
    }, [expectedChars, actualChars, isComposing]);

    // 통계 누적: mistake는 한 번 찍히면 계속 유지 (자가교정해도 기록은 남김)
    useEffect(() => {
      if (isComposing) return;
      for (let i = 0; i < actualChars.length; i += 1) {
        const expected = expectedChars[i] ?? "";
        const actual = actualChars[i];
        if (expected && actual && actual !== expected && !mistakesRef.current.has(i)) {
          mistakesRef.current.set(i, { position: i, expected, actual });
        }
      }
    }, [actualChars, expectedChars, isComposing]);

    // 통계 콜백
    useEffect(() => {
      if (!onStatsChange) return;
      const total = actualChars.length;
      let correct = 0;
      for (let i = 0; i < total; i += 1) {
        if (expectedChars[i] && actualChars[i] === expectedChars[i]) correct += 1;
      }
      const duration =
        startedAtRef.current !== null ? Date.now() - startedAtRef.current : 0;
      onStatsChange({
        total_chars: total,
        correct_chars: correct,
        mistakes: Array.from(mistakesRef.current.values()),
        duration_ms: duration,
      });
    }, [actualChars, expectedChars, onStatsChange]);

    // 다음에 기대되는 자모(키보드 하이라이트용).
    // 조합 중이면 "현재 조합 중 음절에서 아직 남은 자모" 를 표시해야 한다.
    // 기존처럼 actualChars.length 를 nextCharIdx 로 쓰면 'ㄱ' 입력 순간 length=1 이 되어
    // 다음 음절의 첫 자모로 점프해 엉뚱한 키가 하이라이트된다.
    useEffect(() => {
      if (!onNextExpectedJamo) return;

      if (isComposing && actualChars.length > 0) {
        const lastIdx = actualChars.length - 1;
        const currentActual = actualChars[lastIdx];
        const currentExpected = expectedChars[lastIdx];
        if (!currentExpected) {
          onNextExpectedJamo(null);
          return;
        }
        const actualJamos = decomposeSyllable(currentActual);
        const expectedJamos = decomposeSyllable(currentExpected);
        // 이미 입력된 자모 수만큼 건너뛴 위치가 다음 기대 자모.
        // 현재 입력이 기대와 다르더라도(오타 조합 중) 동일 인덱스의 기대 자모를
        // 안내해 학습자가 다음에 눌러야 할 키를 파악할 수 있게 한다.
        onNextExpectedJamo(expectedJamos[actualJamos.length] ?? null);
        return;
      }

      const nextCharIdx = actualChars.length;
      const nextExpectedChar = expectedChars[nextCharIdx];
      if (!nextExpectedChar) {
        onNextExpectedJamo(null);
        return;
      }
      const jamos = decomposeSyllable(nextExpectedChar);
      onNextExpectedJamo(jamos[0] ?? null);
    }, [actualChars, expectedChars, isComposing, onNextExpectedJamo]);

    const handleChange = useCallback(
      (event: React.ChangeEvent<HTMLTextAreaElement>) => {
        if (startedAtRef.current === null && event.target.value.length > 0) {
          startedAtRef.current = Date.now();
        }
        onChange?.(event.target.value);
      },
      [onChange],
    );

    return (
      <div className="space-y-4">
        <div className="rounded-lg border bg-muted/30 p-4">
          <p className="text-xs font-medium uppercase tracking-wide text-muted-foreground">
            {t("study.writing.promptLabel")}
          </p>
          {showLiveFeedback ? (
            <p className="mt-2 text-2xl font-semibold leading-relaxed tracking-wide">
              {charResults.map((cr) => (
                <span
                  key={cr.index}
                  className={cn(
                    cr.status === "correct" && "text-status-success",
                    cr.status === "wrong" && "text-destructive underline decoration-destructive",
                    cr.status === "pending" && "text-foreground/70",
                  )}
                >
                  {cr.expected || "\u00A0"}
                </span>
              ))}
            </p>
          ) : (
            <p className="mt-2 text-2xl font-semibold leading-relaxed tracking-wide">
              {prompt}
            </p>
          )}
          {hint && (
            <p className="mt-2 text-sm text-muted-foreground">
              {t("study.writing.hintLabel", { hint })}
            </p>
          )}
        </div>

        <textarea
          ref={ref}
          className="w-full min-h-[100px] p-3 border rounded-lg resize-none text-lg focus:outline-none focus:ring-2 focus:ring-primary disabled:bg-muted"
          placeholder={t("study.writing.inputPlaceholder")}
          value={text}
          onChange={handleChange}
          onCompositionStart={() => setIsComposing(true)}
          onCompositionEnd={() => setIsComposing(false)}
          disabled={disabled}
          autoFocus
          spellCheck={false}
          autoCorrect="off"
          autoCapitalize="off"
        />

        {showLiveFeedback && actualChars.length > 0 && (
          <div className="flex items-center justify-between text-xs text-muted-foreground">
            <span>
              {t("study.writing.progress", {
                done: actualChars.length,
                total: expectedChars.length,
              })}
            </span>
            <span>
              {t("study.writing.liveAccuracy", {
                percent: Math.round(
                  (charResults.filter((c) => c.status === "correct").length /
                    Math.max(expectedChars.length, 1)) *
                    100,
                ),
              })}
            </span>
          </div>
        )}
      </div>
    );
  },
);
