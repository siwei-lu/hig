use anyhow::Result;
use git2::Repository;

use crate::{
    git::{self, repository::RepoExt},
    Config,
};

pub fn run() -> Result<()> {
    let repo = Repository::current()?;
    let master = repo.master()?;
    let mut head = repo.head_branch()?;

    let target = {
        let conf = Config::load(&repo);
        let base = conf.data.feature.base;

        match base {
            Some(base) if &base == head.name()?.unwrap_or("") => master,
            Some(base) => repo.find_branch(&base, git2::BranchType::Local)?,
            None => master,
        }
    };

    repo.checkout(&target)?;
    head.delete()?;

    git::ggu()?;
    Ok(())
}
