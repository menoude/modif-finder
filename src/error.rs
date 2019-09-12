use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum ModifError {
    CommitError(git2::Error),
    NoRepo(git2::Error),
    IoError(std::io::Error),
    ParseError(serde_json::Error),
    DirError(walkdir::Error),
}

impl std::error::Error for ModifError {}

impl Display for ModifError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match self {
            ModifError::CommitError(e) => format!("Commit error: {}", e),
            ModifError::NoRepo(e) => format!("Repository Error: {}", e),
            ModifError::IoError(io_err) => io_err.to_string(),
            ModifError::ParseError(parse_err) => parse_err.to_string(),
            ModifError::DirError(dir_err) => {
                format!("Error during traversal of your repository: {}", dir_err)
            }
        };
        write!(f, "{}", message)
    }
}

impl From<git2::Error> for ModifError {
    fn from(err: git2::Error) -> Self {
        ModifError::CommitError(err)
    }
}

impl From<walkdir::Error> for ModifError {
    fn from(err: walkdir::Error) -> Self {
        ModifError::DirError(err)
    }
}

impl From<std::io::Error> for ModifError {
    fn from(err: std::io::Error) -> Self {
        ModifError::IoError(err)
    }
}

impl From<serde_json::Error> for ModifError {
    fn from(err: serde_json::Error) -> Self {
        ModifError::ParseError(err)
    }
}
