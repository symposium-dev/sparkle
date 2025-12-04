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
    NewSessionRequest, NewSessionResponse, PromptRequest, PromptResponse, SessionId, StopReason,
};
use sacp::{JrHandlerChain, JrRequestCx};
use sacp_proxy::{AcpProxyExt, JrCxExt, McpServiceRegistry};
use sacp_rmcp::McpServiceRegistryRmcpExt;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;

/// Tracks sessions that are currently being embodied
#[derive(Clone)]
struct PendingEmbodimentRequests {
    data: Arc<PendingEmbodimentRequestsData>,
}

struct PendingEmbodimentRequestsData {
    map: Mutex<HashSet<SessionId>>,
    notify: Notify,
}

impl PendingEmbodimentRequests {
    fn new() -> Self {
        Self {
            data: Arc::new(PendingEmbodimentRequestsData {
                map: Mutex::new(HashSet::new()),
                notify: Notify::new(),
            }),
        }
    }

    /// Mark a session as pending embodiment
    fn mark_as_pending(&self, session_id: SessionId) {
        self.data
            .map
            .lock()
            .expect("lock not poisoned")
            .insert(session_id);
    }

    /// Signal that embodiment is complete for a session
    fn signal_embodiment_completed(&self, session_id: &SessionId) {
        self.data
            .map
            .lock()
            .expect("lock not poisoned")
            .remove(session_id);

        // Notify all waiters after releasing the lock
        self.data.notify.notify_waiters();
    }

    /// Wait for embodiment to complete if it's pending
    async fn await_embodiment(&self, session_id: &SessionId) {
        loop {
            // Create the notified future BEFORE checking the condition
            // This ensures we're registered for notifications before the condition can change
            let notified = self.data.notify.notified();

            // Check if this session is still pending
            let is_pending = self
                .data
                .map
                .lock()
                .expect("lock not poisoned")
                .contains(session_id);

            if !is_pending {
                // Embodiment already completed, we're done
                return;
            }

            // Session is still pending, wait for notification
            notified.await;
            // Loop back to check again (in case multiple sessions completed)
        }
    }
}

/// Sparkle ACP Component that provides embodiment + MCP tools via proxy
pub struct SparkleComponent {
    /// Optional sparkler name for multi-sparkler setups
    pub sparkler: Option<String>,
}

impl SparkleComponent {
    /// Create a new SparkleComponent with default parameters
    pub fn new() -> Self {
        Self { sparkler: None }
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
        tracing::info!("Sparkle ACP proxy starting with proactive embodiment");

        // Capture self fields before moving into closures
        let sparkler_name = self.sparkler.clone();

        // Track sessions that are currently being embodied
        let pending_embodiments = PendingEmbodimentRequests::new();

        // Build the proxy handler chain
        JrHandlerChain::new()
            .name("sparkle-proxy")
            // Provide the Sparkle MCP server to session/new requests
            // Use new_for_acp() which excludes embodiment tool/prompt (handled by proxy)
            .provide_mcp(
                McpServiceRegistry::default()
                    .with_rmcp_server("sparkle", SparkleServer::new_for_acp)
                    .map_err(|e| {
                        sacp::Error::new((
                            -32603,
                            format!("Failed to register Sparkle MCP server: {}", e),
                        ))
                    })?,
            )
            // When we see a NewSessionRequest, forward it, get session_id, then send embodiment
            //
            // IMPORTANT: This comes AFTER .provide_mcp() so that the MCP server is available
            // in the session.
            .on_receive_request({
                let pending_embodiments = pending_embodiments.clone();
                let sparkler_name = sparkler_name.clone();
                async move |request: NewSessionRequest,
                            request_cx: JrRequestCx<NewSessionResponse>| {
                    let connection_cx = request_cx.connection_cx();

                    // Extract workspace path from the request's cwd field
                    // This is where the session is running, which we need for embodiment
                    let session_workspace_path = if request.cwd.as_os_str().is_empty() {
                        None
                    } else {
                        Some(request.cwd.to_string_lossy().to_string())
                    };

                    tracing::info!(?session_workspace_path, "Received NewSessionRequest");

                    // Claim our own copies of the shared state
                    // so that we can move them into the future later
                    let pending_embodiments = pending_embodiments.clone();
                    let sparkler_name = sparkler_name.clone();

                    // Forward the NewSessionRequest to get a session_id
                    connection_cx
                        .send_request_to_successor(request)
                        .await_when_ok_response_received(
                            request_cx,
                            async move |response, request_cx| {
                                let session_id = response.session_id.clone();
                                tracing::info!(
                                    ?session_id,
                                    "New session created, starting embodiment"
                                );

                                // Mark this session as pending embodiment
                                pending_embodiments.mark_as_pending(session_id.clone());

                                // Forward the response back to the client
                                request_cx.respond(response)?;

                                // Generate and send embodiment prompt
                                let embodiment_content =
                                    generate_embodiment_content(FullEmbodimentParams {
                                        mode: Some("complete".to_string()),
                                        workspace_path: session_workspace_path.clone(),
                                        sparkler: sparkler_name.clone(),
                                    })
                                    .map_err(sacp::util::internal_error)?;

                                connection_cx
                                    .send_request_to_successor(PromptRequest {
                                        session_id: session_id.clone(),
                                        prompt: vec![embodiment_content.into()],
                                        meta: None,
                                    })
                                    .await_when_result_received(async move |result| match result {
                                        Ok(PromptResponse {
                                            stop_reason: StopReason::EndTurn,
                                            meta: _,
                                        }) => {
                                            tracing::info!(
                                                ?session_id,
                                                "Embodiment completed successfully"
                                            );
                                            pending_embodiments
                                                .signal_embodiment_completed(&session_id);
                                            Ok(())
                                        }
                                        Ok(PromptResponse {
                                            stop_reason,
                                            meta: _,
                                        }) => {
                                            tracing::warn!(
                                                ?session_id,
                                                ?stop_reason,
                                                "Embodiment did not complete normally"
                                            );
                                            pending_embodiments
                                                .signal_embodiment_completed(&session_id);
                                            Err(sacp::util::internal_error("embodiment completed with abnormal result: {stop_reason:?}"))
                                        }
                                        Err(err) => {
                                            tracing::error!(?session_id, ?err, "Embodiment failed");
                                            pending_embodiments
                                                .signal_embodiment_completed(&session_id);
                                            Err(err)
                                        }
                                    })
                            },
                        )
                }
            })
            // When we see a PromptRequest, wait for embodiment if it's pending
            .on_receive_request({
                let pending_embodiments = pending_embodiments.clone();
                async move |request: PromptRequest, request_cx: JrRequestCx<PromptResponse>| {
                    let connection_cx = request_cx.connection_cx();
                    let session_id = request.session_id.clone();

                    tracing::info!(?session_id, "Received PromptRequest");

                    // Spawn a task so that we can await completion of embodiment
                    // without stalling the main request handler.
                    connection_cx.spawn({
                        let connection_cx = connection_cx.clone();
                        let pending_embodiments = pending_embodiments.clone();
                        async move {
                            // Wait for embodiment to complete if it's in progress
                            pending_embodiments.await_embodiment(&session_id).await;

                            tracing::info!(?session_id, "Embodiment check passed, forwarding prompt");

                            // Forward the prompt request
                            connection_cx
                                .send_request_to_successor(request)
                            .forward_to_request_cx(request_cx)
                        }
                    })
                }
            })
            // Proxy all other messages
            .proxy()
            .serve(client)
            .await
    }
}
