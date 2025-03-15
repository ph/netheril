#![allow(unused)]
use std::borrow::Cow;

use chrono::{DateTime, Local, Utc};

use super::{error::OperationError, states::State, Id};

#[derive(Debug, Clone, PartialEq)]
pub struct TransitionAudit {
    from: State,
    to: State,
    created_at: DateTime<Utc>,
}

impl TransitionAudit {
    fn new(from: State, to: State) -> Self {
        TransitionAudit {
            from,
            to,
            created_at: Local::now().into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Operation {
    id: Id,
    created_at: DateTime<Utc>,
    state: State,
    transitions_audits: Vec<TransitionAudit>,
}

impl Operation {
    pub fn new() -> Operation {
        Operation {
            id: Id::generate(),
            created_at: Local::now().into(),
            state: State::Queued,
            transitions_audits: Vec::new(),
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub fn apply(&mut self, expected: State, new_state: State) -> Result<(), OperationError> {
        if self.state != expected {
            return Err(OperationError::StateMismatch {
                expected,
                current: self.state.clone(),
            });
        }

        let from = self.state.clone();
        let to = new_state.clone();

        self.transitions_audits.push(TransitionAudit::new(from, to));

        self.state = new_state;
        Ok(())
    }

    pub fn transitions_audits(&self) -> Cow<Vec<TransitionAudit>> {
        Cow::Borrowed(&self.transitions_audits)
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
