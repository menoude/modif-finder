pub mod args;
pub mod check;
pub mod commit;
pub mod dependencies;
pub mod error;

use args::get_args;
use git2::Repository;
use std::process::exit;

type Result<T> = std::result::Result<T, error::ModifError>;

fn main() {
    let repo = match Repository::open("../working_repository")
        .map_err(|e| error::ModifError::NoRepo(format!("{}", e)))
    {
        Ok(repository) => repository,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };
    let map = match dependencies::create_dep_map("../working_repository") {
        Ok(map) => map,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };
    match get_args(&repo) {
        Ok((last, reference)) => {
            if let Err(e) = check::check_modifs(&repo, last, reference) {
                println!("{}", e);
                exit(1);
            }
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };
}
