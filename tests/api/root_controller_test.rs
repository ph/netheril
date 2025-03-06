use netheril::{
    api::router,
    version::BUILD,
};
use serde::Deserialize;

use crate::common::{api_server, configure_services};

#[tokio::test]
async fn it_should_return_the_build_information() {
    #[derive(Deserialize)]
    struct BuildResponse {
        version: String,
        git_sha: String,
        build_date: String,
    }

    #[derive(Deserialize)]
    struct Response {
        message: String,
        build: BuildResponse,
    }

    let services = configure_services();

    let router = router().with_state(services);
    let (_server, client) = api_server(router).await;

    let response: Response = client
        .get("/api/")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(response.message, "Hello from Netheril");

    assert_eq!(response.build.version, BUILD.version);
    assert_eq!(response.build.git_sha, BUILD.git_sha);
    assert_eq!(response.build.build_date, BUILD.build_date);
}
