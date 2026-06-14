import { describe, expect, it } from "vitest";

import { isCorrect, isPrefix, normalizeAnswer } from "./guide_grade";

describe("normalizeAnswer", () => {
  it("공백·문장부호 제거", () => {
    expect(normalizeAnswer("저는 행복합니다.")).toBe("저는행복합니다");
    expect(normalizeAnswer("저는  행복합니다 ?")).toBe("저는행복합니다");
  });
  it("NFD(분해형) 한글을 NFC 로 통일", () => {
    // 동일 글자의 NFD 입력(macOS) ↔ NFC 정답이 일치해야
    const nfd = "저는 행복합니다.".normalize("NFD");
    expect(nfd).not.toBe("저는 행복합니다."); // 실제로 분해형임을 확인
    expect(normalizeAnswer(nfd)).toBe(normalizeAnswer("저는 행복합니다."));
  });
});

describe("isCorrect", () => {
  it("정규화 후 완전일치 = 정답", () => {
    expect(isCorrect("저는 행복합니다", "저는 행복합니다.")).toBe(true);
    expect(isCorrect("저는행복합니다.", "저는 행복합니다")).toBe(true);
  });
  it("내용 다르면 오답", () => {
    expect(isCorrect("저는 슬픕니다", "저는 행복합니다.")).toBe(false);
  });
  it("정답 없으면 오답", () => {
    expect(isCorrect("뭐든지", null)).toBe(false);
  });
});

describe("isPrefix", () => {
  it("입력이 정답 접두면 true (타이핑 중)", () => {
    expect(isPrefix("저는", "저는 행복합니다.")).toBe(true);
    expect(isPrefix("저는 행", "저는 행복합니다.")).toBe(true);
  });
  it("틀린 접두면 false", () => {
    expect(isPrefix("나는", "저는 행복합니다.")).toBe(false);
  });
});
