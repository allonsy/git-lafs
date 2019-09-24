use git2::Repository;
use std::path::PathBuf;

pub fn get_repo() -> Result<Repository, git2::Error> {
    Repository::open(".")
}

pub fn get_lafs_objects_folder(repo: &Repository) -> PathBuf {
    get_lafs_folder(repo).join("objects")
}

pub fn get_lafs_folder(repo: &Repository) -> PathBuf {
    repo.path().join("lafs")
}
