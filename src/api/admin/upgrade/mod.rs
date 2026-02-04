//! 관리자 초대/승격 모듈
//!
//! 관리자 계정은 오직 초대를 통해서만 생성 가능.
//! 일반 회원가입 후 승격 불가.
//!
//! ## 엔드포인트
//! - POST /admin/upgrade: 관리자 초대 코드 생성 및 이메일 발송
//! - GET /admin/upgrade/verify: 초대 코드 검증
//! - POST /admin/upgrade/accept: 초대 수락 및 관리자 계정 생성
//!
//! ## 권한 규칙
//! - HYMN -> Admin, Manager 초대 가능
//! - Admin -> Manager만 초대 가능
//! - Manager -> 초대 불가 (403)
//!
//! ## 보안 정책
//! - 관리자 계정: OAuth 로그인 비허용 (이메일/비밀번호만)
//! - 초대 코드: Redis 저장, TTL 10분, 일회용
//! - 기존 이메일로 초대 시: 거부 (이미 가입된 이메일)

pub mod dto;
pub mod handler;
pub mod router;
pub mod service;
