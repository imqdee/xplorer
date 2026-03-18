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
}
