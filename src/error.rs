use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum ModifError {
    CommitError(git2::Error),
    NoRepo(git2::Error),
    IoError(std::io::Error),
}

impl std::error::Error for ModifError {}

impl Display for ModifError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match self {
            ModifError::CommitError(e) => format!("Commit error: {}", e),
            ModifError::NoRepo(e) => format!("Repository Error: {}", e),
            ModifError::IoError(io_err) => io_err.to_string(),
        };
        write!(f, "{}", message)
    }
}

impl From<git2::Error> for ModifError {
    fn from(err: git2::Error) -> Self {
        ModifError::CommitError(err)
    }
}
