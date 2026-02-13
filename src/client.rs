use crate::error::XplorerError;

pub struct EtherscanClient {
    http: reqwest::Client,
    api_key: String,
    chain_id: u64,
    base_url: String,
}

impl EtherscanClient {
    pub fn new(api_key: String, chain_id: u64) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key,
            chain_id,
            base_url: "https://api.etherscan.io/v2/api".to_string(),
        }
    }

    #[cfg(test)]
    pub fn new_with_url(api_key: String, chain_id: u64, base_url: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key,
            chain_id,
            base_url,
        }
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    pub async fn call_api(
        &self,
        module: &str,
        action: &str,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Value, XplorerError> {
        let chain_id_str = self.chain_id.to_string();
        let mut query: Vec<(&str, &str)> = vec![
            ("chainid", &chain_id_str),
            ("module", module),
            ("action", action),
            ("apikey", &self.api_key),
        ];
        query.extend_from_slice(params);

        let response = self
            .http
            .get(&self.base_url)
            .query(&query)
            .header("X-Chain-Id", self.chain_id.to_string())
            .send()
            .await?;

        let body: serde_json::Value = response.json().await?;
        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_api_returns_full_response() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "contract".into()),
                mockito::Matcher::UrlEncoded("action".into(), "getabi".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"1","message":"OK","result":"[{\"test\":\"abi\"}]"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), 1, server.url());

        let response = client
            .call_api("contract", "getabi", &[("address", "0x123")])
            .await
            .unwrap();

        mock.assert_async().await;
        assert_eq!(response["status"], "1");
        assert_eq!(response["message"], "OK");
        assert_eq!(response["result"], "[{\"test\":\"abi\"}]");
    }

    #[tokio::test]
    async fn test_call_api_returns_error_response() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"0","message":"NOTOK","result":"Invalid address"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), 1, server.url());

        let response = client
            .call_api("contract", "getabi", &[("address", "0xinvalid")])
            .await
            .unwrap();

        mock.assert_async().await;
        assert_eq!(response["status"], "0");
        assert_eq!(response["result"], "Invalid address");
    }
}
