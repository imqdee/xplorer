use crate::client::EtherscanClient;
use crate::error::XplorerError;
use serde::Deserialize;

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
    pub block_number: Option<String>,
    pub timestamp: Option<String>,
}

pub async fn format_abi(client: &EtherscanClient, address: &str) -> Result<String, XplorerError> {
    let response = client
        .call_api("contract", "getabi", &[("address", address)])
        .await?;

    let status = response["status"].as_str().unwrap_or("0");
    if status == "0" {
        let message = response["result"].as_str().unwrap_or("Unknown API error");
        return Err(XplorerError::Api(message.to_string()));
    }

    let abi_raw: String = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse ABI: {e}")))?;

    let parsed: serde_json::Value = serde_json::from_str(&abi_raw)
        .map_err(|e| XplorerError::Api(format!("Failed to parse ABI JSON: {e}")))?;

    let pretty = serde_json::to_string_pretty(&parsed)
        .map_err(|e| XplorerError::Api(format!("Failed to format ABI JSON: {e}")))?;

    Ok(format!("{pretty}\n"))
}

pub async fn format_source_code(
    client: &EtherscanClient,
    address: &str,
) -> Result<String, XplorerError> {
    let response = client
        .call_api("contract", "getsourcecode", &[("address", address)])
        .await?;

    let status = response["status"].as_str().unwrap_or("0");
    if status == "0" {
        let message = response["result"].as_str().unwrap_or("Unknown API error");
        return Err(XplorerError::Api(message.to_string()));
    }

    let entries: Vec<SourceCodeEntry> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse source code response: {e}")))?;

    let entry = entries
        .first()
        .ok_or_else(|| XplorerError::Api("No source code data returned".into()))?;

    let mut output = String::new();
    output.push_str(&format!("// Contract Name  : {}\n", entry.contract_name));
    output.push_str(&format!(
        "// Chain ID       : {}\n",
        client
            .chain_id()
            .map_or("N/A".to_string(), |id| id.to_string())
    ));
    output.push_str(&format!("// Compiler       : {}\n", entry.compiler_version));
    output.push_str(&format!("// EVM Version    : {}\n", entry.evm_version));
    output.push_str(&format!(
        "// Optimization   : {} (runs: {})\n",
        entry.optimization_used, entry.runs
    ));
    output.push_str(&format!("// License        : {}\n", entry.license_type));
    output.push_str(&format!("// Proxy          : {}\n", entry.proxy));

    if !entry.implementation.is_empty() {
        output.push_str(&format!("// Implementation : {}\n", entry.implementation));
    }

    output.push('\n');

    if entry.source_code.starts_with("{{") {
        // Standard JSON Input: strip outer braces to get valid JSON
        let inner = &entry.source_code[1..entry.source_code.len() - 1];
        let parsed: serde_json::Value = serde_json::from_str(inner)
            .map_err(|e| XplorerError::Api(format!("Failed to parse source JSON: {e}")))?;
        let sources = parsed["sources"]
            .as_object()
            .ok_or_else(|| XplorerError::Api("Missing 'sources' in source JSON".into()))?;
        for (path, obj) in sources {
            output.push_str(&format!("// File: {path}\n\n"));
            if let Some(content) = obj["content"].as_str() {
                output.push_str(&format!("{content}\n\n"));
            }
        }
    } else {
        output.push_str(&format!("{}\n", entry.source_code));
    }

    Ok(output)
}

pub async fn format_contract_creation(
    client: &EtherscanClient,
    addresses: &[String],
) -> Result<String, XplorerError> {
    let joined = addresses.join(",");
    let response = client
        .call_api(
            "contract",
            "getcontractcreation",
            &[("contractaddresses", &joined)],
        )
        .await?;

    let status = response["status"].as_str().unwrap_or("0");
    if status == "0" {
        let message = response["result"].as_str().unwrap_or("Unknown API error");
        return Err(XplorerError::Api(message.to_string()));
    }

    let entries: Vec<ContractCreationEntry> = serde_json::from_value(response["result"].clone())
        .map_err(|e| {
            XplorerError::Api(format!("Failed to parse contract creation response: {e}"))
        })?;

    let mut output = String::new();
    for (i, entry) in entries.iter().enumerate() {
        output.push_str(&format!("Contract : {}\n", entry.contract_address));
        output.push_str(&format!("Creator  : {}\n", entry.contract_creator));
        output.push_str(&format!("Tx Hash  : {}\n", entry.tx_hash));

        if let Some(ref block_number) = entry.block_number {
            output.push_str(&format!("Block    : {}\n", block_number));
        }

        if let Some(ref timestamp) = entry.timestamp {
            output.push_str(&format!("Time     : {}\n", timestamp));
        }

        if i < entries.len() - 1 {
            output.push('\n');
        }
    }

    Ok(output)
}

pub async fn format_verify_status(
    client: &EtherscanClient,
    guid: &str,
) -> Result<String, XplorerError> {
    let response = client
        .call_api("contract", "checkverifystatus", &[("guid", guid)])
        .await?;

    let status = response["status"].as_str().unwrap_or("0");
    if status == "0" {
        let message = response["result"].as_str().unwrap_or("Unknown API error");
        return Err(XplorerError::Api(message.to_string()));
    }

    let result = response["result"].as_str().unwrap_or("Unknown");
    Ok(format!("Status : {result}\n"))
}

pub async fn format_proxy_verification_status(
    client: &EtherscanClient,
    guid: &str,
) -> Result<String, XplorerError> {
    let response = client
        .call_api("contract", "checkproxyverification", &[("guid", guid)])
        .await?;

    let status = response["status"].as_str().unwrap_or("0");
    if status == "0" {
        let message = response["result"].as_str().unwrap_or("Unknown API error");
        return Err(XplorerError::Api(message.to_string()));
    }

    let result = response["result"].as_str().unwrap_or("Unknown");
    Ok(format!("Status : {result}\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_format_abi_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"1","message":"OK","result":"[{\"name\":\"transfer\"}]"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_abi(&client, "0x123").await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("\"name\": \"transfer\""));
        assert!(result.contains("[\n"));
    }

    #[tokio::test]
    async fn test_format_abi_error() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"0","message":"NOTOK","result":"Contract not verified"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_abi(&client, "0xinvalid").await;
        mock.assert_async().await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Contract not verified")
        );
    }

    #[tokio::test]
    async fn test_format_source_code_success() {
        let mut server = mockito::Server::new_async().await;
        let source_response = r#"{
            "status": "1",
            "message": "OK",
            "result": [{
                "SourceCode": "pragma solidity ^0.8.0;",
                "ABI": "[]",
                "ContractName": "TestContract",
                "CompilerVersion": "v0.8.0",
                "OptimizationUsed": "1",
                "Runs": "200",
                "ConstructorArguments": "",
                "EVMVersion": "Default",
                "Library": "",
                "LicenseType": "MIT",
                "Proxy": "0",
                "Implementation": "",
                "SwarmSource": ""
            }]
        }"#;

        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(source_response)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_source_code(&client, "0x123").await.unwrap();
        mock.assert_async().await;

        assert!(result.contains("TestContract"));
        assert!(result.contains("Chain ID       : 1"));
        assert!(result.contains("Compiler       : v0.8.0"));
        assert!(result.contains("pragma solidity"));
    }

    #[tokio::test]
    async fn test_format_contract_creation_success() {
        let mut server = mockito::Server::new_async().await;
        let creation_response = r#"{
            "status": "1",
            "message": "OK",
            "result": [{
                "contractAddress": "0x123",
                "contractCreator": "0xabc",
                "txHash": "0xdef",
                "blockNumber": "10720863",
                "timestamp": "1598242563"
            }]
        }"#;

        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(creation_response)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_contract_creation(&client, &[String::from("0x123")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Contract : 0x123"));
        assert!(result.contains("Creator  : 0xabc"));
        assert!(result.contains("Tx Hash  : 0xdef"));
        assert!(result.contains("Block    : 10720863"));
        assert!(result.contains("Time     : 1598242563"));
    }

    #[tokio::test]
    async fn test_format_contract_creation_multiple() {
        let mut server = mockito::Server::new_async().await;
        let creation_response = r#"{
            "status": "1",
            "message": "OK",
            "result": [
                {
                    "contractAddress": "0x123",
                    "contractCreator": "0xabc",
                    "txHash": "0xdef",
                    "blockNumber": "10720863",
                    "timestamp": "1598242563"
                },
                {
                    "contractAddress": "0x456",
                    "contractCreator": "0xghi",
                    "txHash": "0xjkl",
                    "blockNumber": "10720999",
                    "timestamp": "1598244321"
                }
            ]
        }"#;

        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(creation_response)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result =
            format_contract_creation(&client, &[String::from("0x123"), String::from("0x456")])
                .await
                .unwrap();
        mock.assert_async().await;

        assert!(result.contains("0x123"));
        assert!(result.contains("0x456"));
        assert!(result.contains("Block    : 10720863"));
        assert!(result.contains("Block    : 10720999"));
        let newlines = result.matches("\n\n").count();
        assert_eq!(newlines, 1);
    }

    #[tokio::test]
    async fn test_format_contract_creation_missing_optional_fields() {
        let mut server = mockito::Server::new_async().await;
        let creation_response = r#"{
            "status": "1",
            "message": "OK",
            "result": [{
                "contractAddress": "0x123",
                "contractCreator": "0xabc",
                "txHash": "0xdef"
            }]
        }"#;

        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(creation_response)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_contract_creation(&client, &[String::from("0x123")])
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("Contract : 0x123"));
        assert!(result.contains("Creator  : 0xabc"));
        assert!(result.contains("Tx Hash  : 0xdef"));
        assert!(!result.contains("Block    :"));
        assert!(!result.contains("Time     :"));
    }

    #[tokio::test]
    async fn test_format_verify_status_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "contract".into()),
                mockito::Matcher::UrlEncoded("action".into(), "checkverifystatus".into()),
                mockito::Matcher::UrlEncoded("guid".into(), "abc123".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"1","message":"OK","result":"Pass - Verified"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_verify_status(&client, "abc123").await.unwrap();
        mock.assert_async().await;

        assert_eq!(result, "Status : Pass - Verified\n");
    }

    #[tokio::test]
    async fn test_format_verify_status_pending() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"0","message":"NOTOK","result":"Pending in queue"}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_verify_status(&client, "abc123").await;
        mock.assert_async().await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Pending in queue"));
    }

    #[tokio::test]
    async fn test_format_proxy_verification_status_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("module".into(), "contract".into()),
                mockito::Matcher::UrlEncoded("action".into(), "checkproxyverification".into()),
                mockito::Matcher::UrlEncoded("guid".into(), "proxy123".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"status":"1","message":"OK","result":"The proxy's implementation contract is found at 0x123"}"#,
            )
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = format_proxy_verification_status(&client, "proxy123")
            .await
            .unwrap();
        mock.assert_async().await;

        assert!(result.contains("implementation contract is found"));
    }
}
