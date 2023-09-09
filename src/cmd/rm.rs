use std::error::Error;

use clap::Command;

use crate::git::branch;

pub const NAME: &str = "remove";

pub fn new() -> Command {
    Command::new("rm").about("Remove the current branch")
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let current = branch::current().ok_or(git2::Error::from_str("No current branch"))?;
    let main_branch = branch::main();

    branch::checkout(&main_branch)?;
    branch::remove(&current)?;

    Ok(())
}