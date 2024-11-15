use anyhow::Context;
use dotenv::dotenv;
use secure_task::{config::CONFIG, router::router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let addr = format!("{}:{}", CONFIG.host, CONFIG.port);

    tracing::info!("Running on http://{addr}");

    let listener = TcpListener::bind(addr)
        .await
        .context("Error at tcp listener")?;

    axum::serve(listener, router())
        .await
        .context("Error at starting server")?;
    Ok(())
}
