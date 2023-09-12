use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Branch {
    pub prefix: Option<String>,
    pub upstream: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub feature: Branch,
    pub hotfix: Branch,
    pub release: Branch,
}

impl Data {
    pub fn default() -> Self {
        Self {
            feature: Branch {
                prefix: None,
                upstream: None,
            },
            hotfix: Branch {
                prefix: None,
                upstream: None,
            },
            release: Branch {
                prefix: None,
                upstream: None,
            },
        }
    }
}
