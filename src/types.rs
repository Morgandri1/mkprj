use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use anyhow::Result;
use dirs::home_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub editor: String,
    /// Command used to run node in case npm is not the PM of choice. this could be bun, yarn, deno, etc.
    /// so long as it uses `{cmd} init -y` to initialize a new project.
    pub node_command: String,
    #[cfg(feature = "check_update")]
    pub auto_update_settings: CheckUpdates,
    
}
#[cfg(feature = "check_update")]
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckUpdates {
    pub check_for_updates: bool,
    pub auto_update: bool,
    pub beta: bool,
}

impl Default for Config {
    #[cfg(not(feature = "check_update"))]
    fn default() -> Self {
        Config {
            editor: "vim".to_string(),
            node_command: "npm".to_string()
        }
    }
    #[cfg(feature = "check_update")]
    fn default() -> Self {
        Config {
            editor: "vim".to_string(),
            node_command: "npm".to_string(),
            auto_update_settings: CheckUpdates {
                check_for_updates: true,
                auto_update: false,
                beta: false,
            }
        }
    }
}

impl Config {
    pub fn read_config() -> Result<Config> {
        // Read the config file
        let home = home_dir()
            .expect("Failed to get home directory")
            .to_str()
            .unwrap()
            .to_owned();
        let config = read_to_string(home + "/.mkprj/config.json")?;
        // Parse the config file
        let config: Config = serde_json::from_str(&config)?;
        Ok(config)
    }
    
    pub fn write_config(self: Self) -> Result<()> {
        let home = home_dir()
            .expect("Failed to get home directory")
            .to_str()
            .unwrap()
            .to_owned();
        // Serialize the config
        let config = serde_json::to_string(&self)?;
        // Check if the config directory exists
        match std::fs::read_dir(home.clone() + "/.mkprj/") {
            Ok(_) => {},
            Err(_) => std::fs::create_dir(home.clone() + "/.mkprj/").expect("Failed to create config directory"),
        }
        // Write the config to the file
        std::fs::write(home + "/.mkprj/config.json", config)?;
        Ok(())
    }
}