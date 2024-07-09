use serde::{Deserialize, Serialize};

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