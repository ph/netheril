#![allow(unused)]
use tokio::sync::mpsc::Sender;

use super::{error::OperationError, operation_model::State, Id, Message};

#[derive(Debug)]
pub struct Queued;
impl TState for Queued {}

#[derive(Debug)]
pub struct Working;
impl TState for Working {}

#[derive(Debug)]
pub struct Failed;
impl TState for Failed {}

#[derive(Debug)]
pub struct Canceled;
impl TState for Canceled {}

#[derive(Debug)]
pub struct Completed;
impl TState for Completed {}

pub trait TState {}

#[derive(Debug, Clone)]
pub struct Data {
    id: Id,
    sender: Sender<Message>,
}

fn new_monitor(id: Id, sender: Sender<Message>) -> Monitor<Queued> {
    Monitor {
        inner: Data { id, sender },
        state: Queued {},
    }
}

pub struct Monitor<S: TState> {
    inner: Data,
    state: S,
}

impl Monitor<Queued> {
    pub async fn start(&self) -> Result<Monitor<Working>, OperationError> {
        self.inner
            .sender
            .send(Message::UpdateOperation {
                id: self.inner.id,
                state: State::Working,
            })
            .await?;

        Ok(Monitor {
            inner: self.inner.clone(),
            state: Working {},
        })
    }

    pub async fn cancel(&self) -> Result<Monitor<Canceled>, OperationError> {
        self.inner
            .sender
            .send(Message::UpdateOperation {
                id: self.inner.id,
                state: State::Canceled,
            })
            .await?;

        Ok(Monitor {
            inner: self.inner.clone(),
            state: Canceled {},
        })
    }
}

impl Monitor<Working> {
    pub async fn fail(&self, _error: OperationError) -> Result<Monitor<Failed>, OperationError> {
        self.inner
            .sender
            .send(Message::UpdateOperation {
                id: self.inner.id,
                state: State::Failed,
            })
            .await?;

        Ok(Monitor {
            inner: self.inner.clone(),
            state: Failed {},
        })
    }

    pub async fn cancel(&self) -> Result<Monitor<Canceled>, OperationError> {
        self.inner
            .sender
            .send(Message::UpdateOperation {
                id: self.inner.id,
                state: State::Canceled,
            })
            .await?;

        Ok(Monitor {
            inner: self.inner.clone(),
            state: Canceled {},
        })
    }

    pub async fn complete(&self) -> Result<Monitor<Completed>, OperationError> {
        self.inner
            .sender
            .send(Message::UpdateOperation {
                id: self.inner.id,
                state: State::Completed,
            })
            .await?;

        Ok(Monitor {
            inner: self.inner.clone(),
            state: Completed {},
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::operation::Message;
    use tokio::sync::mpsc::Receiver;

    fn monitor() -> (Id, Monitor<Queued>, Receiver<Message>) {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<Message>(10);
        let id = Id::generate();

        let monitor = new_monitor(id, tx);
        (id, monitor, rx)
    }

    #[tokio::test]
    async fn transition_queued_to_working() {
        let (id, monitor, mut rx) = monitor();

        let handle = tokio::spawn(async move {
            let m = rx.recv().await.unwrap();
            assert_eq!(
                Message::UpdateOperation {
                    id,
                    state: State::Working
                },
                m
            );
        });

        monitor.start().await.unwrap();
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn transition_queued_to_cancel() {
        let (id, monitor, mut rx) = monitor();

        let handle = tokio::spawn(async move {
            let m = rx.recv().await.unwrap();
            assert_eq!(
                Message::UpdateOperation {
                    id,
                    state: State::Canceled
                },
                m
            );
        });

        monitor.cancel().await.unwrap();
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn transition_working_to_cancel() {
        let (id, monitor, mut rx) = monitor();

        let handle = tokio::spawn(async move {
            let m = rx.recv().await.unwrap();
            assert_eq!(
                Message::UpdateOperation {
                    id,
                    state: State::Working
                },
                m
            );

            let m = rx.recv().await.unwrap();
            assert_eq!(
                Message::UpdateOperation {
                    id,
                    state: State::Canceled
                },
                m
            );
        });

        monitor.start().await.unwrap();
        monitor.cancel().await.unwrap();
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn transition_working_to_fail() {
        let (id, monitor, mut rx) = monitor();

        let handle = tokio::spawn(async move {
            let m = rx.recv().await.unwrap();
            assert_eq!(
                Message::UpdateOperation {
                    id,
                    state: State::Working
                },
                m
            );

            let m = rx.recv().await.unwrap();
            assert_eq!(
                Message::UpdateOperation {
                    id,
                    state: State::Failed
                },
                m
            );
        });

        let monitor = monitor.start().await.unwrap();
        monitor.fail(OperationError::NotFound(id)).await.unwrap();
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn transition_working_to_canceledd() {
        let (id, monitor, mut rx) = monitor();

        let handle = tokio::spawn(async move {
            let m = rx.recv().await.unwrap();
            assert_eq!(
                Message::UpdateOperation {
                    id,
                    state: State::Working
                },
                m
            );

            let m = rx.recv().await.unwrap();
            assert_eq!(
                Message::UpdateOperation {
                    id,
                    state: State::Canceled
                },
                m
            );
        });

        let monitor = monitor.start().await.unwrap();
        monitor.cancel().await.unwrap();
        handle.await.unwrap();
    }
}
