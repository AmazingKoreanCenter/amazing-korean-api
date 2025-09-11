2025-09-10 — Backend Videos: B4 완료 & B1 점검/복원

B4(자막 CRUD) 완료: POST/PUT/DELETE, is_default 토글 트랜잭션 보장, 409/404/204 동작 점검.

B1(Admin POST /admin/videos) 점검: 생성 핸들러/라우터/서비스/레포 존재 여부 확인 후, 누락 시 복구(핸들러 create_video_handler 추가, 라우터 POST 보강, service/repo 최소 구현).

문서 운영 원칙 도입: src/docs.rs는 API 집계기이므로 전체 교체 금지, 향후 문서 경로 추가는 append-only in-place로 진행.

후속 TODO: RBAC 연결(actor_user_id 추출), admin_action_log actor wiring, B5(태그 매핑) 구현.


2025-09-10 — Admin Videos B5 시작 (Tags)

B5: 태그 매핑 API 추가(POST/DELETE). POST는 멱등 200 + 현재 태그 목록 반환, DELETE는 멱등 204.

DB: video_tag ensure + video_tag_map upsert/delete 트랜잭션 구성, slug는 응답에서 계산.

운영: docs.rs는 이번 패치에서 미변경(append-only 원칙 유지), Axum 경로는 {param} 표기 강제.

후속: docs.rs에는 admin_add_tags, admin_remove_tags를 추후 append-only로 줄 추가 예정.