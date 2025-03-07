#[derive(Debug, Clone, PartialEq)]
pub enum OperationError {
    QueueFull,
}

impl std::error::Error for OperationError {}
impl std::fmt::Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationError::QueueFull => write!(f, "internal queue is full"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct OperationService {}

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
}
