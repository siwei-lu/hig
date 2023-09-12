use std::sync::Arc;

use anyhow::Result;

pub struct Github {
    instance: Arc<octocrab::Octocrab>,
}

impl Github {
    pub fn new() -> Self {
        Self {
            instance: octocrab::instance(),
        }
    }
}
