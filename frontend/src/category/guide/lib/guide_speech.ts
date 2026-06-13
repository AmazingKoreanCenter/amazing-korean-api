/**
 * 한국어 TTS — 브라우저 Web Speech API (해설집 HTML speakKorean 이식, D-6).
 * 원어민 녹음(audio_url)은 후속 트랙. 미지원 브라우저면 무동작.
 */

const PREFERRED_VOICES = ["Yuna", "SunHi", "Heami", "Google 한국의"];

let cachedVoice: SpeechSynthesisVoice | null = null;

function pickKoreanVoice(): SpeechSynthesisVoice | null {
  if (typeof window === "undefined" || !window.speechSynthesis) return null;
  if (cachedVoice) return cachedVoice;
  const voices = window.speechSynthesis.getVoices();
  if (voices.length === 0) return null;
  for (const name of PREFERRED_VOICES) {
    const v = voices.find((voice) => voice.name.includes(name));
    if (v) {
      cachedVoice = v;
      return v;
    }
  }
  cachedVoice = voices.find((v) => v.lang?.startsWith("ko")) ?? null;
  return cachedVoice;
}

export function isSpeechSupported(): boolean {
  return typeof window !== "undefined" && !!window.speechSynthesis;
}

/** 한국어 텍스트를 음성으로 재생 (ko-KR, rate 0.85 — HTML 동등) */
export function speakKorean(text: string): void {
  if (!isSpeechSupported() || !text.trim()) return;
  window.speechSynthesis.cancel();
  const u = new SpeechSynthesisUtterance(text);
  u.lang = "ko-KR";
  u.rate = 0.85;
  u.pitch = 1.1;
  const voice = pickKoreanVoice();
  if (voice) u.voice = voice;
  window.speechSynthesis.speak(u);
}
