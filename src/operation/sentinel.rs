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
        Sentinel { id, state, sender }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub async fn start(&mut self) -> Result<(), OperationError> {
        self.apply(State::Working).await
    }

    pub async fn fail(&mut self, _error: OperationError) -> Result<(), OperationError> {
        self.apply(State::Failed).await
    }

    pub async fn cancel(&mut self) -> Result<(), OperationError> {
        self.apply(State::Canceled).await
    }

    pub async fn complete(&mut self) -> Result<(), OperationError> {
        self.apply(State::Completed).await
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
            id: self.id,
            from,
            to,
        };

        self.sender.send(message).await?;
        Ok(())
    }

    async fn apply(&mut self, new_state: State) -> Result<(), OperationError> {
        match (self.state.clone(), new_state.clone()) {
            (State::Queued, State::Working) => self.transition(new_state).await,
            (State::Queued, State::Canceled) => self.transition(new_state).await,

            // transition to terminal states.
            (State::Working, State::Failed) => self.transition(new_state).await,
            (State::Working, State::Canceled) => self.transition(new_state).await,
            (State::Working, State::Completed) => self.transition(new_state).await,
            _ => Err(OperationError::InvalidTransition {
                from: self.state.clone(),
                to: new_state,
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use tokio::sync::mpsc::Receiver;

    use super::*;

    fn sentinel() -> (Id, Receiver<Message>, Sentinel) {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let id = Id::generate();
        let sentinel = Sentinel::new(id, tx);
        (id, rx, sentinel)
    }

    fn sentinel_reify(state: State) -> (Id, Receiver<Message>, Sentinel) {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let id = Id::generate();
        let sentinel = Sentinel::reify(id, state, tx);
        (id, rx, sentinel)
    }

    #[tokio::test]
    async fn valid_from_queued_to_start() {
        let (id, mut rx, mut sentinel) = sentinel();

        let handle = tokio::spawn(async move {
            assert!(matches!(
                rx.recv().await.unwrap(),
                Message::UpdateOperation {
                    id,
                    from: State::Queued,
                    to: State::Working,
                }
            ))
        });

        sentinel.start().await;
        handle.await;
    }

    #[tokio::test]
    async fn valid_from_queued_to_cancel() {
        let (id, mut rx, mut sentinel) = sentinel();

        let handle = tokio::spawn(async move {
            assert!(matches!(
                rx.recv().await.unwrap(),
                Message::UpdateOperation {
                    id,
                    from: State::Queued,
                    to: State::Canceled,
                }
            ))
        });

        sentinel.cancel().await;
        handle.await;
    }

    #[tokio::test]
    async fn invalid_from_queued_to_failed() {
        let (id, mut rx, mut sentinel) = sentinel();

        assert!(matches!(
            sentinel.fail(OperationError::Sender).await,
            Err(OperationError::InvalidTransition {
                from: State::Queued,
                to: State::Failed
            })
        ));
    }

    #[tokio::test]
    async fn invalid_from_queued_to_complete() {
        let (id, mut rx, mut sentinel) = sentinel();

        assert!(matches!(
            sentinel.complete().await,
            Err(OperationError::InvalidTransition {
                from: State::Queued,
                to: State::Completed
            })
        ));
    }

    #[tokio::test]
    async fn valid_from_working_to_cancel() {
        let (id, mut rx, mut sentinel) = sentinel();

        let handle = tokio::spawn(async move {
            assert!(matches!(
                rx.recv().await.unwrap(),
                Message::UpdateOperation {
                    id,
                    from: State::Queued,
                    to: State::Working,
                }
            ));

            assert!(matches!(
                rx.recv().await.unwrap(),
                Message::UpdateOperation {
                    id,
                    from: State::Working,
                    to: State::Canceled,
                }
            ))
        });

        sentinel.start().await;
        sentinel.cancel().await;
        handle.await;
    }

    #[tokio::test]
    async fn valid_from_working_to_failed() {
        let (id, mut rx, mut sentinel) = sentinel();

        let handle = tokio::spawn(async move {
            assert!(matches!(
                rx.recv().await.unwrap(),
                Message::UpdateOperation {
                    id,
                    from: State::Queued,
                    to: State::Working,
                }
            ));

            assert!(matches!(
                rx.recv().await.unwrap(),
                Message::UpdateOperation {
                    id,
                    from: State::Working,
                    to: State::Failed,
                }
            ))
        });

        sentinel.start().await;
        sentinel.fail(OperationError::Sender).await;
        handle.await;
    }

    #[tokio::test]
    async fn valid_from_working_to_complete() {
        let (id, mut rx, mut sentinel) = sentinel();

        let handle = tokio::spawn(async move {
            assert!(matches!(
                rx.recv().await.unwrap(),
                Message::UpdateOperation {
                    id,
                    from: State::Queued,
                    to: State::Working,
                }
            ));

            assert!(matches!(
                rx.recv().await.unwrap(),
                Message::UpdateOperation {
                    id,
                    from: State::Working,
                    to: State::Completed,
                }
            ))
        });

        sentinel.start().await;
        sentinel.complete().await;
        handle.await;
    }

    #[tokio::test]
    async fn invalid_from_complete_to_fail() {
        let (id, mut rx, mut sentinel) = sentinel_reify(State::Completed);

        assert!(matches!(
            sentinel.fail(OperationError::Sender).await,
            Err(OperationError::InvalidTransition {
                from: State::Completed,
                to: State::Failed
            })
        ));
    }

    #[tokio::test]
    async fn invalid_from_complete_to_cancel() {
        let (id, mut rx, mut sentinel) = sentinel_reify(State::Completed);

        assert!(matches!(
            sentinel.cancel().await,
            Err(OperationError::InvalidTransition {
                from: State::Completed,
                to: State::Canceled
            })
        ));
    }

    #[tokio::test]
    async fn reify_with_initial_state() {
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let id = Id::generate();
        let sentinel = Sentinel::reify(id, State::Failed, tx);

        assert_eq!(State::Failed, sentinel.state());
    }
}
