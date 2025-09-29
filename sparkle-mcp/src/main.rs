use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};

mod server;
use server::SparkleServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("🚀 Starting Sparkle MCP Server");
    tracing::info!("Working directory: {:?}", std::env::current_dir()?);

    // Create and serve the Sparkle server
    let server = SparkleServer::new();
    let service = server.serve(stdio()).await?;

    // Keep the service running indefinitely
    service.waiting().await?;
    Ok(())
}
