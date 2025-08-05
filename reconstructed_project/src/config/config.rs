use anyhow::{Context, Result};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use vibestream::app::Settings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub config_path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_dir = config_dir()
            .context("Failed to get config directory")?
            .join("vibestream");

        fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;

        let config_path = config_dir.join("config.json");

        if config_path.exists() {
            let config_str = fs::read_to_string(&config_path)
                .context("Failed to read config file")?;

            let settings: Settings = serde_json::from_str(&config_str)
                .context("Failed to parse config file")?;

            Ok(Self {
                settings,
                config_path,
            })
        } else {
            let settings = Settings::default();
            let config = Self {
                settings: settings.clone(),
                config_path: config_path.clone(),
            };

            config.save()?;

            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_str = serde_json::to_string_pretty(&self.settings)
            .context("Failed to serialize config")?;

        fs::write(&self.config_path, config_str)
            .context("Failed to write config file")?;

        Ok(())
    }
}
