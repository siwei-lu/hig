use crate::{config::BranchType, git::repository::RepoExt, Config};
use anyhow::Result;
use git2::Repository;

pub fn run(name: &str) -> Result<()> {
    let repo = Repository::current()?;
    let conf = Config::load(&repo);
    let branch_name = conf.new_branch_name(name, BranchType::Feature);

    let mut branch = match repo.find_branch(&branch_name, git2::BranchType::Local) {
        Ok(branch) => branch,
        Err(_) => repo.new_branch(&branch_name)?,
    };

    repo.checkout(&branch)?;

    if let Some(upstream) = conf.data.feature.upstream {
        branch.set_upstream(Some(&upstream))?;
    }

    Ok(())
}
