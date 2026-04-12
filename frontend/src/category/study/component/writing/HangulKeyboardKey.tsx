import { cn } from "@/lib/utils";

import type { KeyCap } from "./keyboard_layout";

interface HangulKeyboardKeyProps {
  cap: KeyCap;
  isHighlighted?: boolean;
  isShiftHighlighted?: boolean;
  onPress?: (jamo: string) => void;
  disabled?: boolean;
}

export function HangulKeyboardKey({
  cap,
  isHighlighted = false,
  isShiftHighlighted = false,
  onPress,
  disabled = false,
}: HangulKeyboardKeyProps) {
  const handleClick = () => {
    if (disabled || !onPress) return;
    onPress(isShiftHighlighted && cap.shift ? cap.shift : cap.base);
  };

  return (
    <button
      type="button"
      onClick={handleClick}
      disabled={disabled}
      aria-label={`${cap.english} ${cap.base}${cap.shift ? ` ${cap.shift}` : ""}`}
      className={cn(
        "relative flex h-12 w-10 flex-col items-center justify-center rounded-md border bg-background text-sm font-medium transition-colors",
        "hover:bg-accent hover:text-accent-foreground",
        "disabled:pointer-events-none disabled:opacity-50",
        "sm:h-14 sm:w-12",
        isHighlighted && "border-primary bg-primary/10 text-primary ring-2 ring-primary",
        isShiftHighlighted &&
          !isHighlighted &&
          "border-amber-500 bg-amber-500/10 text-amber-700 ring-2 ring-amber-500",
      )}
    >
      <span className="text-base font-semibold leading-none sm:text-lg">{cap.base}</span>
      {cap.shift && (
        <span className="absolute right-1 top-1 text-[9px] leading-none text-muted-foreground sm:text-[10px]">
          {cap.shift}
        </span>
      )}
      <span className="mt-0.5 text-[9px] leading-none text-muted-foreground sm:text-[10px]">
        {cap.english}
      </span>
    </button>
  );
}
