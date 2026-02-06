//! IP Geolocation client using ip-api.com
//!
//! Free tier: 45 requests/minute (non-commercial use)
//! HTTP only (HTTPS requires paid plan)

use serde::Deserialize;
use tracing::{debug, warn};

/// IP geolocation data
#[derive(Debug, Clone, Default)]
pub struct GeoLocation {
    /// ISO 3166-1 alpha-2 country code (e.g., "KR", "US")
    pub country_code: Option<String>,
    /// Autonomous System Number
    pub asn: Option<i64>,
    /// Organization/ISP name
    pub org: Option<String>,
}

/// ip-api.com response structure
#[derive(Debug, Deserialize)]
struct IpApiResponse {
    status: String,
    #[serde(rename = "countryCode")]
    country_code: Option<String>,
    /// ASN string like "AS1234 Organization Name"
    #[serde(rename = "as")]
    as_info: Option<String>,
    /// Organization name
    org: Option<String>,
    /// ISP name
    isp: Option<String>,
}

/// IP Geolocation client
pub struct IpGeoClient {
    client: reqwest::Client,
    base_url: String,
}

impl IpGeoClient {
    /// Create a new IP geolocation client
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .expect("Failed to build reqwest Client for IpGeoClient"),
            // ip-api.com free tier is HTTP only
            base_url: "http://ip-api.com/json".to_string(),
        }
    }

    /// Lookup geolocation for an IP address
    ///
    /// Returns None if lookup fails (non-blocking, best-effort)
    pub async fn lookup(&self, ip: &str) -> GeoLocation {
        // Skip private/local IPs
        if Self::is_private_ip(ip) {
            debug!(ip = %ip, "Skipping geolocation for private IP");
            return GeoLocation::default();
        }

        let url = format!("{}/{}?fields=status,countryCode,as,org,isp", self.base_url, ip);

        match self.client.get(&url).send().await {
            Ok(response) => {
                match response.json::<IpApiResponse>().await {
                    Ok(data) => {
                        if data.status == "success" {
                            self.parse_response(data)
                        } else {
                            warn!(ip = %ip, "ip-api.com returned failure status");
                            GeoLocation::default()
                        }
                    }
                    Err(e) => {
                        warn!(ip = %ip, error = %e, "Failed to parse ip-api.com response");
                        GeoLocation::default()
                    }
                }
            }
            Err(e) => {
                warn!(ip = %ip, error = %e, "Failed to call ip-api.com");
                GeoLocation::default()
            }
        }
    }

    /// Parse the API response into GeoLocation
    fn parse_response(&self, data: IpApiResponse) -> GeoLocation {
        // Parse ASN from "AS1234 Organization Name" format
        let asn = data.as_info.as_ref().and_then(|s| {
            s.strip_prefix("AS")
                .and_then(|rest| rest.split_whitespace().next())
                .and_then(|num| num.parse::<i64>().ok())
        });

        // Use org, or fall back to ISP
        let org = data.org.or(data.isp);

        GeoLocation {
            country_code: data.country_code,
            asn,
            org,
        }
    }

    /// Check if IP is private/local (RFC 1918, loopback, etc.)
    fn is_private_ip(ip: &str) -> bool {
        use std::net::IpAddr;
        match ip.parse::<IpAddr>() {
            Ok(IpAddr::V4(v4)) => v4.is_private() || v4.is_loopback(),
            Ok(IpAddr::V6(v6)) => v6.is_loopback(),
            Err(_) => ip == "localhost",
        }
    }
}

impl Default for IpGeoClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_asn() {
        let client = IpGeoClient::new();
        let response = IpApiResponse {
            status: "success".to_string(),
            country_code: Some("KR".to_string()),
            as_info: Some("AS4766 Korea Telecom".to_string()),
            org: Some("Korea Telecom".to_string()),
            isp: None,
        };

        let geo = client.parse_response(response);
        assert_eq!(geo.country_code, Some("KR".to_string()));
        assert_eq!(geo.asn, Some(4766));
        assert_eq!(geo.org, Some("Korea Telecom".to_string()));
    }

    #[test]
    fn test_private_ip_detection() {
        assert!(IpGeoClient::is_private_ip("127.0.0.1"));
        assert!(IpGeoClient::is_private_ip("10.0.0.1"));
        assert!(IpGeoClient::is_private_ip("192.168.1.1"));
        assert!(IpGeoClient::is_private_ip("172.16.0.1"));
        assert!(!IpGeoClient::is_private_ip("8.8.8.8"));
    }
}
