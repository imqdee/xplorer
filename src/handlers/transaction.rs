use crate::client::EtherscanClient;
use crate::error::XplorerError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransactionStatus {
    #[serde(rename = "isError")]
    pub is_error: String,
    #[serde(rename = "errDescription")]
    pub err_description: String,
}

pub async fn format_tx_status(
    client: &EtherscanClient,
    txhash: &str,
) -> Result<String, XplorerError> {
    let response = client
        .call_api("transaction", "getstatus", &[("txhash", txhash)])
        .await?;

    let status = response["status"].as_str().unwrap_or("0");
    if status == "0" {
        let message = response["result"].as_str().unwrap_or("Unknown API error");
        return Err(XplorerError::Api(message.to_string()));
    }

    let entry: TransactionStatus = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse transaction status: {e}")))?;

    let mut output = String::new();
    output.push_str(&format!("Tx Hash  : {txhash}\n"));
    if entry.is_error == "1" {
        output.push_str("Status   : Failed\n");
        output.push_str(&format!("Error    : {}\n", entry.err_description));
    } else {
        output.push_str("Status   : Success\n");
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_format_tx_status_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "transaction".into()),
                mockito::Matcher::UrlEncoded("action".into(), "getstatus".into()),
                mockito::Matcher::UrlEncoded("txhash".into(), "0xabc".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"status":"1","message":"OK","result":{"isError":"0","errDescription":""}}"#,
            )
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_tx_status(&client, "0xabc").await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Tx Hash  : 0xabc"));
        assert!(result.contains("Status   : Success"));
        assert!(!result.contains("Error"));
    }

    #[tokio::test]
    async fn test_format_tx_status_failed() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"status":"1","message":"OK","result":{"isError":"1","errDescription":"Bad jump destination"}}"#,
            )
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_tx_status(&client, "0xdef").await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Status   : Failed"));
        assert!(result.contains("Error    : Bad jump destination"));
    }
}
