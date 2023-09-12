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

pub enum ConfigType {
    Local,
    Global,
}

pub struct Config<'a> {
    t: ConfigType,
    repo: &'a Repository,
    pub data: Data,
}

impl<'a> Config<'a> {
    pub fn load(repo: &'a Repository) -> Self {
        let local = Self::load_local(repo);
        let global = Self::load_global(repo);

        Self {
            t: ConfigType::Local,
            repo,
            data: local.data.merge(&global.data),
        }
    }

    pub fn load_local(repo: &'a Repository) -> Self {
        let path = Self::path_in_repo(repo);
        let data = Self::load_data_from_path(&path);

        Self {
            t: ConfigType::Local,
            repo,
            data,
        }
    }

    pub fn load_global(repo: &'a Repository) -> Self {
        let path = Self::path_global();
        let data = Self::load_data_from_path(&path);

        Self {
            t: ConfigType::Global,
            repo,
            data,
        }
    }

    fn load_data_from_path(path: &PathBuf) -> Data {
        fs::read_to_string(path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or(Data::default())
    }

    pub fn save(&self) -> Result<()> {
        let path = match self.t {
            ConfigType::Local => self.path(),
            ConfigType::Global => Self::path_global(),
        };

        let content = toml::to_string(&self.data)?;
        fs::write(path, content)?;

        Ok(())
    }

    pub fn new_branch_name(&self, name: &str, t: BranchType) -> String {
        let prefix = self.get_prefix(t);

        match prefix {
            Some(prefix) => format!("{}{}", prefix, name),
            None => name.to_string(),
        }
    }

    fn get_prefix(&self, t: BranchType) -> Option<String> {
        match t {
            BranchType::Feature => self.data.feature.prefix.to_owned(),
            BranchType::Release => self.data.release.prefix.to_owned(),
            BranchType::Hotfix => self.data.hotfix.prefix.to_owned(),
        }
    }

    fn path(&self) -> PathBuf {
        Self::path_in_repo(self.repo)
    }

    fn path_in_repo(repo: &Repository) -> PathBuf {
        repo.path().join("hig.config")
    }

    fn path_global() -> PathBuf {
        dirs::home_dir()
            .expect("Cannot find the home directory")
            .join(".hig.config")
    }
}
