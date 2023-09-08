use clap::Command;
use git2::Error;

use crate::git::branch;

pub const NAME: &str = "remove";

pub fn new() -> Command {
    Command::new("remove")
        .alias("rm")
        .about("Remove the current branch")
}

pub fn run() -> Result<(), Error> {
    let current = branch::current()?;
    let main_branch = branch::main();

    branch::checkout(&main_branch)?;
    branch::remove(&current)?;

    Ok(())
}
