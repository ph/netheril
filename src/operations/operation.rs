use tracing::trace;

use crate::error::NetherilErr;

use super::Id;

#[derive(Debug)]
pub enum OperationErr {
    InvalidTransition { from: State, to: State },
}

impl std::error::Error for OperationErr {}
impl std::fmt::Display for OperationErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationErr::InvalidTransition { from, to } => {
                write!(f, "invalid transition from `{}` to `{}`", from, to)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum State {
    New,
    Queued,
    Started,
    Completed,
    Canceled,
    Failed { error: NetherilErr },
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::New => write!(f, "new"),
            State::Queued => write!(f, "queued"),
            State::Started => write!(f, "started"),
            State::Completed => write!(f, "completed"),
            State::Canceled => write!(f, "canceled"),
            State::Failed { error } => write!(f, "failed (error: {})", error),
        }
    }
}

#[derive(Debug)]
pub enum Transition {
    Create,
    Enqueue,
    Start,
    Complete,
    Cancel,
    Fail { error: NetherilErr },
}

pub struct Operation {
    id: Id,
    state: State,
    // created_at
    // end_at
}

impl Operation {
    pub fn new() -> Self {
        Operation {
            id: Id::generate(),
            state: State::New,
        }
    }

    pub fn id(&self) -> Id {
        self.id.clone()
    }

    pub fn handle_event(self, transition: Transition) -> Result<(), OperationErr> {
        match transition {
            Transition::Create => Err(OperationErr::InvalidTransition {
                from: State::New,
                to: State::New,
            }),
            Transition::Enqueue => self.apply_transition(State::Queued),
            Transition::Start => self.apply_transition(State::Started),
            Transition::Complete => self.apply_transition(State::Completed),
            Transition::Cancel => self.apply_transition(State::Canceled),
            Transition::Fail { error } => self.apply_transition(State::Failed { error: error }),
        }
    }

    fn apply_transition(mut self, new_state: State) -> Result<(), OperationErr> {
        let valid = match (self.state.clone(), new_state.clone()) {
            (State::New, State::Queued | State::Canceled) => true,
            (State::Queued, State::Started | State::Canceled) => true,
            (State::Started, State::Completed | State::Canceled | State::Failed { .. }) => true,
            _ => false,
        };

        if valid {
            trace!(
                id = ?self.id,
                "transition from `{}` to `{}`",
                self.state,
                new_state
            );
            self.state = new_state;
            return Ok(());
        }

        Err(OperationErr::InvalidTransition {
            from: self.state.clone(),
            to: new_state,
        })
    }
}
