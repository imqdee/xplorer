use crate::error::XplorerError;
use chrono::DateTime;

pub(crate) fn hex_to_u64(hex: &str) -> Option<u64> {
    let stripped = hex.strip_prefix("0x").unwrap_or(hex);
    u64::from_str_radix(stripped, 16).ok()
}

pub(crate) fn format_timestamp(hex: &str) -> String {
    hex_to_u64(hex)
        .and_then(|ts| i64::try_from(ts).ok())
        .and_then(|ts| DateTime::from_timestamp(ts, 0))
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| hex.to_string())
}

pub(crate) fn decimal_timestamp(s: &str) -> String {
    s.parse::<i64>()
        .ok()
        .and_then(|ts| DateTime::from_timestamp(ts, 0))
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| s.to_string())
}

pub(crate) fn check_api_status(response: &serde_json::Value) -> Result<(), XplorerError> {
    let status = response["status"].as_str().unwrap_or("0");
    if status == "0" {
        let message = response["result"].as_str().unwrap_or("Unknown API error");
        return Err(XplorerError::Api(message.to_string()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_u64() {
        assert_eq!(hex_to_u64("0xe62a42"), Some(15_084_098));
        assert_eq!(hex_to_u64("0x0"), Some(0));
        assert_eq!(hex_to_u64("0x4b"), Some(75));
        assert_eq!(hex_to_u64("invalid"), None);
    }

    #[test]
    fn test_format_timestamp() {
        assert_eq!(format_timestamp("0x62c2bc6f"), "2022-07-04 10:09:51 UTC");
        assert_eq!(format_timestamp("invalid"), "invalid");
    }

    #[test]
    fn test_decimal_timestamp() {
        assert_eq!(decimal_timestamp("1598242563"), "2020-08-24 04:16:03 UTC");
        assert_eq!(decimal_timestamp("not-a-number"), "not-a-number");
    }

    #[test]
    fn test_check_api_status_success() {
        let response = serde_json::json!({"status": "1", "message": "OK", "result": "something"});
        assert!(check_api_status(&response).is_ok());
    }

    #[test]
    fn test_check_api_status_error() {
        let response =
            serde_json::json!({"status": "0", "message": "NOTOK", "result": "Invalid API Key"});
        let err = check_api_status(&response).unwrap_err();
        assert!(err.to_string().contains("Invalid API Key"));
    }

    #[test]
    fn test_check_api_status_missing_status() {
        let response = serde_json::json!({"message": "NOTOK"});
        let err = check_api_status(&response).unwrap_err();
        assert!(err.to_string().contains("Unknown API error"));
    }
}
