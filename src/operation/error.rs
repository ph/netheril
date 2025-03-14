#![allow(unused)]
use tokio::sync::mpsc::error::SendError;

use super::{operation_model::State, Id};

#[derive(Debug)]
pub enum OperationError {
    NotFound(Id),
    InvalidTransition { from: State, to: State },
    Sender,
}

impl std::error::Error for OperationError {}

impl std::fmt::Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationError::NotFound(id) => write!(f, "operation {} not found", id),
            OperationError::InvalidTransition { from, to } => {
                write!(f, "invalid transition from: `{}` to: `{}`", from, to)
            }
            OperationError::Sender => write!(f, "sender error"),
        }
    }
}

impl<T> From<SendError<T>> for OperationError {
    fn from(_value: SendError<T>) -> Self {
        OperationError::Sender
    }
}
