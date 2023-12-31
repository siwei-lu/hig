use git2::Repository;
use nanoid::nanoid;
use std::{
    env::temp_dir,
    fs,
    panic::{catch_unwind, UnwindSafe},
    path::PathBuf,
};

fn random_path() -> PathBuf {
    temp_dir().join(nanoid!())
}

fn make_dir_if_not_exists(path: &PathBuf) {
    if !path.exists() {
        fs::create_dir_all(path).unwrap();
    }
}

fn random_repo() -> Repository {
    let path = random_path();
    make_dir_if_not_exists(&path);
    Repository::init(&path).unwrap()
}

fn clear_repo(repo: Repository) {
    let path = repo.path().parent().unwrap();
    fs::remove_dir_all(path).unwrap();
}

pub fn repo<F: FnOnce(&Repository) + UnwindSafe>(runner: F) {
    let repo = random_repo();
    let result = catch_unwind(|| runner(&repo));
    clear_repo(repo);
    assert!(result.is_ok());
}
