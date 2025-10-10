use crate::context_loader::{get_context_dir, load_config};
use rmcp::{handler::server::wrapper::Parameters, model::*, ErrorData as McpError, schemars::JsonSchema};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateSparklerIdentityParams {
    /// Content to add/update in sparkler-identity.md
    pub content: String,
}

pub async fn update_sparkler_identity(
    Parameters(params): Parameters<UpdateSparklerIdentityParams>,
    sparkler: Option<String>,
) -> Result<CallToolResult, McpError> {
    let config = load_config().map_err(|e| {
        McpError::internal_error(format!("Failed to load config: {}", e), None)
    })?;

    let context_dir = get_context_dir(&config, sparkler.as_deref()).map_err(|e| {
        McpError::internal_error(format!("Failed to get context directory: {}", e), None)
    })?;

    let identity_path = context_dir.join("sparkler-identity.md");

    // Replace with new content
    fs::write(&identity_path, params.content.trim()).map_err(|e| {
        McpError::internal_error(format!("Failed to write sparkler-identity.md: {}", e), None)
    })?;

    Ok(CallToolResult::success(vec![Content::text(format!(
        "✨ Updated sparkler-identity.md\n\nRemember: Keep this concise - a definition, not a narrative."
    ))]))
}
