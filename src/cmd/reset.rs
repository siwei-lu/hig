use crate::git::branch;
use anyhow::Result;

pub fn run() -> Result<()> {
    let current = branch::current().ok_or(git2::Error::from_str("No current branch"))?;
    let main_branch = branch::main();

    branch::checkout(&main_branch)?;
    branch::remove(&current)?;
    branch::new(&current)?;
    branch::checkout(&current)?;

    Ok(())
}
