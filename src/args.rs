use clap::{App, Arg};

use crate::Result;
use git2::{Commit, Oid, Repository};

pub fn get_args<'a>(repo: &'a Repository) -> Result<(Commit<'a>, Commit<'a>)> {
    let matches = App::new("Modif-finder")
        .version("1.0")
        .author("menoude")
        .about("Find which packets should you redeploy")
        .arg(
            Arg::with_name("last-commit")
                .short("l")
                .long("last-commit")
                .value_name("LAST")
                .help("Sets the last change to compare")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("reference")
                .short("r")
                .long("reference")
                .value_name("REFERENCE")
                .required(true)
                .help("Sets the reference commit")
                .takes_value(true),
        )
        .get_matches();

    let first = match matches.value_of("last-commit") {
        Some(param) => valid_commit(param, repo)?,
        None => repo.head()?.peel_to_commit()?,
    };

    let second = match matches.value_of("reference") {
        Some(param) => valid_commit(param, repo)?,
        None => panic!(),
    };

    Ok((first, second))
}

pub fn valid_commit<'a>(candidate: &str, repo: &'a Repository) -> Result<Commit<'a>> {
    let commit = repo.find_commit(Oid::from_str(candidate)?)?;
    Ok(commit)
}
