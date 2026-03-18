use crate::client::EtherscanClient;
use crate::error::XplorerError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GasOracle {
    pub last_block: String,
    pub safe_gas_price: String,
    pub propose_gas_price: String,
    pub fast_gas_price: String,
    #[serde(rename = "suggestBaseFee")]
    pub suggest_base_fee: String,
    #[serde(rename = "gasUsedRatio")]
    #[allow(dead_code)]
    pub gas_used_ratio: String,
}

pub async fn format_gas_oracle(client: &EtherscanClient) -> Result<String, XplorerError> {
    let response = client.call_api("gastracker", "gasoracle", &[]).await?;

    let status = response["status"].as_str().unwrap_or("0");
    if status == "0" {
        let message = response["result"].as_str().unwrap_or("Unknown API error");
        return Err(XplorerError::Api(message.to_string()));
    }

    let oracle: GasOracle = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse gas oracle response: {e}")))?;

    let mut output = String::new();
    output.push_str(&format!("Block      : {}\n", oracle.last_block));
    output.push_str(&format!("Safe       : {} Gwei\n", oracle.safe_gas_price));
    output.push_str(&format!("Standard   : {} Gwei\n", oracle.propose_gas_price));
    output.push_str(&format!("Fast       : {} Gwei\n", oracle.fast_gas_price));
    output.push_str(&format!("Base Fee   : {} Gwei\n", oracle.suggest_base_fee));

    Ok(output)
}

pub async fn format_gas_estimate(
    client: &EtherscanClient,
    gasprice: &str,
) -> Result<String, XplorerError> {
    let response = client
        .call_api("gastracker", "gasestimate", &[("gasprice", gasprice)])
        .await?;

    let status = response["status"].as_str().unwrap_or("0");
    if status == "0" {
        let message = response["result"].as_str().unwrap_or("Unknown API error");
        return Err(XplorerError::Api(message.to_string()));
    }

    let seconds = response["result"]
        .as_str()
        .ok_or_else(|| XplorerError::Api("Expected string result for gas estimate".into()))?;
    Ok(format!("Estimated confirmation : {seconds}s\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_format_gas_oracle_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "gastracker".into()),
                mockito::Matcher::UrlEncoded("action".into(), "gasoracle".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "status": "1",
                "message": "OK",
                "result": {
                    "LastBlock": "23467872",
                    "SafeGasPrice": "0.496",
                    "ProposeGasPrice": "0.496",
                    "FastGasPrice": "0.554",
                    "suggestBaseFee": "0.496",
                    "gasUsedRatio": "0.3,0.5,0.2"
                }
            }"#,
            )
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_gas_oracle(&client).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Block      : 23467872"));
        assert!(result.contains("Safe       : 0.496 Gwei"));
        assert!(result.contains("Standard   : 0.496 Gwei"));
        assert!(result.contains("Fast       : 0.554 Gwei"));
        assert!(result.contains("Base Fee   : 0.496 Gwei"));
    }

    #[tokio::test]
    async fn test_format_gas_estimate_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "gastracker".into()),
                mockito::Matcher::UrlEncoded("action".into(), "gasestimate".into()),
                mockito::Matcher::UrlEncoded("gasprice".into(), "2000000000".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"1","message":"OK","result":"45"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_gas_estimate(&client, "2000000000").await.unwrap();
        mock.assert_async().await;

        assert_eq!(result, "Estimated confirmation : 45s\n");
    }

    #[tokio::test]
    async fn test_format_gas_oracle_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"0","message":"NOTOK","result":"Invalid API Key"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_gas_oracle(&client).await;
        mock.assert_async().await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid API Key"));
    }

    #[tokio::test]
    async fn test_format_gas_estimate_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"0","message":"NOTOK","result":"Invalid gas price"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_gas_estimate(&client, "0").await;
        mock.assert_async().await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid gas price")
        );
    }
}
