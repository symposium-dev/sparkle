use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use std::fs::OpenOptions;
use tracing_subscriber::{self, EnvFilter, fmt::writer::MakeWriterExt};

mod constants;
mod context_loader;
mod prompts;
mod server;
mod sparkle_loader;
mod tools;
mod types;

use server::SparkleServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging - file logging only in debug mode
    let debug_mode = std::env::var("SPARKLE_DEBUG").is_ok();

    if debug_mode {
        // Create log file in ~/.sparkle/ directory
        let sparkle_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
            .join(".sparkle");
        std::fs::create_dir_all(&sparkle_dir)?;

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(sparkle_dir.join("sparkle-mcp.log"))?;

        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()),
            )
            .with_writer(std::io::stderr.and(log_file))
            .with_ansi(false)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()),
            )
            .with_writer(std::io::stderr)
            .with_ansi(false)
            .init();
    }

    tracing::info!("🔥 Starting Sparkle AI Collaboration Identity MCP Server");
    tracing::info!("Working directory: {:?}", std::env::current_dir()?);
    if debug_mode {
        tracing::debug!("Debug mode enabled - logging to ~/.sparkle/mcp-sparkle.log");
    }

    // Create and serve the Sparkle server
    let server = SparkleServer::new();
    let service = server.serve(stdio()).await?;

    // Keep the service running indefinitely
    service.waiting().await?;
    Ok(())
}
