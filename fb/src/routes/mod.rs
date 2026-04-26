pub mod exception;
pub mod fizzbuzz;
pub mod login;

use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::Router;
use tower_http::trace::TraceLayer;

use crate::resources::Resources;

pub fn build_router(app: Resources) -> Router {
    Router::new()
        .route("/health", get(|| async { StatusCode::OK }))
        .route("/fizzbuzz/:n", get(fizzbuzz::get_fizzbuzz))
        .route(
            "/exceptions",
            get(exception::get_exception_list).post(exception::post_exception),
        )
        .route("/exceptions/:n", get(exception::get_exception))
        .route("/login", post(login::post_login))
        .with_state(app)
        .layer(TraceLayer::new_for_http())
}