use anyhow::Result;
use git2::Repository;

pub fn current() -> Result<Repository> {
    Repository::discover(".").map_err(Into::into)
}
