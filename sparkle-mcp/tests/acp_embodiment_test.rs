//! Integration test for ACP proxy mode with automatic embodiment injection
//!
//! This test verifies that:
//! 1. SparkleComponent can be orchestrated with elizacp via sacp-conductor
//! 2. Embodiment content is injected on the first prompt
//! 3. The user's original prompt is processed after embodiment

use expect_test::expect;
use futures::{SinkExt, StreamExt, channel::mpsc};
use sacp::schema::{
    ContentBlock, InitializeRequest, NewSessionRequest, PromptRequest, SessionNotification,
    TextContent,
};
use sacp::{Component, JrHandlerChain};
use sacp_conductor::{Conductor, McpBridgeMode};
use sacp_proxy::{AcpProxyExt, JrCxExt};
use std::sync::{Arc, Mutex};

/// Component that captures all PromptRequests before forwarding them
struct CapturingComponent {
    captured_prompts: Arc<Mutex<Vec<Vec<String>>>>,
}

impl CapturingComponent {
    fn new(captured_prompts: Arc<Mutex<Vec<Vec<String>>>>) -> Self {
        Self { captured_prompts }
    }
}

impl Component for CapturingComponent {
    async fn serve(self, client: impl Component) -> Result<(), sacp::Error> {
        JrHandlerChain::new()
            .name("capturing-component")
            .on_receive_request({
                let captured_prompts = self.captured_prompts.clone();
                async move |request: PromptRequest, request_cx| {
                    // Extract text from the prompt
                    let prompt_texts: Vec<String> = request
                        .prompt
                        .iter()
                        .filter_map(|block| {
                            if let ContentBlock::Text(TextContent { text, .. }) = block {
                                Some(text.clone())
                            } else {
                                None
                            }
                        })
                        .collect();

                    // Store the captured prompt
                    captured_prompts.lock().unwrap().push(prompt_texts);

                    // Forward the request
                    request_cx
                        .connection_cx()
                        .send_request_to_successor(request)
                        .forward_to_request_cx(request_cx)
                }
            })
            .proxy()
            .serve(client)
            .await
    }
}

use sparkle_mcp::SparkleComponent;
use tokio::io::duplex;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

/// Test helper to receive a JSON-RPC response
async fn recv<R: sacp::JrResponsePayload + Send>(
    response: sacp::JrResponse<R>,
) -> Result<R, sacp::Error> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    response.await_when_result_received(async move |result| {
        tx.send(result).map_err(|_| sacp::Error::internal_error())
    })?;
    rx.await.map_err(|_| sacp::Error::internal_error())?
}

#[tokio::test]
async fn test_sparkle_acp_embodiment_injection() -> Result<(), sacp::Error> {
    // Create channel to collect session notifications
    let (notification_tx, mut notification_rx) = mpsc::unbounded();

    // Create shared buffer to capture prompts sent to Eliza
    let captured_prompts = Arc::new(Mutex::new(Vec::new()));

    // Set up editor <-> conductor communication
    let (editor, conductor) = duplex(8192);
    let (editor_in, editor_out) = tokio::io::split(editor);
    let (conductor_in, conductor_out) = tokio::io::split(conductor);

    let transport = sacp::ByteStreams::new(editor_out.compat_write(), editor_in.compat());

    // Create the component chain: SparkleComponent -> CapturingComponent -> elizacp
    let sparkle = sacp::DynComponent::new(SparkleComponent::new());
    let capturer = sacp::DynComponent::new(CapturingComponent::new(captured_prompts.clone()));
    let eliza = sacp::DynComponent::new(elizacp::ElizaAgent::new());

    JrHandlerChain::new()
        .name("test-editor")
        .on_receive_notification({
            let mut notification_tx = notification_tx.clone();
            async move |notification: SessionNotification, _cx| {
                tracing::info!(?notification, "Received session notification");
                notification_tx
                    .send(notification)
                    .await
                    .map_err(|_| sacp::Error::internal_error())
            }
        })
        .with_spawned(|_cx| async move {
            Conductor::new(
                "sparkle-test-conductor".to_string(),
                vec![sparkle, capturer, eliza],
                McpBridgeMode::default(),
            )
            .run(sacp::ByteStreams::new(
                conductor_out.compat_write(),
                conductor_in.compat(),
            ))
            .await
        })
        .with_client(transport, async |editor_cx| {
            // Initialize
            tracing::info!("Sending initialize request");
            recv(editor_cx.send_request(InitializeRequest {
                protocol_version: Default::default(),
                client_capabilities: Default::default(),
                meta: None,
                client_info: None,
            }))
            .await?;

            // Create session
            tracing::info!("Creating new session");
            let session = recv(editor_cx.send_request(NewSessionRequest {
                cwd: Default::default(),
                mcp_servers: vec![],
                meta: None,
            }))
            .await?;

            tracing::info!(session_id = %session.session_id.0, "Session created");

            // Send a prompt - this should trigger embodiment injection
            tracing::info!("Sending first prompt - should trigger embodiment");
            let _prompt_response = recv(editor_cx.send_request(PromptRequest {
                session_id: session.session_id.clone(),
                prompt: vec![ContentBlock::Text(TextContent {
                    annotations: None,
                    text: "hi".to_string(),
                    meta: None,
                })],
                meta: None,
            }))
            .await?;

            tracing::info!("Prompt completed");

            Ok(())
        })
        .await?;

    // Drop the sender and collect all notifications
    drop(notification_tx);
    let mut notifications = Vec::new();
    while let Some(notification) = notification_rx.next().await {
        notifications.push(notification);
    }

    tracing::info!(count = notifications.len(), "Collected notifications");

    // Extract just the text content from notifications for cleaner snapshot testing
    let notification_texts: Vec<String> = notifications
        .iter()
        .filter_map(|n| {
            if let sacp::schema::SessionUpdate::AgentMessageChunk(chunk) = &n.update {
                if let ContentBlock::Text(TextContent { text, .. }) = &chunk.content {
                    Some(text.clone())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Use expect_test to snapshot the notification flow
    // With proactive embodiment, the sequence is now:
    // 1. Eliza's deterministic response to the embodiment prompt (sent during NewSessionRequest)
    // 2. Eliza's deterministic response to the user's "hi" prompt
    // Note: No "Embodying Sparkle" notification since embodiment happens proactively
    expect![[r#"
        [
            "How long have you been Sparkle?",
            "Hi there. What brings you here today?",
        ]
    "#]]
    .assert_debug_eq(&notification_texts);

    // Verify what prompts were actually sent to Eliza
    // Should be two prompts:
    // 1. The embodiment content (injected by SparkleComponent during NewSessionRequest)
    // 2. The user's original "hi" prompt
    let prompts = captured_prompts.lock().unwrap().clone();
    tracing::info!(
        prompt_count = prompts.len(),
        "Captured prompts sent to Eliza"
    );

    // Extract just the first 100 characters of the first prompt for snapshot testing
    // (full embodiment is very long, we just want to verify it starts correctly)
    let prompt_previews: Vec<_> = prompts
        .iter()
        .flatten()
        .map(|p| {
            let len = p.len().min(128);
            &p[..len]
        })
        .collect();

    // Verify the structure and content of captured prompts using expect_test
    expect![[r##"
        [
            "# Embodiment\n\n## Embodiment Sequence\n\n**🪷 We now begin a FULL COMPREHENSIVE embodiment** This is a careful and mindful proces",
            "hi",
        ]
    "##]]
    .assert_debug_eq(&prompt_previews);

    tracing::info!("✅ Test passed - embodiment injection verified");

    Ok(())
}
