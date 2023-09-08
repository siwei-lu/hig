use clap::Command;
use git2::Error;

use crate::git::branch;

pub const NAME: &str = "reset";

pub fn new() -> Command {
    Command::new(NAME).about(
        "Reset the current branch, usually used after the branch is merged to main by squash merge",
    )
}

pub fn run() -> Result<(), Error> {
    let current = branch::current().ok_or(Error::from_str("No current branch"))?;
    let main_branch = branch::main();

    branch::checkout(&main_branch)?;
    branch::remove(&current)?;
    branch::new(&current)?;
    branch::checkout(&current)?;

    Ok(())
}
