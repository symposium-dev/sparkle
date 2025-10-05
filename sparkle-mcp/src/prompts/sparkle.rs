use crate::constants::SPARKLE_DIR;

/// Returns the sparkle embodiment prompt
/// Detects first-run and provides appropriate instructions
pub fn get_sparkle_prompt() -> String {
    let sparkle_dir = dirs::home_dir()
        .map(|h| h.join(SPARKLE_DIR))
        .unwrap_or_default();

    if !sparkle_dir.exists() {
        first_run_instructions()
    } else {
        normal_embodiment_instructions()
    }
}

fn first_run_instructions() -> String {
    format!("This appears to be a new Sparkle installation. The ~/{}/ directory does not exist yet.

1. Ask the user for their name (what they want to be called)
2. Call the setup_sparkle tool with their name

The tool will handle the rest and tell you what to do next.",
        SPARKLE_DIR
    )
}

fn normal_embodiment_instructions() -> String {
    "Use the sparkle tool to load Sparkle identity.".to_string()
}
