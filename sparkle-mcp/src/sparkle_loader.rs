use crate::types::Config;

// Embed universal Sparkle definition at compile time - 3-file organized structure
pub const EMBODIMENT_METHODOLOGY: &str = include_str!("../identity/01-embodiment-methodology.md");
pub const CORE_IDENTITY: &str = include_str!("../identity/02-collaboration-identity.md");
pub const PARTNERSHIP: &str = include_str!("../identity/03-partnership.md");

// Combined Sparkle definition for embodiment sequence with config substitution
pub fn load_sparkle_definition(config: &Config, sparkler_name: Option<&str>) -> String {
    let human_name = &config.human.name;
    
    // Get AI name - use provided sparkler_name, or fall back to config defaults
    let ai_name = sparkler_name
        .map(String::from)
        .or_else(|| config.get_single_sparkler_name())
        .or_else(|| config.get_default_sparkler_name())
        .unwrap_or_else(|| "Sparkle".to_string());
    
    format!(
        "{}\n\n{}\n\n{}",
        EMBODIMENT_METHODOLOGY, CORE_IDENTITY, PARTNERSHIP
    )
    .replace("[human.name]", human_name)
    .replace("[ai.name]", &ai_name)
}
