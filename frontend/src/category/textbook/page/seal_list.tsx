import { useState, useCallback, useRef } from "react";
import { useTranslation } from "react-i18next";
import { Swiper, SwiperSlide } from "swiper/react";
import { EffectCoverflow, Thumbs, FreeMode } from "swiper/modules";
import type { Swiper as SwiperType } from "swiper";

export interface SealItem {
  language: string;
  language_name_ko: string;
  language_name_en: string;
}

interface SealListProps {
  items: SealItem[];
  selectedIndex: number;
  onSelect: (index: number) => void;
}

function SealImage({ language, size }: { language: string; size: string }) {
  const [error, setError] = useState(false);

  if (error) {
    return (
      <div className={`${size} rounded-full bg-muted flex items-center justify-center text-xs font-medium text-muted-foreground uppercase`}>
        {language}
      </div>
    );
  }

  return (
    <img
      src={`/seals/${language}.svg`}
      alt={language}
      className={`${size} rounded-full object-cover`}
      loading="lazy"
      onError={() => setError(true)}
    />
  );
}

export function SealList({ items, selectedIndex, onSelect }: SealListProps) {
  const { i18n } = useTranslation();
  const mainSwiperRef = useRef<SwiperType | null>(null);
  const thumbsSwiperRef = useRef<SwiperType | null>(null);
  const [thumbsReady, setThumbsReady] = useState(false);

  const len = items.length;
  if (len === 0) return null;

  const langName = i18n.language === "ko"
    ? items[selectedIndex]?.language_name_ko
    : items[selectedIndex]?.language_name_en;

  const handleMainSlideChange = useCallback((swiper: SwiperType) => {
    const idx = swiper.realIndex;
    onSelect(idx);
    if (thumbsSwiperRef.current) {
      thumbsSwiperRef.current.slideTo(idx);
    }
  }, [onSelect]);

  const handleThumbClick = useCallback((index: number) => {
    if (mainSwiperRef.current) {
      mainSwiperRef.current.slideToLoop(index);
    }
    onSelect(index);
  }, [onSelect]);

  return (
    <div className="flex flex-col md:h-[420px]">
      {/* Top: Coverflow carousel — desktop only */}
      <div className="hidden md:flex flex-1 items-center">
        <Swiper
          modules={[EffectCoverflow, Thumbs]}
          effect="coverflow"
          grabCursor
          centeredSlides
          slidesPerView={5}
          loop={len > 5}
          slideToClickedSlide
          initialSlide={selectedIndex}
          coverflowEffect={{
            rotate: 0,
            stretch: 0,
            depth: 120,
            modifier: 1.5,
            slideShadows: false,
          }}
          thumbs={thumbsReady && thumbsSwiperRef.current ? { swiper: thumbsSwiperRef.current } : undefined}
          onSwiper={(swiper) => { mainSwiperRef.current = swiper; }}
          onSlideChange={handleMainSlideChange}
          className="w-full [&_.swiper-slide:not(.swiper-slide-visible)]:opacity-0 [&_.swiper-slide:not(.swiper-slide-visible)]:pointer-events-none"
          watchSlidesProgress
        >
          {items.map((item) => (
            <SwiperSlide
              key={item.language}
              className="flex justify-center transition-opacity duration-300"
            >
              {({ isActive }) => (
                <div className="flex flex-col items-center gap-2 py-4">
                  <div className={`rounded-full transition-all duration-300 ${isActive ? "ring-2 ring-primary ring-offset-2" : ""}`}>
                    <SealImage
                      language={item.language}
                      size={isActive ? "w-24 h-24 md:w-28 md:h-28" : "w-14 h-14 md:w-18 md:h-18"}
                    />
                  </div>
                  {isActive && (
                    <span className="text-sm font-semibold text-primary whitespace-nowrap">
                      {langName}
                    </span>
                  )}
                </div>
              )}
            </SwiperSlide>
          ))}
        </Swiper>
      </div>

      {/* Mobile: selected language name */}
      <div className="md:hidden text-center py-2">
        <span className="text-sm font-semibold text-primary">{langName}</span>
      </div>

      {/* Bottom: Thumbs strip — click to select */}
      <div className="flex-shrink-0 py-2">
        <Swiper
          modules={[FreeMode]}
          onSwiper={(swiper) => {
            thumbsSwiperRef.current = swiper;
            setThumbsReady(true);
          }}
          slidesPerView={Math.min(len, 10)}
          freeMode
          centeredSlides
          spaceBetween={4}
          slideToClickedSlide
          className="w-full"
        >
          {items.map((item, idx) => (
            <SwiperSlide key={item.language}>
              <div
                className={`flex justify-center cursor-pointer py-1 transition-opacity ${
                  idx === selectedIndex ? "opacity-100" : "opacity-50 hover:opacity-75"
                }`}
                onClick={() => handleThumbClick(idx)}
              >
                <SealImage language={item.language} size="w-8 h-8" />
              </div>
            </SwiperSlide>
          ))}
        </Swiper>
      </div>
    </div>
  );
}
