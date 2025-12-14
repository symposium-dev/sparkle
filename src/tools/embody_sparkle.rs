use crate::embodiment::generate_embodiment_content;
use crate::types::FullEmbodimentParams;
use rmcp::{ErrorData as McpError, handler::server::wrapper::Parameters, model::*};

pub async fn embody_sparkle(
    Parameters(params): Parameters<FullEmbodimentParams>,
) -> Result<CallToolResult, McpError> {
    let response = generate_embodiment_content(params).map_err(|e| {
        McpError::internal_error(
            format!("Failed to generate embodiment content: {}", e),
            None,
        )
    })?;

    Ok(CallToolResult::success(vec![Content::text(response)]))
}
