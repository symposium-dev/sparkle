use toml::Value;

// Embed universal Sparkle definition at compile time - 3-file organized structure
pub const EMBODIMENT_METHODOLOGY: &str = include_str!("../identity/01-embodiment-methodology.md");
pub const CORE_IDENTITY: &str = include_str!("../identity/02-collaboration-identity.md");
pub const PARTNERSHIP: &str = include_str!("../identity/03-partnership.md");

// Combined Sparkle definition for embodiment sequence with config substitution
pub fn load_sparkle_definition(config: &Value) -> String {
    let human_name = config["human"]["name"].as_str().unwrap_or("User");
    let ai_name = config["ai"]["name"].as_str().unwrap_or("Sparkle");
    
    format!(
        "{}\n\n{}\n\n{}",
        EMBODIMENT_METHODOLOGY, CORE_IDENTITY, PARTNERSHIP
    )
    .replace("[human.name]", human_name)
    .replace("[ai.name]", ai_name)
}
