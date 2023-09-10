use anyhow::Result;
use git2::Repository;

use crate::git::repository::RepoExt;

pub fn run() -> Result<()> {
    let repo = Repository::current()?;
    let master = repo.master()?;
    let mut head = repo.head_branch()?;

    repo.checkout(&master)?;
    head.delete().map_err(Into::into)
}
