use std::net::SocketAddr;
use netheril::{api::router, services::{OperationService, ServiceRegistry}, version::BUILD};
use reqwest::RequestBuilder;
use serde::Deserialize;
use tokio::{task::JoinHandle};
use axum::Router;

#[derive(Debug)]
struct Client {
    addr: SocketAddr,
    client: reqwest::Client,
}

impl Client {
    fn new(addr: SocketAddr) -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder().build()?;
        Ok(Self { addr, client })
    }

    fn get<R: Into<RelativeUrl>>(&self, path: R) -> RequestBuilder {
        let url = self.base_url(path.into());
        self.client.get(url)
    }

    fn base_url(&self, path: RelativeUrl) -> String {
        format!("http://{}:{}{}", self.addr.ip(), self.addr.port(), path)
    }
}

#[derive(Debug)]
struct RelativeUrl(String);

impl std::fmt::Display for RelativeUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for RelativeUrl {
    fn from(value: &str) -> Self {
        const HTTP_PREFIX: &str = "http://";
        const HTTPS_PREFIX: &str = "https://";

        let candidate = value.to_lowercase();

        if candidate.starts_with(HTTP_PREFIX) && candidate.starts_with(HTTPS_PREFIX) {
            panic!("bad relative url: `{}`", value)
        }

	RelativeUrl(value.to_string())
    }
}

#[derive(Debug)]
struct TestServer {
    server_handle: JoinHandle<()>,
}

impl TestServer {
    async fn new(router: Router) -> Result<(Self, Client), Box<dyn std::error::Error>> {
        const ANY_LOCAL_PORT: &str = "0.0.0.0:0";

        let listener = tokio::net::TcpListener::bind(ANY_LOCAL_PORT).await?;
        let addr = listener.local_addr()?;

        let server_handle = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        Ok((Self { server_handle }, Client::new(addr)?))
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.server_handle.abort()
    }
}

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

    let services = ServiceRegistry {
	operation_service: OperationService::new(),
    };

    let router = router().with_state(services);
    let (_server, client) = TestServer::new(router).await.unwrap();

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
