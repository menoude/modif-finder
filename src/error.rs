use std::{
    fmt,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum ModifError {
    WrongCommit(String),
}

impl std::error::Error for ModifError {}

impl Display for ModifError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match self {
            ModifError::WrongCommit(error_message) => error_message,
        };
        write!(f, "{}", message)
    }
}
