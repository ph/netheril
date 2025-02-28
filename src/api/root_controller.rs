use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};

use crate::{
    services::ServiceRegistry,
    version::{self, Build},
};

#[derive(OpenApi)]
#[openapi(paths(index))]
pub struct ApiDoc;

pub fn router() -> Router<ServiceRegistry> {
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
