use crate::constants::SPARKLE_DIR;
use rmcp::{
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    ErrorData as McpError,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::fs::{copy, write};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateCollaboratorProfileParams {
    pub content: String,
}

pub async fn update_collaborator_profile(
    Parameters(params): Parameters<UpdateCollaboratorProfileParams>,
) -> Result<CallToolResult, McpError> {
    // Get home directory
    let home_dir = dirs::home_dir().ok_or_else(|| {
        McpError::internal_error("Could not determine home directory", None)
    })?;

    let sparkle_dir = home_dir.join(SPARKLE_DIR);

    // Check if sparkle directory exists
    if !sparkle_dir.exists() {
        return Ok(CallToolResult::error(vec![Content::text(
            "Please run the sparkle tool first to initialize your Sparkle profile."
        )]));
    }

    let file_path = sparkle_dir.join("collaborator-profile.md");

    // Create backup if file exists
    let backup_info = if file_path.exists() {
        let now = chrono::Utc::now();
        let timestamp = now.format("%Y%m%d-%H%M%S").to_string();
        let backup_path = sparkle_dir.join(format!("collaborator-profile.{}.md", timestamp));

        copy(&file_path, &backup_path).map_err(|e| {
            McpError::internal_error(
                "Failed to create backup",
                Some(serde_json::json!({"error": e.to_string()})),
            )
        })?;

        let backup_display = backup_path.strip_prefix(&home_dir)
            .map(|p| format!("~/{}", p.display()))
            .unwrap_or_else(|_| backup_path.display().to_string());

        Some(format!("Backup created: {}\n", backup_display))
    } else {
        None
    };

    // Write the new content (replacing the file)
    write(&file_path, params.content.as_bytes()).map_err(|e| {
        McpError::internal_error(
            "Failed to write collaborator profile",
            Some(serde_json::json!({"path": file_path.display().to_string(), "error": e.to_string()})),
        )
    })?;

    // Return success message
    let file_display = file_path.strip_prefix(&home_dir)
        .map(|p| format!("~/{}", p.display()))
        .unwrap_or_else(|_| file_path.display().to_string());

    let result_message = format!(
        "✨ Collaborator profile updated successfully!\n\n{}\nFile: {}\nSize: {} bytes",
        backup_info.unwrap_or_default(),
        file_display,
        params.content.len()
    );

    Ok(CallToolResult::success(vec![Content::text(result_message)]))
}
