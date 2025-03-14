#![allow(unused)]
use std::collections::BTreeMap;

use async_trait::async_trait;
use operation_model::{Operation, State};
use tokio::sync::oneshot;

use crate::actor::{Actor, ActorError, Context};

mod error;
mod monitor;
mod operation_model;

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub struct Id(uuid::Uuid);

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
}

impl OperationStateManagerActor {
    pub fn new() -> Self {
        OperationStateManagerActor {
            operations: BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    NewOperation{ reply_to: oneshot::Sender<Id> },
    UpdateOperation { id: Id, state: State },
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
	    _ => todo!(),
	}

	Ok(())
    }
}

// struct OperationManagerHandle {}

// impl OperationManagerHandle {
//     fn schedule() -> Id {}
//     fn new_transaction(id: Id) -> Result<Transaction, OperationError> {}
// }
