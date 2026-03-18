use crate::client::EtherscanClient;
use crate::error::XplorerError;
use serde::Deserialize;

use super::fmt::check_api_status;

// --- Structs ---

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct TokenInfo {
    pub contract_address: String,
    pub token_name: String,
    pub symbol: String,
    pub divisor: String,
    pub token_type: String,
    pub total_supply: String,
    pub blue_checkmark: String,
    pub description: String,
    pub website: String,
    pub email: String,
    pub blog: String,
    pub reddit: String,
    pub slack: String,
    pub facebook: String,
    pub twitter: String,
    pub bitcointalk: String,
    pub github: String,
    pub telegram: String,
    pub wechat: String,
    pub linkedin: String,
    pub discord: String,
    pub whitepaper: String,
}

#[derive(Debug, Deserialize)]
pub struct TokenHolder {
    #[serde(rename = "TokenHolderAddress")]
    pub address: String,
    #[serde(rename = "TokenHolderQuantity")]
    pub quantity: String,
}

// --- Format Functions ---

pub async fn format_tokeninfo(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("token", "tokeninfo", params).await?;
    check_api_status(&response)?;

    let entries: Vec<TokenInfo> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse tokeninfo response: {e}")))?;

    let info = entries
        .first()
        .ok_or_else(|| XplorerError::Api("Empty tokeninfo response".to_string()))?;

    let mut output = String::new();
    output.push_str(&format!("Contract    : {}\n", info.contract_address));
    output.push_str(&format!("Name        : {}\n", info.token_name));
    output.push_str(&format!("Symbol      : {}\n", info.symbol));
    output.push_str(&format!("Type        : {}\n", info.token_type));
    output.push_str(&format!("Divisor     : {}\n", info.divisor));
    output.push_str(&format!("Supply      : {}\n", info.total_supply));

    let social_fields: &[(&str, &str)] = &[
        ("Description", &info.description),
        ("Website", &info.website),
        ("Email", &info.email),
        ("Blog", &info.blog),
        ("Reddit", &info.reddit),
        ("Slack", &info.slack),
        ("Facebook", &info.facebook),
        ("Twitter", &info.twitter),
        ("BitcoinTalk", &info.bitcointalk),
        ("GitHub", &info.github),
        ("Telegram", &info.telegram),
        ("WeChat", &info.wechat),
        ("LinkedIn", &info.linkedin),
        ("Discord", &info.discord),
        ("Whitepaper", &info.whitepaper),
    ];

    for (label, value) in social_fields {
        if !value.is_empty() {
            output.push_str(&format!("{:<12}: {}\n", label, value));
        }
    }

    Ok(output)
}

pub async fn format_tokenholdercount(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("token", "tokenholdercount", params).await?;
    check_api_status(&response)?;

    let result = &response["result"];
    let mut output = String::new();

    let contract = params
        .iter()
        .find(|(k, _)| *k == "contractaddress")
        .map(|(_, v)| *v)
        .unwrap_or("unknown");

    output.push_str(&format!("Contract : {contract}\n"));
    output.push_str(&format!("Holders  : {}\n", result.as_str().unwrap_or("0")));

    Ok(output)
}

fn format_holder_list(entries: &[TokenHolder]) -> String {
    let mut output = String::new();
    for (i, holder) in entries.iter().enumerate() {
        output.push_str(&format!("Address  : {}\n", holder.address));
        output.push_str(&format!("Balance  : {}\n", holder.quantity));
        if i < entries.len() - 1 {
            output.push('\n');
        }
    }
    output
}

pub async fn format_tokenholderlist(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("token", "tokenholderlist", params).await?;
    check_api_status(&response)?;

    let entries: Vec<TokenHolder> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse tokenholderlist response: {e}")))?;

    Ok(format_holder_list(&entries))
}

pub async fn format_topholders(
    client: &EtherscanClient,
    params: &[(&str, &str)],
) -> Result<String, XplorerError> {
    let response = client.call_api("token", "topholders", params).await?;
    check_api_status(&response)?;

    let entries: Vec<TokenHolder> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse topholders response: {e}")))?;

    Ok(format_holder_list(&entries))
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

    // --- tokeninfo ---

    #[tokio::test]
    async fn test_format_tokeninfo_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "token".into()),
                mockito::Matcher::UrlEncoded("action".into(), "tokeninfo".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"contractAddress":"0xdac17f958d2ee523a2206206994597c13d831ec7","tokenName":"Tether USD","symbol":"USDT","divisor":"6","tokenType":"ERC-20","totalSupply":"100000000000","blueCheckmark":"true","description":"Tether USD stablecoin","website":"https://tether.to","email":"","blog":"","reddit":"","slack":"","facebook":"","twitter":"https://twitter.com/Tether_to","bitcointalk":"","github":"","telegram":"","wechat":"","linkedin":"","discord":"","whitepaper":""}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokeninfo(&client, &[("contractaddress", "0xdac17f")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Contract    : 0xdac17f958d2ee523a2206206994597c13d831ec7"));
        assert!(result.contains("Name        : Tether USD"));
        assert!(result.contains("Symbol      : USDT"));
        assert!(result.contains("Type        : ERC-20"));
        assert!(result.contains("Divisor     : 6"));
        assert!(result.contains("Supply      : 100000000000"));
        assert!(result.contains("Description : Tether USD stablecoin"));
        assert!(result.contains("Website     : https://tether.to"));
        assert!(result.contains("Twitter     : https://twitter.com/Tether_to"));
        assert!(!result.contains("Email"));
    }

    #[tokio::test]
    async fn test_format_tokeninfo_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Invalid contract address"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokeninfo(&client, &[("contractaddress", "bad")]).await;
        mock.assert_async().await;
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid contract address")
        );
    }

    // --- tokenholdercount ---

    #[tokio::test]
    async fn test_format_tokenholdercount_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "token".into()),
                mockito::Matcher::UrlEncoded("action".into(), "tokenholdercount".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(r#""12345""#))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokenholdercount(&client, &[("contractaddress", "0xtoken")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Contract : 0xtoken"));
        assert!(result.contains("Holders  : 12345"));
    }

    #[tokio::test]
    async fn test_format_tokenholdercount_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokenholdercount(&client, &[("contractaddress", "0x1")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- tokenholderlist ---

    #[tokio::test]
    async fn test_format_tokenholderlist_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "token".into()),
                mockito::Matcher::UrlEncoded("action".into(), "tokenholderlist".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"TokenHolderAddress":"0xholder1","TokenHolderQuantity":"1000000"},{"TokenHolderAddress":"0xholder2","TokenHolderQuantity":"500000"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokenholderlist(&client, &[("contractaddress", "0xtoken")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Address  : 0xholder1"));
        assert!(result.contains("Balance  : 1000000"));
        assert!(result.contains("Address  : 0xholder2"));
        assert!(result.contains("Balance  : 500000"));
    }

    #[tokio::test]
    async fn test_format_tokenholderlist_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_tokenholderlist(&client, &[("contractaddress", "0x1")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }

    // --- topholders ---

    #[tokio::test]
    async fn test_format_topholders_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "token".into()),
                mockito::Matcher::UrlEncoded("action".into(), "topholders".into()),
            ]))
            .with_status(200)
            .with_body(mock_success(
                r#"[{"TokenHolderAddress":"0xtop1","TokenHolderQuantity":"99999999"}]"#,
            ))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_topholders(&client, &[("contractaddress", "0xtoken")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Address  : 0xtop1"));
        assert!(result.contains("Balance  : 99999999"));
    }

    #[tokio::test]
    async fn test_format_topholders_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_body(mock_error("Pro endpoint"))
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let result = format_topholders(&client, &[("contractaddress", "0x1")]).await;
        mock.assert_async().await;
        assert!(result.unwrap_err().to_string().contains("Pro endpoint"));
    }
}
