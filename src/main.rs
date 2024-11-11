use axum::{routing::get, Router};
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

async fn index() -> &'static str {
    "Server is Running"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let service_builder = ServiceBuilder::new().layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("localhost:3000").await?;
    let router = Router::new().route("/", get(index)).layer(service_builder);
    axum::serve(listener, router).await?;
    Ok(())
}
