use crate::client::EtherscanClient;
use crate::error::XplorerError;
use serde::Deserialize;

use super::fmt::{check_api_status, decimal_timestamp};

// --- Structs ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockReward {
    pub block_number: String,
    #[serde(alias = "timeStamp")]
    pub timestamp: String,
    pub block_miner: String,
    pub block_reward: String,
    pub uncles: Vec<Uncle>,
    pub uncle_inclusion_reward: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Uncle {
    pub miner: String,
    #[serde(alias = "blockreward")]
    pub block_reward: String,
    pub uncle_position: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockCountdown {
    pub current_block: String,
    pub countdown_block: String,
    pub remaining_block: String,
    pub estimate_time_in_sec: String,
}

// --- Format Functions ---

pub async fn format_getblockreward(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("block", "getblockreward", params).await?;
    check_api_status(&response)?;

    let reward: BlockReward = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse getblockreward response: {e}")))?;

    let mut output = String::new();
    output.push_str(&format!("Block        : {}\n", reward.block_number));
    output.push_str(&format!(
        "Time         : {}\n",
        decimal_timestamp(&reward.timestamp)
    ));
    output.push_str(&format!("Miner        : {}\n", reward.block_miner));
    output.push_str(&format!("Reward       : {}\n", reward.block_reward));
    output.push_str(&format!(
        "Uncle Reward : {}\n",
        reward.uncle_inclusion_reward
    ));

    for (i, uncle) in reward.uncles.iter().enumerate() {
        output.push_str(&format!("  Uncle {}:\n", i + 1));
        output.push_str(&format!("    Miner    : {}\n", uncle.miner));
        output.push_str(&format!("    Position : {}\n", uncle.uncle_position));
        output.push_str(&format!("    Reward   : {}\n", uncle.block_reward));
    }

    Ok(output)
}

pub async fn format_getblockcountdown(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("block", "getblockcountdown", params)
        .await?;
    check_api_status(&response)?;

    let countdown: BlockCountdown =
        serde_json::from_value(response["result"].clone()).map_err(|e| {
            XplorerError::Api(format!("Failed to parse getblockcountdown response: {e}"))
        })?;

    let mut output = String::new();
    output.push_str(&format!("Current Block : {}\n", countdown.current_block));
    output.push_str(&format!("Target Block  : {}\n", countdown.countdown_block));
    output.push_str(&format!(
        "Remaining     : {} blocks\n",
        countdown.remaining_block
    ));
    output.push_str(&format!(
        "Est. Time     : {}s\n",
        countdown.estimate_time_in_sec
    ));

    Ok(output)
}

pub async fn format_getblocknobytime(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("block", "getblocknobytime", params).await?;
    check_api_status(&response)?;

    let result = &response["result"];
    let mut output = String::new();

    let timestamp = params
        .iter()
        .find(|(k, _)| *k == "timestamp")
        .map(|(_, v)| *v)
        .unwrap_or("unknown");
    let closest = params
        .iter()
        .find(|(k, _)| *k == "closest")
        .map(|(_, v)| *v)
        .unwrap_or("before");

    output.push_str(&format!("Timestamp : {timestamp}\n"));
    output.push_str(&format!("Closest   : {closest}\n"));
    output.push_str(&format!("Block     : {}\n", result.as_str().unwrap_or("0")));

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    fn mock_success(result: &str) -> String {
        format!(r#"{{"status":"1","message":"OK","result":{result}}}"#)
    }

    fn mock_error(msg: &str) -> String {
        format!(r#"{{"status":"0","message":"NOTOK","result":"{msg}"}}"#)
    }

    // --- getblockreward ---

    #[tokio::test]
    async fn test_format_getblockreward_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "block".into()),
                mockito::Matcher::UrlEncoded("action".into(), "getblockreward".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"{"blockNumber":"2165403","timeStamp":"1472533979","blockMiner":"0x13a06d3dfe21e0db5c016c03ea7d2f7dcdda4850","blockReward":"5314181600000000000","uncles":[{"miner":"0xbcdfc35b86bedf72f0cda046a3c16829a2ef41d1","unclePosition":"0","blockreward":"3750000000000000000"}],"uncleInclusionReward":"312500000000000000"}"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_getblockreward(&client, &[("blockno", "2165403")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Block        : 2165403"));
        assert!(result.contains("Time         : 2016-08-30 05:12:59 UTC"));
        assert!(result.contains("Miner        : 0x13a06d3dfe21e0db5c016c03ea7d2f7dcdda4850"));
        assert!(result.contains("Reward       : 5314181600000000000"));
        assert!(result.contains("Uncle Reward : 312500000000000000"));
        assert!(result.contains("  Uncle 1:"));
        assert!(result.contains("    Miner    : 0xbcdfc35b86bedf72f0cda046a3c16829a2ef41d1"));
        assert!(result.contains("    Position : 0"));
        assert!(result.contains("    Reward   : 3750000000000000000"));
    }

    #[tokio::test]
    async fn test_format_getblockreward_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Block does not exist"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_getblockreward(&client, &[("blockno", "999999999")]).await;
        mock.assert_async().await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Block does not exist")
        );
    }

    // --- getblockcountdown ---

    #[tokio::test]
    async fn test_format_getblockcountdown_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "block".into()),
                mockito::Matcher::UrlEncoded("action".into(), "getblockcountdown".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"{"CurrentBlock":"24685913","CountdownBlock":"25000000","RemainingBlock":"314087","EstimateTimeInSec":"3769059.0"}"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_getblockcountdown(&client, &[("blockno", "25000000")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Current Block : 24685913"));
        assert!(result.contains("Target Block  : 25000000"));
        assert!(result.contains("Remaining     : 314087 blocks"));
        assert!(result.contains("Est. Time     : 3769059.0s"));
    }

    #[tokio::test]
    async fn test_format_getblockcountdown_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Block already passed"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_getblockcountdown(&client, &[("blockno", "1")]).await;
        mock.assert_async().await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Block already passed")
        );
    }

    // --- getblocknobytime ---

    #[tokio::test]
    async fn test_format_getblocknobytime_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "block".into()),
                mockito::Matcher::UrlEncoded("action".into(), "getblocknobytime".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(r#""9251482""#))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_getblocknobytime(
            &client,
            &[("timestamp", "1578638524"), ("closest", "before")],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Timestamp : 1578638524"));
        assert!(result.contains("Closest   : before"));
        assert!(result.contains("Block     : 9251482"));
    }

    #[tokio::test]
    async fn test_format_getblocknobytime_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Invalid timestamp"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result =
            format_getblocknobytime(&client, &[("timestamp", "0"), ("closest", "before")]).await;
        mock.assert_async().await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid timestamp")
        );
    }
}
