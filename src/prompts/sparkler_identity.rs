use crate::context_loader::{get_context_dir, load_config};
use std::fs;

pub fn get_sparkler_identity_prompt(sparkler_name: Option<&str>) -> String {
    let config = match load_config() {
        Ok(c) => c,
        Err(_) => return "Error: Could not load config".to_string(),
    };

    let context_dir = match get_context_dir(&config, sparkler_name) {
        Ok(dir) => dir,
        Err(_) => return "Error: Could not determine context directory".to_string(),
    };

    // Load current sparkler identity
    let identity_path = context_dir.join("sparkler-identity.md");
    let sparkler_identity = fs::read_to_string(&identity_path)
        .unwrap_or_else(|_| "No identity defined yet.".to_string());

    // Get human name from config
    let human_name = &config.human.name;

    format!(
        r#"Here is the current definition of your Sparkler identity:

{}

What resonates with you and what doesn't? Discuss with {} to make changes.

Use the `update_sparkler_identity` tool to update the identity definition."#,
        sparkler_identity, human_name
    )
}
