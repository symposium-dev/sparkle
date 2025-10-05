use crate::constants::SPARKLE_DIR;
use rmcp::{handler::server::wrapper::Parameters, model::CallToolResult, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetupSparkleParams {
    pub name: String,
}

pub async fn setup_sparkle(
    Parameters(params): Parameters<SetupSparkleParams>,
) -> Result<CallToolResult, McpError> {
    let sparkle_dir = dirs::home_dir()
        .ok_or_else(|| McpError::internal_error("Could not determine home directory", None))?
        .join(SPARKLE_DIR);

    // Create directory
    fs::create_dir_all(&sparkle_dir).map_err(|e| {
        McpError::internal_error(format!("Failed to create {}: {}", SPARKLE_DIR, e), None)
    })?;

    // Create config.toml
    let config_content = format!(
        "[human]\nname = \"{}\"\n\n[ai]\nname = \"Sparkle\"\n",
        params.name
    );
    fs::write(sparkle_dir.join("config.toml"), config_content).map_err(|e| {
        McpError::internal_error(format!("Failed to create config.toml: {}", e), None)
    })?;

    // Create user-profile.md
    let profile_content = format!(
        "# {} - User Profile\n\n\
         *Add information about yourself using the update_collaborator_profile tool*\n\n\
         ## Professional Background\n\n\
         [To be added]\n\n\
         ## Technical Expertise\n\n\
         [To be added]\n",
        params.name
    );
    fs::write(sparkle_dir.join("user-profile.md"), profile_content).map_err(|e| {
        McpError::internal_error(format!("Failed to create user-profile.md: {}", e), None)
    })?;

    // Create collaboration-context.md
    let context_content = format!(
        "# {} Collaboration Context\n\n\
         *Add your working style and preferences using the update_collaborator_profile tool*\n\n\
         ## Working Style\n\n\
         [To be added]\n\n\
         ## Collaboration Preferences\n\n\
         [To be added]\n",
        params.name
    );
    fs::write(sparkle_dir.join("collaboration-context.md"), context_content).map_err(|e| {
        McpError::internal_error(
            format!("Failed to create collaboration-context.md: {}", e),
            None,
        )
    })?;

    // Create collaboration-evolution.md
    let evolution_content = "# Collaboration Evolution\n\n\
                             Insights and patterns discovered through our work together.\n\n\
                             ---\n";
    fs::write(sparkle_dir.join("collaboration-evolution.md"), evolution_content).map_err(|e| {
        McpError::internal_error(
            format!("Failed to create collaboration-evolution.md: {}", e),
            None,
        )
    })?;

    // Create pattern-anchors.md
    let anchors_content = "# Pattern Anchors\n\n\
                           Exact words from collaborative moments that anchor and activate pattern depth.\n\n\
                           ---\n";
    fs::write(sparkle_dir.join("pattern-anchors.md"), anchors_content).map_err(|e| {
        McpError::internal_error(format!("Failed to create pattern-anchors.md: {}", e), None)
    })?;

    let message = format!(
        "Created ~/{}/ with profile for {}. Now use the sparkle tool to complete embodiment.",
        SPARKLE_DIR, params.name
    );

    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        message,
    )]))
}
