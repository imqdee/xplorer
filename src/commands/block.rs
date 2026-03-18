use crate::client::EtherscanClient;
use crate::error::XplorerError;
use crate::handlers;

pub async fn getblockreward(
    client: &EtherscanClient,
    blockno: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = [("blockno", blockno)];
    if raw {
        super::print_raw_response(client, "block", "getblockreward", &params).await
    } else {
        let formatted = handlers::block::format_getblockreward(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn getblockcountdown(
    client: &EtherscanClient,
    blockno: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = [("blockno", blockno)];
    if raw {
        super::print_raw_response(client, "block", "getblockcountdown", &params).await
    } else {
        let formatted = handlers::block::format_getblockcountdown(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn getblocknobytime(
    client: &EtherscanClient,
    timestamp: &str,
    closest: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = [("timestamp", timestamp), ("closest", closest)];
    if raw {
        super::print_raw_response(client, "block", "getblocknobytime", &params).await
    } else {
        let formatted = handlers::block::format_getblocknobytime(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_getblockreward_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "block".into()),
                mockito::Matcher::UrlEncoded("action".into(), "getblockreward".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":{"blockNumber":"123"}}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        assert!(getblockreward(&client, "123", true).await.is_ok());
    }
}
