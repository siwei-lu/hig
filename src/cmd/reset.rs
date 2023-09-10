use crate::git::repository::RepoExt;
use anyhow::Result;
use git2::Repository;

pub fn run() -> Result<()> {
    let repo = Repository::current()?;
    let master = repo.master()?;
    let mut head = repo.head_branch()?;
    let head_name = head.name()?.unwrap().to_string();

    repo.checkout(&master)?;
    head.delete()?;

    let head = repo.new_branch(&head_name)?;
    repo.checkout(&head)
}
