use clap::{arg, Command};

pub const NAME: &str = "feature";

pub fn new() -> Command {
    Command::new(NAME)
        .about("Create a new feature branch")
        .arg_required_else_help(true)
        .arg(arg!(<name> "The name of the feature branch"))
}

pub fn run(name: &str) {
    println!("Creating a new feature branch: {name}")
}
