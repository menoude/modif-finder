use crate::error::ModifError;
use crate::Result;

use serde_json::{from_reader, Value};

use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use walkdir::{DirEntry, WalkDir};

pub fn create_dep_map<P: AsRef<Path>>(working_dir: P) -> Result<HashMap<String, Vec<String>>> {
	let package_files = WalkDir::new(working_dir)
		.into_iter()
		.filter_entry(|entry| entry.file_type().is_dir() || entry.file_name() == "package.json");

	let mut dependency_map = HashMap::new();

	for entry in package_files {
		let safe_entry = entry?;
		if safe_entry.file_name() == "package.json" {
			let path = safe_entry.path();
			let package_path = path
				.parent()
				.and_then(|path| path.to_str())
				.unwrap()
				.to_owned();
			let dependant_packages = parse_dependendencies(path)?;
			dependency_map.insert(package_path, dependant_packages);
		}
	}
	println!("Package dependances: ");
	for entry in dependency_map.iter() {
		println!("{} depends on {:?}", entry.0, entry.1);
	}
	println!();
	Ok(dependency_map)
}

fn parse_dependendencies(config_path: &Path) -> Result<Vec<String>> {
	let file_reader = File::open(config_path)?;
	let file_content: Value = from_reader(file_reader)?;
	let mut deps = Vec::new();
	match file_content["dependencies"].as_object() {
		Some(map) => {
			for key in map.keys() {
				deps.push(map[key].as_str().unwrap().to_owned())
			}
			Ok(deps)
		}
		None => Ok(vec![]),
	}
}
