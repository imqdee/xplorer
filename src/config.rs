use crate::error::XplorerError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub api_key: Option<String>,
}

impl Config {
    pub fn path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".xplorer").join("config.toml"))
    }

    pub fn load() -> Self {
        let Some(path) = Self::path() else {
            return Self::default();
        };

        if !path.exists() {
            return Self::default();
        }

        fs::read_to_string(&path)
            .ok()
            .and_then(|content| toml::from_str(&content).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), XplorerError> {
        let path = Self::path()
            .ok_or_else(|| XplorerError::Config("Could not determine home directory".into()))?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                XplorerError::Config(format!("Failed to create config directory: {e}"))
            })?;
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| XplorerError::Config(format!("Failed to serialize config: {e}")))?;
        fs::write(&path, content)
            .map_err(|e| XplorerError::Config(format!("Failed to write config: {e}")))?;

        Ok(())
    }

    pub fn set_api_key(&mut self, key: String) -> Result<(), XplorerError> {
        self.api_key = Some(key);
        self.save()
    }

    pub fn require_api_key(&self) -> Result<&str, XplorerError> {
        self.api_key.as_deref().ok_or(XplorerError::MissingApiKey)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_no_api_key() {
        let config = Config::default();
        assert!(config.api_key.is_none());
    }

    #[test]
    fn config_serializes_to_toml() {
        let config = Config {
            api_key: Some("test-key-123".to_string()),
        };
        let toml_str = toml::to_string(&config).unwrap();
        assert!(toml_str.contains("api_key"));
        assert!(toml_str.contains("test-key-123"));
    }

    #[test]
    fn config_deserializes_from_toml() {
        let toml_str = r#"api_key = "my-secret-key""#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.api_key, Some("my-secret-key".to_string()));
    }

    #[test]
    fn config_deserializes_empty_toml() {
        let toml_str = "";
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.api_key.is_none());
    }

    #[test]
    fn config_deserializes_with_missing_api_key() {
        let toml_str = "# empty config\n";
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.api_key.is_none());
    }

    #[test]
    fn config_path_ends_with_expected_segments() {
        if let Some(path) = Config::path() {
            let path_str = path.to_string_lossy();
            assert!(path_str.ends_with(".xplorer/config.toml"));
        }
    }

    #[test]
    fn config_roundtrip_serialization() {
        let original = Config {
            api_key: Some("roundtrip-test-key".to_string()),
        };

        let toml_str = toml::to_string(&original).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(original.api_key, deserialized.api_key);
    }

    #[test]
    fn require_api_key_returns_key_when_present() {
        let config = Config {
            api_key: Some("my-key".to_string()),
        };
        assert_eq!(config.require_api_key().unwrap(), "my-key");
    }

    #[test]
    fn require_api_key_errors_when_missing() {
        let config = Config::default();
        assert!(config.require_api_key().is_err());
    }
}
