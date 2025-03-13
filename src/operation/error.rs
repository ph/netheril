use super::{operation::State, Id};

#[derive(Debug)]
pub enum OperationError {
    NotFound(Id),
    InvalidTransition { from: State, to: State },
}

impl std::error::Error for OperationError {}

impl std::fmt::Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationError::NotFound(id) => write!(f, "operation {} not found", id),
            OperationError::InvalidTransition { from, to } => {
                write!(f, "invalid transition from: `{}` to: `{}`", from, to)
            }
        }
    }
}
