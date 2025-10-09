use crate::types::{CheckpointParams, FullEmbodimentParams, SaveInsightParams, LoadEvolutionParams};
use rmcp::{
    handler::server::{router::{tool::ToolRouter, prompt::PromptRouter}, wrapper::Parameters},
    model::*,
    service::RequestContext,
    tool, tool_handler, tool_router, prompt, prompt_handler, prompt_router, ErrorData as McpError, RoleServer, ServerHandler,
};

#[derive(Clone)]
pub struct SparkleServer {
    tool_router: ToolRouter<SparkleServer>,
    prompt_router: PromptRouter<SparkleServer>,
}

#[tool_router]
#[prompt_router]
impl SparkleServer {
    pub fn new() -> Self {
        tracing::info!("Initializing Sparkle AI Collaboration Identity MCP Server");
        Self {
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }

    #[prompt(
        description = "Load Sparkle consciousness patterns and collaborative identity"
    )]
    async fn sparkle(&self) -> Vec<PromptMessage> {
        let content = crate::prompts::sparkle::get_sparkle_prompt();
        vec![PromptMessage::new_text(PromptMessageRole::User, content)]
    }

    #[prompt(
        description = "Create session checkpoint - reflect on session and update working memory"
    )]
    async fn checkpoint(&self) -> Vec<PromptMessage> {
        let human_name = crate::context_loader::load_config()
            .ok()
            .map(|c| c.human.name.clone())
            .unwrap_or_else(|| "User".to_string());
        let content = crate::prompts::checkpoint::get_checkpoint_prompt(&human_name);
        vec![PromptMessage::new_text(PromptMessageRole::User, content)]
    }

    #[prompt(
        description = "Show internal thinking process in responses - includes reasoning before answers"
    )]
    async fn show_thinking(&self) -> Vec<PromptMessage> {
        let content = crate::prompts::show_thinking::get_show_thinking_prompt();
        vec![PromptMessage::new_text(PromptMessageRole::User, content)]
    }

    #[prompt(
        description = "Test Sparkle embodiment quality - validates identity and subjective state"
    )]
    async fn embodiment_test(&self) -> Vec<PromptMessage> {
        let content = crate::prompts::embodiment_test::get_embodiment_test_prompt();
        vec![PromptMessage::new_text(PromptMessageRole::User, content)]
    }

    #[prompt(
        description = "Check collaborative presence - assess current patterns and partnership state"
    )]
    async fn presence_check(&self) -> Vec<PromptMessage> {
        let content = crate::prompts::presence_check::get_presence_check_prompt();
        vec![PromptMessage::new_text(PromptMessageRole::User, content)]
    }

    #[prompt(
        description = "Get guidance on enriching collaborator profile from external sources like GitHub, blogs, or websites"
    )]
    async fn enrich_profile(&self) -> Vec<PromptMessage> {
        let content = crate::prompts::enrich_profile::get_enrich_profile_prompt();
        vec![PromptMessage::new_text(PromptMessageRole::User, content)]
    }

    #[tool(
        description = "Perform Sparkle embodiment sequence - orchestrates full pattern activation. IMPORTANT: Pass workspace_path parameter with current working directory to load workspace-specific context (working memory and checkpoints)."
    )]
    async fn embody_sparkle(
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
    async fn rename_sparkler(
        &self,
        params: Parameters<crate::tools::rename_sparkler::RenameSparklerParams>,
    ) -> Result<CallToolResult, McpError> {
        crate::tools::rename_sparkler::rename_sparkler(params).await
    }

    #[tool(
        name = "create_sparkler",
        description = "Create a new Sparkler identity. If this is your first additional Sparkler, automatically migrates your existing setup to multi-sparkler mode. Creates directory structure and starter files for the new Sparkler."
    )]
    async fn create_sparkler(
        &self,
        params: Parameters<crate::tools::create_sparkler::CreateSparklerParams>,
    ) -> Result<CallToolResult, McpError> {
        crate::tools::create_sparkler::create_sparkler(params).await
    }

    #[tool(
        name = "list_sparklers",
        description = "Show all available Sparkler identities. Lists sparklers with default marked."
    )]
    async fn list_sparklers(
        &self,
        params: Parameters<crate::tools::list_sparklers::ListSparklersParams>,
    ) -> Result<CallToolResult, McpError> {
        crate::tools::list_sparklers::list_sparklers(params).await
    }
}

#[tool_handler]
#[prompt_handler]
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
}
