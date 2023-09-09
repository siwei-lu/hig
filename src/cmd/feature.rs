use crate::{git::branch, Config};
use anyhow::Result;

pub fn run(name: &str) -> Result<()> {
    let conf = Config::load();
    let branch_name = new_branch_name(&conf.feature.prefix, name);

    if !branch::is_exist(&branch_name) {
        branch::new(&branch_name)?;
    }

    branch::checkout(&branch_name)?;
    Ok(())
}

fn new_branch_name(prefix: &str, name: &str) -> String {
    format!("{prefix}{name}")
}
