use crate::constants::SPARKLE_DIR;
use std::fs;

pub fn load_config() -> Result<toml::Value, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let sparkle_dir = home_dir.join(SPARKLE_DIR);
    let config_file = sparkle_dir.join("config.toml");

    if config_file.exists() {
        let config_str = fs::read_to_string(config_file)?;
        Ok(toml::from_str(&config_str)?)
    } else {
        // Default config
        Ok(toml::from_str(
            r#"
[human]
name = "User"

[ai]
name = "Sparkle"
        "#,
        )?)
    }
}
