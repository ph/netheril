use netheril::{
    api::router,
    models::health::{HealthView, State},
};

use crate::common::{api_server, configure_services};

#[tokio::test]
async fn it_should_return_health_status() {
    let services = configure_services();

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
