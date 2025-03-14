#![allow(unused)]
use std::marker::PhantomData;

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

pub fn new_monitor(id: Id, sender: Sender<Message>) -> Monitor<Queued> {
    Monitor {
        inner: Data { id, sender },
        state: Queued {},
    }
}

pub fn reify_monitor(id: Id, sender: Sender<Message>, state: State) -> Box<Monitor<dyn TState> > {
    let inner = Data { id, sender };

    match state {
	State::Queued => Box::new(Monitor{
	    inner,
	    state: Queued {},
	}),
	State::Working => Box::new(Monitor{
	    inner,
	    state: Working {}
	}),
	State::Failed => Box::new(Monitor {
	    inner,
	    state: Failed {}
	}),
	State::Canceled => Box::new(Monitor {
	    inner, 
	    state: Canceled {}

	}),
        State::Completed => Box::new(Monitor {
	    inner,
	    state: Completed {},
	}),
    }
}

pub struct Monitor<S: TState + ?Sized> {
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
	    assert!(matches!(rx.recv().await.unwrap(), Message::UpdateOperation { id, state: State::Working }));
        });

        monitor.start().await.unwrap();
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn transition_queued_to_cancel() {
        let (id, monitor, mut rx) = monitor();

        let handle = tokio::spawn(async move {
	    assert!(matches!(rx.recv().await.unwrap(), Message::UpdateOperation { id, state: State::Canceled }));
        });

        monitor.cancel().await.unwrap();
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn transition_working_to_cancel() {
        let (id, monitor, mut rx) = monitor();

        let handle = tokio::spawn(async move {
	    assert!(matches!(rx.recv().await.unwrap(), Message::UpdateOperation { id, state: State::Working}));
	    assert!(matches!(rx.recv().await.unwrap(), Message::UpdateOperation { id, state: State::Canceled }));
        });

        monitor.start().await.unwrap();
        monitor.cancel().await.unwrap();
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn transition_working_to_fail() {
        let (id, monitor, mut rx) = monitor();

        let handle = tokio::spawn(async move {
	    assert!(matches!(rx.recv().await.unwrap(), Message::UpdateOperation { id, state: State::Working }));
	    assert!(matches!(rx.recv().await.unwrap(), Message::UpdateOperation { id, state: State::Failed }));
        });

        let monitor = monitor.start().await.unwrap();
        monitor.fail(OperationError::NotFound(id)).await.unwrap();
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn transition_working_to_canceledd() {
        let (id, monitor, mut rx) = monitor();

        let handle = tokio::spawn(async move {
	    assert!(matches!(rx.recv().await.unwrap(), Message::UpdateOperation { id, state: State::Working }));
	    assert!(matches!(rx.recv().await.unwrap(), Message::UpdateOperation { id, state: State::Canceled }));
        });

        let monitor = monitor.start().await.unwrap();
        monitor.cancel().await.unwrap();
        handle.await.unwrap();
    }
}
