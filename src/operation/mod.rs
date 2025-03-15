#![allow(unused)]
use std::collections::BTreeMap;

use async_trait::async_trait;
use error::OperationError;
use operation_model::{Operation};
use sentinel::Sentinel;
use states::State;
use tokio::sync::oneshot;

use crate::actor::{Actor, ActorError, Context};

mod error;
mod sentinel;
mod states;
mod operation_model;

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub struct Id(uuid::Uuid);

const OPERATION_STATE_MANAGER_CAPACITY: usize = 100;

impl Id {
    pub fn generate() -> Id {
        Id(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct OperationStateManagerActor {
    operations: BTreeMap<Id, Operation>,
    receiver: tokio::sync::mpsc::Receiver<Message>,
}

impl OperationStateManagerActor {
    pub fn new(receiver: tokio::sync::mpsc::Receiver<Message>) -> Self {
        OperationStateManagerActor {
            operations: BTreeMap::new(),
	    receiver,
        }
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    LookupOperation { id: Id, reply_to: oneshot::Sender<Option<Operation>> },
    NewOperation { reply_to: oneshot::Sender<Id> },
    UpdateOperation { id: Id, from: State, to: State },
}

#[async_trait]
impl Actor for OperationStateManagerActor {
    type Message = Message;

    async fn handle(&mut self, _ctx: &Context, message: Self::Message) -> Result<(), ActorError> {
	use Message::*;


	match message {
	    NewOperation { reply_to } =>  {
		let operation = Operation::new();
		let id = operation.id().clone();
		self.operations.insert(id.clone(), operation);
		reply_to.send(id.clone());
	    },
	    LookupOperation { id, reply_to } => {
		let operation = self.operations.get(&id).cloned();
		reply_to.send(operation);
	    }
	    UpdateOperation { id, from, to} => {
		if let Some(operation) = self.operations.get_mut(&id) {
		    operation.apply(from, to);
		}
	    }
	    Quit => { }
	}
	Ok(())
    }
}

struct OperationStateManagerHandle {
    sender: tokio::sync::mpsc::Sender<Message>,
}

impl OperationStateManagerHandle {
    pub fn new() -> Self {
	let (sender, receiver) = tokio::sync::mpsc::channel(OPERATION_STATE_MANAGER_CAPACITY);
	let manager = OperationStateManagerActor::new(receiver);
	let handle = OperationStateManagerHandle {
	    sender,
	};

	tokio::spawn(execute_operation_state_manager(manager));

	handle
    }

    pub async fn new_operation(&self) -> Result<Id, OperationError> {
	let (tx, rx) = oneshot::channel();
	self.sender.send(Message::NewOperation { reply_to: tx }).await?;
	Ok(rx.await?)
    }

    pub async fn lookup_operation(&self, id: &Id) -> Result<Option<Operation>, OperationError> {
	let (tx, rx) = oneshot::channel();
	self.sender.send(Message::LookupOperation { id: id.clone(), reply_to: tx }).await?;
	Ok(rx.await?)
    }

    pub async fn new_sentinel(&self, id: Id) -> Result<Sentinel, OperationError> {
	match self.lookup_operation(&id).await? {
	    Some(operation) => Ok(Sentinel::reify(id.clone(), operation.state(), self.sender.clone())),
	    None => Err(OperationError::NotFound(id)),
	}
    }
}

async fn execute_operation_state_manager(mut manager: OperationStateManagerActor) {
    let ctx = Context::new();
    while let Some(message) = manager.receiver.recv().await {
	manager.handle(&ctx, message).await.unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn create_new_operation() {
	let op_state = OperationStateManagerHandle::new();
	let id = op_state.new_operation().await.unwrap();
    }

    #[tokio::test]
    async fn return_true_when_operation_exists() {
	let op_state = OperationStateManagerHandle::new();
	let id = op_state.new_operation().await.unwrap();
	let operation = op_state.lookup_operation(&id).await.unwrap().unwrap();
	assert_eq!(operation.id(), id);
    }

    #[tokio::test]
    async fn cant_create_sentinel_when_operation_does_not_exist() {
	let op_state = OperationStateManagerHandle::new();
	let id = Id::generate();
	assert!(matches!(op_state.new_sentinel(id.clone()).await, Err(OperationError::NotFound(id))))
    }

    #[tokio::test]
    async fn create_sentinel_when_operation_exists() {
	let op_state = OperationStateManagerHandle::new();
	let id = op_state.new_operation().await.unwrap();
	let sentinel = op_state.new_sentinel(id).await.unwrap();
	assert_eq!(sentinel.id(), id);
    }

    #[tokio::test]
    async fn update_operation_state_from_sentinel() {
	let op_state = OperationStateManagerHandle::new();
	let id = op_state.new_operation().await.unwrap();
	let mut sentinel = op_state.new_sentinel(id.clone()).await.unwrap();

	sentinel.start().await.unwrap();
	let operation = op_state.lookup_operation(&id).await.unwrap().unwrap();
	assert_eq!(operation.state(), State::Working);

	sentinel.complete().await.unwrap();
	let operation = op_state.lookup_operation(&id).await.unwrap().unwrap();
	assert_eq!(operation.state(), State::Completed);
    }
}
