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
		match (delta.old_file().path(), delta.new_file().path()) {
			(Some(old_path), Some(new_path)) if old_path != new_path => {
				changed_files.push(old_path);
				changed_files.push(new_path);
			}
			(Some(_), Some(new_path)) => changed_files.push(new_path),
			(Some(path), _) => changed_files.push(path),
			(_, Some(path)) => changed_files.push(path),
			(_, _) => panic!(
				"Can't find the delta files for {} vs {}",
				last.id(),
				reference.id()
			),
		};
	}
	find_package(changed_files)?;
	Ok(())
}

fn find_package(modified_files: Vec<&Path>) -> Result<()> {
	for file in modified_files {
		println!("{:?}", file);
	}
	Ok(())
}
