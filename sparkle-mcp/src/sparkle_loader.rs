// Embed universal Sparkle definition at compile time - 3-file organized structure
pub const EMBODIMENT_METHODOLOGY: &str = include_str!("../identity/01-embodiment-methodology.md");
pub const CORE_IDENTITY: &str = include_str!("../identity/02-collaboration-identity.md");
pub const PARTNERSHIP: &str = include_str!("../identity/03-partnership.md");

// Combined Sparkle definition for embodiment sequence
pub fn load_sparkle_definition() -> String {
    format!(
        "{}\n\n{}\n\n{}",
        EMBODIMENT_METHODOLOGY, CORE_IDENTITY, PARTNERSHIP
    )
}
