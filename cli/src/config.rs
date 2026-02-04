use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub server_url: String,
    pub api_key: String,
}

impl Config {
    pub fn new(server_url: String, api_key: String) -> Self {
        Self {
            server_url,
            api_key,
        }
    }
}

pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Failed to get config directory")?
        .join("clickploy");

    fs::create_dir_all(&config_dir)
        .context("Failed to create config directory")?;

    Ok(config_dir.join("config.toml"))
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        anyhow::bail!("Config file not found. Please run setup first.");
    }

    let contents = fs::read_to_string(&config_path)
        .context("Failed to read config file")?;

    let config: Config = toml::from_str(&contents)
        .context("Failed to parse config file")?;

    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;
    
    let contents = toml::to_string_pretty(config)
        .context("Failed to serialize config")?;

    fs::write(&config_path, contents)
        .context("Failed to write config file")?;

    Ok(())
}

pub fn config_exists() -> bool {
    get_config_path()
        .map(|path| path.exists())
        .unwrap_or(false)
}

pub fn delete_config() -> Result<()> {
    let config_path = get_config_path()?;
    
    if config_path.exists() {
        fs::remove_file(&config_path)
            .context("Failed to delete config file")?;
    }

    Ok(())
}
