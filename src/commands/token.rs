use crate::client::EtherscanClient;
use crate::error::XplorerError;
use crate::handlers;

pub async fn tokeninfo(
    client: &EtherscanClient,
    contractaddress: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = [("contractaddress", contractaddress)];
    if raw {
        super::print_raw_response(client, "token", "tokeninfo", &params).await
    } else {
        let formatted = handlers::token::format_tokeninfo(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn tokenholdercount(
    client: &EtherscanClient,
    contractaddress: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    let params = [("contractaddress", contractaddress)];
    if raw {
        super::print_raw_response(client, "token", "tokenholdercount", &params).await
    } else {
        let formatted = handlers::token::format_tokenholdercount(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn tokenholderlist(
    client: &EtherscanClient,
    contractaddress: &str,
    page: Option<&str>,
    offset: Option<&str>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("contractaddress", contractaddress)];
    if let Some(p) = page {
        params.push(("page", p));
    }
    if let Some(o) = offset {
        params.push(("offset", o));
    }
    if raw {
        super::print_raw_response(client, "token", "tokenholderlist", &params).await
    } else {
        let formatted = handlers::token::format_tokenholderlist(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn topholders(
    client: &EtherscanClient,
    contractaddress: &str,
    page: Option<&str>,
    offset: Option<&str>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("contractaddress", contractaddress)];
    if let Some(p) = page {
        params.push(("page", p));
    }
    if let Some(o) = offset {
        params.push(("offset", o));
    }
    if raw {
        super::print_raw_response(client, "token", "topholders", &params).await
    } else {
        let formatted = handlers::token::format_topholders(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_tokeninfo_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "token".into()),
                mockito::Matcher::UrlEncoded("action".into(), "tokeninfo".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":[{"contractAddress":"0x1"}]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        assert!(tokeninfo(&client, "0x1", true).await.is_ok());
    }

    #[tokio::test]
    async fn test_tokenholderlist_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "token".into()),
                mockito::Matcher::UrlEncoded("action".into(), "tokenholderlist".into()),
            ]))
            .with_status(200)
            .with_body(r#"{"status":"1","message":"OK","result":[]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("k".into(), Some(1), server.url());
        assert!(
            tokenholderlist(&client, "0x1", None, None, true)
                .await
                .is_ok()
        );
    }
}
