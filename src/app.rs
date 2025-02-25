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

        let router = self.router();

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .map_err(|e| NetherilErr::Api(e.to_string()))?;

        axum::serve(listener, router)
            .await
            .map_err(|e| NetherilErr::Api(e.to_string()))?;

        Ok(())
    }

    pub fn router(&self) -> Router {
        //.merge(self.swagger_ui())
        Router::new()
            .merge(self.swagger_ui())
            .nest("/api/", root::router())
    }

    pub fn swagger_ui(&self) -> SwaggerUi {
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
    struct RootResponse {
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
