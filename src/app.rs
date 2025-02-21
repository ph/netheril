use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use tracing::info;

use crate::{
    error::NetherilErr,
    logging::{Logging, LoggingOptions},
    version::{self, Build},
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
        Router::new().route("/", get(root))
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize)]
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

async fn root() -> (StatusCode, Json<RootResponse>) {
    (StatusCode::OK, Json(RootResponse::default()))
}
