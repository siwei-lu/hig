use std::error::Error;

use clap::{arg, Command};

use crate::Config;

pub const NAME: &str = "config";

pub fn new() -> Command {
    Command::new(NAME)
        .about("Configure the tool")
        .arg_required_else_help(true)
        .arg(arg!(<key> "The key to set"))
        .arg(arg!(<value> "The value to set"))
}

pub fn run(key: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let mut conf = Config::load();

    match key {
        "feature.prefix" => {
            conf.feature.prefix = value.to_string();
        }
        _ => return Ok(()),
    }

    conf.save()
}
