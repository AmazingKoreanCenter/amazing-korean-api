import { useEffect, useState } from "react";

import type { WritingPayload, WritingSessionRes } from "@/category/study/types";

import { HangulKeyboard } from "./HangulKeyboard";
import { WritingPracticeInput, type WritingStats } from "./WritingPracticeInput";
import { WritingResultPanel } from "./WritingResultPanel";
import { startWritingSession } from "@/category/study/study_api";

interface WritingTaskProps {
  taskId: number | null;
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

  // 태스크 마운트 / key 변경 시 세션 1회 시작.
  // 재시도는 상위에서 key prop 변경으로 트리거 → 컴포넌트 재마운트.
  // StrictMode 의 이펙트 이중 호출 대비: cancelled 플래그로 첫 번째 호출의 결과를
  // 두 번째 마운트에서는 무시한다. useMutation 대신 startWritingSession 을 직접
  // 호출하는 이유는 useMutation 의 observer 가 StrictMode 에서 파괴되어 onSuccess
  // 콜백이 드랍되는 이슈를 피하기 위함.
  useEffect(() => {
    let cancelled = false;
    startWritingSession({
      study_task_id: taskId ?? null,
      writing_level: payload.level,
      writing_practice_type: payload.practice_type,
    })
      .then((session) => {
        if (cancelled) return;
        onSessionStart(session.session_id);
      })
      .catch(() => {
        // toast 는 상위에서 listWritingSessions 훅 등이 잡아줌. 여기선 silent.
      });
    return () => {
      cancelled = true;
    };
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
