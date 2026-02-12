use crate::client::EtherscanClient;
use crate::error::XplorerError;

pub async fn get_abi(client: &EtherscanClient, address: &str) -> Result<(), XplorerError> {
    let abi_raw = client.get_abi(address).await?;

    let parsed: serde_json::Value = serde_json::from_str(&abi_raw)
        .map_err(|e| XplorerError::Api(format!("Failed to parse ABI JSON: {e}")))?;

    let pretty = serde_json::to_string_pretty(&parsed)
        .map_err(|e| XplorerError::Api(format!("Failed to format ABI JSON: {e}")))?;

    println!("{pretty}");
    Ok(())
}

pub async fn get_source_code(client: &EtherscanClient, address: &str) -> Result<(), XplorerError> {
    let entries = client.get_source_code(address).await?;

    let entry = entries
        .first()
        .ok_or_else(|| XplorerError::Api("No source code data returned".into()))?;

    println!("// Contract Name  : {}", entry.contract_name);
    println!("// Chain ID       : {}", client.chain_id());
    println!("// Compiler       : {}", entry.compiler_version);
    println!("// EVM Version    : {}", entry.evm_version);
    println!(
        "// Optimization   : {} (runs: {})",
        entry.optimization_used, entry.runs
    );
    println!("// License        : {}", entry.license_type);
    println!("// Proxy          : {}", entry.proxy);

    if !entry.implementation.is_empty() {
        println!("// Implementation : {}", entry.implementation);
    }

    println!();

    if entry.source_code.starts_with("{{") {
        // Standard JSON Input: strip outer braces to get valid JSON
        let inner = &entry.source_code[1..entry.source_code.len() - 1];
        let parsed: serde_json::Value = serde_json::from_str(inner)
            .map_err(|e| XplorerError::Api(format!("Failed to parse source JSON: {e}")))?;
        let sources = parsed["sources"]
            .as_object()
            .ok_or_else(|| XplorerError::Api("Missing 'sources' in source JSON".into()))?;
        for (path, obj) in sources {
            println!("// File: {path}\n");
            if let Some(content) = obj["content"].as_str() {
                println!("{content}\n");
            }
        }
    } else {
        println!("{}", entry.source_code);
    }

    Ok(())
}

pub async fn get_contract_creation(
    client: &EtherscanClient,
    addresses: &[String],
) -> Result<(), XplorerError> {
    let entries = client.get_contract_creation(addresses).await?;

    for entry in &entries {
        println!("Contract : {}", entry.contract_address);
        println!("Creator  : {}", entry.contract_creator);
        println!("Tx Hash  : {}", entry.tx_hash);
        if entries.len() > 1 {
            println!();
        }
    }

    Ok(())
}
