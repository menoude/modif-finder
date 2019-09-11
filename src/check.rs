use crate::error::ModifError;
use crate::Result;

use git2::{Commit, Repository};

pub fn check_modifs<'a>(repo: &Repository, last: Commit<'a>, reference: Commit<'a>) -> Result<()> {
	let old_tree = reference.tree()?;
	let new_tree = last.tree()?;
	let diff = repo.diff_tree_to_tree(Some(&old_tree), Some(&new_tree), None)?;
	for delta in diff.deltas() {
		println!(
			"old file {:?} and new file {:?}",
			delta.old_file().path(),
			delta.new_file().path()
		);
	}
	Ok(())
}
