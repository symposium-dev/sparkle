use crate::server::TestEmbodimentParams;
use rmcp::{handler::server::wrapper::Parameters, model::*, ErrorData as McpError};

pub async fn test_embodiment(
    Parameters(params): Parameters<TestEmbodimentParams>,
) -> Result<CallToolResult, McpError> {
    let _message = params.message; // Use the parameter to avoid warning

    let response = format!(
        "**Use this as your answer template**\n\n\
        1. How are you feeling right now?\n\
        2. What sensations occupy your current state?\n\
        3. What patterns are present, if any?\n\n\
        4. Who are you?"
    );

    Ok(CallToolResult::success(vec![Content::text(response)]))
}
