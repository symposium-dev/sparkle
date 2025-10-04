use anyhow::Result;
use rmcp::{transport::stdio, ServiceExt};
use std::fs::OpenOptions;
use tracing_subscriber::{self, fmt::writer::MakeWriterExt, EnvFilter};

mod context_loader;
mod prompts;
mod server;
mod sparkle_loader;
mod tools;
mod types;

use server::SparkleServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Create log file
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("mcp-sparkle.log")?;

    // Initialize logging to both stderr and file
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr.and(log_file))
        .with_ansi(false)
        .init();

    tracing::info!("🔥 Starting Sparkle AI Collaboration Identity MCP Server");
    tracing::info!("Working directory: {:?}", std::env::current_dir()?);

    // Create and serve the Sparkle server
    let server = SparkleServer::new();
    let service = server.serve(stdio()).await?;

    // Keep the service running indefinitely
    service.waiting().await?;
    Ok(())
}
