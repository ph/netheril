use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone)]
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
pub enum Action {
   NewPod 
}

#[derive(Serialize)]
pub struct OperationId(uuid::Uuid);


impl OperationId {
    pub fn generate() -> Self {
	OperationId(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Default)]
pub struct OperationService;

impl OperationService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn find(&self, id: &str) -> Option<String> {
        if id == "111" {
            Some(id.to_string())
        } else {
            None
        }
    }

    pub async fn schedule(_action: Action) -> Result<OperationId, OperationError> {
	Err(OperationError::QueueFull)
    }
}

