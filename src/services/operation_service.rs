use serde::Serialize;
use tokio::sync::mpsc::{self, error::TrySendError, Receiver, Sender};
use tracing::debug;
use utoipa::ToSchema;
use uuid::Uuid;

const OPERATION_QUEUE_SIZE: usize = 100;

#[derive(Debug, Clone, PartialEq)]
pub enum OperationError {
    QueueFull,
}

impl std::error::Error for OperationError {}
impl std::fmt::Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
	match self {
	    OperationError::QueueFull => write!(f, "internal queue is full")
	}
    }
}

#[derive(Debug, Clone)]
pub struct Configuration {}

#[derive(Debug)]
pub enum Action {
    NewPod(Configuration),
}

#[derive(Serialize, Clone, Debug, PartialEq, ToSchema)]
pub struct OperationId(uuid::Uuid);


impl OperationId {
    pub fn generate() -> Self {
	OperationId(Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
pub struct OperationService {
    tx: Sender<Operation>,
}

impl OperationService {
    pub fn build()  -> (Receiver<Operation>, Self){
	let (tx, rx) = mpsc::channel::<Operation>(OPERATION_QUEUE_SIZE);
	let service = Self::new(tx);
	(rx, service)
    }

    pub fn new(tx: Sender<Operation>) -> Self {
        Self { tx }
    }

    pub fn find(&self, id: &str) -> Option<String> {
        if id == "111" {
            Some(id.to_string())
        } else {
            None
        }
    }

    pub fn schedule(&self, action: Action) -> Result<OperationId, OperationError> {
	debug!(action=?action, "schedule action");

	let operation = Operation::new(action);
	let id = operation.id();

	match self.tx.try_send(operation) {
	    Ok(()) => Ok(id),
	    Err(TrySendError::<_>::Full(_)) => Err(OperationError::QueueFull),
	    Err(TrySendError::<_>::Closed(_)) => panic!("closed channel shot not happen"),
	}
    }
}

pub struct Operation {
    id: OperationId,
    kind: Action,
}

impl Operation {
    pub fn new(kind: Action) -> Self {
	Operation {
	    id: OperationId::generate(),
	    kind,
	}
    }

    pub fn id(&self) -> OperationId {
	self.id.clone()
    }
}

#[cfg(test)]
mod test {
    use tokio::sync::mpsc;

    use super::*;

    fn service() -> (Receiver<Operation>, OperationService) {
	let (tx, rx) = mpsc::channel::<Operation>(1);
	let service = OperationService::new(tx);

	(rx, service)
    }
    
    #[tokio::test]
    async fn schedule_action_successfully() {
	let (mut rx, service) = service();
	let config = Configuration{};

	tokio::spawn(async move {
	    let _ = rx.recv().await.unwrap();
	});

	assert!(service.schedule(Action::NewPod(config)).is_ok())
    }

    #[tokio::test]
    async fn schedule_action_return_error_when_queue_is_full() {
	let (mut rx, service) = service();
	let config = Configuration{};

	tokio::spawn(async move {
	    let _ = rx.recv().await.unwrap();
	});

	assert!(service.schedule(Action::NewPod(config.clone())).is_ok());
	assert_eq!(service.schedule(Action::NewPod(config.clone())), Err(OperationError::QueueFull));
    }
}
