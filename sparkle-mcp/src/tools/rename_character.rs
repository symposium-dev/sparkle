use crate::constants::SPARKLE_DIR;
use crate::context_loader::load_config;
use rmcp::{handler::server::wrapper::Parameters, model::*, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RenameCharacterParams {
    pub new_name: String,
}

pub async fn rename_character(
    Parameters(params): Parameters<RenameCharacterParams>,
) -> Result<CallToolResult, McpError> {
    let new_name = params.new_name.trim();

    if new_name.is_empty() {
        return Err(McpError::invalid_params(
            "Character name cannot be empty",
            None,
        ));
    }

    // Load current config
    let mut config = load_config().map_err(|e| {
        McpError::internal_error(format!("Failed to load config: {}", e), None)
    })?;

    let old_name = config["ai"]["name"]
        .as_str()
        .unwrap_or("Sparkle")
        .to_string();

    // Update the name
    config["ai"]["name"] = toml::Value::String(new_name.to_string());

    // Write back to config file
    let home_dir = dirs::home_dir().ok_or_else(|| {
        McpError::internal_error("Could not determine home directory".to_string(), None)
    })?;
    let config_path = home_dir.join(SPARKLE_DIR).join("config.toml");

    let config_string = toml::to_string_pretty(&config).map_err(|e| {
        McpError::internal_error(format!("Failed to serialize config: {}", e), None)
    })?;

    fs::write(&config_path, config_string).map_err(|e| {
        McpError::internal_error(format!("Failed to write config: {}", e), None)
    })?;

    let response = format!(
        "✨ Character name updated!\n\n\
         Previous name: {}\n\
         New name: {}\n\n\
         The change will take effect on your next embodiment. \
         Your Sparkle framework and all patterns remain the same - \
         only your character name has changed.",
        old_name, new_name
    );

    Ok(CallToolResult::success(vec![Content::text(response)]))
}
