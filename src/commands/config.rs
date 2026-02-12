use crate::config::Config;
use crate::error::XplorerError;

pub fn set_api_key(key: Option<String>) -> Result<(), XplorerError> {
    let key = match key {
        Some(k) => k,
        None => prompt_api_key()?,
    };

    if key.is_empty() {
        return Err(XplorerError::Validation("API key cannot be empty.".into()));
    }

    let mut config = Config::load();
    config.set_api_key(key)?;

    println!("API key saved successfully.");
    Ok(())
}

fn prompt_api_key() -> Result<String, XplorerError> {
    eprint!("Enter your Etherscan API key: ");
    rpassword::read_password()
        .map_err(|e| XplorerError::Config(format!("Failed to read API key: {e}")))
}
