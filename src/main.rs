use std::borrow::Borrow;
use std::env::args;

mod types;
use crate::types::Config;

mod new_project;

#[cfg(feature = "check_update")]
mod check_update;

fn main() { 
    let command = args().nth(1).expect("No args given!");
    let args = args().skip(2);
    
    let mut config = match Config::read_config() {
        Ok(config) => config,
        Err(_) => {
            Config::patch_config().expect("Failed to patch config"); 
            Config::read_config().expect("Failed to read config after patch")
        }
    };
        
    #[cfg(feature = "check_update")]
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(check_update::check_update(&config))
        .expect("Failed to check for updates. Please try again later.");
    
    match command.as_str() {
        "config" | "-c" | "--config" => {
            let should_write = args.len().borrow() > &0;
            args.into_iter().for_each(|arg| {
                let mut arg = arg.split("=");
                let key = arg.next().unwrap();
                let value = arg.next().unwrap().to_string();
                match key {
                    #[cfg(feature = "check_update")]
                    "--check_for_updates" => config.auto_update_settings.check_for_updates = value.parse().expect("Invalid value"),
                    #[cfg(feature = "check_update")]
                    "--auto_update" => config.auto_update_settings.auto_update = value.parse().expect("Invalid value"),
                    #[cfg(feature = "check_update")]
                    "--beta" => config.auto_update_settings.beta = value.parse().expect("Invalid value"),
                    "--editor_command" => config.editor = value,
                    "--node_command" => config.node_command = value,
                    _ => println!("Invalid config flag: '{}'", key),
                }
            });
            if should_write {
                config.write_config().expect("Failed to write config");
                println!("Updated config!")
            } else {
                println!("Editor Command: {}\nNode Command: {}", config.editor, config.node_command)
            }
        },
        // Help command with all language flags from new_project.rs switch statement
        "help" | "-h" | "--help" => println!("mkprj [command] [args]\n\nCommands:\n\tconfig\n\thelp\n\nConfig Flags:\n\t--editor_command\n\t--node_command\n\nLanguage Flags:\n\t--node / --deno / --ts / --js\n\t--py\n\t--rust\n\t--go\n\t--cpp\n\t--c\n\t--java\n\t--tsc\n\t--cs\n\t--swift\n\t--rb\n\t--php\n\t--lua\n\t--perl\n\t--haskell\n\t--erlang\n\t--elixir\n\t--crystal\n\t--dart\n\t--kotlin\n\t--scala\n\t--clojure\n\t--groovy\n\t--r\n\t--julia"),
        "" => println!("No command given"),
        _ => new_project::handler(command, args, &config),
    }
}