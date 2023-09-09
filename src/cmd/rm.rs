use crate::git::branch;
use anyhow::Result;
use clap::Command;

pub const NAME: &str = "remove";

pub fn new() -> Command {
    Command::new("rm").about("Remove the current branch")
}

pub fn run() -> Result<()> {
    let current = branch::current().ok_or(git2::Error::from_str("No current branch"))?;
    let main_branch = branch::main();

    branch::checkout(&main_branch)?;
    branch::remove(&current)?;

    Ok(())
}
