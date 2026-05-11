use axum::http::HeaderMap;

/// HTTP 요청 헤더에서 클라이언트 IP 추출 (x-forwarded-for → x-real-ip → fallback)
pub fn extract_client_ip(headers: &HeaderMap) -> String {
    let try_parse_ip = |ip: &str| -> Option<String> {
        let trimmed = ip.trim();
        if trimmed.is_empty() {
            return None;
        }
        trimmed
            .parse::<std::net::IpAddr>()
            .ok()
            .map(|addr| addr.to_string())
    };

    // 1. x-forwarded-for
    if let Some(v) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(first) = v.split(',').next() {
            if let Some(ip) = try_parse_ip(first) {
                return ip;
            }
        }
    }
    // 2. x-real-ip
    if let Some(v) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        if let Some(ip) = try_parse_ip(v) {
            return ip;
        }
    }

    // 3. Fallback
    let use_fallback = std::env::var("AK_DEV_IP_FALLBACK")
        .ok()
        .map(|v| v == "true" || v == "1")
        .unwrap_or(true);
    if use_fallback {
        "127.0.0.1".to_string()
    } else {
        "0.0.0.0".to_string()
    }
}

#[cfg(test)]
#[allow(unsafe_code)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    /// AK_DEV_IP_FALLBACK env var 영향 격리용 = 명시적으로 설정.
    /// SAFETY: tests 단일 스레드 가정 (cargo test default = test threads 활용,
    /// 본 module 의 env-dependent tests 가 set_var 후 즉시 사용 = 같은 thread 내
    /// 순차 실행 가정). 병렬 race 시 #[serial_test] 도입 필요.
    fn with_fallback_env<F: FnOnce()>(value: &str, f: F) {
        // Rust 2024 edition 에서 set_var = unsafe (cross-thread race) — test 한정 허용.
        unsafe { std::env::set_var("AK_DEV_IP_FALLBACK", value) };
        f();
    }

    #[test]
    fn extract_returns_x_forwarded_for_first_ip() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            HeaderValue::from_static("203.0.113.1, 10.0.0.1"),
        );
        assert_eq!(extract_client_ip(&headers), "203.0.113.1");
    }

    #[test]
    fn extract_returns_x_forwarded_for_single_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", HeaderValue::from_static("203.0.113.42"));
        assert_eq!(extract_client_ip(&headers), "203.0.113.42");
    }

    #[test]
    fn extract_trims_whitespace_in_x_forwarded_for() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            HeaderValue::from_static("  203.0.113.7  , 10.0.0.1"),
        );
        assert_eq!(extract_client_ip(&headers), "203.0.113.7");
    }

    #[test]
    fn extract_falls_back_to_x_real_ip_when_forwarded_invalid() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", HeaderValue::from_static("not-an-ip"));
        headers.insert("x-real-ip", HeaderValue::from_static("198.51.100.5"));
        assert_eq!(extract_client_ip(&headers), "198.51.100.5");
    }

    #[test]
    fn extract_uses_x_real_ip_when_forwarded_missing() {
        let mut headers = HeaderMap::new();
        headers.insert("x-real-ip", HeaderValue::from_static("198.51.100.99"));
        assert_eq!(extract_client_ip(&headers), "198.51.100.99");
    }

    #[test]
    fn extract_falls_back_when_both_headers_missing() {
        with_fallback_env("true", || {
            let headers = HeaderMap::new();
            assert_eq!(extract_client_ip(&headers), "127.0.0.1");
        });
    }

    #[test]
    fn extract_returns_0_0_0_0_when_dev_fallback_disabled() {
        with_fallback_env("false", || {
            let headers = HeaderMap::new();
            assert_eq!(extract_client_ip(&headers), "0.0.0.0");
        });
    }

    #[test]
    fn extract_returns_0_0_0_0_when_dev_fallback_env_is_0() {
        with_fallback_env("0", || {
            let headers = HeaderMap::new();
            assert_eq!(extract_client_ip(&headers), "0.0.0.0");
        });
    }

    #[test]
    fn extract_supports_ipv6_in_x_forwarded_for() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-forwarded-for",
            HeaderValue::from_static("2001:db8::1, 10.0.0.1"),
        );
        assert_eq!(extract_client_ip(&headers), "2001:db8::1");
    }

    #[test]
    fn extract_skips_empty_first_segment_of_x_forwarded_for() {
        // 첫 segment 가 빈 문자열 = invalid IP → x-real-ip 시도.
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", HeaderValue::from_static(", 10.0.0.1"));
        headers.insert("x-real-ip", HeaderValue::from_static("198.51.100.1"));
        assert_eq!(extract_client_ip(&headers), "198.51.100.1");
    }
}
