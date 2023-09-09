use std::{error::Error, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct FeatureConfig {
    prefix: String,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    feature: FeatureConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            feature: FeatureConfig {
                prefix: "feature/".to_string(),
            },
        }
    }

    fn path() -> PathBuf {
        PathBuf::from("./.git/hig.config")
    }

    pub fn load() -> Option<Self> {
        let path = Self::path();
        if !path.exists() {
            return None;
        }

        let content = std::fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let content = toml::to_string(self)?;
        std::fs::write(Self::path(), content)?;

        Ok(())
    }
}
