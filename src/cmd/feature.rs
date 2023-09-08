use clap::{arg, error::Result, Command};
use git2::Error;

use crate::git;

pub const NAME: &str = "feature";

pub fn new() -> Command {
    Command::new(NAME)
        .about("Create a new feature branch")
        .arg_required_else_help(true)
        .arg(arg!(<name> "The name of the feature branch"))
}

pub fn run(name: &str) -> Result<(), Error> {
    let branch_name = new_branch_name(name);
    git::branch::new(&branch_name)?;
    git::branch::checkout(&branch_name)?;
    
    Ok(())
}

fn new_branch_name(name: &str) -> String {
    format!("feature/NEUT-{}", name)
}
