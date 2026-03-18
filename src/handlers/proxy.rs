use crate::client::EtherscanClient;
use crate::error::XplorerError;
use serde::Deserialize;

use super::fmt::hex_to_u64;

// --- Helpers ---

fn check_proxy_response(response: &serde_json::Value) -> Result<(), XplorerError> {
    if response.get("status").and_then(|s| s.as_str()) == Some("0") {
        let message = response["result"].as_str().unwrap_or("Unknown API error");
        return Err(XplorerError::Api(message.to_string()));
    }
    if let Some(error) = response.get("error") {
        let message = error["message"].as_str().unwrap_or("Unknown proxy error");
        return Err(XplorerError::Api(message.to_string()));
    }
    Ok(())
}

pub fn print_raw_proxy_result(response: &serde_json::Value) -> Result<(), XplorerError> {
    check_proxy_response(response)?;
    let result_compact = serde_json::to_string(&response["result"])
        .map_err(|e| XplorerError::Api(format!("Failed to serialize result: {e}")))?;
    println!("{result_compact}");
    Ok(())
}

fn hex_display(hex: &Option<String>) -> String {
    hex.as_ref()
        .and_then(|h| hex_to_u64(h))
        .map(|n| n.to_string())
        .unwrap_or_else(|| hex.as_deref().unwrap_or("N/A").to_string())
}

fn hex_or_na(opt: &Option<String>) -> &str {
    opt.as_deref().unwrap_or("N/A")
}

// --- Structs ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyBlock {
    pub number: Option<String>,
    pub hash: Option<String>,
    pub timestamp: Option<String>,
    pub miner: Option<String>,
    pub gas_used: Option<String>,
    pub gas_limit: Option<String>,
    pub base_fee_per_gas: Option<String>,
    pub transactions: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyTransaction {
    pub hash: Option<String>,
    pub block_number: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: Option<String>,
    pub gas: Option<String>,
    pub gas_price: Option<String>,
    pub nonce: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyReceipt {
    pub transaction_hash: Option<String>,
    pub block_number: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub status: Option<String>,
    pub gas_used: Option<String>,
    pub logs: Option<serde_json::Value>,
}

// --- Format Functions ---

pub async fn format_eth_block_number(client: &EtherscanClient) -> Result<String, XplorerError> {
    let response = client.call_api("proxy", "eth_blockNumber", &[]).await?;
    check_proxy_response(&response)?;

    let hex = response["result"].as_str().unwrap_or("0x0");
    let num = hex_to_u64(hex)
        .map(|n| n.to_string())
        .unwrap_or_else(|| hex.to_string());
    Ok(format!("Block Number: {num}\n"))
}

pub async fn format_eth_get_block_by_number(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("proxy", "eth_getBlockByNumber", params)
        .await?;
    check_proxy_response(&response)?;

    if response["result"].is_null() {
        return Ok("Block not found\n".to_string());
    }

    let block: ProxyBlock = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse block response: {e}")))?;

    let tx_count = block
        .transactions
        .as_ref()
        .and_then(|t| t.as_array())
        .map(|a| a.len())
        .unwrap_or(0);

    let mut output = String::new();
    output.push_str(&format!("Block        : {}\n", hex_display(&block.number)));
    output.push_str(&format!("Hash         : {}\n", hex_or_na(&block.hash)));
    output.push_str(&format!(
        "Timestamp    : {}\n",
        block
            .timestamp
            .as_deref()
            .map(super::fmt::decimal_timestamp)
            .unwrap_or_else(|| "N/A".to_string())
    ));
    output.push_str(&format!("Miner        : {}\n", hex_or_na(&block.miner)));
    output.push_str(&format!(
        "Gas Used     : {}\n",
        hex_display(&block.gas_used)
    ));
    output.push_str(&format!(
        "Gas Limit    : {}\n",
        hex_display(&block.gas_limit)
    ));
    output.push_str(&format!(
        "Base Fee     : {} wei\n",
        hex_display(&block.base_fee_per_gas)
    ));
    output.push_str(&format!("Transactions : {tx_count}\n"));
    Ok(output)
}

pub async fn format_eth_get_block_transaction_count_by_number(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("proxy", "eth_getBlockTransactionCountByNumber", params)
        .await?;
    check_proxy_response(&response)?;

    let hex = response["result"].as_str().unwrap_or("0x0");
    let count = hex_to_u64(hex)
        .map(|n| n.to_string())
        .unwrap_or_else(|| hex.to_string());
    Ok(format!("Transaction Count: {count}\n"))
}

pub async fn format_eth_get_uncle_by_block_number_and_index(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("proxy", "eth_getUncleByBlockNumberAndIndex", params)
        .await?;
    check_proxy_response(&response)?;

    if response["result"].is_null() {
        return Ok("Uncle not found\n".to_string());
    }

    let block: ProxyBlock = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse uncle response: {e}")))?;

    let mut output = String::new();
    output.push_str(&format!("Block     : {}\n", hex_display(&block.number)));
    output.push_str(&format!("Hash      : {}\n", hex_or_na(&block.hash)));
    output.push_str(&format!(
        "Timestamp : {}\n",
        block
            .timestamp
            .as_deref()
            .map(super::fmt::decimal_timestamp)
            .unwrap_or_else(|| "N/A".to_string())
    ));
    output.push_str(&format!("Miner     : {}\n", hex_or_na(&block.miner)));
    output.push_str(&format!("Gas Used  : {}\n", hex_display(&block.gas_used)));
    output.push_str(&format!("Gas Limit : {}\n", hex_display(&block.gas_limit)));
    Ok(output)
}

fn format_transaction(tx: &ProxyTransaction) -> String {
    let mut output = String::new();
    output.push_str(&format!("Hash      : {}\n", hex_or_na(&tx.hash)));
    output.push_str(&format!("Block     : {}\n", hex_display(&tx.block_number)));
    output.push_str(&format!("From      : {}\n", hex_or_na(&tx.from)));
    output.push_str(&format!("To        : {}\n", hex_or_na(&tx.to)));
    output.push_str(&format!("Value     : {} wei\n", hex_display(&tx.value)));
    output.push_str(&format!("Gas       : {}\n", hex_display(&tx.gas)));
    output.push_str(&format!("Gas Price : {} wei\n", hex_display(&tx.gas_price)));
    output.push_str(&format!("Nonce     : {}\n", hex_display(&tx.nonce)));
    output
}

pub async fn format_eth_get_transaction_by_hash(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("proxy", "eth_getTransactionByHash", params)
        .await?;
    check_proxy_response(&response)?;

    if response["result"].is_null() {
        return Ok("Transaction not found\n".to_string());
    }

    let tx: ProxyTransaction = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse transaction response: {e}")))?;

    Ok(format_transaction(&tx))
}

pub async fn format_eth_get_transaction_by_block_number_and_index(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("proxy", "eth_getTransactionByBlockNumberAndIndex", params)
        .await?;
    check_proxy_response(&response)?;

    if response["result"].is_null() {
        return Ok("Transaction not found\n".to_string());
    }

    let tx: ProxyTransaction = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse transaction response: {e}")))?;

    Ok(format_transaction(&tx))
}

pub async fn format_eth_get_transaction_count(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("proxy", "eth_getTransactionCount", params)
        .await?;
    check_proxy_response(&response)?;

    let hex = response["result"].as_str().unwrap_or("0x0");
    let count = hex_to_u64(hex)
        .map(|n| n.to_string())
        .unwrap_or_else(|| hex.to_string());
    Ok(format!("Transaction Count: {count}\n"))
}

pub async fn format_eth_get_transaction_receipt(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("proxy", "eth_getTransactionReceipt", params)
        .await?;
    check_proxy_response(&response)?;

    if response["result"].is_null() {
        return Ok("Receipt not found\n".to_string());
    }

    let receipt: ProxyReceipt = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse receipt response: {e}")))?;

    let status_label = receipt
        .status
        .as_deref()
        .map(|s| if s == "0x1" { "Success" } else { "Failed" })
        .unwrap_or("N/A");

    let log_count = receipt
        .logs
        .as_ref()
        .and_then(|l| l.as_array())
        .map(|a| a.len())
        .unwrap_or(0);

    let mut output = String::new();
    output.push_str(&format!(
        "Tx Hash   : {}\n",
        hex_or_na(&receipt.transaction_hash)
    ));
    output.push_str(&format!(
        "Block     : {}\n",
        hex_display(&receipt.block_number)
    ));
    output.push_str(&format!("From      : {}\n", hex_or_na(&receipt.from)));
    output.push_str(&format!("To        : {}\n", hex_or_na(&receipt.to)));
    output.push_str(&format!("Status    : {status_label}\n"));
    output.push_str(&format!("Gas Used  : {}\n", hex_display(&receipt.gas_used)));
    output.push_str(&format!("Log Count : {log_count}\n"));
    Ok(output)
}

pub async fn format_eth_call(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("proxy", "eth_call", params).await?;
    check_proxy_response(&response)?;

    let result = response["result"].as_str().unwrap_or("0x");
    Ok(format!("Result: {result}\n"))
}

pub async fn format_eth_get_code(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("proxy", "eth_getCode", params).await?;
    check_proxy_response(&response)?;

    let code = response["result"].as_str().unwrap_or("0x");
    Ok(format!("Code: {code}\n"))
}

pub async fn format_eth_get_storage_at(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("proxy", "eth_getStorageAt", params).await?;
    check_proxy_response(&response)?;

    let storage = response["result"].as_str().unwrap_or("0x");
    Ok(format!("Storage: {storage}\n"))
}

pub async fn format_eth_gas_price(client: &EtherscanClient) -> Result<String, XplorerError> {
    let response = client.call_api("proxy", "eth_gasPrice", &[]).await?;
    check_proxy_response(&response)?;

    let hex = response["result"].as_str().unwrap_or("0x0");
    let wei = hex_to_u64(hex).unwrap_or(0);
    let gwei = wei as f64 / 1_000_000_000.0;
    Ok(format!("Gas Price: {wei} wei ({gwei:.2} Gwei)\n"))
}

pub async fn format_eth_estimate_gas(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("proxy", "eth_estimateGas", params).await?;
    check_proxy_response(&response)?;

    let hex = response["result"].as_str().unwrap_or("0x0");
    let gas = hex_to_u64(hex)
        .map(|n| n.to_string())
        .unwrap_or_else(|| hex.to_string());
    Ok(format!("Estimated Gas: {gas}\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    fn mock_proxy_success(result: &str) -> String {
        format!(r#"{{"jsonrpc":"2.0","id":1,"result":{result}}}"#)
    }

    fn mock_proxy_error(msg: &str) -> String {
        format!(r#"{{"jsonrpc":"2.0","id":1,"error":{{"code":-32602,"message":"{msg}"}}}}"#)
    }

    // --- eth_blockNumber ---

    #[tokio::test]
    async fn test_format_eth_block_number_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded("action".into(), "eth_blockNumber".into()),
            ]))
            .with_status(200)
            .with_body(mock_proxy_success(r#""0x10d4f""#))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_block_number(&client).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Block Number: 68943"));
    }

    #[tokio::test]
    async fn test_format_eth_block_number_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_proxy_error("invalid request"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_block_number(&client).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("invalid request"));
    }

    // --- eth_getBlockByNumber ---

    #[tokio::test]
    async fn test_format_eth_get_block_by_number_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded("action".into(), "eth_getBlockByNumber".into()),
            ]))
            .with_status(200)
            .with_body(mock_proxy_success(
                r#"{"number":"0xa","hash":"0xabc","timestamp":"0x5f5e100","miner":"0xdef","gasUsed":"0x5208","gasLimit":"0x1c9c380","baseFeePerGas":"0x3b9aca00","transactions":["0x1","0x2"]}"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result =
            format_eth_get_block_by_number(&client, &[("tag", "0xa"), ("boolean", "false")])
                .await
                .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Block        : 10"));
        assert!(result.contains("Hash         : 0xabc"));
        assert!(result.contains("Gas Used     : 21000"));
        assert!(result.contains("Transactions : 2"));
    }

    #[tokio::test]
    async fn test_format_eth_get_block_by_number_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_proxy_error("invalid argument"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result =
            format_eth_get_block_by_number(&client, &[("tag", "latest"), ("boolean", "true")])
                .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("invalid argument"));
    }

    // --- eth_getBlockTransactionCountByNumber ---

    #[tokio::test]
    async fn test_format_eth_get_block_transaction_count_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded(
                    "action".into(),
                    "eth_getBlockTransactionCountByNumber".into(),
                ),
            ]))
            .with_status(200)
            .with_body(mock_proxy_success(r#""0x96""#))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result =
            format_eth_get_block_transaction_count_by_number(&client, &[("tag", "latest")])
                .await
                .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Transaction Count: 150"));
    }

    #[tokio::test]
    async fn test_format_eth_get_block_transaction_count_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_proxy_error("invalid argument"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result =
            format_eth_get_block_transaction_count_by_number(&client, &[("tag", "latest")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("invalid argument"));
    }

    // --- eth_getUncleByBlockNumberAndIndex ---

    #[tokio::test]
    async fn test_format_eth_get_uncle_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded(
                    "action".into(),
                    "eth_getUncleByBlockNumberAndIndex".into(),
                ),
            ]))
            .with_status(200)
            .with_body(mock_proxy_success(
                r#"{"number":"0xa","hash":"0xuncle","timestamp":"0x5f5e100","miner":"0xabc","gasUsed":"0x0","gasLimit":"0x1c9c380"}"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_get_uncle_by_block_number_and_index(
            &client,
            &[("tag", "0xa"), ("index", "0x0")],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Hash      : 0xuncle"));
        assert!(result.contains("Miner     : 0xabc"));
    }

    #[tokio::test]
    async fn test_format_eth_get_uncle_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_proxy_error("invalid argument"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_get_uncle_by_block_number_and_index(
            &client,
            &[("tag", "0xa"), ("index", "0x0")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("invalid argument"));
    }

    // --- eth_getTransactionByHash ---

    #[tokio::test]
    async fn test_format_eth_get_transaction_by_hash_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded(
                    "action".into(),
                    "eth_getTransactionByHash".into(),
                ),
            ]))
            .with_status(200)
            .with_body(mock_proxy_success(
                r#"{"hash":"0xtx","blockNumber":"0xa","from":"0xsender","to":"0xrecv","value":"0xde0b6b3a7640000","gas":"0x5208","gasPrice":"0x3b9aca00","nonce":"0x1"}"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_get_transaction_by_hash(&client, &[("txhash", "0xtx")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Hash      : 0xtx"));
        assert!(result.contains("From      : 0xsender"));
        assert!(result.contains("Gas       : 21000"));
    }

    #[tokio::test]
    async fn test_format_eth_get_transaction_by_hash_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_proxy_error("invalid argument"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_get_transaction_by_hash(&client, &[("txhash", "0x")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("invalid argument"));
    }

    // --- eth_getTransactionByBlockNumberAndIndex ---

    #[tokio::test]
    async fn test_format_eth_get_transaction_by_block_number_and_index_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded(
                    "action".into(),
                    "eth_getTransactionByBlockNumberAndIndex".into(),
                ),
            ]))
            .with_status(200)
            .with_body(mock_proxy_success(
                r#"{"hash":"0xtx2","blockNumber":"0xa","from":"0xsender","to":"0xrecv","value":"0x0","gas":"0x5208","gasPrice":"0x0","nonce":"0x0"}"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_get_transaction_by_block_number_and_index(
            &client,
            &[("tag", "0xa"), ("index", "0x0")],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Hash      : 0xtx2"));
        assert!(result.contains("Block     : 10"));
    }

    #[tokio::test]
    async fn test_format_eth_get_transaction_by_block_number_and_index_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_proxy_error("invalid argument"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_get_transaction_by_block_number_and_index(
            &client,
            &[("tag", "0xa"), ("index", "0x0")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("invalid argument"));
    }

    // --- eth_getTransactionCount ---

    #[tokio::test]
    async fn test_format_eth_get_transaction_count_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded("action".into(), "eth_getTransactionCount".into()),
            ]))
            .with_status(200)
            .with_body(mock_proxy_success(r#""0x2a""#))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result =
            format_eth_get_transaction_count(&client, &[("address", "0xabc"), ("tag", "latest")])
                .await
                .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Transaction Count: 42"));
    }

    #[tokio::test]
    async fn test_format_eth_get_transaction_count_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_proxy_error("invalid argument"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result =
            format_eth_get_transaction_count(&client, &[("address", "0xabc"), ("tag", "latest")])
                .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("invalid argument"));
    }

    // --- eth_getTransactionReceipt ---

    #[tokio::test]
    async fn test_format_eth_get_transaction_receipt_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded(
                    "action".into(),
                    "eth_getTransactionReceipt".into(),
                ),
            ]))
            .with_status(200)
            .with_body(mock_proxy_success(
                r#"{"transactionHash":"0xtx","blockNumber":"0xa","from":"0xsender","to":"0xrecv","status":"0x1","gasUsed":"0x5208","logs":[{},{}]}"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_get_transaction_receipt(&client, &[("txhash", "0xtx")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Tx Hash   : 0xtx"));
        assert!(result.contains("Status    : Success"));
        assert!(result.contains("Gas Used  : 21000"));
        assert!(result.contains("Log Count : 2"));
    }

    #[tokio::test]
    async fn test_format_eth_get_transaction_receipt_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_proxy_error("invalid argument"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_get_transaction_receipt(&client, &[("txhash", "0x")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("invalid argument"));
    }

    // --- eth_call ---

    #[tokio::test]
    async fn test_format_eth_call_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded("action".into(), "eth_call".into()),
            ]))
            .with_status(200)
            .with_body(mock_proxy_success(
                r#""0x00000000000000000000000000000000000000000000000000000000000f4240""#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_call(
            &client,
            &[("to", "0xabc"), ("data", "0x70a08231"), ("tag", "latest")],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains(
            "Result: 0x00000000000000000000000000000000000000000000000000000000000f4240"
        ));
    }

    #[tokio::test]
    async fn test_format_eth_call_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_proxy_error("execution reverted"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_call(
            &client,
            &[("to", "0xabc"), ("data", "0x"), ("tag", "latest")],
        )
        .await;
        mock.assert_async().await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("execution reverted")
        );
    }

    // --- eth_getCode ---

    #[tokio::test]
    async fn test_format_eth_get_code_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded("action".into(), "eth_getCode".into()),
            ]))
            .with_status(200)
            .with_body(mock_proxy_success(r#""0x6060604052""#))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_get_code(&client, &[("address", "0xabc"), ("tag", "latest")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Code: 0x6060604052"));
    }

    #[tokio::test]
    async fn test_format_eth_get_code_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_proxy_error("invalid argument"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_get_code(&client, &[("address", "0xabc"), ("tag", "latest")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("invalid argument"));
    }

    // --- eth_getStorageAt ---

    #[tokio::test]
    async fn test_format_eth_get_storage_at_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded("action".into(), "eth_getStorageAt".into()),
            ]))
            .with_status(200)
            .with_body(mock_proxy_success(
                r#""0x0000000000000000000000000000000000000000000000000000000000000001""#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_get_storage_at(
            &client,
            &[("address", "0xabc"), ("position", "0x0"), ("tag", "latest")],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains(
            "Storage: 0x0000000000000000000000000000000000000000000000000000000000000001"
        ));
    }

    #[tokio::test]
    async fn test_format_eth_get_storage_at_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_proxy_error("invalid argument"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_get_storage_at(
            &client,
            &[("address", "0xabc"), ("position", "0x0"), ("tag", "latest")],
        )
        .await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("invalid argument"));
    }

    // --- eth_gasPrice ---

    #[tokio::test]
    async fn test_format_eth_gas_price_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded("action".into(), "eth_gasPrice".into()),
            ]))
            .with_status(200)
            .with_body(mock_proxy_success(r#""0x3b9aca00""#))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_gas_price(&client).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Gas Price: 1000000000 wei (1.00 Gwei)"));
    }

    #[tokio::test]
    async fn test_format_eth_gas_price_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_proxy_error("server error"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_gas_price(&client).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("server error"));
    }

    // --- eth_estimateGas ---

    #[tokio::test]
    async fn test_format_eth_estimate_gas_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded("action".into(), "eth_estimateGas".into()),
            ]))
            .with_status(200)
            .with_body(mock_proxy_success(r#""0x5208""#))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_estimate_gas(
            &client,
            &[("to", "0xabc"), ("data", "0x"), ("tag", "latest")],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Estimated Gas: 21000"));
    }

    #[tokio::test]
    async fn test_format_eth_estimate_gas_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_proxy_error("gas required exceeds allowance"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_eth_estimate_gas(
            &client,
            &[("to", "0xabc"), ("data", "0x"), ("tag", "latest")],
        )
        .await;
        mock.assert_async().await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("gas required exceeds allowance")
        );
    }
}
