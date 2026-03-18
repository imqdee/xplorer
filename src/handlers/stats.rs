use crate::client::EtherscanClient;
use crate::error::XplorerError;
use serde::Deserialize;

use super::fmt::check_api_status;

// --- Structs ---

#[derive(Debug, Deserialize)]
pub struct DailyAvgBlockSize {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "blockSize_bytes")]
    pub block_size_bytes: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct DailyBlockCount {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "blockCount")]
    pub block_count: serde_json::Value,
    #[serde(rename = "blockRewards_Eth")]
    pub block_rewards_eth: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyBlockRewards {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "blockRewards_Eth")]
    pub block_rewards_eth: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyAvgBlockTime {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "blockTime_sec")]
    pub block_time_sec: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyUncleBlockCount {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "uncleBlockCount")]
    pub uncle_block_count: serde_json::Value,
    #[serde(rename = "uncleBlockRewards_Eth")]
    pub uncle_block_rewards_eth: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyAvgGasLimit {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyAvgGasPrice {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "maxGasPrice_Wei")]
    pub max_gas_price_wei: String,
    #[serde(rename = "minGasPrice_Wei")]
    pub min_gas_price_wei: String,
    #[serde(rename = "avgGasPrice_Wei")]
    pub avg_gas_price_wei: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyGasUsed {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EthSupply2 {
    pub eth_supply: String,
    pub eth2_staking: String,
    pub burnt_fees: String,
    pub withdrawn_total: String,
}

#[derive(Debug, Deserialize)]
pub struct EthPrice {
    pub ethbtc: String,
    pub ethbtc_timestamp: String,
    pub ethusd: String,
    pub ethusd_timestamp: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeCount {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    pub total_node_count: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainSizeEntry {
    pub block_number: String,
    pub chain_time_stamp: String,
    pub chain_size: String,
    pub client_type: String,
    pub sync_mode: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyPrice {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyHashRate {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "networkHashRate")]
    pub network_hash_rate: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyNetDifficulty {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "networkDifficulty")]
    pub network_difficulty: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyNetUtilization {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "networkUtilization")]
    pub network_utilization: String,
}

#[derive(Debug, Deserialize)]
pub struct DailyNewAddress {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "newAddressCount")]
    pub new_address_count: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct DailyTxCount {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "transactionCount")]
    pub transaction_count: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct DailyTxnFee {
    #[serde(rename = "UTCDate")]
    pub utc_date: String,
    #[serde(rename = "transactionFee_Eth")]
    pub transaction_fee_eth: String,
}

// --- Format Functions ---

pub async fn format_dailyavgblocksize(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("stats", "dailyavgblocksize", params)
        .await?;
    check_api_status(&response)?;

    let entries: Vec<DailyAvgBlockSize> = serde_json::from_value(response["result"].clone())
        .map_err(|e| {
            XplorerError::Api(format!("Failed to parse dailyavgblocksize response: {e}"))
        })?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date       : {}\n", entry.utc_date));
        output.push_str(&format!("Block Size : {} bytes\n", entry.block_size_bytes));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_dailyblkcount(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "dailyblkcount", params).await?;
    check_api_status(&response)?;

    let entries: Vec<DailyBlockCount> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse dailyblkcount response: {e}")))?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date         : {}\n", entry.utc_date));
        output.push_str(&format!("Block Count  : {}\n", entry.block_count));
        output.push_str(&format!("Rewards (ETH): {}\n", entry.block_rewards_eth));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_dailyblockrewards(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("stats", "dailyblockrewards", params)
        .await?;
    check_api_status(&response)?;

    let entries: Vec<DailyBlockRewards> = serde_json::from_value(response["result"].clone())
        .map_err(|e| {
            XplorerError::Api(format!("Failed to parse dailyblockrewards response: {e}"))
        })?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date         : {}\n", entry.utc_date));
        output.push_str(&format!("Rewards (ETH): {}\n", entry.block_rewards_eth));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_dailyavgblocktime(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("stats", "dailyavgblocktime", params)
        .await?;
    check_api_status(&response)?;

    let entries: Vec<DailyAvgBlockTime> = serde_json::from_value(response["result"].clone())
        .map_err(|e| {
            XplorerError::Api(format!("Failed to parse dailyavgblocktime response: {e}"))
        })?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date       : {}\n", entry.utc_date));
        output.push_str(&format!("Block Time : {}s\n", entry.block_time_sec));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_dailyuncleblkcount(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("stats", "dailyuncleblkcount", params)
        .await?;
    check_api_status(&response)?;

    let entries: Vec<DailyUncleBlockCount> = serde_json::from_value(response["result"].clone())
        .map_err(|e| {
            XplorerError::Api(format!("Failed to parse dailyuncleblkcount response: {e}"))
        })?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date           : {}\n", entry.utc_date));
        output.push_str(&format!("Uncle Count    : {}\n", entry.uncle_block_count));
        output.push_str(&format!(
            "Rewards (ETH)  : {}\n",
            entry.uncle_block_rewards_eth
        ));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_dailyavggaslimit(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "dailyavggaslimit", params).await?;
    check_api_status(&response)?;

    let entries: Vec<DailyAvgGasLimit> = serde_json::from_value(response["result"].clone())
        .map_err(|e| {
            XplorerError::Api(format!("Failed to parse dailyavggaslimit response: {e}"))
        })?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date      : {}\n", entry.utc_date));
        output.push_str(&format!("Gas Limit : {}\n", entry.gas_limit));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_dailyavggasprice(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "dailyavggasprice", params).await?;
    check_api_status(&response)?;

    let entries: Vec<DailyAvgGasPrice> = serde_json::from_value(response["result"].clone())
        .map_err(|e| {
            XplorerError::Api(format!("Failed to parse dailyavggasprice response: {e}"))
        })?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date      : {}\n", entry.utc_date));
        output.push_str(&format!("Avg (Wei) : {}\n", entry.avg_gas_price_wei));
        output.push_str(&format!("Min (Wei) : {}\n", entry.min_gas_price_wei));
        output.push_str(&format!("Max (Wei) : {}\n", entry.max_gas_price_wei));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_dailygasused(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "dailygasused", params).await?;
    check_api_status(&response)?;

    let entries: Vec<DailyGasUsed> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse dailygasused response: {e}")))?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date     : {}\n", entry.utc_date));
        output.push_str(&format!("Gas Used : {}\n", entry.gas_used));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_ethsupply(client: &EtherscanClient) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "ethsupply", &[]).await?;
    check_api_status(&response)?;

    let supply = response["result"]
        .as_str()
        .ok_or_else(|| XplorerError::Api("Failed to parse ethsupply response".to_string()))?;

    Ok(format!("ETH Supply (wei): {supply}\n"))
}

pub async fn format_ethsupply2(client: &EtherscanClient) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "ethsupply2", &[]).await?;
    check_api_status(&response)?;

    let data: EthSupply2 = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse ethsupply2 response: {e}")))?;

    let mut output = String::new();
    output.push_str(&format!("ETH Supply     : {}\n", data.eth_supply));
    output.push_str(&format!("ETH2 Staking   : {}\n", data.eth2_staking));
    output.push_str(&format!("Burnt Fees     : {}\n", data.burnt_fees));
    output.push_str(&format!("Withdrawn Total: {}\n", data.withdrawn_total));
    Ok(output)
}

pub async fn format_ethprice(client: &EtherscanClient) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "ethprice", &[]).await?;
    check_api_status(&response)?;

    let data: EthPrice = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse ethprice response: {e}")))?;

    let mut output = String::new();
    output.push_str(&format!(
        "ETH/USD : {} ({})\n",
        data.ethusd,
        super::fmt::decimal_timestamp(&data.ethusd_timestamp)
    ));
    output.push_str(&format!(
        "ETH/BTC : {} ({})\n",
        data.ethbtc,
        super::fmt::decimal_timestamp(&data.ethbtc_timestamp)
    ));
    Ok(output)
}

pub async fn format_nodecount(client: &EtherscanClient) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "nodecount", &[]).await?;
    check_api_status(&response)?;

    let data: NodeCount = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse nodecount response: {e}")))?;

    let mut output = String::new();
    output.push_str(&format!("Date  : {}\n", data.utc_date));
    output.push_str(&format!("Nodes : {}\n", data.total_node_count));
    Ok(output)
}

pub async fn format_chainsize(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "chainsize", params).await?;
    check_api_status(&response)?;

    let entries: Vec<ChainSizeEntry> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse chainsize response: {e}")))?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Block      : {}\n", entry.block_number));
        output.push_str(&format!("Time       : {}\n", entry.chain_time_stamp));
        output.push_str(&format!("Chain Size : {} bytes\n", entry.chain_size));
        output.push_str(&format!("Client     : {}\n", entry.client_type));
        output.push_str(&format!("Sync Mode  : {}\n", entry.sync_mode));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_ethdailyprice(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "ethdailyprice", params).await?;
    check_api_status(&response)?;

    let entries: Vec<DailyPrice> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse ethdailyprice response: {e}")))?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date      : {}\n", entry.utc_date));
        output.push_str(&format!("Price USD : {}\n", entry.value));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_dailyavghashrate(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "dailyavghashrate", params).await?;
    check_api_status(&response)?;

    let entries: Vec<DailyHashRate> =
        serde_json::from_value(response["result"].clone()).map_err(|e| {
            XplorerError::Api(format!("Failed to parse dailyavghashrate response: {e}"))
        })?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date      : {}\n", entry.utc_date));
        output.push_str(&format!("Hash Rate : {}\n", entry.network_hash_rate));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_dailyavgnetdifficulty(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("stats", "dailyavgnetdifficulty", params)
        .await?;
    check_api_status(&response)?;

    let entries: Vec<DailyNetDifficulty> = serde_json::from_value(response["result"].clone())
        .map_err(|e| {
            XplorerError::Api(format!(
                "Failed to parse dailyavgnetdifficulty response: {e}"
            ))
        })?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date       : {}\n", entry.utc_date));
        output.push_str(&format!("Difficulty : {}\n", entry.network_difficulty));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_dailynetutilization(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("stats", "dailynetutilization", params)
        .await?;
    check_api_status(&response)?;

    let entries: Vec<DailyNetUtilization> = serde_json::from_value(response["result"].clone())
        .map_err(|e| {
            XplorerError::Api(format!("Failed to parse dailynetutilization response: {e}"))
        })?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date        : {}\n", entry.utc_date));
        output.push_str(&format!("Utilization : {}\n", entry.network_utilization));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_dailynewaddress(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "dailynewaddress", params).await?;
    check_api_status(&response)?;

    let entries: Vec<DailyNewAddress> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse dailynewaddress response: {e}")))?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date          : {}\n", entry.utc_date));
        output.push_str(&format!("New Addresses : {}\n", entry.new_address_count));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_dailytx(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "dailytx", params).await?;
    check_api_status(&response)?;

    let entries: Vec<DailyTxCount> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse dailytx response: {e}")))?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date  : {}\n", entry.utc_date));
        output.push_str(&format!("Tx    : {}\n", entry.transaction_count));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_dailytxnfee(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "dailytxnfee", params).await?;
    check_api_status(&response)?;

    let entries: Vec<DailyTxnFee> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse dailytxnfee response: {e}")))?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Date     : {}\n", entry.utc_date));
        output.push_str(&format!("Fee ETH  : {}\n", entry.transaction_fee_eth));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    Ok(output)
}

pub async fn format_tokensupply(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("stats", "tokensupply", params).await?;
    check_api_status(&response)?;

    let supply = response["result"]
        .as_str()
        .ok_or_else(|| XplorerError::Api("Failed to parse tokensupply response".to_string()))?;

    Ok(format!("Token Supply: {supply}\n"))
}

pub async fn format_tokensupplyhistory(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("stats", "tokensupplyhistory", params)
        .await?;
    check_api_status(&response)?;

    let supply = response["result"].as_str().ok_or_else(|| {
        XplorerError::Api("Failed to parse tokensupplyhistory response".to_string())
    })?;

    Ok(format!("Token Supply: {supply}\n"))
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

    // --- dailyavgblocksize ---

    #[tokio::test]
    async fn test_format_dailyavgblocksize_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailyavgblocksize".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","blockSize_bytes":20373},{"UTCDate":"2019-02-02","unixTimeStamp":"1549065600","blockSize_bytes":17499}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let params = [
            ("startdate", "2019-02-01"),
            ("enddate", "2019-02-28"),
            ("sort", "asc"),
        ];
        let result = format_dailyavgblocksize(&client, &params).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date       : 2019-02-01"));
        assert!(result.contains("Block Size : 20373 bytes"));
        assert!(result.contains("Date       : 2019-02-02"));
        assert!(result.contains("Block Size : 17499 bytes"));
    }

    #[tokio::test]
    async fn test_format_dailyavgblocksize_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyavgblocksize(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- dailyblkcount ---

    #[tokio::test]
    async fn test_format_dailyblkcount_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailyblkcount".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","blockCount":4848,"blockRewards_Eth":"14929.464690870590355682"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let params = [
            ("startdate", "2019-02-01"),
            ("enddate", "2019-02-28"),
            ("sort", "asc"),
        ];
        let result = format_dailyblkcount(&client, &params).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date         : 2019-02-01"));
        assert!(result.contains("Block Count  : 4848"));
        assert!(result.contains("Rewards (ETH): 14929.464690870590355682"));
    }

    #[tokio::test]
    async fn test_format_dailyblkcount_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyblkcount(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- dailyblockrewards ---

    #[tokio::test]
    async fn test_format_dailyblockrewards_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailyblockrewards".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","blockRewards_Eth":"15300.65625"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyblockrewards(
            &client,
            &[
                ("startdate", "2019-02-01"),
                ("enddate", "2019-02-28"),
                ("sort", "asc"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date         : 2019-02-01"));
        assert!(result.contains("Rewards (ETH): 15300.65625"));
    }

    #[tokio::test]
    async fn test_format_dailyblockrewards_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyblockrewards(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- dailyavgblocktime ---

    #[tokio::test]
    async fn test_format_dailyavgblocktime_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailyavgblocktime".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","blockTime_sec":"17.67"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyavgblocktime(
            &client,
            &[
                ("startdate", "2019-02-01"),
                ("enddate", "2019-02-28"),
                ("sort", "asc"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date       : 2019-02-01"));
        assert!(result.contains("Block Time : 17.67s"));
    }

    #[tokio::test]
    async fn test_format_dailyavgblocktime_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyavgblocktime(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- dailyuncleblkcount ---

    #[tokio::test]
    async fn test_format_dailyuncleblkcount_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailyuncleblkcount".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","uncleBlockCount":287,"uncleBlockRewards_Eth":"729.75"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyuncleblkcount(
            &client,
            &[
                ("startdate", "2019-02-01"),
                ("enddate", "2019-02-28"),
                ("sort", "asc"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date           : 2019-02-01"));
        assert!(result.contains("Uncle Count    : 287"));
        assert!(result.contains("Rewards (ETH)  : 729.75"));
    }

    #[tokio::test]
    async fn test_format_dailyuncleblkcount_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyuncleblkcount(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- dailyavggaslimit ---

    #[tokio::test]
    async fn test_format_dailyavggaslimit_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailyavggaslimit".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","gasLimit":"8001360"},{"UTCDate":"2019-02-02","unixTimeStamp":"1549065600","gasLimit":"8001269"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyavggaslimit(
            &client,
            &[
                ("startdate", "2019-02-01"),
                ("enddate", "2019-02-28"),
                ("sort", "asc"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date      : 2019-02-01"));
        assert!(result.contains("Gas Limit : 8001360"));
        assert!(result.contains("Date      : 2019-02-02"));
        assert!(result.contains("Gas Limit : 8001269"));
    }

    #[tokio::test]
    async fn test_format_dailyavggaslimit_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyavggaslimit(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- dailyavggasprice ---

    #[tokio::test]
    async fn test_format_dailyavggasprice_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailyavggasprice".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","maxGasPrice_Wei":"60814303896257","minGasPrice_Wei":"432495","avgGasPrice_Wei":"13234562600"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyavggasprice(
            &client,
            &[
                ("startdate", "2019-02-01"),
                ("enddate", "2019-02-28"),
                ("sort", "asc"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date      : 2019-02-01"));
        assert!(result.contains("Avg (Wei) : 13234562600"));
        assert!(result.contains("Min (Wei) : 432495"));
        assert!(result.contains("Max (Wei) : 60814303896257"));
    }

    #[tokio::test]
    async fn test_format_dailyavggasprice_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyavggasprice(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- dailygasused ---

    #[tokio::test]
    async fn test_format_dailygasused_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailygasused".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","gasUsed":"32761450415"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailygasused(
            &client,
            &[
                ("startdate", "2019-02-01"),
                ("enddate", "2019-02-28"),
                ("sort", "asc"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date     : 2019-02-01"));
        assert!(result.contains("Gas Used : 32761450415"));
    }

    #[tokio::test]
    async fn test_format_dailygasused_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailygasused(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- ethsupply ---

    #[tokio::test]
    async fn test_format_ethsupply_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "ethsupply".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(r#""122373866217800000000000000""#))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_ethsupply(&client).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("ETH Supply (wei): 122373866217800000000000000"));
    }

    #[tokio::test]
    async fn test_format_ethsupply_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Invalid API Key"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_ethsupply(&client).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Invalid API Key"));
    }

    // --- ethsupply2 ---

    #[tokio::test]
    async fn test_format_ethsupply2_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "ethsupply2".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"{"EthSupply":"122373866217800000000000000","Eth2Staking":"1234567890","BurntFees":"9876543210","WithdrawnTotal":"5555555555"}"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_ethsupply2(&client).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("ETH Supply     : 122373866217800000000000000"));
        assert!(result.contains("ETH2 Staking   : 1234567890"));
        assert!(result.contains("Burnt Fees     : 9876543210"));
        assert!(result.contains("Withdrawn Total: 5555555555"));
    }

    #[tokio::test]
    async fn test_format_ethsupply2_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Invalid API Key"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_ethsupply2(&client).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Invalid API Key"));
    }

    // --- ethprice ---

    #[tokio::test]
    async fn test_format_ethprice_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "ethprice".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"{"ethbtc":"0.05432","ethbtc_timestamp":"1700000000","ethusd":"1234.56","ethusd_timestamp":"1700000000"}"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_ethprice(&client).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("ETH/USD : 1234.56"));
        assert!(result.contains("ETH/BTC : 0.05432"));
    }

    #[tokio::test]
    async fn test_format_ethprice_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Invalid API Key"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_ethprice(&client).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Invalid API Key"));
    }

    // --- nodecount ---

    #[tokio::test]
    async fn test_format_nodecount_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "nodecount".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"{"UTCDate":"2024-01-01","TotalNodeCount":6432}"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_nodecount(&client).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date  : 2024-01-01"));
        assert!(result.contains("Nodes : 6432"));
    }

    #[tokio::test]
    async fn test_format_nodecount_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Invalid API Key"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_nodecount(&client).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Invalid API Key"));
    }

    // --- chainsize ---

    #[tokio::test]
    async fn test_format_chainsize_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "chainsize".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"blockNumber":"12345678","chainTimeStamp":"2024-01-01","chainSize":"123456789","clientType":"Geth","syncMode":"Default"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let params = [
            ("startdate", "2024-01-01"),
            ("enddate", "2024-01-31"),
            ("clienttype", "geth"),
            ("syncmode", "default"),
            ("sort", "asc"),
        ];
        let result = format_chainsize(&client, &params).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Block      : 12345678"));
        assert!(result.contains("Chain Size : 123456789 bytes"));
        assert!(result.contains("Client     : Geth"));
        assert!(result.contains("Sync Mode  : Default"));
    }

    #[tokio::test]
    async fn test_format_chainsize_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_chainsize(
            &client,
            &[("startdate", "2024-01-01"), ("enddate", "2024-01-31")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- ethdailyprice ---

    #[tokio::test]
    async fn test_format_ethdailyprice_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "ethdailyprice".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","value":"107.12"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_ethdailyprice(
            &client,
            &[
                ("startdate", "2019-02-01"),
                ("enddate", "2019-02-28"),
                ("sort", "asc"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date      : 2019-02-01"));
        assert!(result.contains("Price USD : 107.12"));
    }

    #[tokio::test]
    async fn test_format_ethdailyprice_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_ethdailyprice(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- dailyavghashrate ---

    #[tokio::test]
    async fn test_format_dailyavghashrate_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailyavghashrate".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","networkHashRate":"152,777,738,509,163"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyavghashrate(
            &client,
            &[
                ("startdate", "2019-02-01"),
                ("enddate", "2019-02-28"),
                ("sort", "asc"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date      : 2019-02-01"));
        assert!(result.contains("Hash Rate : 152,777,738,509,163"));
    }

    #[tokio::test]
    async fn test_format_dailyavghashrate_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyavghashrate(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- dailyavgnetdifficulty ---

    #[tokio::test]
    async fn test_format_dailyavgnetdifficulty_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailyavgnetdifficulty".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","networkDifficulty":"2,345,678,901,234"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyavgnetdifficulty(
            &client,
            &[
                ("startdate", "2019-02-01"),
                ("enddate", "2019-02-28"),
                ("sort", "asc"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date       : 2019-02-01"));
        assert!(result.contains("Difficulty : 2,345,678,901,234"));
    }

    #[tokio::test]
    async fn test_format_dailyavgnetdifficulty_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailyavgnetdifficulty(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- dailynetutilization ---

    #[tokio::test]
    async fn test_format_dailynetutilization_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailynetutilization".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","networkUtilization":"0.4562"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailynetutilization(
            &client,
            &[
                ("startdate", "2019-02-01"),
                ("enddate", "2019-02-28"),
                ("sort", "asc"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date        : 2019-02-01"));
        assert!(result.contains("Utilization : 0.4562"));
    }

    #[tokio::test]
    async fn test_format_dailynetutilization_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailynetutilization(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- dailynewaddress ---

    #[tokio::test]
    async fn test_format_dailynewaddress_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailynewaddress".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","newAddressCount":57527}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailynewaddress(
            &client,
            &[
                ("startdate", "2019-02-01"),
                ("enddate", "2019-02-28"),
                ("sort", "asc"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date          : 2019-02-01"));
        assert!(result.contains("New Addresses : 57527"));
    }

    #[tokio::test]
    async fn test_format_dailynewaddress_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailynewaddress(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- dailytx ---

    #[tokio::test]
    async fn test_format_dailytx_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailytx".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","transactionCount":456789}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailytx(
            &client,
            &[
                ("startdate", "2019-02-01"),
                ("enddate", "2019-02-28"),
                ("sort", "asc"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date  : 2019-02-01"));
        assert!(result.contains("Tx    : 456789"));
    }

    #[tokio::test]
    async fn test_format_dailytx_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailytx(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- dailytxnfee ---

    #[tokio::test]
    async fn test_format_dailytxnfee_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "dailytxnfee".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"UTCDate":"2019-02-01","unixTimeStamp":"1548979200","transactionFee_Eth":"123.456"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailytxnfee(
            &client,
            &[
                ("startdate", "2019-02-01"),
                ("enddate", "2019-02-28"),
                ("sort", "asc"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Date     : 2019-02-01"));
        assert!(result.contains("Fee ETH  : 123.456"));
    }

    #[tokio::test]
    async fn test_format_dailytxnfee_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_dailytxnfee(
            &client,
            &[("startdate", "2019-02-01"), ("enddate", "2019-02-28")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- tokensupply ---

    #[tokio::test]
    async fn test_format_tokensupply_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "tokensupply".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(r#""21000000000000000000000000""#))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokensupply(
            &client,
            &[(
                "contractaddress",
                "0xdAC17F958D2ee523a2206206994597C13D831ec7",
            )],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Token Supply: 21000000000000000000000000"));
    }

    #[tokio::test]
    async fn test_format_tokensupply_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Invalid address"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokensupply(&client, &[("contractaddress", "0xinvalid")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Invalid address"));
    }

    // --- tokensupplyhistory ---

    #[tokio::test]
    async fn test_format_tokensupplyhistory_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "stats".into()),
                mockito::Matcher::UrlEncoded("action".into(), "tokensupplyhistory".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(r#""20000000000000000000000000""#))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokensupplyhistory(
            &client,
            &[
                (
                    "contractaddress",
                    "0xdAC17F958D2ee523a2206206994597C13D831ec7",
                ),
                ("blockno", "8000000"),
            ],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Token Supply: 20000000000000000000000000"));
    }

    #[tokio::test]
    async fn test_format_tokensupplyhistory_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokensupplyhistory(
            &client,
            &[
                (
                    "contractaddress",
                    "0xdAC17F958D2ee523a2206206994597C13D831ec7",
                ),
                ("blockno", "8000000"),
            ],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }
}
