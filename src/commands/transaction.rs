use crate::client::EtherscanClient;
use crate::error::XplorerError;
use crate::handlers;

pub async fn get_status(
    client: &EtherscanClient,
    txhash: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    if raw {
        super::print_raw_response(client, "transaction", "getstatus", &[("txhash", txhash)]).await
    } else {
        let formatted = handlers::transaction::format_tx_status(client, txhash).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn get_tx_receipt_status(
    client: &EtherscanClient,
    txhash: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    if raw {
        super::print_raw_response(
            client,
            "transaction",
            "gettxreceiptstatus",
            &[("txhash", txhash)],
        )
        .await
    } else {
        let formatted =
            handlers::transaction::format_tx_receipt_status(client, txhash).await?;
        print!("{formatted}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_get_status_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"status":"1","message":"OK","result":{"isError":"0","errDescription":""}}"#,
            )
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = get_status(&client, "0xabc", true).await;
        assert!(result.is_ok());
    }
}
