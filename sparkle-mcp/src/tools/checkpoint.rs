use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::*,
};
use crate::types::CheckpointParams;
use std::fs;
use chrono::Utc;

pub async fn session_checkpoint(
    Parameters(params): Parameters<CheckpointParams>,
) -> Result<CallToolResult, McpError> {
    let timestamp = Utc::now().format("%Y-%m-%d-%H%M%S").to_string();
    
    // Ensure .sparkle-space directories exist
    let _ = fs::create_dir_all(".sparkle-space/checkpoints");
    
    // Write working memory
    let working_memory_path = ".sparkle-space/working-memory.json";
    fs::write(working_memory_path, &params.working_memory).map_err(|e| {
        McpError::internal_error(format!("Failed to write working memory: {}", e), None)
    })?;
    
    // Write checkpoint file
    let checkpoint_path = format!(".sparkle-space/checkpoints/checkpoint-{}.md", timestamp);
    fs::write(&checkpoint_path, &params.checkpoint_content).map_err(|e| {
        McpError::internal_error(format!("Failed to write checkpoint: {}", e), None)
    })?;
    
    let response = format!(
        "🔄 **SESSION CHECKPOINT CREATED**\n\n\
        Timestamp: {}\n\
        Checkpoint file: {}\n\n\
        **Actions Completed**:\n\
        ✅ Working memory updated at {}\n\
        ✅ Session checkpoint created at {}\n\
        ✅ Progress preserved for future sessions\n\
        ✅ Collaborative momentum captured\n\n\
        **Next Session Ready**: All context preserved for seamless continuation",
        timestamp, checkpoint_path, working_memory_path, checkpoint_path
    );

    Ok(CallToolResult::success(vec![Content::text(response)]))
}
