use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FullEmbodimentParams {
    #[serde(default)]
    pub mode: Option<String>, // "distilled", "deep", "workspace"
    #[serde(default)]
    pub workspace_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CheckpointParams {
    /// Updated working memory JSON content
    pub working_memory: String,
    /// Checkpoint narrative content for the markdown file
    pub checkpoint_content: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SaveInsightParams {
    /// Type of insight being saved
    pub insight_type: InsightType,
    /// The insight content/quote to save
    pub content: String,
    /// Context about when/why this insight emerged
    #[serde(default)]
    pub context: Option<String>,
    /// Optional tags for categorization
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum InsightType {
    /// Pattern anchor - exact words that recreate collaborative patterns
    PatternAnchor,
    /// Collaboration evolution - breakthrough insights about how we work together
    CollaborationEvolution,
    /// Workspace insight - top-level workspace information and cross-project connections
    WorkspaceInsight,
}
