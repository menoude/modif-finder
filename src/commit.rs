use crate::error::ModifError;
use crate::Result;

pub enum Commit {
	Head,
	Hash(String),
}

impl Commit {
	pub fn new(candidate: Option<&str>) -> Result<Self> {
		let commit = match candidate {
			Some(hash) => {
				Self::valid_hash(hash)?;
				Commit::Hash(hash.to_owned())
			}
			None => Commit::Head,
		};
		Ok(commit)
	}

	pub fn valid_hash(candidate: &str) -> Result<()> {
		unimplemented!()
	}
}
