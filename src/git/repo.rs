use git2::{Repository, Error};

pub fn current() -> Result<Repository, Error> {
  Repository::discover(".")
}