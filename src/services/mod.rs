#[derive(Debug, Clone)]
pub struct OperationService {}

impl OperationService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn find(&self, id: &str) -> Option<String> {
        println!("find: {}", id);
        Some(id.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ServiceRegistry {
    pub operation_service: OperationService,
}
