use netheril::{api::router, domains::health::{HealthView, State}, services::{OperationService, ServiceRegistry}};

use crate::common::api_server;

#[tokio::test]
async fn it_should_return_health_status() {
    let services = ServiceRegistry {
        operation_service: OperationService::new(),
    };

    let router = router().with_state(services);
    let (_server, client) = api_server(router).await;

    let response: HealthView = client
        .get("/api/health")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(response.status, State::Healthy);
}
