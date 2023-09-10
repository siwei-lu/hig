use anyhow::Result;
use git2::{BranchType, Repository};

pub fn main_of_repo(repo: &Repository) -> String {
    const REFS_PATTERNS: [&str; 6] = [
        "refs/heads/main",
        "refs/heads/trunk",
        "refs/remotes/origin/main",
        "refs/remotes/origin/trunk",
        "refs/remotes/upstream/main",
        "refs/remotes/upstream/trunk",
    ];

    for pattern in REFS_PATTERNS.iter() {
        if let Ok(reference) = repo.find_reference(pattern) {
            let name = reference.shorthand().unwrap_or("master");
            return name.to_string();
        }
    }

    "master".to_string()
}

pub fn new_in_repo(name: &str, repo: &Repository) -> Result<()> {
    let target = repo.head()?.peel_to_commit()?;
    repo.branch(name, &target, false)?;
    Ok(())
}

pub fn checkout_in_repo(name: &str, repo: &Repository) -> Result<()> {
    let branch_ref = format!("refs/heads/{}", name);
    repo.set_head(&branch_ref)?;

    let mut builder = git2::build::CheckoutBuilder::new();
    builder.force();
    repo.checkout_head(Some(&mut builder))?;

    Ok(())
}

pub fn current_in_repo(repo: &Repository) -> Option<String> {
    let head = repo.head().ok()?;
    let result = head.shorthand();
    result.map(Into::into)
}

pub fn remove_from_repo(name: &str, repo: &Repository) -> Result<()> {
    let mut branch = repo.find_branch(name, BranchType::Local)?;
    branch.delete().map_err(Into::into)
}

pub fn is_exist_in_repo(name: &str, repo: &Repository) -> bool {
    let branch = repo.find_branch(name, BranchType::Local);
    branch.is_ok()
}
