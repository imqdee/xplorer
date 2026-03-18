use crate::client::EtherscanClient;
use crate::error::XplorerError;
use crate::handlers;

pub struct PaginationParams<'a> {
    pub startblock: Option<&'a str>,
    pub endblock: Option<&'a str>,
    pub page: Option<&'a str>,
    pub offset: Option<&'a str>,
    pub sort: Option<&'a str>,
}

impl<'a> PaginationParams<'a> {
    pub fn to_query_pairs(&self) -> Vec<(&'a str, &'a str)> {
        let mut pairs = Vec::new();
        if let Some(v) = self.startblock {
            pairs.push(("startblock", v));
        }
        if let Some(v) = self.endblock {
            pairs.push(("endblock", v));
        }
        if let Some(v) = self.page {
            pairs.push(("page", v));
        }
        if let Some(v) = self.offset {
            pairs.push(("offset", v));
        }
        if let Some(v) = self.sort {
            pairs.push(("sort", v));
        }
        pairs
    }
}

pub async fn balance(
    client: &EtherscanClient,
    address: &str,
    tag: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = [("address", address), ("tag", tag)];
    if raw {
        super::print_raw_response(client, "account", "balance", &params).await
    } else {
        let formatted = handlers::account::format_balance(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn balancehistory(
    client: &EtherscanClient,
    address: &str,
    blockno: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = [("address", address), ("blockno", blockno)];
    if raw {
        super::print_raw_response(client, "account", "balancehistory", &params).await
    } else {
        let formatted = handlers::account::format_balancehistory(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn txlist(
    client: &EtherscanClient,
    address: &str,
    pagination: &PaginationParams<'_>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("address", address)];
    params.extend(pagination.to_query_pairs());
    if raw {
        super::print_raw_response(client, "account", "txlist", &params).await
    } else {
        let formatted = handlers::account::format_txlist(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn txlistinternal(
    client: &EtherscanClient,
    address: &str,
    pagination: &PaginationParams<'_>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("address", address)];
    params.extend(pagination.to_query_pairs());
    if raw {
        super::print_raw_response(client, "account", "txlistinternal", &params).await
    } else {
        let formatted = handlers::account::format_txlistinternal(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn tokentx(
    client: &EtherscanClient,
    address: &str,
    contractaddress: Option<&str>,
    pagination: &PaginationParams<'_>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("address", address)];
    if let Some(ca) = contractaddress {
        params.push(("contractaddress", ca));
    }
    params.extend(pagination.to_query_pairs());
    if raw {
        super::print_raw_response(client, "account", "tokentx", &params).await
    } else {
        let formatted = handlers::account::format_tokentx(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn tokennfttx(
    client: &EtherscanClient,
    address: &str,
    contractaddress: Option<&str>,
    pagination: &PaginationParams<'_>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("address", address)];
    if let Some(ca) = contractaddress {
        params.push(("contractaddress", ca));
    }
    params.extend(pagination.to_query_pairs());
    if raw {
        super::print_raw_response(client, "account", "tokennfttx", &params).await
    } else {
        let formatted = handlers::account::format_tokennfttx(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn token1155tx(
    client: &EtherscanClient,
    address: &str,
    contractaddress: Option<&str>,
    pagination: &PaginationParams<'_>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("address", address)];
    if let Some(ca) = contractaddress {
        params.push(("contractaddress", ca));
    }
    params.extend(pagination.to_query_pairs());
    if raw {
        super::print_raw_response(client, "account", "token1155tx", &params).await
    } else {
        let formatted = handlers::account::format_token1155tx(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn addresstokenbalance(
    client: &EtherscanClient,
    address: &str,
    page: Option<&str>,
    offset: Option<&str>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("address", address)];
    if let Some(p) = page {
        params.push(("page", p));
    }
    if let Some(o) = offset {
        params.push(("offset", o));
    }
    if raw {
        super::print_raw_response(client, "account", "addresstokenbalance", &params).await
    } else {
        let formatted = handlers::account::format_addresstokenbalance(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn addresstokennftbalance(
    client: &EtherscanClient,
    address: &str,
    page: Option<&str>,
    offset: Option<&str>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("address", address)];
    if let Some(p) = page {
        params.push(("page", p));
    }
    if let Some(o) = offset {
        params.push(("offset", o));
    }
    if raw {
        super::print_raw_response(client, "account", "addresstokennftbalance", &params).await
    } else {
        let formatted = handlers::account::format_addresstokennftbalance(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn addresstokennftinventory(
    client: &EtherscanClient,
    address: &str,
    contractaddress: Option<&str>,
    page: Option<&str>,
    offset: Option<&str>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("address", address)];
    if let Some(ca) = contractaddress {
        params.push(("contractaddress", ca));
    }
    if let Some(p) = page {
        params.push(("page", p));
    }
    if let Some(o) = offset {
        params.push(("offset", o));
    }
    if raw {
        super::print_raw_response(client, "account", "addresstokennftinventory", &params).await
    } else {
        let formatted = handlers::account::format_addresstokennftinventory(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn getminedblocks(
    client: &EtherscanClient,
    address: &str,
    blocktype: &str,
    page: Option<&str>,
    offset: Option<&str>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("address", address), ("blocktype", blocktype)];
    if let Some(p) = page {
        params.push(("page", p));
    }
    if let Some(o) = offset {
        params.push(("offset", o));
    }
    if raw {
        super::print_raw_response(client, "account", "getminedblocks", &params).await
    } else {
        let formatted = handlers::account::format_getminedblocks(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn getdeposittxs(
    client: &EtherscanClient,
    address: &str,
    pagination: &PaginationParams<'_>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("address", address)];
    params.extend(pagination.to_query_pairs());
    if raw {
        super::print_raw_response(client, "account", "getdeposittxs", &params).await
    } else {
        let formatted =
            handlers::account::format_bridge_txs(client, "getdeposittxs", &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn getwithdrawaltxs(
    client: &EtherscanClient,
    address: &str,
    pagination: &PaginationParams<'_>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("address", address)];
    params.extend(pagination.to_query_pairs());
    if raw {
        super::print_raw_response(client, "account", "getwithdrawaltxs", &params).await
    } else {
        let formatted =
            handlers::account::format_bridge_txs(client, "getwithdrawaltxs", &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn txsbeaconwithdrawal(
    client: &EtherscanClient,
    address: &str,
    pagination: &PaginationParams<'_>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("address", address)];
    params.extend(pagination.to_query_pairs());
    if raw {
        super::print_raw_response(client, "account", "txsBeaconWithdrawal", &params).await
    } else {
        let formatted = handlers::account::format_txsbeaconwithdrawal(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn txnbridge(
    client: &EtherscanClient,
    address: &str,
    pagination: &PaginationParams<'_>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("address", address)];
    params.extend(pagination.to_query_pairs());
    if raw {
        super::print_raw_response(client, "account", "txnbridge", &params).await
    } else {
        let formatted = handlers::account::format_bridge_txs(client, "txnbridge", &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn fundedby(
    client: &EtherscanClient,
    address: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = [("address", address)];
    if raw {
        super::print_raw_response(client, "account", "fundedby", &params).await
    } else {
        let formatted = handlers::account::format_fundedby(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_balance_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "balance".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":"12345"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        assert!(balance(&client, "0x1", "latest", true).await.is_ok());
    }

    #[tokio::test]
    async fn test_txlist_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "txlist".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":[]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let pagination = PaginationParams {
            startblock: Some("0"),
            endblock: None,
            page: None,
            offset: None,
            sort: None,
        };
        assert!(txlist(&client, "0x1", &pagination, true).await.is_ok());
    }

    #[tokio::test]
    async fn test_tokentx_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "tokentx".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":[]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let pagination = PaginationParams {
            startblock: None,
            endblock: None,
            page: None,
            offset: None,
            sort: None,
        };
        assert!(
            tokentx(&client, "0x1", None, &pagination, true)
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_getminedblocks_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "getminedblocks".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":[]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        assert!(
            getminedblocks(&client, "0x1", "blocks", None, None, true)
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_fundedby_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "fundedby".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":[]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        assert!(fundedby(&client, "0x1", true).await.is_ok());
    }

    #[tokio::test]
    async fn test_bridge_txs_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "getdeposittxs".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":[]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let pagination = PaginationParams {
            startblock: None,
            endblock: None,
            page: None,
            offset: None,
            sort: None,
        };
        assert!(
            getdeposittxs(&client, "0x1", &pagination, true)
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_txsbeaconwithdrawal_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "txsBeaconWithdrawal".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":[]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        let pagination = PaginationParams {
            startblock: None,
            endblock: None,
            page: None,
            offset: None,
            sort: None,
        };
        assert!(
            txsbeaconwithdrawal(&client, "0x1", &pagination, true)
                .await
                .is_ok()
        );
    }
}
