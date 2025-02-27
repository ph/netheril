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

#[derive(Debug, Clone)]
pub struct ServiceRegistry {
    pub operation_service: OperationService,
}
