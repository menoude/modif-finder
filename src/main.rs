pub mod args;
pub mod commit;
pub mod error;

use args::get_args;
use std::process::exit;

type Result<T> = std::result::Result<T, error::ModifError>;

fn main() {
    if let Err(e) = get_args() {
        println!("{}", e);
        exit(1);
    }
    println!("Hello, world!");
}
