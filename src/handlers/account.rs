use crate::client::EtherscanClient;
use crate::error::XplorerError;
use serde::Deserialize;

use super::fmt::{check_api_status, decimal_timestamp};

// --- Structs ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalTx {
    pub hash: String,
    pub block_number: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas_used: String,
    pub is_error: String,
    #[serde(rename = "functionName")]
    pub function_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTx {
    pub hash: String,
    pub block_number: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
    pub from: String,
    pub to: String,
    pub value: String,
    #[serde(rename = "type")]
    pub tx_type: String,
    pub gas_used: String,
    pub is_error: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Erc20Transfer {
    pub hash: String,
    pub block_number: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub contract_address: String,
    pub token_name: String,
    pub token_symbol: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Erc721Transfer {
    pub hash: String,
    pub block_number: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
    pub from: String,
    pub to: String,
    pub contract_address: String,
    pub token_name: String,
    pub token_symbol: String,
    #[serde(rename = "tokenID")]
    pub token_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Erc1155Transfer {
    pub hash: String,
    pub block_number: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
    pub from: String,
    pub to: String,
    pub contract_address: String,
    pub token_name: String,
    pub token_symbol: String,
    #[serde(rename = "tokenID")]
    pub token_id: String,
    pub token_value: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TokenHolding {
    #[serde(rename = "TokenAddress")]
    pub token_address: String,
    #[serde(rename = "TokenName")]
    pub token_name: String,
    #[serde(rename = "TokenSymbol")]
    pub token_symbol: String,
    #[serde(rename = "TokenQuantity")]
    pub token_quantity: String,
    #[serde(rename = "TokenDivisor")]
    pub token_divisor: String,
    #[serde(rename = "TokenPriceUSD")]
    pub token_price_usd: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct NftHolding {
    #[serde(rename = "TokenAddress")]
    pub token_address: String,
    #[serde(rename = "TokenName")]
    pub token_name: String,
    #[serde(rename = "TokenSymbol")]
    pub token_symbol: String,
    #[serde(rename = "TokenQuantity")]
    pub token_quantity: String,
}

#[derive(Debug, Deserialize)]
pub struct NftInventoryItem {
    #[serde(rename = "TokenAddress")]
    pub token_address: String,
    #[serde(rename = "TokenId")]
    pub token_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinedBlock {
    pub block_number: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
    pub block_reward: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeaconWithdrawal {
    pub block_number: String,
    #[serde(alias = "timeStamp")]
    pub timestamp: String,
    pub withdrawal_index: String,
    pub validator_index: String,
    pub address: String,
    pub amount: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingInfo {
    pub block: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
    pub funding_address: String,
    pub funding_txn: String,
    pub value: String,
}

// --- Format Functions ---

pub async fn format_balance(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("account", "balance", params).await?;
    check_api_status(&response)?;

    let result = &response["result"];
    let mut output = String::new();

    let address = params
        .iter()
        .find(|(k, _)| *k == "address")
        .map(|(_, v)| *v)
        .unwrap_or("unknown");

    output.push_str(&format!("Address  : {address}\n"));
    output.push_str(&format!(
        "Balance  : {} wei\n",
        result.as_str().unwrap_or("0")
    ));

    Ok(output)
}

pub async fn format_balancehistory(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("account", "balancehistory", params).await?;
    check_api_status(&response)?;

    let result = &response["result"];
    let mut output = String::new();

    let address = params
        .iter()
        .find(|(k, _)| *k == "address")
        .map(|(_, v)| *v)
        .unwrap_or("unknown");
    let blockno = params
        .iter()
        .find(|(k, _)| *k == "blockno")
        .map(|(_, v)| *v)
        .unwrap_or("unknown");

    output.push_str(&format!("Address  : {address}\n"));
    output.push_str(&format!("Block    : {blockno}\n"));
    output.push_str(&format!(
        "Balance  : {} wei\n",
        result.as_str().unwrap_or("0")
    ));

    Ok(output)
}

pub async fn format_txlist(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("account", "txlist", params).await?;
    check_api_status(&response)?;

    let entries: Vec<NormalTx> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse txlist response: {e}")))?;

    let mut output = String::new();
    for (i, tx) in entries.iter().enumerate() {
        output.push_str(&format!("Tx Hash  : {}\n", tx.hash));
        output.push_str(&format!("Block    : {}\n", tx.block_number));
        output.push_str(&format!(
            "Time     : {}\n",
            decimal_timestamp(&tx.timestamp)
        ));
        output.push_str(&format!("From     : {}\n", tx.from));
        output.push_str(&format!("To       : {}\n", tx.to));
        output.push_str(&format!("Value    : {}\n", tx.value));
        output.push_str(&format!("Gas Used : {}\n", tx.gas_used));
        let status = if tx.is_error == "0" {
            "Success"
        } else {
            "Failed"
        };
        output.push_str(&format!("Status   : {status}\n"));
        output.push_str(&format!("Method   : {}\n", tx.function_name));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

pub async fn format_txlistinternal(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("account", "txlistinternal", params).await?;
    check_api_status(&response)?;

    let entries: Vec<InternalTx> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse txlistinternal response: {e}")))?;

    let mut output = String::new();
    for (i, tx) in entries.iter().enumerate() {
        output.push_str(&format!("Tx Hash  : {}\n", tx.hash));
        output.push_str(&format!("Block    : {}\n", tx.block_number));
        output.push_str(&format!(
            "Time     : {}\n",
            decimal_timestamp(&tx.timestamp)
        ));
        output.push_str(&format!("From     : {}\n", tx.from));
        output.push_str(&format!("To       : {}\n", tx.to));
        output.push_str(&format!("Value    : {}\n", tx.value));
        output.push_str(&format!("Type     : {}\n", tx.tx_type));
        output.push_str(&format!("Gas Used : {}\n", tx.gas_used));
        let status = if tx.is_error == "0" {
            "Success"
        } else {
            "Failed"
        };
        output.push_str(&format!("Status   : {status}\n"));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

pub async fn format_tokentx(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("account", "tokentx", params).await?;
    check_api_status(&response)?;

    let entries: Vec<Erc20Transfer> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse tokentx response: {e}")))?;

    let mut output = String::new();
    for (i, tx) in entries.iter().enumerate() {
        output.push_str(&format!("Tx Hash  : {}\n", tx.hash));
        output.push_str(&format!("Block    : {}\n", tx.block_number));
        output.push_str(&format!(
            "Time     : {}\n",
            decimal_timestamp(&tx.timestamp)
        ));
        output.push_str(&format!("From     : {}\n", tx.from));
        output.push_str(&format!("To       : {}\n", tx.to));
        output.push_str(&format!(
            "Token    : {} ({})\n",
            tx.token_symbol, tx.contract_address
        ));
        output.push_str(&format!("Value    : {}\n", tx.value));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

pub async fn format_tokennfttx(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("account", "tokennfttx", params).await?;
    check_api_status(&response)?;

    let entries: Vec<Erc721Transfer> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse tokennfttx response: {e}")))?;

    let mut output = String::new();
    for (i, tx) in entries.iter().enumerate() {
        output.push_str(&format!("Tx Hash  : {}\n", tx.hash));
        output.push_str(&format!("Block    : {}\n", tx.block_number));
        output.push_str(&format!(
            "Time     : {}\n",
            decimal_timestamp(&tx.timestamp)
        ));
        output.push_str(&format!("From     : {}\n", tx.from));
        output.push_str(&format!("To       : {}\n", tx.to));
        output.push_str(&format!(
            "Token    : {} ({})\n",
            tx.token_symbol, tx.contract_address
        ));
        output.push_str(&format!("Token ID : {}\n", tx.token_id));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

pub async fn format_token1155tx(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("account", "token1155tx", params).await?;
    check_api_status(&response)?;

    let entries: Vec<Erc1155Transfer> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse token1155tx response: {e}")))?;

    let mut output = String::new();
    for (i, tx) in entries.iter().enumerate() {
        output.push_str(&format!("Tx Hash  : {}\n", tx.hash));
        output.push_str(&format!("Block    : {}\n", tx.block_number));
        output.push_str(&format!(
            "Time     : {}\n",
            decimal_timestamp(&tx.timestamp)
        ));
        output.push_str(&format!("From     : {}\n", tx.from));
        output.push_str(&format!("To       : {}\n", tx.to));
        output.push_str(&format!(
            "Token    : {} ({})\n",
            tx.token_symbol, tx.contract_address
        ));
        output.push_str(&format!("Token ID : {}\n", tx.token_id));
        output.push_str(&format!("Value    : {}\n", tx.token_value));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

pub async fn format_addresstokenbalance(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("account", "addresstokenbalance", params)
        .await?;
    check_api_status(&response)?;

    let entries: Vec<TokenHolding> =
        serde_json::from_value(response["result"].clone()).map_err(|e| {
            XplorerError::Api(format!("Failed to parse addresstokenbalance response: {e}"))
        })?;

    let mut output = String::new();
    for (i, h) in entries.iter().enumerate() {
        output.push_str(&format!(
            "Token    : {} ({})\n",
            h.token_symbol, h.token_address
        ));
        output.push_str(&format!("Balance  : {}\n", h.token_quantity));
        if !h.token_price_usd.is_empty() {
            output.push_str(&format!("Price    : ${}\n", h.token_price_usd));
        }
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

pub async fn format_addresstokennftbalance(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("account", "addresstokennftbalance", params)
        .await?;
    check_api_status(&response)?;

    let entries: Vec<NftHolding> =
        serde_json::from_value(response["result"].clone()).map_err(|e| {
            XplorerError::Api(format!(
                "Failed to parse addresstokennftbalance response: {e}"
            ))
        })?;

    let mut output = String::new();
    for (i, h) in entries.iter().enumerate() {
        output.push_str(&format!(
            "Token    : {} ({})\n",
            h.token_symbol, h.token_address
        ));
        output.push_str(&format!("Balance  : {}\n", h.token_quantity));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

pub async fn format_addresstokennftinventory(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("account", "addresstokennftinventory", params)
        .await?;
    check_api_status(&response)?;

    let entries: Vec<NftInventoryItem> = serde_json::from_value(response["result"].clone())
        .map_err(|e| {
            XplorerError::Api(format!(
                "Failed to parse addresstokennftinventory response: {e}"
            ))
        })?;

    let mut output = String::new();
    for (i, item) in entries.iter().enumerate() {
        output.push_str(&format!("Token    : {}\n", item.token_address));
        output.push_str(&format!("Token ID : {}\n", item.token_id));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

pub async fn format_getminedblocks(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("account", "getminedblocks", params).await?;
    check_api_status(&response)?;

    let entries: Vec<MinedBlock> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse getminedblocks response: {e}")))?;

    let mut output = String::new();
    for (i, block) in entries.iter().enumerate() {
        output.push_str(&format!("Block    : {}\n", block.block_number));
        output.push_str(&format!(
            "Time     : {}\n",
            decimal_timestamp(&block.timestamp)
        ));
        output.push_str(&format!("Reward   : {}\n", block.block_reward));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

pub async fn format_bridge_txs(
    client: &EtherscanClient,
    action: &str,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("account", action, params).await?;
    check_api_status(&response)?;

    let entries = response["result"]
        .as_array()
        .ok_or_else(|| XplorerError::Api("Expected array result".to_string()))?;

    let mut output = String::new();
    for (i, tx) in entries.iter().enumerate() {
        if let Some(hash) = tx.get("hash").and_then(|v| v.as_str()) {
            output.push_str(&format!("Tx Hash  : {hash}\n"));
        }
        if let Some(block) = tx.get("blockNumber").and_then(|v| v.as_str()) {
            output.push_str(&format!("Block    : {block}\n"));
        }
        if let Some(ts) = tx.get("timeStamp").and_then(|v| v.as_str()) {
            output.push_str(&format!("Time     : {}\n", decimal_timestamp(ts)));
        }
        if let Some(from) = tx.get("from").and_then(|v| v.as_str()) {
            output.push_str(&format!("From     : {from}\n"));
        }
        if let Some(to) = tx.get("to").and_then(|v| v.as_str()) {
            output.push_str(&format!("To       : {to}\n"));
        }
        if let Some(value) = tx.get("value").and_then(|v| v.as_str()) {
            output.push_str(&format!("Value    : {value}\n"));
        }
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

pub async fn format_txsbeaconwithdrawal(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client
        .call_api("account", "txsBeaconWithdrawal", params)
        .await?;
    check_api_status(&response)?;

    let entries: Vec<BeaconWithdrawal> = serde_json::from_value(response["result"].clone())
        .map_err(|e| {
            XplorerError::Api(format!("Failed to parse txsBeaconWithdrawal response: {e}"))
        })?;

    let mut output = String::new();
    for (i, w) in entries.iter().enumerate() {
        output.push_str(&format!("Block     : {}\n", w.block_number));
        output.push_str(&format!(
            "Time      : {}\n",
            decimal_timestamp(&w.timestamp)
        ));
        output.push_str(&format!("Validator : {}\n", w.validator_index));
        output.push_str(&format!("Index     : {}\n", w.withdrawal_index));
        output.push_str(&format!("Address   : {}\n", w.address));
        output.push_str(&format!("Amount    : {}\n", w.amount));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

pub async fn format_fundedby(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("account", "fundedby", params).await?;
    check_api_status(&response)?;

    let entries: Vec<FundingInfo> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse fundedby response: {e}")))?;

    let mut output = String::new();
    for (i, info) in entries.iter().enumerate() {
        output.push_str(&format!("Funded By : {}\n", info.funding_address));
        output.push_str(&format!("Tx Hash   : {}\n", info.funding_txn));
        output.push_str(&format!("Block     : {}\n", info.block));
        output.push_str(&format!(
            "Time      : {}\n",
            decimal_timestamp(&info.timestamp)
        ));
        output.push_str(&format!("Value     : {}\n", info.value));
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

    // --- balance ---

    #[tokio::test]
    async fn test_format_balance_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "balance".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(r#""40807178574558435714""#))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let params = [("address", "0xde0b2"), ("tag", "latest")];
        let result = format_balance(&client, &params).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Address  : 0xde0b2"));
        assert!(result.contains("Balance  : 40807178574558435714 wei"));
    }

    #[tokio::test]
    async fn test_format_balance_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Invalid address"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_balance(&client, &[("address", "bad")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Invalid address"));
    }

    // --- balancehistory ---

    #[tokio::test]
    async fn test_format_balancehistory_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "balancehistory".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(r#""12345678""#))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let params = [("address", "0xabc"), ("blockno", "8000000")];
        let result = format_balancehistory(&client, &params).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Address  : 0xabc"));
        assert!(result.contains("Block    : 8000000"));
        assert!(result.contains("Balance  : 12345678 wei"));
    }

    #[tokio::test]
    async fn test_format_balancehistory_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_balancehistory(&client, &[("address", "0x1"), ("blockno", "1")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- txlist ---

    #[tokio::test]
    async fn test_format_txlist_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "txlist".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"hash":"0xabc","blockNumber":"123","timeStamp":"1598242563","from":"0xfrom","to":"0xto","value":"1000","gasUsed":"21000","isError":"0","functionName":"transfer(address,uint256)"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_txlist(&client, &[("address", "0x1")]).await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("Tx Hash  : 0xabc"));
        assert!(result.contains("Block    : 123"));
        assert!(result.contains("Time     : 2020-08-24 04:16:03 UTC"));
        assert!(result.contains("Status   : Success"));
        assert!(result.contains("Method   : transfer(address,uint256)"));
    }

    #[tokio::test]
    async fn test_format_txlist_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("No transactions found"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_txlist(&client, &[("address", "0x1")]).await;
        mock.assert_async().await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("No transactions found")
        );
    }

    // --- txlistinternal ---

    #[tokio::test]
    async fn test_format_txlistinternal_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "txlistinternal".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"hash":"0xdef","blockNumber":"456","timeStamp":"1598242563","from":"0xf","to":"0xt","value":"500","type":"call","gasUsed":"3000","isError":"1"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_txlistinternal(&client, &[("address", "0x1")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Tx Hash  : 0xdef"));
        assert!(result.contains("Type     : call"));
        assert!(result.contains("Status   : Failed"));
    }

    #[tokio::test]
    async fn test_format_txlistinternal_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("No records"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_txlistinternal(&client, &[("address", "0x1")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("No records"));
    }

    // --- tokentx ---

    #[tokio::test]
    async fn test_format_tokentx_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "tokentx".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"hash":"0x1","blockNumber":"100","timeStamp":"1598242563","from":"0xf","to":"0xt","value":"1000000","contractAddress":"0xa0b8","tokenName":"USD Coin","tokenSymbol":"USDC"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokentx(&client, &[("address", "0x1")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Token    : USDC (0xa0b8)"));
        assert!(result.contains("Value    : 1000000"));
    }

    #[tokio::test]
    async fn test_format_tokentx_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("No token transfers"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokentx(&client, &[("address", "0x1")]).await;
        mock.assert_async().await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("No token transfers")
        );
    }

    // --- tokennfttx ---

    #[tokio::test]
    async fn test_format_tokennfttx_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "tokennfttx".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"hash":"0x1","blockNumber":"100","timeStamp":"1598242563","from":"0xf","to":"0xt","contractAddress":"0xnft","tokenName":"Bored Ape","tokenSymbol":"BAYC","tokenID":"42"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokennfttx(&client, &[("address", "0x1")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Token    : BAYC (0xnft)"));
        assert!(result.contains("Token ID : 42"));
    }

    #[tokio::test]
    async fn test_format_tokennfttx_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("No NFT transfers"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokennfttx(&client, &[("address", "0x1")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("No NFT transfers"));
    }

    // --- token1155tx ---

    #[tokio::test]
    async fn test_format_token1155tx_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "token1155tx".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"hash":"0x1","blockNumber":"100","timeStamp":"1598242563","from":"0xf","to":"0xt","contractAddress":"0x1155","tokenName":"Multi","tokenSymbol":"MT","tokenID":"7","tokenValue":"10"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_token1155tx(&client, &[("address", "0x1")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Token ID : 7"));
        assert!(result.contains("Value    : 10"));
    }

    #[tokio::test]
    async fn test_format_token1155tx_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("No 1155 transfers"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_token1155tx(&client, &[("address", "0x1")]).await;
        mock.assert_async().await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("No 1155 transfers")
        );
    }

    // --- addresstokenbalance ---

    #[tokio::test]
    async fn test_format_addresstokenbalance_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "addresstokenbalance".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"TokenAddress":"0xa0b8","TokenName":"USD Coin","TokenSymbol":"USDC","TokenQuantity":"1000000","TokenDivisor":"6","TokenPriceUSD":"1.00"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_addresstokenbalance(&client, &[("address", "0x1")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Token    : USDC (0xa0b8)"));
        assert!(result.contains("Balance  : 1000000"));
        assert!(result.contains("Price    : $1.00"));
    }

    #[tokio::test]
    async fn test_format_addresstokenbalance_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_addresstokenbalance(&client, &[("address", "0x1")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- addresstokennftbalance ---

    #[tokio::test]
    async fn test_format_addresstokennftbalance_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "addresstokennftbalance".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"TokenAddress":"0xnft","TokenName":"Bored Ape","TokenSymbol":"BAYC","TokenQuantity":"3"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_addresstokennftbalance(&client, &[("address", "0x1")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Token    : BAYC (0xnft)"));
        assert!(result.contains("Balance  : 3"));
    }

    #[tokio::test]
    async fn test_format_addresstokennftbalance_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_addresstokennftbalance(&client, &[("address", "0x1")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- addresstokennftinventory ---

    #[tokio::test]
    async fn test_format_addresstokennftinventory_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "addresstokennftinventory".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"TokenAddress":"0xnft","TokenId":"42"},{"TokenAddress":"0xnft","TokenId":"99"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_addresstokennftinventory(
            &client,
            &[("address", "0x1"), ("contractaddress", "0xnft")],
        )
        .await
        .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Token ID : 42"));
        assert!(result.contains("Token ID : 99"));
    }

    #[tokio::test]
    async fn test_format_addresstokennftinventory_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_addresstokennftinventory(&client, &[("address", "0x1")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- getminedblocks ---

    #[tokio::test]
    async fn test_format_getminedblocks_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "getminedblocks".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"blockNumber":"12345678","timeStamp":"1598242563","blockReward":"2000000000000000000"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_getminedblocks(&client, &[("address", "0x1")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Block    : 12345678"));
        assert!(result.contains("Time     : 2020-08-24 04:16:03 UTC"));
        assert!(result.contains("Reward   : 2000000000000000000"));
    }

    #[tokio::test]
    async fn test_format_getminedblocks_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("No blocks found"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_getminedblocks(&client, &[("address", "0x1")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("No blocks found"));
    }

    // --- bridge_txs ---

    #[tokio::test]
    async fn test_format_bridge_txs_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "getdeposittxs".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"hash":"0xbridge","blockNumber":"999","timeStamp":"1598242563","from":"0xf","to":"0xt","value":"5000"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_bridge_txs(&client, "getdeposittxs", &[("address", "0x1")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Tx Hash  : 0xbridge"));
        assert!(result.contains("Block    : 999"));
        assert!(result.contains("Value    : 5000"));
    }

    #[tokio::test]
    async fn test_format_bridge_txs_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("No deposits"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_bridge_txs(&client, "getdeposittxs", &[("address", "0x1")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("No deposits"));
    }

    // --- txsbeaconwithdrawal ---

    #[tokio::test]
    async fn test_format_txsbeaconwithdrawal_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "txsBeaconWithdrawal".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"blockNumber":"17000000","timeStamp":"1598242563","withdrawalIndex":"67890","validatorIndex":"12345","address":"0xval","amount":"32000000000"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_txsbeaconwithdrawal(&client, &[("address", "0x1")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Block     : 17000000"));
        assert!(result.contains("Validator : 12345"));
        assert!(result.contains("Index     : 67890"));
        assert!(result.contains("Address   : 0xval"));
        assert!(result.contains("Amount    : 32000000000"));
    }

    #[tokio::test]
    async fn test_format_txsbeaconwithdrawal_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("No withdrawals"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_txsbeaconwithdrawal(&client, &[("address", "0x1")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("No withdrawals"));
    }

    // --- fundedby ---

    #[tokio::test]
    async fn test_format_fundedby_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "fundedby".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"block":"12345678","timeStamp":"1598242563","fundingAddress":"0xfunder","fundingTxn":"0xtxhash","value":"1000000000000000000"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_fundedby(&client, &[("address", "0x1")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Funded By : 0xfunder"));
        assert!(result.contains("Tx Hash   : 0xtxhash"));
        assert!(result.contains("Block     : 12345678"));
        assert!(result.contains("Time      : 2020-08-24 04:16:03 UTC"));
        assert!(result.contains("Value     : 1000000000000000000"));
    }

    #[tokio::test]
    async fn test_format_fundedby_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("No funding info"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_fundedby(&client, &[("address", "0x1")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("No funding info"));
    }
}
