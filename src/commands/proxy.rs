use crate::client::EtherscanClient;
use crate::error::XplorerError;
use crate::handlers;

async fn print_raw_proxy(
    client: &EtherscanClient,
    action: &str,
    params: &[(&str, &str)],
) -> Result<(), XplorerError> {
    let response = client.call_api("proxy", action, params).await?;
    handlers::proxy::print_raw_proxy_result(&response)
}

pub async fn eth_block_number(client: &EtherscanClient, raw: bool) -> Result<(), XplorerError> {
    if raw {
        print_raw_proxy(client, "eth_blockNumber", &[]).await
    } else {
        let formatted = handlers::proxy::format_eth_block_number(client).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn eth_get_block_by_number(
    client: &EtherscanClient,
    tag: &str,
    boolean: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = vec![("tag", tag), ("boolean", boolean)];
    if raw {
        print_raw_proxy(client, "eth_getBlockByNumber", &params).await
    } else {
        let formatted = handlers::proxy::format_eth_get_block_by_number(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn eth_get_block_transaction_count_by_number(
    client: &EtherscanClient,
    tag: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = vec![("tag", tag)];
    if raw {
        print_raw_proxy(client, "eth_getBlockTransactionCountByNumber", &params).await
    } else {
        let formatted =
            handlers::proxy::format_eth_get_block_transaction_count_by_number(client, &params)
                .await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn eth_get_uncle_by_block_number_and_index(
    client: &EtherscanClient,
    tag: &str,
    index: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = vec![("tag", tag), ("index", index)];
    if raw {
        print_raw_proxy(client, "eth_getUncleByBlockNumberAndIndex", &params).await
    } else {
        let formatted =
            handlers::proxy::format_eth_get_uncle_by_block_number_and_index(client, &params)
                .await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn eth_get_transaction_by_hash(
    client: &EtherscanClient,
    txhash: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = vec![("txhash", txhash)];
    if raw {
        print_raw_proxy(client, "eth_getTransactionByHash", &params).await
    } else {
        let formatted =
            handlers::proxy::format_eth_get_transaction_by_hash(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn eth_get_transaction_by_block_number_and_index(
    client: &EtherscanClient,
    tag: &str,
    index: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = vec![("tag", tag), ("index", index)];
    if raw {
        print_raw_proxy(client, "eth_getTransactionByBlockNumberAndIndex", &params).await
    } else {
        let formatted =
            handlers::proxy::format_eth_get_transaction_by_block_number_and_index(client, &params)
                .await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn eth_get_transaction_count(
    client: &EtherscanClient,
    address: &str,
    tag: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = vec![("address", address), ("tag", tag)];
    if raw {
        print_raw_proxy(client, "eth_getTransactionCount", &params).await
    } else {
        let formatted = handlers::proxy::format_eth_get_transaction_count(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn eth_get_transaction_receipt(
    client: &EtherscanClient,
    txhash: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = vec![("txhash", txhash)];
    if raw {
        print_raw_proxy(client, "eth_getTransactionReceipt", &params).await
    } else {
        let formatted =
            handlers::proxy::format_eth_get_transaction_receipt(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn eth_call(
    client: &EtherscanClient,
    to: &str,
    data: &str,
    tag: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = vec![("to", to), ("data", data), ("tag", tag)];
    if raw {
        print_raw_proxy(client, "eth_call", &params).await
    } else {
        let formatted = handlers::proxy::format_eth_call(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn eth_get_code(
    client: &EtherscanClient,
    address: &str,
    tag: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = vec![("address", address), ("tag", tag)];
    if raw {
        print_raw_proxy(client, "eth_getCode", &params).await
    } else {
        let formatted = handlers::proxy::format_eth_get_code(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn eth_get_storage_at(
    client: &EtherscanClient,
    address: &str,
    position: &str,
    tag: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = vec![("address", address), ("position", position), ("tag", tag)];
    if raw {
        print_raw_proxy(client, "eth_getStorageAt", &params).await
    } else {
        let formatted = handlers::proxy::format_eth_get_storage_at(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn eth_gas_price(client: &EtherscanClient, raw: bool) -> Result<(), XplorerError> {
    if raw {
        print_raw_proxy(client, "eth_gasPrice", &[]).await
    } else {
        let formatted = handlers::proxy::format_eth_gas_price(client).await?;
        print!("{formatted}");
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn eth_estimate_gas(
    client: &EtherscanClient,
    to: &str,
    data: &str,
    value: &Option<String>,
    gas: &Option<String>,
    gasprice: &Option<String>,
    tag: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params = vec![("to", to), ("data", data), ("tag", tag)];
    if let Some(v) = value {
        params.push(("value", v));
    }
    if let Some(g) = gas {
        params.push(("gas", g));
    }
    if let Some(gp) = gasprice {
        params.push(("gasPrice", gp));
    }
    if raw {
        print_raw_proxy(client, "eth_estimateGas", &params).await
    } else {
        let formatted = handlers::proxy::format_eth_estimate_gas(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_eth_block_number_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded("action".into(), "eth_blockNumber".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"jsonrpc":"2.0","id":1,"result":"0x10d4f"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        assert!(eth_block_number(&client, true).await.is_ok());
    }

    #[tokio::test]
    async fn test_eth_get_transaction_receipt_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "proxy".into()),
                mockito::Matcher::UrlEncoded("action".into(), "eth_getTransactionReceipt".into()),
            ]))
            .with_status(200)
            .with_body(
                r#"{"jsonrpc":"2.0","id":1,"result":{"transactionHash":"0x1","status":"0x1"}}"#,
            )
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        assert!(
            eth_get_transaction_receipt(&client, "0x1", true)
                .await
                .is_ok()
        );
    }
}
