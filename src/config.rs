use std::{error::Error, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FeatureConfig {
    pub prefix: String,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub feature: FeatureConfig,
}

impl Config {
    fn new() -> Self {
        Self {
            feature: FeatureConfig {
                prefix: "feature/".to_string(),
            },
        }
    }

    fn path() -> PathBuf {
        PathBuf::from("./.git/hig.config")
    }

    pub fn load() -> Self {
        let path = Self::path();
        if !path.exists() {
            return Self::new();
        }

        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return Self::new(),
        };

        match toml::from_str(&content) {
            Ok(config) => config,
            Err(_) => Self::new(),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let content = toml::to_string(self)?;
        fs::write(Self::path(), content)?;
        Ok(())
    }
}
