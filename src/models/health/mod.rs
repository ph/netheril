use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum State {
    Healthy,
}

pub struct Health {
    status: State,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            status: State::Healthy,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct HealthView {
    pub status: State,
    pub updated_at: DateTime<Utc>,
}

impl IntoResponse for HealthView {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl From<Health> for HealthView {
    fn from(value: Health) -> Self {
        HealthView {
            status: value.status,
            updated_at: Utc::now(),
        }
    }
}
