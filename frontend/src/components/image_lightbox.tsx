import { useCallback, useEffect } from "react";
import { createPortal } from "react-dom";
import { ChevronLeft, ChevronRight, X } from "lucide-react";

interface ImageLightboxProps {
  src: string;
  alt: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onPrev?: () => void;
  onNext?: () => void;
}

export function ImageLightbox({ src, alt, open, onOpenChange, onPrev, onNext }: ImageLightboxProps) {
  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      if (!open) return;
      if (e.key === "Escape") {
        e.stopPropagation();
        e.preventDefault();
        onOpenChange(false);
      }
      if (e.key === "ArrowLeft" && onPrev) {
        e.stopPropagation();
        onPrev();
      }
      if (e.key === "ArrowRight" && onNext) {
        e.stopPropagation();
        onNext();
      }
    },
    [open, onOpenChange, onPrev, onNext],
  );

  useEffect(() => {
    if (!open) return;
    document.addEventListener("keydown", handleKeyDown, true);
    return () => document.removeEventListener("keydown", handleKeyDown, true);
  }, [open, handleKeyDown]);

  if (!open) return null;

  return createPortal(
    <div
      className="fixed inset-0 z-[100] flex items-center justify-center pointer-events-auto"
      onClick={(e) => e.stopPropagation()}
      onPointerDown={(e) => e.stopPropagation()}
    >
      {/* Backdrop: blur */}
      <div
        className="absolute inset-0 bg-background/60 backdrop-blur-sm"
        onClick={() => onOpenChange(false)}
      />

      {/* Close button */}
      <button
        type="button"
        onClick={(e) => { e.stopPropagation(); onOpenChange(false); }}
        className="absolute top-4 right-4 z-10 w-11 h-11 rounded-full bg-background/80 border shadow-sm flex items-center justify-center hover:bg-background transition-colors"
      >
        <X className="h-5 w-5" />
      </button>

      {/* Navigation: prev */}
      {onPrev && (
        <button
          type="button"
          onClick={(e) => { e.stopPropagation(); onPrev(); }}
          className="absolute left-4 z-10 w-11 h-11 rounded-full bg-background/80 border shadow-sm flex items-center justify-center hover:bg-background transition-colors"
        >
          <ChevronLeft className="h-5 w-5" />
        </button>
      )}

      {/* Image */}
      <img
        src={src}
        alt={alt}
        className="relative z-[1] max-w-[90vw] max-h-[90vh] object-contain rounded-xl shadow-2xl"
        onClick={(e) => e.stopPropagation()}
      />

      {/* Navigation: next */}
      {onNext && (
        <button
          type="button"
          onClick={(e) => { e.stopPropagation(); onNext(); }}
          className="absolute right-4 z-10 w-11 h-11 rounded-full bg-background/80 border shadow-sm flex items-center justify-center hover:bg-background transition-colors"
        >
          <ChevronRight className="h-5 w-5" />
        </button>
      )}
    </div>,
    document.body,
  );
}
