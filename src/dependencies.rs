use crate::Result;

use std::collections::HashMap;
use std::path::Path;

use walkdir::{DirEntry, WalkDir};

pub fn create_dep_map<P: AsRef<Path>>(working_dir: P) -> Result<HashMap<String, Vec<String>>> {
	let configs = WalkDir::new(working_dir).into_iter().filter_entry(|e| {
		e.file_name()
			.to_str()
			.map(|s| s == "package.json")
			.unwrap_or(false)
	});

	let mut dep_map = HashMap::new();
	for config in configs {
		if let Ok(file) = config {
			parse_dependendencies(file, &mut dep_map)?;
		}
	}
	Ok(dep_map)
}

fn parse_dependendencies(
	package_entry: DirEntry,
	deps_map: &mut HashMap<String, Vec<String>>,
) -> Result<()> {
	println!("{:?}", package_entry.file_name());
	Ok(())
}
