//! Configuration with hot-reload support.

use mazerion_core::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app_name: String,
    pub version: String,
    pub precision: PrecisionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecisionConfig {
    pub sg_decimals: u32,
    pub ph_decimals: u32,
    pub brix_decimals: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_name: "Mazerion".into(),
            version: "0.1.0".into(),
            precision: PrecisionConfig {
                sg_decimals: 4,
                ph_decimals: 3,
                brix_decimals: 2,
            },
        }
    }
}

/// Ingredient definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub category: String,
    pub sugar_content: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredients {
    pub items: Vec<Ingredient>,
}

/// File watcher for hot-reload.
pub struct FileWatcher {
    path: PathBuf,
    last_modified: Option<SystemTime>,
    last_size: Option<u64>,
}

impl FileWatcher {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            last_modified: None,
            last_size: None,
        }
    }

    /// Check if file changed (returns true on first call).
    pub fn check_changed(&mut self) -> Result<bool> {
        let metadata = fs::metadata(&self.path)
            .map_err(|e| Error::Io(format!("Failed to read metadata: {}", e)))?;

        let modified = metadata
            .modified()
            .map_err(|e| Error::Io(format!("Failed to get mtime: {}", e)))?;
        let size = metadata.len();

        let changed = (self.last_modified != Some(modified)) || (self.last_size != Some(size));

        if changed {
            self.last_modified = Some(modified);
            self.last_size = Some(size);
        }

        Ok(changed)
    }

    /// Load file content.
    pub fn load(&self) -> Result<String> {
        fs::read_to_string(&self.path)
            .map_err(|e| Error::Io(format!("Failed to read {}: {}", self.path.display(), e)))
    }
}

/// Load config from TOML file.
pub fn load_config(path: impl AsRef<Path>) -> Result<Config> {
    let content = fs::read_to_string(path.as_ref())
        .map_err(|e| Error::Config(format!("Failed to read config: {}", e)))?;
    toml::from_str(&content).map_err(|e| Error::Config(format!("Failed to parse config: {}", e)))
}

/// Load ingredients from TOML file.
pub fn load_ingredients(path: impl AsRef<Path>) -> Result<Ingredients> {
    let content = fs::read_to_string(path.as_ref())
        .map_err(|e| Error::Config(format!("Failed to read ingredients: {}", e)))?;
    toml::from_str(&content)
        .map_err(|e| Error::Config(format!("Failed to parse ingredients: {}", e)))
}
