use operation_service::OperationService;
use pod_service::PodService;

pub mod operation_service;
pub mod pod_service;

#[derive(Debug, Clone)]
pub struct ServiceRegistry {
    pub operation_service: OperationService<Action>,
    pub pod_service: PodService,
}
