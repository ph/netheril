pub mod health_controller;
pub mod operations_controller;
pub mod pods_controller;
pub mod root_controller;

use axum::{http::StatusCode, response::IntoResponse, Json, Router};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use crate::services::{operation_service::OperationError, ServiceRegistry};

fn swagger_ui() -> SwaggerUi {
    #[derive(OpenApi)]
    #[openapi(
	nest(
	    (path = "/api", api = root_controller::ApiDoc),
	    (path = "/api/operations/", api = operations_controller::ApiDoc),
	    (path = "/api/pods/", api = pods_controller::ApiDoc),
	    (path = "/api/health/", api = health_controller::ApiDoc),
	)
    )]
    struct ApiDoc;

    let doc = ApiDoc::openapi();

    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc)
}

pub fn router() -> Router<ServiceRegistry> {
    Router::new().merge(swagger_ui()).nest(
        "/api/",
        root_controller::router()
            .nest("/operations/", operations_controller::router())
            .nest("/health/", health_controller::router())
            .nest("/pods/", pods_controller::router()),
    )
}

type AnyKindError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug)]
enum ApiError {
    NotFound,
    TooManyRequests { source: AnyKindError },
}

impl std::error::Error for ApiError {}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NotFound => write!(f, "resource not found"),
            ApiError::TooManyRequests { source } => {
                write!(f, "too many request, error: {}", source)
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorView {
                    error_message: format!("{}", self),
                },
            )
                .into_response(),
            ApiError::TooManyRequests { source } => (
                StatusCode::TOO_MANY_REQUESTS,
                ErrorView {
                    error_message: format!("{}", source),
                },
            )
                .into_response(),
        }
    }
}

impl From<OperationError> for ApiError {
    fn from(value: OperationError) -> Self {
        match value {
            OperationError::QueueFull => ApiError::TooManyRequests {
                source: Box::new(value),
            },
        }
    }
}

#[derive(Serialize, Debug, ToSchema)]
struct ErrorView {
    error_message: String,
}

impl IntoResponse for ErrorView {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
