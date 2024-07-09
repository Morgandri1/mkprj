use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub editor: String,
    /// Command used to run node in case npm is not the PM of choice. this could be bun, yarn, deno, etc.
    /// so long as it uses `{cmd} init -y` to initialize a new project.
    pub node_command: String
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: "vim".to_string(),
            node_command: "npm".to_string()
        }
    }
}

impl Config {
    pub fn read_config() -> Result<Config> {
        // Read the config file
        let config = read_to_string("~/mkprj/config.json")?;
        // Parse the config file
        let config: Config = serde_json::from_str(&config)?;
        Ok(config)
    }
    
    pub fn write_config(self: Self) -> Result<()> {
        // Serialize the config
        let config = serde_json::to_string(&self)?;
        // Check if the config directory exists
        match std::fs::read_dir("~/.mkprj/") {
            Ok(_) => {},
            Err(_) => std::fs::create_dir("~/.mkprj/").expect("Failed to create config directory"),
        }
        // Write the config to the file
        std::fs::write("~/.mkprj/config.json", config)?;
        Ok(())
    }
}