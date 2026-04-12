import { useEffect, useRef, useState } from "react";

import type { WritingPayload, WritingSessionRes } from "@/category/study/types";

import { HangulKeyboard } from "./HangulKeyboard";
import { WritingPracticeInput, type WritingStats } from "./WritingPracticeInput";
import { WritingResultPanel } from "./WritingResultPanel";
import { useStartWritingSession } from "@/category/study/hook/use_writing_session";

interface WritingTaskProps {
  taskId: number;
  payload: WritingPayload;
  text: string;
  onChange: (text: string) => void;
  onStatsChange: (stats: WritingStats) => void;
  onSessionStart: (sessionId: number) => void;
  finishedSession: WritingSessionRes | null;
  disabled?: boolean;
}

export function WritingTask({
  taskId,
  payload,
  text,
  onChange,
  onStatsChange,
  onSessionStart,
  finishedSession,
  disabled,
}: WritingTaskProps) {
  const [keyboardVisible, setKeyboardVisible] = useState(payload.keyboard_visible);
  const [nextJamo, setNextJamo] = useState<string | null>(null);
  const startMutation = useStartWritingSession();
  const startRef = useRef(startMutation);
  startRef.current = startMutation;
  const startedRef = useRef(false);

  // 태스크 마운트 / key 변경 시 세션 1회 시작.
  // 재시도는 상위에서 key prop 변경으로 트리거 → 컴포넌트 재마운트.
  useEffect(() => {
    if (startedRef.current) return;
    startedRef.current = true;
    startRef.current.mutate(
      {
        study_task_id: taskId,
        writing_level: payload.level,
        writing_practice_type: payload.practice_type,
      },
      {
        onSuccess: (session) => onSessionStart(session.session_id),
      },
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="space-y-4">
      <WritingPracticeInput
        prompt={payload.prompt}
        answer={payload.answer ?? null}
        hint={payload.hint ?? null}
        level={payload.level}
        text={text}
        onChange={onChange}
        onStatsChange={onStatsChange}
        onNextExpectedJamo={setNextJamo}
        disabled={disabled}
      />

      {/* 가상 키보드는 "다음에 누를 키" 시각 참조용. 한글 IME 조합이 필요해서 클릭 입력은 막음 */}
      <HangulKeyboard
        level={payload.level}
        visible={keyboardVisible}
        onToggle={() => setKeyboardVisible((v) => !v)}
        highlightKeys={nextJamo ? [nextJamo] : []}
        disabled={disabled}
      />

      {finishedSession && <WritingResultPanel session={finishedSession} />}
    </div>
  );
}
