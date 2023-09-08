use git2::Error;

use super::repo;


pub fn main() -> String {
    "main".to_string()
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
