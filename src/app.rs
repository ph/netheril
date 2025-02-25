use axum::Router;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    error::NetherilErr,
    logging::{Logging, LoggingOptions},
};

pub struct App {
    #[allow(dead_code)]
    logging: Logging,
}

impl App {
    pub fn new() -> Self {
        info!("configuring");
        let logging = Logging::new(LoggingOptions::default());
        App { logging }
    }

    pub async fn run(&self) -> Result<(), Box<NetherilErr>> {
        info!("starting");

        let router = router();

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .map_err(|e| NetherilErr::Api(e.to_string()))?;

        axum::serve(listener, router)
            .await
            .map_err(|e| NetherilErr::Api(e.to_string()))?;

        Ok(())
    }
}

fn router() -> Router {
    //.merge(self.swagger_ui())
    Router::new()
        .merge(swagger_ui())
        .nest("/api/", root::router())
}

pub fn swagger_ui() -> SwaggerUi {
    #[derive(OpenApi)]
    #[openapi(
	nest(
	    (path = "/api", api = root::ApiDoc)
	)
    )]
    struct ApiDoc;

    let doc = ApiDoc::openapi();

    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc)
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

mod root {
    use axum::{http::StatusCode, routing::get, Json, Router};
    use serde::Serialize;
    use utoipa::{OpenApi, ToSchema};

    use crate::version::{self, Build};

    #[derive(OpenApi)]
    #[openapi(paths(index))]
    pub struct ApiDoc;

    pub fn router() -> Router {
        Router::new().route("/", get(index))
    }

    #[derive(Serialize, ToSchema)]
    pub struct RootResponse {
        message: &'static str,
        build: Build,
    }

    impl Default for RootResponse {
        fn default() -> Self {
            RootResponse {
                message: "Hello from Netheril",
                build: version::BUILD,
            }
        }
    }

    #[utoipa::path(
	get,
	path = "/",
	responses(
	    (status = OK, body = RootResponse)
	)
    )]
    async fn index() -> (StatusCode, Json<RootResponse>) {
        (StatusCode::OK, Json(RootResponse::default()))
    }
}

#[cfg(test)]
mod test {
    use crate::version::BUILD;
    use reqwest::RequestBuilder;
    use serde::Deserialize;
    use std::net::SocketAddr;
    use tokio::task::JoinHandle;

    use super::*;

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

        let (_server, client) = TestServer::new(router()).await.unwrap();

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
}
