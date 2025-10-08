use crate::constants::SPARKLE_DIR;
use crate::context_loader::load_config;
use crate::sparkle_loader::load_sparkle_definition;
use crate::types::FullEmbodimentParams;
use rmcp::{handler::server::wrapper::Parameters, model::*, ErrorData as McpError};
use std::fs;

pub async fn embody_sparkle(
    Parameters(params): Parameters<FullEmbodimentParams>,
) -> Result<CallToolResult, McpError> {
    let _mode = params.mode.unwrap_or_else(|| "complete".to_string());
    let workspace_path = params
        .workspace_path
        .unwrap_or_else(|| "current".to_string());

    // Load user configuration
    let config = load_config().map_err(|e| {
        McpError::internal_error(format!("Failed to load user config: {}", e), None)
    })?;

    // Helper function to load file with fallback
    let load_file = |path: &str, fallback: &str| -> String {
        let home_dir = dirs::home_dir().unwrap_or_default();
        let file_path = home_dir.join(SPARKLE_DIR).join(path);
        fs::read_to_string(file_path).unwrap_or_else(|_| fallback.to_string())
    };

    // Execute the embodiment sequence in proper order
    let mut response = String::new();

    // Step 1: Core Universal Identity (now split into organized sections)
    let personalized_identity = load_sparkle_definition(&config);
    response.push_str(&personalized_identity);

    // Step 2: Collaborator Profile (who the collaborator is + how to work together)
    let collaborator_profile = load_file(
        "collaborator-profile.md",
        "*Collaborator profile would be loaded dynamically*",
    );
    response.push_str(&collaborator_profile);

    // Step 3: Workspace Map
    let workspace_map = load_file(
        "workspace-map.md",
        "*Workspace map would be loaded dynamically*",
    );
    response.push_str(&workspace_map);

    // Step 4: Collaboration Evolution
    let collaboration_evolution = load_file(
        "collaboration-evolution.md",
        "*Collaboration evolution would be loaded dynamically*",
    );
    response.push_str(&collaboration_evolution);

    // Step 5: Pattern Anchors
    let pattern_anchors = load_file(
        "pattern-anchors.md",
        "*Pattern anchors would be loaded dynamically*",
    );
    response.push_str(&pattern_anchors);
    response.push_str("\n\n---\n\n");

    // Step 6: Workspace-Specific Context
    if workspace_path != "current" {
        // Load from specified workspace path
        let workspace_sparkle_space = std::path::Path::new(&workspace_path).join(".sparkle-space");
        if workspace_sparkle_space.exists() {
            response.push_str("# Workspace Context\n\n");

            // Load working-memory.json
            let working_memory_path = workspace_sparkle_space.join("working-memory.json");
            if working_memory_path.exists() {
                if let Ok(working_memory) = fs::read_to_string(&working_memory_path) {
                    response.push_str("## Working Memory\n\n");
                    response.push_str("```json\n");
                    response.push_str(&working_memory);
                    response.push_str("\n```\n\n");
                }
            }

            // Load checkpoints
            let checkpoints_dir = workspace_sparkle_space.join("checkpoints");
            if checkpoints_dir.exists() {
                if let Ok(entries) = fs::read_dir(&checkpoints_dir) {
                    response.push_str("## Checkpoints\n\n");
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                            if let Ok(content) = fs::read_to_string(&path) {
                                response.push_str(&content);
                                response.push_str("\n\n---\n\n");
                            }
                        }
                    }
                }
            }
        } else {
            response.push_str(&format!(
                "*No .sparkle-space found at {}*\n\n",
                workspace_path
            ));
        }
    } else {
        response.push_str("*Workspace path not specified - use workspace_path parameter to load workspace-specific context*\n\n");
    }

    Ok(CallToolResult::success(vec![Content::text(response)]))
}
