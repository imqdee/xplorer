use crate::client::EtherscanClient;
use crate::error::XplorerError;
use crate::handlers;

pub async fn chainlist(client: &EtherscanClient, raw: bool) -> Result<(), XplorerError> {
    let url = client.sibling_url("/chainlist");
    let response = client.fetch_url(&url).await?;

    if raw {
        let output = serde_json::to_string(&response["result"])
            .map_err(|e| XplorerError::Api(format!("Failed to serialize result: {e}")))?;
        println!("{output}");
    } else {
        let formatted = handlers::chainlist::format_chainlist(&response)?;
        print!("{formatted}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_chainlist_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/v2/chainlist")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"totalcount":1,"result":[{"chainname":"Ethereum","chainid":"1","blockexplorer":"https://etherscan.io","apiurl":"https://api.etherscan.io","status":1}]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_minimal_with_url(format!("{}/v2/api", server.url()));

        let result = chainlist(&client, true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_chainlist_formatted_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/v2/chainlist")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"totalcount":1,"result":[{"chainname":"Ethereum","chainid":"1","blockexplorer":"https://etherscan.io","apiurl":"https://api.etherscan.io","status":1}]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_minimal_with_url(format!("{}/v2/api", server.url()));

        let result = chainlist(&client, false).await;
        assert!(result.is_ok());
    }
}
