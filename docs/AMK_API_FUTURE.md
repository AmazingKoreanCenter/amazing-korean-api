# AMK_API_FUTURE — 미구현 Phase 스펙

> 향후 구현 예정 Phase (13~16). 구현 시 해당 도메인 문서로 이관.
> 공통 규칙(인증, 에러, 페이징): [AMK_API_MASTER.md §3](./AMK_API_MASTER.md)
> DB 스키마: [AMK_SCHEMA_PATCHED.md](./AMK_SCHEMA_PATCHED.md)
> 코드 패턴: [AMK_CODE_PATTERNS.md](./AMK_CODE_PATTERNS.md)

---

### 5.13 Phase 13 — 학습 콘텐츠 시딩 (Content Seeding)

> 교재 JSON 데이터를 DB(study, study_task, lesson, course)로 시딩하여 웹 학습 콘텐츠를 구성한다.
> 교재 페이지 순서 = 학습 순서 (page_manifest.json 기준).

**데이터 소스**: `scripts/textbook/data/` (11개 JSON 파일)

| JSON 파일 | 내용 | Study Program |
|-----------|------|---------------|
| vocabulary.json | 어휘 카드 (한국어 + 20개 언어 번역, 280+) | basic_word |
| sentences.json | 문법 예문 (한국어 + 번역, 496+) | basic_500 |
| pronunciation.json | 한글 조합표 (자음×모음, 테이블 7+) | basic_pronunciation |
| pronunciation_test.json | 발음 테스트 연습 문제 | basic_pronunciation |
| particles.json | 조사 활용표 | basic_500 |
| conjugation.json | 동사/형용사 활용 (현재/과거/미래) | basic_500 |
| structure.json | 문장 구조 (의문사 패턴) | basic_500 |
| appendix.json | 숫자, 문법 연습 | basic_500 |

**DB 계층 구조**:
```
Course "놀라운 한국어 기초"
├── Part I. 발음 (Lesson 1~7): pronunciation + pronunciation_test + vocabulary(발음)
├── Part II. 문법 기초 (Lesson 8~10): particles + structure
├── Part III~IV. 서술어/부사어 문법 (Lesson 11~30): sentences (섹션별 1 Lesson)
├── Part V. 동사 활용 (Lesson 31~33): conjugation
└── Part VI. 부록 (Lesson 34~35): appendix
```

**문제 유형**: choice (4지선다), typing (직접 입력 / 클릭 배열), voice (발음)

**구현**: Seed Script (`scripts/textbook/JSON → seed_script → DB`)

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | Seed Script 설계 (JSON → DB 매핑) | ⬜ |
| 2 | Study 세트 생성 (program별) | ⬜ |
| 3 | StudyTask 생성 (choice/typing 문제 자동 생성 + 오답 생성) | ⬜ |
| 4 | Lesson 생성 + LessonItem 연결 | ⬜ |
| 5 | Course 생성 + course_lesson 연결 | ⬜ |
| 6 | 다국어 explain 시딩 (20개 언어) | ⬜ |

### 5.14 Phase 14 — AI 발음 평가 (Pronunciation Assessment)

> 한국어 학습자 발음을 음소 단위로 평가하고 교정 피드백을 제공한다.
> 3단계 접근: Phase 1(따라하기 안내) → Phase 2(SpeechSuper API) → Phase 3(커스텀 모델).

**Phase 14-1**: 따라하기 안내 (녹음/판별 없음, UI에서 "따라 해보세요" 안내)

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | Study voice task에 "따라하기" UI 안내 추가 | ⬜ |
| 2 | audio_url 개별 음성 재생 기능 | ⬜ |

**Phase 14-2**: SpeechSuper API 프로토타이핑 (콘텐츠 완성 후)

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | SpeechSuper API 통합 (Rust 백엔드) | ⬜ |
| 2 | 한 글자 / 단어 / 문장 발음 평가 엔드포인트 | ⬜ |
| 3 | 음소별 점수 + 오발음 피드백 UI | ⬜ |
| 4 | 사용자 반응/평가 데이터 수집 | ⬜ |

**Phase 14-3**: 커스텀 모델 개발 (기술 검증 후)

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | AIHub 71469 데이터셋 확보 (HYMN 법인 명의) | ⬜ |
| 2 | wav2vec2-large-xlsr-korean 파인튜닝 | ⬜ |
| 3 | 초성/중성/종성 3-way 분류 + GOP 점수화 | ⬜ |
| 4 | L1별(20개 언어) 맞춤 오류 피드백 | ⬜ |
| 5 | API 서버 배포 + 백엔드 통합 | ⬜ |

### 5.15 Phase 15 — 조음 애니메이션 (Articulation Animation)

> 한국어 음소별 입모양/혀위치를 SVG 애니메이션으로 시각화한다.
> 15~17개 다이어그램으로 전체 음소 커버. 한국어 전용 조음 애니메이션 도구 부재 → 차별화 기회.

**기술 스택**: Wikimedia CC0 SVG + Figma 수정 → GSAP MorphSVG + React

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | CC0 SVG 다운로드 (IPA 조음도) + 한국어 특화 수정 (Figma) | ⬜ |
| 2 | 자음 7개 조음 위치 SVG path 데이터 추출 | ⬜ |
| 3 | 모음 7~9개 혀 위치 SVG path 데이터 추출 | ⬜ |
| 4 | 성문 상태도 (평음/경음/격음) SVG 제작 | ⬜ |
| 5 | ㅈ/ㅊ/ㅉ 치경구개 파찰음 SVG 자체 제작 | ⬜ |
| 6 | JSON 데이터 모델 (phoneme → path + audio + metadata) | ⬜ |
| 7 | React `<ArticulationDiagram>` 컴포넌트 + GSAP MorphSVG | ⬜ |
| 8 | TTS 오디오 동기화 (남성/여성) | ⬜ |

### 5.16 Phase 16 — AI TTS 영상 제작 (Video Production)

> 교재 JSON 데이터 기반 AI 음성 + 애니메이션 학습 영상 자동 생성 파이프라인.
> 영상 1개 + 자막 20개 언어 = 20개 언어 커버.

**파트별 구성**:
- **Part I. 발음**: AI 음성(남/여) + 조음 애니메이션 + 자막(한글 + 발음기호 + 학습자 모국어)
- **Part II~VI. 문법/문장**: AI 음성 + 텍스트 + 예문 순차 재생 + 모국어 자막

**학습 흐름**: [영상] 전체 흐름 한 번 시청 → [Study] 자기 페이스 연습/복습 → [이후] Study만 반복

| # | 작업 | 상태 |
|:-:|------|:----:|
| 1 | TTS 기술 선정 (Google Cloud TTS / CLOVA / OpenAI TTS 비교) | ⬜ |
| 2 | TTS 녹음 파이프라인 (JSON → 스크립트 → TTS → 오디오 파일) | ⬜ |
| 3 | 자막 자동 생성 (교재 JSON translations → SRT/VTT, 20개 언어) | ⬜ |
| 4 | 영상 템플릿 제작 (파트별 레이아웃) | ⬜ |
| 5 | 영상 자동 렌더링 파이프라인 (오디오 + 자막 + 애니메이션 → 영상) | ⬜ |
| 6 | Lesson/Video DB 연결 + lesson_item kind=video 시딩 | ⬜ |

[⬆️ 문서 상단으로 돌아가기](#amk_api_future--미구현-phase-스펙)

---

[⬆️ AMK_API_MASTER.md로 돌아가기](./AMK_API_MASTER.md)
