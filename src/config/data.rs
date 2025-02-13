use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Branch {
    pub prefix: Option<String>,
    pub upstream: Option<String>,
    pub base: Option<String>,
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
                base: None,
            },
            hotfix: Branch {
                prefix: None,
                upstream: None,
                base: None,
            },
            release: Branch {
                prefix: None,
                upstream: None,
                base: None,
            },
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            feature: Branch {
                prefix: self.feature.prefix.clone().or(other.feature.prefix.clone()),
                upstream: self
                    .feature
                    .upstream
                    .clone()
                    .or(other.feature.upstream.clone()),
                base: self.feature.base.clone().or(other.feature.base.clone()),
            },
            hotfix: Branch {
                prefix: self.hotfix.prefix.clone().or(other.hotfix.prefix.clone()),
                upstream: self
                    .hotfix
                    .upstream
                    .clone()
                    .or(other.hotfix.upstream.clone()),
                base: self.feature.base.clone().or(other.feature.base.clone()),
            },
            release: Branch {
                prefix: self.release.prefix.clone().or(other.release.prefix.clone()),
                upstream: self
                    .release
                    .upstream
                    .clone()
                    .or(other.release.upstream.clone()),
                base: self.feature.base.clone().or(other.feature.base.clone()),
            },
        }
    }
}
