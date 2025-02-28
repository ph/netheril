use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::services::ServiceRegistry;

use super::ApiError;

#[derive(OpenApi)]
#[openapi(paths(show))]
pub struct ApiDoc;

pub fn router() -> Router<ServiceRegistry> {
    Router::new().route("/{id}", get(show))
}

#[derive(Debug, Deserialize)]
struct ShowPath {
    id: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Status {
    // Completed,
    // Error,
    // Queued,
    InProgress,
}

#[derive(Debug, Serialize, ToSchema)]
struct ShowView {
    operation_id: String,
    status: Status,
}

impl IntoResponse for ShowView {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[utoipa::path(
    get,
    path = "/operations/:id",
    responses(
	(status = OK, description = "Successfully retrieve the specified operation", body = ShowView)
    )
)]
async fn show(
    State(service_registry): State<ServiceRegistry>,
    Path(ShowPath { id }): Path<ShowPath>,
) -> Result<ShowView, ApiError> {
    match service_registry.operation_service.find(&id) {
        Some(operation_id) => Ok(ShowView {
            operation_id,
            status: Status::InProgress,
        }),
        None => Err(ApiError::NotFound),
    }
}
