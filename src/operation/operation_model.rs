#![allow(unused)]
use std::borrow::Cow;

use chrono::{DateTime, Local, Utc};

use super::{error::OperationError, Id};

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Queued,
    Working,
    Failed,
    Canceled,
    Completed,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Queued => write!(f, "queued"),
            State::Working => write!(f, "working"),
            State::Failed => write!(f, "failed"),
            State::Canceled => write!(f, "canceled"),
            State::Completed => write!(f, "completed"),
        }
    }
}

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

    pub fn apply(&mut self, new_state: State) -> Result<(), OperationError> {
        match (self.state.clone(), new_state.clone()) {
            (State::Queued, State::Working) => self.transition(new_state),
            // transition to terminal states.
            (State::Queued, State::Failed) => self.transition(new_state),
            (State::Queued, State::Canceled) => self.transition(new_state),
            (State::Working, State::Failed) => self.transition(new_state),
            (State::Working, State::Canceled) => self.transition(new_state),
            (State::Working, State::Completed) => self.transition(new_state),
            _ => Err(OperationError::InvalidTransition {
                from: self.state.clone(),
                to: new_state,
            }),
        }
    }

    pub fn transitions_audits(&self) -> Cow<Vec<TransitionAudit>> {
        Cow::Borrowed(&self.transitions_audits)
    }

    fn transition(&mut self, new_state: State) -> Result<(), OperationError> {
        let from = self.state.clone();
        let to = new_state.clone();

        self.state = new_state;

        self.transitions_audits.push(TransitionAudit::new(from, to));

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn apply_valid_transitions_from_queued_state() {
        use State::*;

        let transitions = vec![(Queued, Working), (Queued, Failed), (Queued, Canceled)];

        for (_, new_state) in transitions {
            let mut op = Operation::new();
            op.apply(new_state).unwrap();
        }
    }

    #[test]
    fn apply_valid_transitions_from_working() {
        use State::*;

        let transitions = vec![(Working, Failed), (Working, Canceled), (Working, Completed)];

        for (_, new_state) in transitions {
            let mut op = Operation::new();
            op.apply(Working).unwrap();
            op.apply(new_state).unwrap();
        }
    }

    #[test]
    fn apply_invalid_transitions_from_failed() {
        use State::*;

        let transitions = vec![Working, Queued, Canceled, Completed, Failed];

        for transition in transitions {
            let mut op = Operation::new();
            op.apply(Failed).unwrap();
            assert!(op.apply(transition).is_err());
        }
    }

    #[test]
    fn apply_invalid_transitions_from_canceled() {
        use State::*;

        let transitions = vec![Working, Queued, Canceled, Failed, Completed];

        for transition in transitions {
            let mut op = Operation::new();
            op.apply(Canceled).unwrap();
            assert!(op.apply(transition).is_err());
        }
    }

    #[test]
    fn apply_invalid_transitions_from_completed() {
        use State::*;

        let transitions = vec![Working, Queued, Canceled, Failed, Completed];

        for transition in transitions {
            let mut op = Operation::new();
            op.apply(Working).unwrap();
            op.apply(Completed).unwrap();
            assert!(op.apply(transition).is_err());
        }
    }

    #[test]
    fn keep_transitions_audit() {
        use State::*;

        let mut op = Operation::new();

        op.apply(Working).unwrap();
        op.apply(Completed).unwrap();

        let audits = op.transitions_audits();

        assert_eq!(2, audits.len());

        let TransitionAudit { from, to, .. } = audits[0].clone();

        assert_eq!((Queued, Working), (from, to));

        let TransitionAudit { from, to, .. } = audits[1].clone();
        assert_eq!((Working, Completed), (from, to));
    }
}
