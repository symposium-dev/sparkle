use crate::context_loader::load_config;
use rmcp::{
    ErrorData as McpError, handler::server::wrapper::Parameters, model::*, schemars::JsonSchema,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListSparklersParams {}

pub async fn list_sparklers(
    Parameters(_params): Parameters<ListSparklersParams>,
) -> Result<CallToolResult, McpError> {
    // Load config
    let config = load_config()
        .map_err(|e| McpError::internal_error(format!("Failed to load config: {}", e), None))?;

    let mut response = String::from("**Available Sparklers:**\n\n");

    if config.is_multi_sparkler() {
        // Multi-sparkler mode: list all sparklers
        if let Some(sparklers) = &config.sparklers {
            for sparkler in sparklers {
                if sparkler.default {
                    response.push_str(&format!("• {} (default)\n", sparkler.name));
                } else {
                    response.push_str(&format!("• {}\n", sparkler.name));
                }
            }
        }
    } else {
        // Single-sparkler mode: show current sparkler
        let name = config
            .get_single_sparkler_name()
            .unwrap_or_else(|| "Sparkle".to_string());
        response.push_str(&format!("• {} (single-sparkler mode)\n", name));
    }

    Ok(CallToolResult::success(vec![Content::text(response)]))
}
