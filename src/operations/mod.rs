use std::collections::BTreeMap;

use async_trait::async_trait;
use operation::{Operation, OperationState, Queue, State};
use serde::Serialize;
use tokio::sync::{mpsc::Sender, oneshot};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::actor::{Actor, ActorError, Context};
mod operation;


#[derive(Serialize, Clone, Debug, PartialEq, ToSchema, Copy, PartialOrd, Ord, Eq)]
pub struct Id(uuid::Uuid);

type BoxedError = Box<dyn std::error::Error + Sync + Send + 'static>; 

impl Id {
    pub fn generate() -> Self {
        Id(Uuid::new_v4())
    }
}

enum Message<S: State> {
    Find {
	id: Id,
	reply_to: oneshot::Sender<Option<Operation<S>>>,
    },
    Create {
	reply_to: oneshot::Sender<Id>,
    },
    Start {
	id: Id,
    },
    Cancel {
	id: Id,
    },
    Complete {
	id: Id,
    },
    Fail {
	id: Id,
	error: BoxedError,
    },
}

struct OperationMonitor {
    operations: BTreeMap<Id, OperationState>,
}

impl<S: State> OperationMonitor<S> {
    fn new() -> Self {
	OperationMonitor {
	    operations: BTreeMap::new(),
	}
    }
}

#[async_trait]
impl<S: State> Actor for OperationMonitor<S> {
    type Message = Message<S>;

    async fn handle(
	&mut self,
	_ctx: &Context,
	message: Message<S>,
    ) -> Result<(), ActorError> {
	match message {
	    Message::Create { reply_to } => {
		let operation:Operation<Queue> = Operation::new();
		let operation_id = operation.id();
		self.operations.insert(operation_id, Box::new(operation));

		reply_to.send(operation_id);

		Ok(())
	    }
	    Message::Start { .. } => Ok(()),
	    Message::Cancel {.. } => Ok(()),
	    Message::Complete { .. } => Ok(()),
	    Message::Fail { .. } => Ok(()),
	    Message::Find { .. } => Ok(()),
	}
    }
}

struct OperationMonitorHandle<T: State> {
    sender: Sender<Message<T>>,
}

impl<T: State> OperationMonitorHandle<T> {
    async fn find(&self, id: Id) -> Option<Operation<T>> {
	let (tx, rx) = oneshot::channel();
	self.sender.send(Message::Find { id: id, reply_to: tx });
	rx.await.expect("What")
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn return_the_operation_if_id_exist() {
	
    }
}
