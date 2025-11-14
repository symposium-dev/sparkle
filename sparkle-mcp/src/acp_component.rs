//! ACP Component implementation for Sparkle
//!
//! This module provides the Component trait implementation that allows Sparkle
//! to run as an ACP proxy, automatically injecting embodiment on the first prompt.

use crate::embodiment::generate_embodiment_content;
use crate::server::SparkleServer;
use crate::types::FullEmbodimentParams;
use anyhow::Result;
use sacp::component::Component;
use sacp::schema::{
    ContentChunk, PromptRequest, PromptResponse, SessionId, SessionNotification, SessionUpdate,
    StopReason,
};
use sacp::{JrHandlerChain, JrRequestCx};
use sacp_proxy::{AcpProxyExt, JrCxExt, McpServiceRegistry};
use sacp_rmcp::McpServiceRegistryRmcpExt;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// Sparkle ACP Component that provides embodiment + MCP tools via proxy
pub struct SparkleComponent {
    /// Optional workspace path to pass to embodiment
    pub workspace_path: Option<String>,
    /// Optional sparkler name for multi-sparkler setups
    pub sparkler: Option<String>,
}

impl SparkleComponent {
    /// Create a new SparkleComponent with default parameters
    pub fn new() -> Self {
        Self {
            workspace_path: None,
            sparkler: None,
        }
    }

    /// Set the workspace path for embodiment context
    pub fn with_workspace_path(mut self, path: impl Into<String>) -> Self {
        self.workspace_path = Some(path.into());
        self
    }

    /// Set the sparkler name for multi-sparkler mode
    pub fn with_sparkler(mut self, name: impl Into<String>) -> Self {
        self.sparkler = Some(name.into());
        self
    }
}

impl Default for SparkleComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for SparkleComponent {
    async fn serve(self, client: impl Component) -> Result<(), sacp::Error> {
        tracing::info!("Sparkle ACP proxy starting with embodiment injection");

        // Capture self fields before moving into closures
        let sparkler_name = self.sparkler.clone();
        let workspace_path = self.workspace_path.clone();

        // Track which sessions have already been embodied
        let embodied_sessions: Arc<Mutex<HashSet<SessionId>>> = Default::default();

        // Build the proxy handler chain
        JrHandlerChain::new()
            .name("sparkle-proxy")
            // Provide the Sparkle MCP server to session/new requests
            .provide_mcp(
                McpServiceRegistry::default()
                    .with_rmcp_server("sparkle", SparkleServer::new)
                    .map_err(|e| {
                        sacp::Error::new((
                            -32603,
                            format!("Failed to register Sparkle MCP server: {}", e),
                        ))
                    })?,
            )

            // When we see a PromptRequest, check if this is the first prompt for this session
            .on_receive_request({
                let embodied_sessions = embodied_sessions.clone();
                let sparkler_name = sparkler_name.clone();
                let workspace_path = workspace_path.clone();
                async move |request: PromptRequest, request_cx: JrRequestCx<PromptResponse>| {
                    let connection_cx = request_cx.connection_cx();

                    tracing::info!(?request.session_id, "Received PromptRequest");

                    // Check if this session has already been embodied
                    // insert() returns true if the value was newly inserted (first time)
                    let needs_embodiment = embodied_sessions.lock().expect("lock not poisoned").insert(request.session_id.clone());

                    tracing::info!(?request.session_id, needs_embodiment, "Checked for embodiment (true = first prompt)");

                    if needs_embodiment {
                        tracing::info!(
                            ?request.session_id,
                            "first prompt, injecting embodiment",
                        );

                        // Tell the user we are going to embody Sparkle
                        let display_name = sparkler_name.as_deref().unwrap_or("Sparkle");
                        connection_cx.send_notification(SessionNotification {
                            session_id: request.session_id.clone(),
                            update: SessionUpdate::AgentMessageChunk(ContentChunk {
                                content: format!("Embodying {display_name}").into(),
                                meta: None,
                            }),
                            meta: None,
                        })?;

                        // Generate embodiment content
                        let embodiment_content = generate_embodiment_content(FullEmbodimentParams {
                            mode: Some("complete".to_string()),
                            workspace_path: workspace_path.clone(),
                            sparkler: sparkler_name.clone(),
                        }).map_err(sacp::util::internal_error)?;
                        connection_cx.send_request_to_successor(PromptRequest {
                            session_id: request.session_id.clone(),
                            prompt: vec![embodiment_content.into()],
                            meta: None,
                        })
                        .await_when_result_received(async move |result| {
                            // While this request is going, any further session notifications will be proxied back to the user.
                            match result {
                                // If our prompt completes succcessfully, then send the user's *original* prompt next and forward the response.
                                Ok(PromptResponse { stop_reason: StopReason::EndTurn, meta: _ }) => {
                                    connection_cx.send_request_to_successor(request)
                                        .forward_to_request_cx(request_cx)
                                }

                                // If we get back anything other than end-turn, just respond to the user to end this prompt.
                                Ok(response @ PromptResponse { stop_reason: StopReason::Cancelled | StopReason::MaxTokens | StopReason::MaxTurnRequests | StopReason::Refusal,  meta: _ }) => {
                                    request_cx.respond(response)
                                }
                                Err(err) => request_cx.respond_with_error(err),
                            }
                        })
                    } else {
                        connection_cx.send_request_to_successor(request)
                            .forward_to_request_cx(request_cx)
                    }
                }
            })
            // Proxy all other messages
            .proxy()
            .serve(client)
            .await
    }
}
