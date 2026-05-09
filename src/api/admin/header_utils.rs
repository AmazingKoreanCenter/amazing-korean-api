//! Admin handler 공통 HTTP 헤더 추출 헬퍼.
//!
//! 4 도메인 (lesson/study/payment/user) handler 가 동일한 IP / User-Agent 추출
//! 로직을 중복 보유하던 것을 통합. payment 스타일 (trim + `USER_AGENT` 상수 +
//! `?` operator) 채택.

use axum::http::{header::USER_AGENT, HeaderMap};
use std::net::IpAddr;

/// 클라이언트 IP 추출. Cloudflare/Nginx proxy 환경 대응.
///
/// 우선순위: `x-forwarded-for` 첫 IP → `x-real-ip` → None.
/// 잘못된 IP 형식은 None.
pub fn extract_client_ip(headers: &HeaderMap) -> Option<IpAddr> {
    let forwarded = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|v| v.trim().to_string());

    let direct = headers
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.trim().to_string());

    let ip_str = forwarded.or(direct)?;
    ip_str.parse().ok()
}

/// User-Agent 헤더 추출. 부재 시 None.
pub fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get(USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    fn make_headers(pairs: &[(&'static str, &str)]) -> HeaderMap {
        let mut h = HeaderMap::new();
        for (k, v) in pairs {
            h.insert(*k, HeaderValue::from_str(v).unwrap());
        }
        h
    }

    // ------------------------------------------------------------------------
    // extract_client_ip
    // ------------------------------------------------------------------------

    #[test]
    fn test_extract_client_ip_uses_x_forwarded_for_first_value() {
        let headers = make_headers(&[("x-forwarded-for", "1.2.3.4, 5.6.7.8")]);
        let ip = extract_client_ip(&headers).expect("should parse");
        assert_eq!(ip.to_string(), "1.2.3.4");
    }

    #[test]
    fn test_extract_client_ip_trims_whitespace_in_forwarded() {
        // Cloudflare 가 "  1.2.3.4  ,5.6.7.8" 형태로 보낼 수도
        let headers = make_headers(&[("x-forwarded-for", "  1.2.3.4 ")]);
        let ip = extract_client_ip(&headers).expect("should parse trimmed");
        assert_eq!(ip.to_string(), "1.2.3.4");
    }

    #[test]
    fn test_extract_client_ip_falls_back_to_x_real_ip() {
        let headers = make_headers(&[("x-real-ip", "10.0.0.1")]);
        let ip = extract_client_ip(&headers).expect("should parse");
        assert_eq!(ip.to_string(), "10.0.0.1");
    }

    #[test]
    fn test_extract_client_ip_trims_x_real_ip() {
        let headers = make_headers(&[("x-real-ip", "  10.0.0.1  ")]);
        let ip = extract_client_ip(&headers).expect("should parse trimmed");
        assert_eq!(ip.to_string(), "10.0.0.1");
    }

    #[test]
    fn test_extract_client_ip_prefers_x_forwarded_over_x_real() {
        let headers = make_headers(&[("x-forwarded-for", "1.2.3.4"), ("x-real-ip", "10.0.0.1")]);
        let ip = extract_client_ip(&headers).expect("should parse");
        assert_eq!(ip.to_string(), "1.2.3.4", "x-forwarded-for 우선");
    }

    #[test]
    fn test_extract_client_ip_returns_none_for_missing_headers() {
        let headers = HeaderMap::new();
        assert!(extract_client_ip(&headers).is_none());
    }

    #[test]
    fn test_extract_client_ip_returns_none_for_invalid_format() {
        let headers = make_headers(&[("x-forwarded-for", "not-an-ip")]);
        assert!(extract_client_ip(&headers).is_none());
    }

    #[test]
    fn test_extract_client_ip_parses_ipv6() {
        let headers = make_headers(&[("x-forwarded-for", "2001:db8::1")]);
        let ip = extract_client_ip(&headers).expect("ipv6 should parse");
        assert!(ip.is_ipv6());
    }

    // ------------------------------------------------------------------------
    // extract_user_agent
    // ------------------------------------------------------------------------

    #[test]
    fn test_extract_user_agent_returns_value() {
        let headers = make_headers(&[("user-agent", "Mozilla/5.0 (Test)")]);
        assert_eq!(
            extract_user_agent(&headers),
            Some("Mozilla/5.0 (Test)".to_string())
        );
    }

    #[test]
    fn test_extract_user_agent_returns_none_when_missing() {
        let headers = HeaderMap::new();
        assert!(extract_user_agent(&headers).is_none());
    }
}
