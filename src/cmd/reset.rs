use crate::{git::repository::RepoExt, Config};
use anyhow::{Ok, Result};
use git2::Repository;

pub fn run() -> Result<()> {
    let repo = Repository::current()?;
    let master = repo.master()?;
    let mut head = repo.head_branch()?;
    let head_name = head.name()?.unwrap().to_string();

    repo.checkout(&master)?;
    head.delete()?;

    let mut head = repo.new_branch(&head_name)?;
    repo.checkout(&head)?;

    let conf = Config::load(&repo);
    if let Some(upstream) = conf.data.feature.upstream {
        head.set_upstream(Some(&upstream))?;
    }

    Ok(())
}
