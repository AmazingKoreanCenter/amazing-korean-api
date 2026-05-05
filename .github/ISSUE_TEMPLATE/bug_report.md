---
name: 🐛 버그 리포트
about: production 또는 dev 환경 버그 추적
title: "[Bug] "
labels: bug
---

## 증상

<!-- 1-2 줄 요약 -->

## 재현 절차

1. 
2. 
3. 

## 예상 vs 실제

- **예상**: 
- **실제**: 

## 환경

- **영역**: backend / frontend / 인프라 / DB / 외부 서비스
- **환경**: production (api.amazingkorean.net) / staging / dev (localhost)
- **재현 빈도**: 항상 / 가끔 / 1회만

## 로그 / 스크린샷

<!-- production 로그 (`docker logs amk-api`) 또는 브라우저 콘솔 등 -->

## 의심 위치

<!-- 코드 위치 추정 (file:line) — 비워둬도 OK -->

## 우선순위

- [ ] CRITICAL (production 다운 / 결제 차단 / 데이터 손실)
- [ ] HIGH (핵심 기능 영향)
- [ ] MEDIUM (일부 사용자 / 우회 가능)
- [ ] LOW (UX 미세 / 향후 처리)
