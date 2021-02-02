use std::fmt::{Display, Formatter, Result};

pub enum RepositoryErrorKind {
    SaveErr(String),
    UpdateErr(String),
    DeleteErr(String),
}

impl Display for RepositoryErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            RepositoryErrorKind::SaveErr(msg) => write!(f, "SaveErr: {}", msg.as_str()),
            RepositoryErrorKind::UpdateErr(msg) => write!(f, "UpdateErr: {}", msg.as_str()),
            RepositoryErrorKind::DeleteErr(msg) => write!(f, "DeleteErr: {}", msg.as_str()),
        }
    }
}