use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleConfig {
    pub level: i32,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub roles: HashMap<String, RoleConfig>,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Path::new("config.toml");
        
        // Create default config if it doesn't exist
        if !config_path.exists() {
            let default_config = include_str!("../config.toml");
            fs::write(config_path, default_config)?;
        }

        // Read and parse config
        let content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
} 