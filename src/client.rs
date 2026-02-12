use crate::error::XplorerError;
use serde::Deserialize;
use serde::de::DeserializeOwned;

pub struct EtherscanClient {
    http: reqwest::Client,
    api_key: String,
    chain_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct SourceCodeEntry {
    pub source_code: String,
    #[serde(rename = "ABI")]
    pub abi: String,
    pub contract_name: String,
    pub compiler_version: String,
    pub optimization_used: String,
    pub runs: String,
    pub constructor_arguments: String,
    #[serde(rename = "EVMVersion")]
    pub evm_version: String,
    pub library: String,
    pub license_type: String,
    pub proxy: String,
    pub implementation: String,
    pub swarm_source: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractCreationEntry {
    pub contract_address: String,
    pub contract_creator: String,
    pub tx_hash: String,
}

impl EtherscanClient {
    pub fn new(api_key: String, chain_id: u64) -> Self {
        Self {
            http: reqwest::Client::new(),
            api_key,
            chain_id,
        }
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    async fn get<T: DeserializeOwned>(
        &self,
        module: &str,
        action: &str,
        params: &[(&str, &str)],
    ) -> Result<T, XplorerError> {
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
            .get("https://api.etherscan.io/v2/api")
            .query(&query)
            .send()
            .await?;

        let body: serde_json::Value = response.json().await?;

        let status = body["status"].as_str().unwrap_or("0");
        if status == "0" {
            let message = body["result"].as_str().unwrap_or("Unknown API error");
            return Err(XplorerError::Api(message.to_string()));
        }

        let result = serde_json::from_value(body["result"].clone())
            .map_err(|e| XplorerError::Api(format!("Failed to parse response: {e}")))?;

        Ok(result)
    }

    pub async fn get_abi(&self, address: &str) -> Result<String, XplorerError> {
        self.get("contract", "getabi", &[("address", address)])
            .await
    }

    pub async fn get_source_code(
        &self,
        address: &str,
    ) -> Result<Vec<SourceCodeEntry>, XplorerError> {
        self.get("contract", "getsourcecode", &[("address", address)])
            .await
    }

    pub async fn get_contract_creation(
        &self,
        addresses: &[String],
    ) -> Result<Vec<ContractCreationEntry>, XplorerError> {
        let joined = addresses.join(",");
        self.get(
            "contract",
            "getcontractcreation",
            &[("contractaddresses", &joined)],
        )
        .await
    }
}
