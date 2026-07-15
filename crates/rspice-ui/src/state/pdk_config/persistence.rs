use std::path::{Path, PathBuf};

use super::*;

impl PdkConfig {
    // =========================================================================
    // Persistence
    // =========================================================================

    /// Get the default configuration file path
    pub fn default_config_path() -> PathBuf {
        dirs::config_dir()
            .or_else(dirs::home_dir)
            .unwrap_or_else(|| PathBuf::from("."))
            .join("rspice")
            .join(CONFIG_FILE_NAME)
    }

    /// Load configuration from the default path
    pub fn load() -> Result<Self, ConfigError> {
        Self::load_from(&Self::default_config_path())
    }

    /// Load configuration from a specific path
    pub fn load_from(path: &Path) -> Result<Self, ConfigError> {
        #[cfg(target_arch = "wasm32")]
        {
            load_pdk_config_from_browser_storage(path)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            crate::io::durable_file::reconcile_publication(path)
                .map_err(|e| ConfigError::Io(e.to_string()))?;
            if !path.exists() {
                return Ok(Self::default());
            }

            let content =
                std::fs::read_to_string(path).map_err(|e| ConfigError::Io(e.to_string()))?;

            serde_json::from_str(&content).map_err(|e| ConfigError::Parse(e.to_string()))
        }
    }

    /// Save configuration to the default path
    pub fn save(&self) -> Result<(), ConfigError> {
        self.save_to(&Self::default_config_path())
    }

    /// Save configuration to a specific path
    pub fn save_to(&self, path: &Path) -> Result<(), ConfigError> {
        #[cfg(target_arch = "wasm32")]
        {
            save_pdk_config_to_browser_storage(path, self)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Ensure parent directory exists
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| ConfigError::Io(e.to_string()))?;
            }

            let expected = crate::io::durable_file::observe_expected_content(path)
                .map_err(|e| ConfigError::Io(e.to_string()))?;
            let content = serde_json::to_vec_pretty(self)
                .map_err(|e| ConfigError::Serialize(e.to_string()))?;

            publish_pdk_config(path, expected, &content)
        }
    }

    /// Check if configuration has any content
    pub fn is_empty(&self) -> bool {
        self.library_paths.is_empty()
            && self.environment_variables.is_empty()
            && self.recent_files.is_empty()
    }

    /// Get total discovered file count
    pub fn total_file_count(&self) -> usize {
        self.discovered_files.len()
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn publish_pdk_config(
    path: &Path,
    expected: crate::io::durable_file::ExpectedContent,
    content: &[u8],
) -> Result<(), ConfigError> {
    crate::io::durable_file::compare_exchange_bytes(path, expected, content)
        .map_err(|e| ConfigError::Io(e.to_string()))
}

#[cfg(target_arch = "wasm32")]
fn browser_storage_key(path: &Path) -> String {
    format!("rspice:pdk-config:v1:{}", path.to_string_lossy())
}

#[cfg(target_arch = "wasm32")]
fn browser_local_storage() -> Result<web_sys::Storage, ConfigError> {
    web_sys::window()
        .ok_or_else(|| ConfigError::Io("browser window is unavailable".to_string()))?
        .local_storage()
        .map_err(browser_storage_error)?
        .ok_or_else(|| ConfigError::Io("browser local storage is unavailable".to_string()))
}

#[cfg(target_arch = "wasm32")]
fn load_pdk_config_from_browser_storage(path: &Path) -> Result<PdkConfig, ConfigError> {
    let storage = browser_local_storage()?;
    let Some(content) = storage
        .get_item(&browser_storage_key(path))
        .map_err(browser_storage_error)?
    else {
        return Ok(PdkConfig::default());
    };
    serde_json::from_str(&content).map_err(|error| ConfigError::Parse(error.to_string()))
}

#[cfg(target_arch = "wasm32")]
fn save_pdk_config_to_browser_storage(path: &Path, config: &PdkConfig) -> Result<(), ConfigError> {
    let content = serde_json::to_string_pretty(config)
        .map_err(|error| ConfigError::Serialize(error.to_string()))?;
    browser_local_storage()?
        .set_item(&browser_storage_key(path), &content)
        .map_err(browser_storage_error)
}

#[cfg(target_arch = "wasm32")]
fn browser_storage_error(error: wasm_bindgen::JsValue) -> ConfigError {
    ConfigError::Io(
        error
            .as_string()
            .unwrap_or_else(|| "browser storage operation failed".to_string()),
    )
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn save_round_trips_through_durable_publication() {
        let root = unique_temp_dir("round-trip");
        let path = root.join(CONFIG_FILE_NAME);
        let mut config = PdkConfig::new();
        config
            .environment_variables
            .insert("PDK_ROOT".to_string(), "/models".to_string());

        config.save_to(&path).expect("save config");
        let loaded = PdkConfig::load_from(&path).expect("load config");

        assert_eq!(loaded, config);
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn publication_rejects_late_external_change() {
        let root = unique_temp_dir("late-change");
        let path = root.join(CONFIG_FILE_NAME);
        std::fs::write(&path, b"authorized predecessor").expect("write predecessor");
        let expected =
            crate::io::durable_file::observe_expected_content(&path).expect("observe destination");
        let content = serde_json::to_vec_pretty(&PdkConfig::new()).expect("serialize config");
        std::fs::write(&path, b"late external edit").expect("race destination");

        let result = publish_pdk_config(&path, expected, &content);

        assert!(result.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"late external edit");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rspice-pdk-config-{label}-{}",
            uuid::Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("create fixture");
        root
    }
}
