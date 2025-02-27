pub mod operations_controller;
pub mod root_controller;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::services::{OperationService, ServiceRegistry};

fn swagger_ui() -> SwaggerUi {
    #[derive(OpenApi)]
    #[openapi(
	nest(
	    (path = "/api", api = root_controller::ApiDoc),
	    (path = "/api/operations/", api = operations_controller::ApiDoc),
	)
    )]
    struct ApiDoc;

    let doc = ApiDoc::openapi();

    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc)
}

pub fn router() -> Router<ServiceRegistry> {
    Router::new().merge(swagger_ui()).nest(
        "/api/",
        root_controller::router().nest("/operations/", operations_controller::router()),
    )
}
