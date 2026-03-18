use crate::client::EtherscanClient;
use crate::error::XplorerError;
use serde::Deserialize;

use super::fmt::{format_timestamp, hex_to_u64};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct LogEntry {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub block_number: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
    pub gas_price: String,
    pub gas_used: String,
    pub log_index: String,
    pub transaction_hash: String,
    pub transaction_index: String,
}

pub async fn format_logs(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("logs", "getLogs", params).await?;

    let status = response["status"].as_str().unwrap_or("0");
    if status == "0" {
        let message = response["result"].as_str().unwrap_or("Unknown API error");
        return Err(XplorerError::Api(message.to_string()));
    }

    let entries: Vec<LogEntry> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse logs response: {e}")))?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Tx Hash  : {}\n", entry.transaction_hash));
        output.push_str(&format!(
            "Block    : {}\n",
            hex_to_u64(&entry.block_number)
                .map(|n| n.to_string())
                .unwrap_or_else(|| entry.block_number.clone())
        ));
        output.push_str(&format!(
            "Time     : {}\n",
            format_timestamp(&entry.timestamp)
        ));
        output.push_str(&format!("Address  : {}\n", entry.address));

        if !entry.topics.is_empty() {
            output.push_str("Topics   :\n");
            for (j, topic) in entry.topics.iter().enumerate() {
                output.push_str(&format!("  [{j}] {topic}\n"));
            }
        }

        output.push_str(&format!("Data     : {}\n", entry.data));
        output.push_str(&format!(
            "Gas Price: {}\n",
            hex_to_u64(&entry.gas_price)
                .map(|n| n.to_string())
                .unwrap_or_else(|| entry.gas_price.clone())
        ));
        output.push_str(&format!(
            "Gas Used : {}\n",
            hex_to_u64(&entry.gas_used)
                .map(|n| n.to_string())
                .unwrap_or_else(|| entry.gas_used.clone())
        ));
        output.push_str(&format!(
            "Log Index: {}\n",
            hex_to_u64(&entry.log_index)
                .map(|n| n.to_string())
                .unwrap_or_else(|| entry.log_index.clone())
        ));

        if i < entries.len() - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_format_logs_success() {
        let mut server = mockito::Server::new_async().await;
        let log_response = r#"{
            "status": "1",
            "message": "OK",
            "result": [{
                "address": "0x5972",
                "topics": ["0x27c4f040", "0x00000000"],
                "data": "0x00000000",
                "blockNumber": "0xe62a42",
                "timeStamp": "0x62c2bc6f",
                "gasPrice": "0x5e7008e19",
                "gasUsed": "0xfb818",
                "logIndex": "0x4b",
                "transactionHash": "0x26fe",
                "transactionIndex": "0x3a"
            }]
        }"#;

        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "logs".into()),
                mockito::Matcher::UrlEncoded("action".into(), "getLogs".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(log_response)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let params = [("fromBlock", "12878196"), ("toBlock", "12878300")];
        let result = format_logs(&client, &params).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Tx Hash  : 0x26fe"));
        assert!(result.contains("Block    : 15084098"));
        assert!(result.contains("Time     : 2022-07-04 10:09:51 UTC"));
        assert!(result.contains("Address  : 0x5972"));
        assert!(result.contains("[0] 0x27c4f040"));
        assert!(result.contains("[1] 0x00000000"));
        assert!(result.contains("Gas Price: 25350409753"));
        assert!(result.contains("Gas Used : 1030168"));
        assert!(result.contains("Log Index: 75"));
    }

    #[tokio::test]
    async fn test_format_logs_multiple_entries() {
        let mut server = mockito::Server::new_async().await;
        let log_response = r#"{
            "status": "1",
            "message": "OK",
            "result": [
                {
                    "address": "0xaaa",
                    "topics": ["0xtopic1"],
                    "data": "0xdata1",
                    "blockNumber": "0x1",
                    "timeStamp": "0x0",
                    "gasPrice": "0x1",
                    "gasUsed": "0x1",
                    "logIndex": "0x0",
                    "transactionHash": "0xtx1",
                    "transactionIndex": "0x0"
                },
                {
                    "address": "0xbbb",
                    "topics": [],
                    "data": "0xdata2",
                    "blockNumber": "0x2",
                    "timeStamp": "0x0",
                    "gasPrice": "0x2",
                    "gasUsed": "0x2",
                    "logIndex": "0x1",
                    "transactionHash": "0xtx2",
                    "transactionIndex": "0x1"
                }
            ]
        }"#;

        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(log_response)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let params = [("fromBlock", "0"), ("toBlock", "latest")];
        let result = format_logs(&client, &params).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("0xtx1"));
        assert!(result.contains("0xtx2"));
        let newlines = result.matches("\n\n").count();
        assert_eq!(newlines, 1);
    }

    #[tokio::test]
    async fn test_format_logs_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"0","message":"NOTOK","result":"No records found"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let params = [("fromBlock", "0"), ("toBlock", "1")];
        let result = format_logs(&client, &params).await;
        mock.assert_async().await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No records found"));
    }
}
