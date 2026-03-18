use crate::client::EtherscanClient;
use crate::error::XplorerError;
use crate::handlers;

#[allow(clippy::too_many_arguments)]
pub async fn get_logs(
    client: &EtherscanClient,
    from_block: &str,
    to_block: &str,
    address: Option<&str>,
    topic0: Option<&str>,
    topic1: Option<&str>,
    topic2: Option<&str>,
    topic3: Option<&str>,
    topic0_1_opr: Option<&str>,
    topic0_2_opr: Option<&str>,
    topic0_3_opr: Option<&str>,
    topic1_2_opr: Option<&str>,
    topic1_3_opr: Option<&str>,
    topic2_3_opr: Option<&str>,
    page: Option<&str>,
    offset: Option<&str>,
    raw: bool,
) -> Result<(), XplorerError> {
    let mut params: Vec<(&str, &str)> = vec![("fromBlock", from_block), ("toBlock", to_block)];

    let optionals: &[(&str, Option<&str>)] = &[
        ("address", address),
        ("topic0", topic0),
        ("topic1", topic1),
        ("topic2", topic2),
        ("topic3", topic3),
        ("topic0_1_opr", topic0_1_opr),
        ("topic0_2_opr", topic0_2_opr),
        ("topic0_3_opr", topic0_3_opr),
        ("topic1_2_opr", topic1_2_opr),
        ("topic1_3_opr", topic1_3_opr),
        ("topic2_3_opr", topic2_3_opr),
        ("page", page),
        ("offset", offset),
    ];

    for &(key, val) in optionals {
        if let Some(v) = val {
            params.push((key, v));
        }
    }

    if raw {
        super::print_raw_response(client, "logs", "getLogs", &params).await
    } else {
        let formatted = handlers::logs::format_logs(client, &params).await?;
        print!("{formatted}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_get_logs_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "logs".into()),
                mockito::Matcher::UrlEncoded("action".into(), "getLogs".into()),
                mockito::Matcher::UrlEncoded("fromBlock".into(), "12878196".into()),
                mockito::Matcher::UrlEncoded("toBlock".into(), "12878300".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"1","message":"OK","result":[]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = get_logs(
            &client, "12878196", "12878300", None, None, None, None, None, None, None, None, None,
            None, None, None, None, true,
        )
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_logs_with_optional_params() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("fromBlock".into(), "100".into()),
                mockito::Matcher::UrlEncoded("toBlock".into(), "200".into()),
                mockito::Matcher::UrlEncoded("address".into(), "0xabc".into()),
                mockito::Matcher::UrlEncoded("topic0".into(), "0xtopic".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"1","message":"OK","result":[]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = get_logs(
            &client,
            "100",
            "200",
            Some("0xabc"),
            Some("0xtopic"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            true,
        )
        .await;
        assert!(result.is_ok());
    }
}
