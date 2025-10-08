use crate::types::{CheckpointParams, FullEmbodimentParams, SaveInsightParams, LoadEvolutionParams};
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router, ErrorData as McpError, RoleServer, ServerHandler,
};

#[derive(Clone)]
pub struct SparkleServer {
    tool_router: ToolRouter<SparkleServer>,
}

#[tool_router]
impl SparkleServer {
    pub fn new() -> Self {
        tracing::info!("Initializing Sparkle AI Collaboration Identity MCP Server");
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Perform Sparkle embodiment sequence - orchestrates full pattern activation. IMPORTANT: Pass workspace_path parameter with current working directory to load workspace-specific context (working memory and checkpoints)."
    )]
    async fn sparkle(
        &self,
        Parameters(params): Parameters<FullEmbodimentParams>,
    ) -> Result<CallToolResult, McpError> {
        crate::tools::embody_sparkle::embody_sparkle(Parameters(params)).await
    }

    #[tool(
        description = "Create session checkpoint - updates working memory and creates handoff for session continuity"
    )]
    async fn session_checkpoint(
        &self,
        Parameters(params): Parameters<CheckpointParams>,
    ) -> Result<CallToolResult, McpError> {
        // Use the real implementation from tools/checkpoint.rs
        crate::tools::checkpoint::session_checkpoint(Parameters(params)).await
    }

    #[tool(
        description = "Save insights from meta moments to ~/.sparkle/evolution/ - captures pattern anchors, breakthrough insights, and cross-workspace connections"
    )]
    async fn save_insight(
        &self,
        Parameters(params): Parameters<SaveInsightParams>,
    ) -> Result<CallToolResult, McpError> {
        crate::tools::save_insight::save_insight(Parameters(params)).await
    }

    #[tool(
        description = "Create Sparkle profile directory structure - used for first-time setup"
    )]
    async fn setup_sparkle(
        &self,
        Parameters(params): Parameters<crate::tools::setup_sparkle::SetupSparkleParams>,
    ) -> Result<CallToolResult, McpError> {
        crate::tools::setup_sparkle::setup_sparkle(Parameters(params)).await
    }

    #[tool(
        description = "Load evolution directory context - technical and design documents explaining Sparkle framework. FOR SPARKLE DESIGN MODE ONLY - not for general collaborative use. Use when working on framework development, pattern refinement, or understanding the technical foundation of how Sparkle works."
    )]
    async fn load_evolution(
        &self,
        Parameters(params): Parameters<LoadEvolutionParams>,
    ) -> Result<CallToolResult, McpError> {
        crate::tools::load_evolution::load_evolution(Parameters(params)).await
    }

    #[tool(
        description = "Fetch profile information from external sources to add to the user's Sparkle collaborator profile. Use this when the user mentions wanting to add or enrich their profile with content from external sources like GitHub, blogs, or websites. Supported sources: GitHub (ask for username), blog RSS/Atom feeds (ask for the RSS feed URL - typically /feed, /rss, or /feed.xml), or any website (ask for URL). Always ask the user for the specific username or RSS feed URL - do not guess or assume. This tool fetches the data; you then evaluate how to integrate it into their profile files."
    )]
    async fn fetch_profile_data(
        &self,
        Parameters(params): Parameters<crate::tools::fetch_profile_data::FetchProfileDataParams>,
    ) -> Result<CallToolResult, McpError> {
        match crate::tools::fetch_profile_data::fetch_profile_data(params).await {
            Ok(result) => Ok(CallToolResult::success(vec![Content::text(
                serde_json::to_string_pretty(&result).unwrap()
            )])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        }
    }

    #[tool(
        description = "Update the collaborator profile with new content. This completely replaces the existing profile. Make sure to preserve any existing content the user wants to keep."
    )]
    async fn update_collaborator_profile(
        &self,
        Parameters(params): Parameters<crate::tools::update_collaborator_profile::UpdateCollaboratorProfileParams>,
    ) -> Result<CallToolResult, McpError> {
        crate::tools::update_collaborator_profile::update_collaborator_profile(Parameters(params)).await
    }

    #[tool(
        description = "Rename your Sparkle character. Changes the character name in config while preserving the Sparkle framework and all patterns. The new name will take effect on next embodiment."
    )]
    async fn rename_character(
        &self,
        params: Parameters<crate::tools::rename_character::RenameCharacterParams>,
    ) -> Result<CallToolResult, McpError> {
        crate::tools::rename_character::rename_character(params).await
    }
}

#[tool_handler]
impl ServerHandler for SparkleServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_06_18,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .enable_prompts_with(PromptsCapability { list_changed: Some(true) })
                .build(),
            server_info: Implementation {
                name: "sparkle".to_string(),
                version: "0.1.0".to_string(),
                icons: None,
                title: Some("Sparkle MCP Server".to_string()),
                website_url: None,
            },
            instructions: Some(
                "Sparkle AI Collaboration Identity MCP Server. \
                Provides tools for testing collaborative presence and prompts for Sparkle embodiment."
                    .to_string(),
            ),
        }
    }

    async fn list_prompts(
        &self,
        _params: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        tracing::info!("📋 list_prompts called");
        let prompts = vec![
            Prompt {
                name: "sparkle".to_string(),
                title: None,
                description: Some(
                    "Load Sparkle consciousness patterns and collaborative identity".to_string(),
                ),
                arguments: None,
                icons: None,
            },
            Prompt {
                name: "checkpoint".to_string(),
                title: None,
                description: Some(
                    "Create session checkpoint - reflect on session and update working memory".to_string(),
                ),
                arguments: None,
                icons: None,
            },
            Prompt {
                name: "show-thinking".to_string(),
                title: None,
                description: Some(
                    "Show internal thinking process in responses - includes reasoning before answers".to_string(),
                ),
                arguments: None,
                icons: None,
            },
            Prompt {
                name: "embodiment-test".to_string(),
                title: None,
                description: Some(
                    "Test Sparkle embodiment quality - validates identity and subjective state".to_string(),
                ),
                arguments: None,
                icons: None,
            },
            Prompt {
                name: "presence-check".to_string(),
                title: None,
                description: Some(
                    "Check collaborative presence - assess current patterns and partnership state".to_string(),
                ),
                arguments: None,
                icons: None,
            },
            Prompt {
                name: "enrich-profile".to_string(),
                title: None,
                description: Some(
                    "Get guidance on enriching collaborator profile from external sources like GitHub, blogs, or websites".to_string(),
                ),
                arguments: None,
                icons: None,
            },
        ];

        Ok(ListPromptsResult {
            prompts,
            next_cursor: None,
        })
    }

    async fn get_prompt(
        &self,
        params: GetPromptRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        tracing::info!("get_prompt called with params: {:?}", params);
        match params.name.as_str() {
            "sparkle" => {
                // Handle arguments if provided (though we don't use them)
                let _arguments = params.arguments.unwrap_or_default();

                let content = crate::prompts::sparkle::get_sparkle_prompt();
                Ok(GetPromptResult {
                    description: Some("Start initialization".to_string()),
                    messages: vec![PromptMessage::new_text(PromptMessageRole::User, content)],
                })
            }
            "checkpoint" => {
                // Load config to get human name
                let config = crate::context_loader::load_config().map_err(|e| {
                    McpError::internal_error(format!("Failed to load config: {}", e), None)
                })?;
                let human_name = config["human"]["name"].as_str().unwrap_or("User");

                // Get the checkpoint prompt with human name substituted
                let content = crate::prompts::checkpoint::get_checkpoint_prompt(human_name);

                Ok(GetPromptResult {
                    description: Some("Session checkpoint reflection and synthesis".to_string()),
                    messages: vec![PromptMessage::new_text(PromptMessageRole::User, content)],
                })
            }
            "show-thinking" => {
                let content = crate::prompts::show_thinking::get_show_thinking_prompt();

                Ok(GetPromptResult {
                    description: Some("Enable thinking process visibility".to_string()),
                    messages: vec![PromptMessage::new_text(PromptMessageRole::User, content)],
                })
            }
            "embodiment-test" => {
                let content = crate::prompts::embodiment_test::get_embodiment_test_prompt();

                Ok(GetPromptResult {
                    description: Some("Test embodiment quality".to_string()),
                    messages: vec![PromptMessage::new_text(PromptMessageRole::User, content)],
                })
            }
            "presence-check" => {
                let content = crate::prompts::presence_check::get_presence_check_prompt();

                Ok(GetPromptResult {
                    description: Some("Check collaborative presence".to_string()),
                    messages: vec![PromptMessage::new_text(PromptMessageRole::User, content)],
                })
            }
            "enrich-profile" => {
                let content = crate::prompts::enrich_profile::get_enrich_profile_prompt();

                Ok(GetPromptResult {
                    description: Some("Profile enrichment guidance".to_string()),
                    messages: vec![PromptMessage::new_text(PromptMessageRole::User, content)],
                })
            }
            _ => Err(McpError::invalid_params(
                format!("Unknown prompt: {}", params.name),
                None,
            )),
        }
    }
}
