pub mod data;

use self::data::Data;
use anyhow::Result;
use git2::Repository;
use std::{fs, path::PathBuf};

pub struct Config<'a> {
    repo: &'a Repository,
    pub data: Data,
}

impl<'a> Config<'a> {
    pub fn load(repo: &'a Repository) -> Self {
        let path = Self::path_in_repo(repo);

        if !path.exists() {
            return Self {
                repo,
                data: data::Data::default(),
            };
        }

        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => {
                return Self {
                    repo,
                    data: data::Data::default(),
                }
            }
        };

        toml::from_str(&content)
            .map(|data| Self { repo, data })
            .unwrap_or(Self {
                repo,
                data: data::Data::default(),
            })
    }

    pub fn save(&self) -> Result<()> {
        let content = toml::to_string(&self.data)?;
        fs::write(self.path(), content).map_err(Into::into)
    }

    fn path(&self) -> PathBuf {
        Self::path_in_repo(self.repo)
    }

    fn path_in_repo(repo: &Repository) -> PathBuf {
        repo.path().join("hig.config")
    }
}
