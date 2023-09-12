pub mod data;

use self::data::Data;
use anyhow::Result;
use git2::Repository;
use std::{fs, path::PathBuf};

pub enum BranchType {
    Feature,
    Release,
    Hotfix,
}

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

    pub fn new_branch_name(&self, name: &str, t: BranchType) -> String {
        let prefix = self.get_prefix(t);

        match prefix {
            Some(prefix) => format!("{}/{}", prefix, name),
            None => name.to_string(),
        }
    }

    fn get_prefix(&self, t: BranchType) -> Option<String> {
        match t {
            BranchType::Feature => self.data.feature.prefix.to_owned(),
            // BranchType::Release => &self.data.release.prefix,
            // BranchType::Hotfix => &self.data.hotfix.prefix,
            _ => None,
        }
    }

    fn path(&self) -> PathBuf {
        Self::path_in_repo(self.repo)
    }

    fn path_in_repo(repo: &Repository) -> PathBuf {
        repo.path().join("hig.config")
    }
}
