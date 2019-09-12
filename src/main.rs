pub mod args;
pub mod check;
pub mod dependencies;
pub mod error;

use args::get_args;
use git2::Repository;
use std::env::current_dir;
use std::process::exit;

type Result<T> = std::result::Result<T, error::ModifError>;

fn main() {
    let working_dir = current_dir().unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });

    let repo = Repository::open(&working_dir).unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });
    let map = dependencies::create_dep_map(&working_dir).unwrap_or_else(|e| {
        println!("{}", e);
        exit(1);
    });
    get_args(&repo)
        .and_then(|(last, reference)| check::check_modifs(&repo, last, reference))
        .unwrap_or_else(|e| {
            println!("{}", e);
            exit(1);
        })
}
