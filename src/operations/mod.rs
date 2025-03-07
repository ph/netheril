use std::collections::BTreeMap;

use async_trait::async_trait;
use operation::Operation;
use serde::Serialize;
use tokio::sync::oneshot::{self, error::RecvError};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{actor::{mailbox::Mailbox, Actor, ActorError, Context}, error::NetherilErr};
mod operation;


#[derive(Serialize, Clone, Debug, PartialEq, ToSchema, Copy, PartialOrd, Ord, Eq)]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn generate() -> Self {
        Id(Uuid::new_v4())
    }
}

enum Message {
    Find {
	id: Id,
	reply_to: oneshot::Sender<Option<Operation>>,
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
	error: NetherilErr,
    },
}

struct OperationMonitor {
    operations: BTreeMap<Id, Operation>,
}

impl OperationMonitor {
    fn new() -> Self {
	OperationMonitor {
	    operations: BTreeMap::new(),
	}
    }
}

#[async_trait]
impl Actor for OperationMonitor {
    type Message = Message;

    async fn handle(
	&mut self,
	_ctx: &Context,
	message: Self::Message,
    ) -> Result<(), ActorError> {
	match message {
	    Message::Create { reply_to } => {
		let operation = Operation::new();
		let operation_id = operation.id();
		self.operations.insert(operation_id.clone(), operation);
		reply_to.send(operation_id).unwrap();
		Ok(())
	    }
	    Message::Start { .. } => Ok(()),
	    Message::Cancel { .. } => Ok(()),
	    Message::Complete { .. } => Ok(()),
	    Message::Fail { .. } => Ok(()),
	    Message::Find { .. } => Ok(()),
	}
    }
}

struct OperationMonitorHandle {
    mailbox: Mailbox<OperationMonitor>,
}

impl OperationMonitorHandle {
    fn new(mailbox: Mailbox<OperationMonitor>)  -> Self {
	OperationMonitorHandle{
	    mailbox
	}
    }
    async fn find(&self, id: Id) -> Result<Option<Operation>, RecvError> {
	let (tx, rx) = oneshot::channel();
	self.mailbox.send(Message::Find { id: id, reply_to: tx });
	rx.await
    }

    async fn create(&self) -> Result<Id, RecvError> {
	let (tx, rx) = oneshot::channel();
	self.mailbox.send(Message::Create { reply_to: tx });
	rx.await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::actor::Workspace;

    #[tokio::test]
    async fn create_new_operation() {
	let workspace = Workspace::default();
	let monitor = OperationMonitor::new();
	let (mailbox, monitor_loop) = workspace.spawn(monitor);
	let handle = OperationMonitorHandle::new(mailbox);

	let job = async {
	    let id = handle.create().await.unwrap();
	};

	tokio::join!(
	    job,
	    monitor_loop
	);
    }
}
