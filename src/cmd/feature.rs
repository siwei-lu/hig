use crate::{
    git::{self, branch},
    Config,
};
use anyhow::Result;

pub fn run(name: &str) -> Result<()> {
    let repo = git::repo::current()?;
    let conf = Config::load(&repo);
    let branch_name = new_branch_name(&conf.data.feature.prefix, name);

    if !branch::is_exist_in_repo(&branch_name, &repo) {
        branch::new_in_repo(&branch_name, &repo)?;
    }

    branch::checkout_in_repo(&branch_name, &repo)?;
    Ok(())
}

fn new_branch_name(prefix: &str, name: &str) -> String {
    format!("{prefix}{name}")
}
