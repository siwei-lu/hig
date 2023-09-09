use crate::Config;
use anyhow::Result;

pub fn run(key: &str, value: &str) -> Result<()> {
    let mut conf = Config::load();

    match key {
        "feature.prefix" => {
            conf.feature.prefix = value.to_string();
        }
        _ => return Ok(()),
    }

    conf.save()
}
