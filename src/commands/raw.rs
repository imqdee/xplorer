use crate::client::EtherscanClient;
use crate::error::XplorerError;

pub async fn execute(
    client: &EtherscanClient,
    module: &str,
    action: &str,
    params: &[String],
    compact: bool,
) -> Result<(), XplorerError> {
    let parsed: Vec<(&str, &str)> = params
        .iter()
        .map(|p| {
            p.split_once('=').ok_or_else(|| {
                XplorerError::Api(format!(
                    "Invalid param format '{p}': expected key=value"
                ))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let response = client.call_api(module, action, &parsed).await?;

    let status = response["status"].as_str().unwrap_or("0");
    if status == "0" {
        let error_json = serde_json::json!({
            "status": status,
            "message": response["result"].as_str().unwrap_or("Unknown error"),
            "result": response["result"]
        });
        println!("{}", serde_json::to_string(&error_json).unwrap());
        std::process::exit(1);
    }

    let result = &response["result"];
    if compact {
        let output = serde_json::to_string(result)
            .map_err(|e| XplorerError::Api(format!("Failed to serialize result: {e}")))?;
        println!("{output}");
    } else {
        let output = serde_json::to_string_pretty(result)
            .map_err(|e| XplorerError::Api(format!("Failed to serialize result: {e}")))?;
        println!("{output}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_execute_success_pretty() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "account".into()),
                mockito::Matcher::UrlEncoded("action".into(), "balance".into()),
                mockito::Matcher::UrlEncoded("address".into(), "0x123".into()),
                mockito::Matcher::UrlEncoded("tag".into(), "latest".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"1","message":"OK","result":"1000000000000000000"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), 1, server.url());

        let params = vec![
            "address=0x123".to_string(),
            "tag=latest".to_string(),
        ];
        let result = execute(&client, "account", "balance", &params, false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_success_compact() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"1","message":"OK","result":[{"hash":"0xabc"}]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), 1, server.url());

        let params = vec!["address=0x123".to_string()];
        let result = execute(&client, "account", "txlist", &params, true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_invalid_param_format() {
        let client = EtherscanClient::new_with_url(
            "test_key".to_string(),
            1,
            "http://unused".to_string(),
        );

        let params = vec!["bad_param_no_equals".to_string()];
        let result = execute(&client, "account", "balance", &params, false).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("expected key=value"));
    }

    #[tokio::test]
    async fn test_execute_param_with_equals_in_value() {
        let mut server = mockito::Server::new_async().await;
        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"1","message":"OK","result":"ok"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), 1, server.url());

        let params = vec!["key=val=ue".to_string()];
        let result = execute(&client, "test", "test", &params, false).await;
        assert!(result.is_ok());
    }
}
