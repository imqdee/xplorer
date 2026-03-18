use crate::client::EtherscanClient;
use crate::error::XplorerError;
use crate::handlers;

pub async fn gas_oracle(client: &EtherscanClient, raw: bool) -> Result<(), XplorerError> {
    if raw {
        super::print_raw_response(client, "gastracker", "gasoracle", &[]).await
    } else {
        let formatted = handlers::gas::format_gas_oracle(client).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn gas_estimate(
    client: &EtherscanClient,
    gasprice: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    if raw {
        super::print_raw_response(
            client,
            "gastracker",
            "gasestimate",
            &[("gasprice", gasprice)],
        )
        .await
    } else {
        let formatted = handlers::gas::format_gas_estimate(client, gasprice).await?;
        print!("{formatted}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_gas_oracle_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "gastracker".into()),
                mockito::Matcher::UrlEncoded("action".into(), "gasoracle".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"1","message":"OK","result":{"LastBlock":"1"}}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = gas_oracle(&client, true).await;
        assert!(result.is_ok());
    }
}
