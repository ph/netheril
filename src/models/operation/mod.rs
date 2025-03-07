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
