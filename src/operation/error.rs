#![allow(unused)]
use tokio::sync::{mpsc::error::SendError, oneshot};

use super::{states::State, Id};

#[derive(Debug)]
pub enum OperationError {
    NotFound(Id),
    InvalidTransition { from: State, to: State },
    Sender,
    Receiver,
    StateMismatch { expected: State, current: State },
}

impl std::error::Error for OperationError {}

impl std::fmt::Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationError::NotFound(id) => write!(f, "operation {} not found", id),
            OperationError::InvalidTransition { from, to } => {
                write!(f, "invalid transition from: `{}` to: `{}`", from, to)
            }
            OperationError::Sender => write!(f, "sender error on channel"),
            OperationError::Receiver => write!(f, "receiver error on channel"),
            OperationError::StateMismatch { expected, current } => {
                write!(
                    f,
                    "expected state `{}`, current state: `{}`",
                    expected, current
                )
            }
        }
    }
}

impl<T> From<SendError<T>> for OperationError {
    fn from(_value: SendError<T>) -> Self {
        OperationError::Sender
    }
}

impl From<oneshot::error::RecvError> for OperationError {
    fn from(value: oneshot::error::RecvError) -> Self {
        OperationError::Receiver
    }
}
