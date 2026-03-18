use crate::error::XplorerError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChainEntry {
    pub chainname: String,
    pub chainid: String,
    pub blockexplorer: String,
    #[allow(dead_code)]
    pub apiurl: String,
    pub status: u8,
}

fn status_label(status: u8) -> &'static str {
    match status {
        1 => "OK",
        2 => "Degraded",
        _ => "Offline",
    }
}

pub fn format_chainlist(response: &serde_json::Value) -> Result<String, XplorerError> {
    let entries: Vec<ChainEntry> = serde_json::from_value(response["result"].clone())
        .map_err(|e| XplorerError::Api(format!("Failed to parse chainlist response: {e}")))?;

    let mut output = String::new();
    output.push_str(&format!(
        "{:<10}{:<30}{:<10}{}\n",
        "Chain ID", "Name", "Status", "Explorer"
    ));

    for entry in &entries {
        output.push_str(&format!(
            "{:<10}{:<30}{:<10}{}\n",
            entry.chainid,
            entry.chainname,
            status_label(entry.status),
            entry.blockexplorer
        ));
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_chainlist_success() {
        let response = serde_json::json!({
            "totalcount": 2,
            "result": [
                {
                    "chainname": "Ethereum Mainnet",
                    "chainid": "1",
                    "blockexplorer": "https://etherscan.io",
                    "apiurl": "https://api.etherscan.io",
                    "status": 1
                },
                {
                    "chainname": "Base",
                    "chainid": "8453",
                    "blockexplorer": "https://basescan.org",
                    "apiurl": "https://api.basescan.org",
                    "status": 1
                }
            ]
        });

        let result = format_chainlist(&response).unwrap();

        assert!(result.contains("Ethereum Mainnet"));
        assert!(result.contains("Base"));
        assert!(result.contains("OK"));
        assert!(result.contains("https://etherscan.io"));
        assert!(result.contains("8453"));
    }

    #[test]
    fn test_status_labels() {
        assert_eq!(status_label(0), "Offline");
        assert_eq!(status_label(1), "OK");
        assert_eq!(status_label(2), "Degraded");
    }
}
