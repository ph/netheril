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

enum ActionType {
    NewPod,
    KillPod,
}

#[derive(Debug)]
pub struct Operation> {
    id: OperationId,
    action: ActionType,
    status: Status,
}

impl Operation {
    fn new(action: ActionType) -> Self {
        let id = Operation::generate();
        Operation { id, action }
    }
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    // Completed,
    // Error,
    Queued,
    // InProgress,
}
