use std::borrow::Borrow;
use std::env::args;
use std::fs::read_to_string;
use anyhow::Result;

mod types;
use crate::types::Config;

mod new_project;

fn main() {
    let command = args().nth(1).expect("No args given!");
    let args = args().skip(2);
    
    match command.as_str() {
        "config" | "-c" | "--config" => {
            let mut config = read_config().unwrap_or_default();
            let should_write = args.len().borrow() > &0;
            args.into_iter().for_each(|arg| {
                let mut arg = arg.split("=");
                let key = arg.next().unwrap();
                let value = arg.next().unwrap().to_string();
                match key {
                    "--editor_command" => config.editor = value,
                    "--node_command" => config.node_command = value,
                    _ => println!("Invalid config flag: '{}'", key),
                }
            });
            if should_write {
                write_config(config).expect("Failed to write config");
                println!("Updated config!")
            }
        },
        "help" | "-h" | "--help" => println!("mkprj [command] [args]\n\nCommands:\n\tconfig\n\thelp\n\tnew\n\nFlags:\n\t--editor_command\n\t--node_command"),
        "" => println!("No command given"),
        _ => new_project::handler(command, args, read_config().unwrap_or_default()),
    }
}

fn read_config() -> Result<Config> {
    // Read the config file
    let config = read_to_string("~/mkprj/config.json")?;
    // Parse the config file
    let config: Config = serde_json::from_str(&config)?;
    Ok(config)
}

fn write_config(config: Config) -> Result<()> {
    // Serialize the config
    let config = serde_json::to_string(&config)?;
    // Check if the config directory exists
    match std::fs::read_dir("~/.mkprj/") {
        Ok(_) => {},
        Err(_) => std::fs::create_dir("~/.mkprj/").expect("Failed to create config directory"),
    }
    // Write the config to the file
    std::fs::write("~/.mkprj/config.json", config)?;
    Ok(())
}