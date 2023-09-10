use std::fmt::Display;

use crate::{git::repo, Config};
use anyhow::Result;

pub fn run(key: &str, value: &Option<String>) -> Result<()> {
    let repo = repo::current()?;
    let mut conf = Config::load(&repo);

    match key {
        "feature.prefix" => {
            handle(&mut conf.data.feature.prefix, value);
        }
        _ => return Ok(()),
    }

    conf.save()
}

fn handle<T: Clone + Display>(data: &mut T, value: &Option<T>) {
    if let Some(value) = value {
        *data = value.clone();
        return;
    }

    println!("{}", data)
}
