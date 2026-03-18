use crate::client::EtherscanClient;
use crate::error::XplorerError;
use crate::handlers;

pub async fn get_abi(
    client: &EtherscanClient,
    address: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    if raw {
        super::print_raw_response(client, "contract", "getabi", &[("address", address)]).await
    } else {
        let formatted = handlers::contract::format_abi(client, address).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn get_source_code(
    client: &EtherscanClient,
    address: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    if raw {
        super::print_raw_response(client, "contract", "getsourcecode", &[("address", address)])
            .await
    } else {
        let formatted = handlers::contract::format_source_code(client, address).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn get_contract_creation(
    client: &EtherscanClient,
    addresses: &[String],
    raw: bool,
) -> Result<(), XplorerError> {
    if raw {
        let joined = addresses.join(",");
        super::print_raw_response(
            client,
            "contract",
            "getcontractcreation",
            &[("contractaddresses", &joined)],
        )
        .await
    } else {
        let formatted = handlers::contract::format_contract_creation(client, addresses).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn check_verify_status(
    client: &EtherscanClient,
    guid: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    if raw {
        super::print_raw_response(client, "contract", "checkverifystatus", &[("guid", guid)]).await
    } else {
        let formatted = handlers::contract::format_verify_status(client, guid).await?;
        print!("{formatted}");
        Ok(())
    }
}

pub async fn check_proxy_verification(
    client: &EtherscanClient,
    guid: &str,
    raw: bool,
) -> Result<(), XplorerError> {
    if raw {
        super::print_raw_response(
            client,
            "contract",
            "checkproxyverification",
            &[("guid", guid)],
        )
        .await
    } else {
        let formatted = handlers::contract::format_proxy_verification_status(client, guid).await?;
        print!("{formatted}");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::EtherscanClient;

    #[tokio::test]
    async fn test_print_raw_response_success() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"1","message":"OK","result":[{"test":"data"}]}"#)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = crate::commands::print_raw_response(
            &client,
            "contract",
            "getabi",
            &[("address", "0x123")],
        )
        .await;
        mock.assert_async().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_abi_raw_vs_formatted() {
        let mut server = mockito::Server::new_async().await;

        let _mock_raw = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"status":"1","message":"OK","result":"[{\"name\":\"test\"}]"}"#)
            .expect(2)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result_raw = get_abi(&client, "0x123", true).await;
        assert!(result_raw.is_ok());

        let result_formatted = get_abi(&client, "0x123", false).await;
        assert!(result_formatted.is_ok());
    }

    #[tokio::test]
    async fn test_get_source_code_raw_mode() {
        let mut server = mockito::Server::new_async().await;
        let source_response = r#"{
            "status": "1",
            "message": "OK",
            "result": [{
                "SourceCode": "pragma solidity ^0.8.0;",
                "ABI": "[]",
                "ContractName": "Test",
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

        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(source_response)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let result = get_source_code(&client, "0x123", true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_contract_creation_raw_mode() {
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

        let _mock = server
            .mock("GET", "/")
            .match_query(mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(creation_response)
            .create_async()
            .await;

        let client = EtherscanClient::new_with_url("test_key".to_string(), Some(1), server.url());

        let addresses = vec![String::from("0x123")];
        let result = get_contract_creation(&client, &addresses, true).await;
        assert!(result.is_ok());
    }
}
