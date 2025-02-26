use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(paths(show))]
pub struct ApiDoc;

pub fn router() -> Router {
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
struct ShowResponse {
    operation_id: String,
    status: Status,
}

#[utoipa::path(
    get,
    path = "/operations/:id",
    responses(
	(status = OK, description = "Successfully retrieve the specified operation", body = ShowResponse)
    )
)]
async fn show(Path(ShowPath { id }): Path<ShowPath>) -> (StatusCode, Json<ShowResponse>) {
    (
        StatusCode::OK,
        Json(ShowResponse {
            operation_id: id,
            status: Status::InProgress,
        }),
    )
}
