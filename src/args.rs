use clap::{App, Arg};

use crate::{commit::Commit, error::ModifError, Result};

pub fn get_args() -> Result<(Commit, Commit)> {
    let matches = App::new("Modif-finder")
        .version("1.0")
        .author("menoude")
        .about("Find which packets should you redeploy")
        .arg(
            Arg::with_name("first-commit")
                .short("c1")
                .long("commit1")
                .value_name("COMMIT1")
                .help("Sets the first compare commit")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("second-commit")
                .short("c2")
                .long("commit2")
                .value_name("COMMIT2")
                .help("Sets the second compare commit")
                .takes_value(true),
        )
        .get_matches();

    let first = match matches.value_of("first-commit") {
        Some(param) => Commit::new(Some(param))?,
        None => Commit::new(None).unwrap(),
    };

    let second = match matches.value_of("second-commit") {
        Some(param) => Commit::new(Some(param))?,
        None => Commit::new(None).unwrap(),
    };

    Ok((first, second))
}
