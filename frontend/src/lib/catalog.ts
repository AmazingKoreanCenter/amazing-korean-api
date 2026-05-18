/**
 * 교재 역본 수 — 단일 소스 (Single Source of Truth).
 *
 * books 가 36개 언어 역본을 전부 빌드(student/teacher-inner·cover 각 36).
 * 이 상수만 바꾸면 i18n defaultVariables(langCount/editionCount) 를 통해
 * 전 화면·36개 locale 의 "N개 언어 / N종 교재" 문구가 동시 자동 갱신된다.
 * (locale json 에는 숫자를 박지 않음 → 드리프트 구조적 불가)
 */
export const TEXTBOOK_LANGUAGE_COUNT = 36;
/** 학생용 + 교사용 = 언어 수 × 2 */
export const TEXTBOOK_EDITION_COUNT = TEXTBOOK_LANGUAGE_COUNT * 2;
