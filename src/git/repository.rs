use anyhow::Result;
use git2::{Branch, BranchType, Repository};

use crate::ApplicationError;

pub trait RepoExt {
    fn current() -> Result<Repository>;
    fn master(&self) -> Result<Branch>;
    fn head_branch(&self) -> Result<Branch>;
    fn new_branch(&self, name: &str) -> Result<Branch>;
    fn is_branch_exist(&self, name: &str) -> bool;
    fn checkout(&self, branch: &Branch) -> Result<()>;
}

impl RepoExt for Repository {
    fn current() -> Result<Repository> {
        Repository::discover(".").map_err(|_| ApplicationError::NotRepository.into())
    }

    fn master(&self) -> Result<Branch> {
        const BRANCH_NAMES: [&str; 7] = [
            "main",
            "trunk",
            "origin/main",
            "origin/trunk",
            "upstream/main",
            "upstream/trunk",
            "master",
        ];

        for name in BRANCH_NAMES.iter() {
            if let Ok(branch) = self.find_branch(name, BranchType::Local) {
                return Ok(branch);
            }
        }

        Err(ApplicationError::NoMainBranch.into())
    }

    fn head_branch(&self) -> Result<Branch> {
        let head = self.head()?;
        let result = head.shorthand();
        result
            .and_then(|name| self.find_branch(name, BranchType::Local).ok())
            .ok_or(ApplicationError::HeadIsNotBranch.into())
    }

    fn new_branch(&self, name: &str) -> Result<Branch> {
        let target = self.head()?.peel_to_commit()?;
        self.branch(name, &target, false).map_err(Into::into)
    }

    fn is_branch_exist(&self, name: &str) -> bool {
        self.find_branch(name, BranchType::Local).is_ok()
    }

    fn checkout(&self, branch: &Branch) -> Result<()> {
        let branch_name = branch
            .name()?
            .map(|v| format!("refs/heads/{}", v))
            .ok_or(ApplicationError::HeadIsNotBranch)?;

        let mut opts = git2::build::CheckoutBuilder::new();
        opts.force();

        self.set_head(&branch_name)?;
        self.checkout_head(Some(&mut opts)).map_err(Into::into)
    }
}
