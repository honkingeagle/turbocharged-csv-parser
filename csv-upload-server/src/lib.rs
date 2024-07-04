mod csv;
mod errors;

use axum::{
    extract::{DefaultBodyLimit, Multipart, State},
    http::{HeaderValue, HeaderName, Method, header::{ACCESS_CONTROL_ALLOW_ORIGIN}},
    http::StatusCode,
    routing::post,
    Router,
};
use tower_http::cors::CorsLayer;
use errors::ApiError;
use sqlx::MySqlPool;
use std::sync::Arc;

type SharedState = Arc<AppState>;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub frontend_url: String
}

impl AppState {
    pub fn new(pool: MySqlPool, frontend_url: String) -> AppState {
        AppState { pool, frontend_url }
    }
}

async fn upload_csv(
    State(state): State<SharedState>,
    mut multipart: Multipart,
) -> Result<StatusCode, ApiError> {
    while let Some(field) = multipart.next_field().await? {
        if let Some("text/csv") = field.content_type() {
            let data = field.text().await?;

            csv::process_csv(state, data).await?;

            return Ok(StatusCode::OK);
        } else {
            return Err(ApiError::ContentType);
        }
    }

    Err(ApiError::ZeroFieldsProvided)
}

pub fn create_router(state: SharedState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_headers([ACCESS_CONTROL_ALLOW_ORIGIN, "x-requested-with".parse::<HeaderName>().unwrap()])
        .allow_origin(state.frontend_url.parse::<HeaderValue>().unwrap());

    Router::new()
        .route("/upload", post(upload_csv))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 50))
        .with_state(state)
        .layer(cors)
}
