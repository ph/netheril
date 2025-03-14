use tokio::sync::mpsc::Sender;

use super::{error::OperationError, states::State, Id, Message};

#[derive(Debug, Clone)]
pub struct Sentinel {
    id: Id,
    state: State,
    sender: Sender<Message>,
}

impl Sentinel {
    pub fn new(id: Id, sender: tokio::sync::mpsc::Sender<Message>) -> Self {
	Self::reify(id, State::Queued, sender)
    }

    pub fn reify(id: Id, state: State, sender: tokio::sync::mpsc::Sender<Message>) -> Self {
	Sentinel {
	    id,
	    state,
	    sender,
	}
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub async fn apply(&mut self, new_state: State) -> Result<(), OperationError> {
        match (self.state.clone(), new_state.clone()) {
            (State::Queued, State::Working) => self.transition(new_state).await,
            // transition to terminal states.
            (State::Queued, State::Failed) => self.transition(new_state).await,
            (State::Queued, State::Canceled) => self.transition(new_state).await,
            (State::Working, State::Failed) => self.transition(new_state).await,
            (State::Working, State::Canceled) => self.transition(new_state).await,
            (State::Working, State::Completed) => self.transition(new_state).await,
            _ => Err(OperationError::InvalidTransition {
                from: self.state.clone(),
                to: new_state,
            }),
        }
    }

    async fn transition(&mut self, new_state: State) -> Result<(), OperationError> {
        let from = self.state.clone();
        let to = new_state.clone();

	self.communicate_changes(from, to).await?;
        self.state = new_state;

        Ok(())
    }

    async fn communicate_changes(&self, from: State, to: State) -> Result<(), OperationError> {
	let message = Message::UpdateOperation {
	    id: self.id.clone(),
	    from, to,
	};

	self.sender.send(message).await?;
	Ok(())
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn apply_valid_transitions_from_queued_state() {
//         use State::*;

//         let transitions = vec![(Queued, Working), (Queued, Failed), (Queued, Canceled)];

//         for (_, new_state) in transitions {
//             let mut op = Operation::new();
//             op.apply(new_state).unwrap();
//         }
//     }

//     #[test]
//     fn apply_valid_transitions_from_working() {
//         use State::*;

//         let transitions = vec![(Working, Failed), (Working, Canceled), (Working, Completed)];

//         for (_, new_state) in transitions {
//             let mut op = Operation::new();
//             op.apply(Working).unwrap();
//             op.apply(new_state).unwrap();
//         }
//     }

//     #[test]
//     fn apply_invalid_transitions_from_failed() {
//         use State::*;

//         let transitions = vec![Working, Queued, Canceled, Completed, Failed];

//         for transition in transitions {
//             let mut op = Operation::new();
//             op.apply(Failed).unwrap();
//             assert!(op.apply(transition).is_err());
//         }
//     }

//     #[test]
//     fn apply_invalid_transitions_from_canceled() {
//         use State::*;

//         let transitions = vec![Working, Queued, Canceled, Failed, Completed];

//         for transition in transitions {
//             let mut op = Operation::new();
//             op.apply(Canceled).unwrap();
//             assert!(op.apply(transition).is_err());
//         }
//     }

//     #[test]
//     fn apply_invalid_transitions_from_completed() {
//         use State::*;

//         let transitions = vec![Working, Queued, Canceled, Failed, Completed];

//         for transition in transitions {
//             let mut op = Operation::new();
//             op.apply(Working).unwrap();
//             op.apply(Completed).unwrap();
//             assert!(op.apply(transition).is_err());
//         }
//     }

//     #[test]
//     fn keep_transitions_audit() {
//         use State::*;

//         let mut op = Operation::new();

//         op.apply(Working).unwrap();
//         op.apply(Completed).unwrap();

//         let audits = op.transitions_audits();

//         assert_eq!(2, audits.len());

//         let TransitionAudit { from, to, .. } = audits[0].clone();

//         assert_eq!((Queued, Working), (from, to));

//         let TransitionAudit { from, to, .. } = audits[1].clone();
//         assert_eq!((Working, Completed), (from, to));
//     }
// }
