use std::process::Command;

use anyhow::{Ok, Result};

pub fn ggu() -> Result<()>  {
    Command::new("git")
        .arg("pull")
        .arg("--rebase")
        .output()?;

    Ok(())
}