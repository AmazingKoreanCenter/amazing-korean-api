//! 시간 기반 세션 reaper.
//!
//! `login_expire_at` 이 지난 `active` 행을 주기적으로 `expired` 로 정리한다.
//! 이 reaper 가 없으면 DB 에 물리적으로 죽은 세션의 active 행이 무기한 잔존하여
//! (2026-05-30 MFA 세션 limit 사건의 잔여 원인) 카운트/대시보드를 왜곡한다.
//! Redis 키는 각자의 TTL 로 자연 만료되므로 여기서 건드리지 않는다.

use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio::time::{interval, MissedTickBehavior};

use crate::api::auth::repo::AuthRepo;

/// 세션 reaper 를 백그라운드 task 로 띄운다.
/// `interval_sec <= 0` 이면 비활성(부팅 안전, panic 게이트 없음).
/// `interval` 의 첫 tick 은 즉시 발화하므로 부팅 직후 누적분을 1회 정리한다.
pub fn spawn(db: Pool<Postgres>, interval_sec: i64) {
    if interval_sec <= 0 {
        tracing::info!("session reaper disabled (SESSION_REAPER_INTERVAL_SEC <= 0)");
        return;
    }
    let period = Duration::from_secs(interval_sec as u64);
    tokio::spawn(async move {
        let mut ticker = interval(period);
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
        loop {
            ticker.tick().await;
            match AuthRepo::reap_expired_sessions(&db).await {
                Ok(0) => {}
                Ok(n) => {
                    tracing::info!(reaped = n, "session reaper: expired stale active sessions")
                }
                Err(e) => tracing::warn!(error = %e, "session reaper run failed"),
            }
        }
    });
}
