use anyhow::{anyhow, Result};
use git2::Repository;

use crate::{git::repository::RepoExt, platform::Github};

pub async fn run() -> Result<()> {
    let repo = Repository::current()?;
    let origin = repo.find_remote("origin")?;

    let (owner, repo_name) = {
        let url = origin
            .url()
            .ok_or(anyhow::anyhow!("No url of remote origin"))?;

        let mut iter = url.split('/').skip(3);
        let owner = iter.next().ok_or(anyhow::anyhow!("No owner"))?;
        let repo_name = iter
            .next()
            .and_then(|s| s.split('.').next())
            .ok_or(anyhow::anyhow!("No repo name"))?;

        (owner, repo_name)
    };

    let head = repo.head_branch()?;
    let title = head.name()?.ok_or(anyhow!("No branch name"))?;

    octocrab::instance()
        .pulls(owner, repo_name)
        .create("Test Pr", "hig:main", "main")
        .send()
        .await?;

    Ok(())
}
