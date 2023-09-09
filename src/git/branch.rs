use git2::{BranchType, Error};

use super::repo;

pub fn main() -> String {
    let repo = match repo::current() {
        Ok(repo) => repo,
        Err(_) => return "master".to_string(),
    };

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
            return reference.shorthand().unwrap_or("master").to_string();
        }
    }

    "master".to_string()
}

pub fn new(name: &str) -> Result<(), Error> {
    let repo = repo::current()?;
    let target = repo.head()?.peel_to_commit()?;

    repo.branch(name, &target, false)?;
    Ok(())
}

pub fn checkout(name: &str) -> Result<(), Error> {
    let repo = repo::current()?;
    let branch_ref = format!("refs/heads/{}", name);
    repo.set_head(&branch_ref)?;

    let mut builder = git2::build::CheckoutBuilder::new();
    builder.force();
    repo.checkout_head(Some(&mut builder))?;

    Ok(())
}

pub fn current() -> Option<String> {
    let repo = repo::current().ok()?;
    let head = repo.head().ok()?;
    head.shorthand().map(|s| s.to_string())
}

pub fn remove(name: &str) -> Result<(), Error> {
    let repo = repo::current()?;
    let mut branch = repo.find_branch(name, BranchType::Local)?;
    branch.delete()
}

pub fn is_exist(name: &str) -> bool {
    let repo = match repo::current() {
        Ok(it) => it,
        Err(_) => return false,
    };

    let branch = repo.find_branch(name, BranchType::Local);
    branch.is_ok()
}
