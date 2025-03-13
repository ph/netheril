use chrono::{DateTime, Local, Utc};

use super::{error::OperationError, Id};

#[derive(Debug, Clone)]
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

pub struct Operation {
    id: Id,
    created_at: DateTime<Utc>,
    state: State,
}

impl Operation {
    fn new() -> Operation {
        Operation {
            id: Id::generate(),
            created_at: Local::now().into(),
            state: State::Queued,
        }
    }

    fn transition(&mut self, new_state: State) -> Result<(), OperationError> {
        self.state = new_state;
        Ok(())
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn apply_valid_transitions_from_queued_state() {
        use State::*;

        let transitions = vec![(Queued, Working), (Queued, Failed), (Queued, Canceled)];

        for (_, new_state) in transitions {
            let mut operation = Operation::new();
            let _ = operation.apply(new_state);
        }
    }

    #[test]
    fn apply_valid_transitions_from_working() {
        use State::*;

        let transitions = vec![(Working, Failed), (Working, Canceled), (Working, Completed)];

        for (_, new_state) in transitions {
            let mut operation = Operation::new();
            let _ = operation.apply(Working).unwrap();
            let _ = operation.apply(new_state);
        }
    }

    #[test]
    fn apply_invalid_transitions_from_failed() {
        use State::*;

        let transitions = vec![Working, Queued, Canceled, Completed, Failed];

        for transition in transitions {
            let mut operation = Operation::new();
            let _ = operation.apply(Failed).unwrap();
            let _ = operation.apply(transition).is_err();
        }
    }

    #[test]
    fn apply_invalid_transitions_from_canceled() {
        use State::*;

        let transitions = vec![Working, Queued, Canceled, Failed, Completed];

        for transition in transitions {
            let mut operation = Operation::new();
            let _ = operation.apply(Canceled).unwrap();
            let _ = operation.apply(transition).is_err();
        }
    }

    #[test]
    fn apply_invalid_transitions_from_completed() {
        use State::*;

        let transitions = vec![Working, Queued, Canceled, Failed, Completed];

        for transition in transitions {
            let mut operation = Operation::new();
            let _ = operation.apply(Working).unwrap();
            let _ = operation.apply(Completed).unwrap();
            let _ = operation.apply(transition).is_err();
        }
    }
}
