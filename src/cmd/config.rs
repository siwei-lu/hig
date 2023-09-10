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
        "feature.upstream" => {
            handle_option(&mut conf.data.feature.upstream, value);
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

fn handle_option<T: Clone + Display>(data: &mut Option<T>, value: &Option<T>) {
    if let Some(value) = value {
        *data = Some(value.clone());
        return;
    }

    if let Some(data) = data {
        println!("{}", data);
    }
}
