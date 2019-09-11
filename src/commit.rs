use crate::error::ModifError;
use crate::Result;
use git2::{Commit, Oid, Repository};

pub fn valid_commit<'a>(candidate: &str, repo: &'a Repository) -> Result<Commit<'a>> {
    let commit = repo.find_commit(Oid::from_str(candidate)?)?;
    Ok(commit)
}
