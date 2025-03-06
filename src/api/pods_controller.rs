use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router, debug_handler};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};

use crate::services::{operation_service::OperationId, ServiceRegistry};

use super::ApiError;

#[derive(OpenApi)]
#[openapi(paths(create))]
pub struct ApiDoc;

pub fn router() -> Router<ServiceRegistry> {
    Router::new().route("/", post(create))
}

#[derive(Debug, Serialize, ToSchema)]
struct NewOperationView{
    operation_id: OperationId,
}

impl IntoResponse for NewOperationView{
    fn into_response(self) -> axum::response::Response {
	(StatusCode::CREATED, Json(self)).into_response()
    }
}

#[debug_handler]
#[utoipa::path(
    post,
    path = "/operations/",
    responses(
	(status = CREATED, description = "Schedule a new pod on the runtime", body = NewOperationView)
    )
)]
async fn create(State(_service_registry): State<ServiceRegistry>) -> Result<NewOperationView, ApiError> {
    println!("create");
    Ok(NewOperationView {
	operation_id: OperationId::generate(),
    })
}
