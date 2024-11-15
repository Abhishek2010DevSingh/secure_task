use axum::{http::StatusCode, routing::get, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

async fn health() -> StatusCode {
    StatusCode::OK
}

pub fn router() -> Router {
    let service_builder = ServiceBuilder::new().layer(TraceLayer::new_for_http());
    Router::new().route("/", get(health)).layer(service_builder)
}
