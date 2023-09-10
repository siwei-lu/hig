use crate::git::{branch, repo};
use anyhow::Result;

pub fn run() -> Result<()> {
    let repo = repo::current()?;
    let current =
        branch::current_in_repo(&repo).ok_or(git2::Error::from_str("No current branch"))?;

    let main_branch = branch::main_of_repo(&repo);

    branch::checkout_in_repo(&main_branch, &repo)?;
    branch::remove_from_repo(&current, &repo)?;
    branch::new_in_repo(&current, &repo)?;
    branch::checkout_in_repo(&current, &repo)?;

    Ok(())
}
