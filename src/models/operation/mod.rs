use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Clone, Debug, PartialEq, ToSchema)]
pub struct OperationId(uuid::Uuid);

impl OperationId {
    pub fn generate() -> Self {
        OperationId(Uuid::new_v4())
    }
}

#[derive(Debug)]
struct Operation {
    id: OperationId,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    // Completed,
    // Error,
    Queued,
    // InProgress,
}
