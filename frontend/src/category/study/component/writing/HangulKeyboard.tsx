import { useMemo } from "react";
import { Eye, EyeOff, Keyboard } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { cn } from "@/lib/utils";
import type { WritingLevel } from "@/category/study/types";

import { HangulKeyboardKey } from "./HangulKeyboardKey";
import { DUBEOLSIK_ROWS, findKeyForJamo } from "./keyboard_layout";

interface HangulKeyboardProps {
  highlightKeys?: string[];
  onKeyPress?: (jamo: string) => void;
  visible: boolean;
  onToggle?: () => void;
  level: WritingLevel;
  disabled?: boolean;
  className?: string;
}

export function HangulKeyboard({
  highlightKeys = [],
  onKeyPress,
  visible,
  onToggle,
  level,
  disabled = false,
  className,
}: HangulKeyboardProps) {
  const { t } = useTranslation();

  const { baseHighlightCodes, shiftHighlightCodes } = useMemo(() => {
    const baseSet = new Set<string>();
    const shiftSet = new Set<string>();
    for (const jamo of highlightKeys) {
      const match = findKeyForJamo(jamo);
      if (!match) continue;
      if (match.needsShift) shiftSet.add(match.cap.code);
      else baseSet.add(match.cap.code);
    }
    return { baseHighlightCodes: baseSet, shiftHighlightCodes: shiftSet };
  }, [highlightKeys]);

  // 초급은 항상 노출, 중급/고급은 visible prop에 따름
  const isForceShown = level === "beginner";
  const isShown = isForceShown || visible;
  const canToggle = !isForceShown && onToggle !== undefined;

  if (!isShown) {
    return (
      <div className={cn("flex justify-center", className)}>
        {canToggle && (
          <Button variant="outline" size="sm" onClick={onToggle}>
            <Keyboard className="mr-2" />
            {t("study.writing.showKeyboard")}
          </Button>
        )}
      </div>
    );
  }

  return (
    <Card className={cn("w-full", className)}>
      <CardContent className="space-y-2 p-3 sm:p-4">
        {canToggle && (
          <div className="flex justify-end">
            <Button variant="ghost" size="sm" onClick={onToggle}>
              {visible ? <EyeOff className="mr-2" /> : <Eye className="mr-2" />}
              {visible ? t("study.writing.hideKeyboard") : t("study.writing.showKeyboard")}
            </Button>
          </div>
        )}
        <div className="flex flex-col items-center gap-1.5 sm:gap-2">
          {DUBEOLSIK_ROWS.map((row, rowIdx) => (
            <div
              key={rowIdx}
              className="flex gap-1 sm:gap-1.5"
              style={{ paddingLeft: `${rowIdx * 1}rem` }}
            >
              {row.map((cap) => (
                <HangulKeyboardKey
                  key={cap.code}
                  cap={cap}
                  isHighlighted={baseHighlightCodes.has(cap.code)}
                  isShiftHighlighted={shiftHighlightCodes.has(cap.code)}
                  onPress={onKeyPress}
                  disabled={disabled}
                />
              ))}
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}
