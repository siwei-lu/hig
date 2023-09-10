use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("not a git repository (or any of the parent directories): .git")]
    NotRepository,

    #[error("cannot find the main branch")]
    NoMainBranch,

    #[error("head is not a branch")]
    HeadIsNotBranch,
}
