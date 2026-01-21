use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from: String,
    pub to: String,
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub smtp: SmtpConfig,
    pub default_message: String,
}

pub fn load_config(path: Option<&Path>) -> Result<GlobalConfig> {
    let config_path = path
        .map(PathBuf::from)
        .or_else(default_config_path)
        .context("no config path provided and no default path available")?;

    if !config_path.exists() {
        anyhow::bail!(
            "config file not found at {}",
            config_path.display()
        );
    }

    let raw = fs::read_to_string(&config_path)
        .with_context(|| format!("failed to read config file at {}", config_path.display()))?;
    let config: GlobalConfig = toml::from_str(&raw)
        .with_context(|| format!("invalid config file at {}", config_path.display()))?;
    Ok(config)
}

fn default_config_path() -> Option<PathBuf> {
    if cfg!(windows) {
        env::var_os("APPDATA").map(|base| PathBuf::from(base).join("task-notify.toml"))
    } else {
        env::var_os("HOME").map(|base| PathBuf::from(base).join(".config/task-notify.toml"))
    }
}
