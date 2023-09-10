use crate::{git::repository::RepoExt, Config};
use anyhow::Result;
use git2::{BranchType, Repository};

pub fn run(name: &str) -> Result<()> {
    let repo = Repository::current()?;
    let conf = Config::load(&repo);
    let branch_name = new_branch_name(&conf.data.feature.prefix, name);

    let branch = match repo.find_branch(&branch_name, BranchType::Local) {
        Ok(branch) => branch,
        Err(_) => repo.new_branch(&branch_name)?,
    };

    repo.checkout(&branch)
}

fn new_branch_name(prefix: &str, name: &str) -> String {
    format!("{prefix}{name}")
}
