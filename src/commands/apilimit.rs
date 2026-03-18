use crate::client::EtherscanClient;
use crate::error::XplorerError;
use crate::handlers;

pub async fn apilimit(client: &EtherscanClient, raw: bool) -> Result<(), XplorerError> {
    if raw {
        super::print_raw_response(client, "getapilimit", "getapilimit", &[]).await
    } else {
        let formatted = handlers::apilimit::format_api_limit(client).await?;
        print!("{formatted}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_apilimit_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"1","message":"OK","result":{"creditsUsed":0}}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), None, server.url());

        let result = apilimit(&client, true).await;
        assert!(result.is_ok());
    }
}
