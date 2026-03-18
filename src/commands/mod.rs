pub mod account;
pub mod apilimit;
pub mod block;
pub mod chainlist;
pub mod config;
pub mod contract;
pub mod gas;
pub mod logs;
pub mod raw;
pub mod token;
pub mod transaction;

use crate::client::EtherscanClient;
use crate::error::XplorerError;

pub(crate) async fn print_raw_response(
    client: &EtherscanClient,
    module: &str,
    action: &str,
    params: &[(&str, &str)],
) -> Result<(), XplorerError> {
    let response = client.call_api(module, action, params).await?;

    let status = response["status"].as_str().unwrap_or("0");
    if status == "0" {
        let message = response["result"].as_str().unwrap_or("Unknown error");
        return Err(XplorerError::Api(message.to_string()));
    }

    let result_compact = serde_json::to_string(&response["result"])
        .map_err(|e| XplorerError::Api(format!("Failed to serialize result: {e}")))?;
    println!("{result_compact}");
    Ok(())
}
