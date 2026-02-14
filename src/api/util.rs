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
    if let Some(v) = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
    {
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
