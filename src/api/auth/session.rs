//! 세션 폐기(revocation) 검증 — 보안 감사 2.1
//!
//! JWT 디코드 직후 호출. access token 의 `session_id` 가 Redis
//! `ak:session:{sid}` 에 살아있는지 확인해 로그아웃 / 비밀번호 변경 /
//! 강제 퇴장된 토큰을 만료(기본 15분) 전이라도 즉시 차단한다.
//! (삭제 인프라는 service.rs 에 이미 존재 — 본 모듈은 *검증* 만 연결)
//!
//! 정책 = **fail-open + 관찰성** (`docs/AMK_API_SECURITY_AUDIT.md` 2.1 결정):
//!   - Redis 정상·키 존재  → 통과
//!   - Redis 정상·키 부재  → 폐기됨 → 401
//!   - Redis 접속/조회 실패 → 검증 스킵(통과) + `tracing::warn!`
//!
//! fail-open 근거: 단일 Redis(deadpool) SPOF 에서 fail-closed 는 Redis 장애 시
//! 전면 인증 마비(가용성 사고). fail-open 의 노출은 access TTL 15분 상한 이내로,
//! 2.1 이전 이미 수용된 베이스라인으로만 후퇴할 뿐 그 이상 악화되지 않는다.
//! 관찰성: 스킵·거부를 `target = "security.session_revocation"` 로그로 가시화
//! (메트릭 facility 부재 → 로그 기반 알림. role_guard 의 tracing 패턴과 동일).

use redis::AsyncCommands;

use crate::error::AppError;
use crate::state::AppState;

/// 세션이 Redis 에 살아있으면 `Ok(())`, 명확히 폐기됐으면 `Err(Unauthorized)`,
/// Redis 불가 시 fail-open 으로 `Ok(())`(+warn).
pub async fn ensure_session_active(
    state: &AppState,
    session_id: &str,
    user_id: i64,
) -> Result<(), AppError> {
    let mut conn = match state.redis.get().await {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!(
                target: "security.session_revocation",
                reason = "redis_unavailable",
                session_id = %session_id,
                user_id,
                error = %e,
                "session revocation 검증 SKIP (fail-open) — Redis 접속 실패"
            );
            return Ok(());
        }
    };

    let key = format!("ak:session:{session_id}");
    match conn.exists::<_, bool>(&key).await {
        Ok(true) => Ok(()),
        Ok(false) => {
            tracing::info!(
                target: "security.session_revocation",
                session_id = %session_id,
                user_id,
                "401 reject — 폐기된 세션 (로그아웃/비번변경/강제퇴장)"
            );
            Err(AppError::Unauthorized("Session has been revoked".into()))
        }
        Err(e) => {
            tracing::warn!(
                target: "security.session_revocation",
                reason = "redis_query_failed",
                session_id = %session_id,
                user_id,
                error = %e,
                "session revocation 검증 SKIP (fail-open) — Redis 조회 실패"
            );
            Ok(())
        }
    }
}
