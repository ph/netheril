use tracing::trace;

use super::{BoxedError, Id};

pub trait State: Send + Sync + std::fmt::Debug + 'static {}

pub enum OperationState {
    Queue(Operation<Queue>),
    Working(Operation<Working>),
    Completed(Operation<Completed>),
    Canceled(Operation<Canceled>),
    Failed(Operation<Failed>),
}

// Possible State
#[derive(Debug)]
pub struct Queue;

#[derive(Debug)]
pub struct Working;

#[derive(Debug)]
pub struct Completed;

#[derive(Debug)]
pub struct Canceled;

#[derive(Debug)]
pub struct Failed {
    error: BoxedError,
}

impl State for Queue {}
impl State for Working {}
impl State for Completed {}
impl State for Canceled {}
impl State for Failed {}

struct Data {
    id: Id,
}

pub struct Operation<S: State + ?Sized> {
    inner: Data,
    state: Box<S>,
}

impl<S: State> Operation<S> {
    pub fn id(&self) -> Id {
        self.inner.id
    }
}

impl<T: State> Operation<T> {
    pub fn new() -> Operation<Queue> {
        Operation {
            inner: Data { id: Id::generate() },
            state: Box::new(Queue {}),
        }
    }
}

impl Operation<Queue> {
    fn start(self) -> Operation<Working> {
        trace!(id = ?self.inner.id, "start operation");

        Operation {
            inner: self.inner,
            state: Box::new(Working {}),
        }
    }

    fn cancel(self) -> Operation<Canceled> {
        trace!(id = ?self.inner.id, "cancel operation");

        Operation {
            inner: self.inner,
            state: Box::new(Canceled {}),
        }
    }
}

impl Operation<Working> {
    fn cancel(self) -> Operation<Canceled> {
        trace!(id = ?self.inner.id, "cancel operation");

        Operation {
            inner: self.inner,
            state: Box::new(Canceled {}),
        }
    }

    fn complete(self) -> Operation<Completed> {
        trace!(id = ?self.inner.id, "complete operation");

        Operation {
            inner: self.inner,
            state: Box::new(Completed {}),
        }
    }

    fn fail(self, error: BoxedError) -> Operation<Failed> {
        trace!(id = ?self.inner.id, error = error, "fail operation");

        Operation {
            inner: self.inner,
            state: Box::new(Failed { error: error }),
        }
    }
}

impl Operation<Completed> {}
impl Operation<Canceled> {}
impl Operation<Failed> {
    fn error(self) -> BoxedError {
        self.state.error
    }
}
