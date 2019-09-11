use crate::error::ModifError;
use crate::Result;

use git2::{Commit, Repository};
use std::path::Path;

pub fn check_modifs<'a>(repo: &Repository, last: Commit<'a>, reference: Commit<'a>) -> Result<()> {
	let old_tree = reference.tree()?;
	let new_tree = last.tree()?;
	let diff = repo.diff_tree_to_tree(Some(&old_tree), Some(&new_tree), None)?;
	let mut changed_files = Vec::new();
	for delta in diff.deltas() {
		if let Some(path) = delta.new_file().path() {
			changed_files.push(path);
		}
	}
	find_packets(changed_files)?;
	Ok(())
}

fn find_packets(modified_files: Vec<&Path>) -> Result<()> {
	for file in modified_files {
		println!("{:?}", file);
	}
	Ok(())
}
