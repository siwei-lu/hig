use std::fmt::Display;

use crate::{config::ConfigType, git::repository::RepoExt, Config};
use anyhow::Result;
use git2::Repository;

pub fn run(key: &str, value: &Option<String>, t: ConfigType) -> Result<()> {
    let repo = Repository::current()?;
    let mut conf = match t {
        ConfigType::Local => Config::load_local(&repo),
        ConfigType::Global => Config::load_global(&repo),
    };

    match key {
        "feature.prefix" => handle(&mut conf.data.feature.prefix, value),
        "feature.upstream" => handle(&mut conf.data.feature.upstream, value),
        "feature.base" => handle(&mut conf.data.feature.base, value),
        "hotfix.prefix" => handle(&mut conf.data.hotfix.prefix, value),
        "hotfix.upstream" => handle(&mut conf.data.hotfix.upstream, value),
        "hotfix.base" => handle(&mut conf.data.hotfix.base, value),
        "release.prefix" => handle(&mut conf.data.release.prefix, value),
        "release.upstream" => handle(&mut conf.data.release.upstream, value),
        "release.base" => handle(&mut conf.data.release.base, value),
        _ => {
            return Ok(());
        }
    }

    conf.save()
}

fn handle<T: Clone + Display>(data: &mut Option<T>, value: &Option<T>) {
    if let Some(value) = value {
        *data = Some(value.clone());
        return;
    }

    if let Some(data) = data {
        println!("{}", data);
    }
}
