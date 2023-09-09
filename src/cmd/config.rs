use std::fmt::Display;

use crate::Config;
use anyhow::Result;

pub fn run(key: &str, value: &Option<String>) -> Result<()> {
    let mut conf = Config::load();

    match key {
        "feature.prefix" => {
            handle(&mut conf.feature.prefix, value);
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
