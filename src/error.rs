use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum ModifError {
    CommitError(git2::Error),
    NoRepo(String)
}

impl std::error::Error for ModifError {}

impl Display for ModifError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match self {
            ModifError::CommitError(e) => e.message(),
            ModifError::NoRepo(error_message) => error_message
        };
        write!(f, "{}", message)
    }
}

impl From<git2::Error> for ModifError {
    fn from(err: git2::Error) -> Self {
        ModifError::CommitError(err)
    }
}