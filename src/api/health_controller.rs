use axum::{routing::get, Router};
use utoipa::OpenApi;

use crate::{
    models::health::{Health, HealthView},
    services::ServiceRegistry,
};

#[derive(OpenApi)]
#[openapi(paths(index))]
pub struct ApiDoc;

pub fn router() -> Router<ServiceRegistry> {
    Router::new().route("/", get(index))
}

#[utoipa::path(
    get,
    path = "/",
    responses(
	(status = OK, body = HealthView)
    )
)]
async fn index() -> HealthView {
    Health::default().into()
}
