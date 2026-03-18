use crate::client::EtherscanClient;
use crate::error::XplorerError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiLimit {
    pub credits_used: u64,
    pub credits_available: u64,
    pub credit_limit: u64,
    pub limit_interval: String,
    pub interval_expiry_timespan: String,
}

fn format_number_with_commas(n: u64) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut result = String::new();
    for (i, &b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(b as char);
    }
    result
}

pub async fn format_api_limit(client: &EtherscanClient) -> Result<String, XplorerError> {
    let response = client
        .call_api("getapilimit", "getapilimit", &[])
        .await?;

    let status = response["status"].as_str().unwrap_or("0");
    if status == "0" {
        let message = response["result"].as_str().unwrap_or("Unknown API error");
        return Err(XplorerError::Api(message.to_string()));
    }

    let limit: ApiLimit = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse API limit response: {e}")))?;

    let mut output = String::new();
    output.push_str(&format!(
        "Credits Used       : {}\n",
        format_number_with_commas(limit.credits_used)
    ));
    output.push_str(&format!(
        "Credits Available  : {}\n",
        format_number_with_commas(limit.credits_available)
    ));
    output.push_str(&format!(
        "Credit Limit       : {}\n",
        format_number_with_commas(limit.credit_limit)
    ));
    output.push_str(&format!("Interval           : {}\n", limit.limit_interval));
    output.push_str(&format!(
        "Resets In          : {}\n",
        limit.interval_expiry_timespan
    ));

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[test]
    fn test_format_number_with_commas() {
        assert_eq!(format_number_with_commas(0), "0");
        assert_eq!(format_number_with_commas(22), "22");
        assert_eq!(format_number_with_commas(1_000), "1,000");
        assert_eq!(format_number_with_commas(1_500_000), "1,500,000");
        assert_eq!(format_number_with_commas(1_499_978), "1,499,978");
    }

    #[tokio::test]
    async fn test_format_api_limit_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "getapilimit".into()),
                mockito::Matcher::UrlEncoded("action".into(), "getapilimit".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "status": "1",
                "message": "OK",
                "result": {
                    "creditsUsed": 22,
                    "creditsAvailable": 1499978,
                    "creditLimit": 1500000,
                    "limitInterval": "daily",
                    "intervalExpiryTimespan": "08:42:34"
                }
            }"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), None, server.url());

        let result = format_api_limit(&client).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Credits Used       : 22"));
        assert!(result.contains("Credits Available  : 1,499,978"));
        assert!(result.contains("Credit Limit       : 1,500,000"));
        assert!(result.contains("Interval           : daily"));
        assert!(result.contains("Resets In          : 08:42:34"));
    }
}
