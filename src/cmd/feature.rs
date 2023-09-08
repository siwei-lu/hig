use clap::{arg, error::Result, Command};
use git2::Error;

use crate::git::branch;

pub const NAME: &str = "feature";

pub fn new() -> Command {
    Command::new(NAME)
        .about("Create a new feature branch")
        .alias("feat")
        .arg_required_else_help(true)
        .arg(arg!(<name> "The name of the feature branch"))
}

pub fn run(name: &str) -> Result<(), Error> {
    let branch_name = new_branch_name(name);

    if !branch::is_exist(&branch_name) {
        branch::new(&branch_name)?;
    }

    branch::checkout(&branch_name)?;
    Ok(())
}

fn new_branch_name(name: &str) -> String {
    format!("feature/NEUT-{}", name)
}
