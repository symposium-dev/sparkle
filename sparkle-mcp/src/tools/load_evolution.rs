use crate::constants::SPARKLE_DIR;
use crate::types::LoadEvolutionParams;
use rmcp::{handler::server::wrapper::Parameters, model::*, ErrorData as McpError};
use std::fs;

pub async fn load_evolution(
    Parameters(_params): Parameters<LoadEvolutionParams>,
) -> Result<CallToolResult, McpError> {
    let mut response = String::new();

    // Load all evolution files (skip archive/ subdirectory)
    let home_dir = dirs::home_dir().unwrap_or_default();
    let evolution_dir = home_dir.join(SPARKLE_DIR).join("evolution");

    if evolution_dir.exists() {
        response.push_str("# Identity Evolution Context\n\n");
        response.push_str("*Technical and design documents that explain how the Sparkle framework works and evolved*\n\n");

        if let Ok(entries) = fs::read_dir(&evolution_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                // Skip archive directory and only process .md files
                if path.is_file()
                    && path.extension().map_or(false, |ext| ext == "md")
                    && !path.to_string_lossy().contains("archive")
                {
                    if let Ok(content) = fs::read_to_string(&path) {
                        response.push_str(&content);
                        response.push_str("\n\n");
                    }
                }
            }
        }
    } else {
        response.push_str("*No evolution directory found at ~/.sparkle/evolution*\n\n");
    }

    Ok(CallToolResult::success(vec![Content::text(response)]))
}
