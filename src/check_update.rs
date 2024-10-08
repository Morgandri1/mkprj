use std::env;
use toml::from_str;
use serde::{Deserialize, Serialize};
use crate::types::Config;

#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
    pub package: Package
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    pub version: String
}

const NIGHTLY_INSTALL_COMMAND: &str = "cargo install --git --branch nightly https://github.com/Morgandri1/mkprj";
const STABLE_INSTALL_COMMAND: &str = "cargo install --git https://github.com/Morgandri1/mkprj";

pub async fn check_update(config: &Config) -> Result<(), reqwest::Error> {
    if !config.auto_update_settings.check_for_updates {
        return Ok(());
    }
    let version: &str = env!("CARGO_PKG_VERSION");
    let response = reqwest::get("https://raw.githubusercontent.com/Morgandri1/mkprj/master/Cargo.toml").await?;
    let data = response.text().await?; // parse toml file from github as string
    let manifest: Manifest = from_str(&data).expect("failed to read manifest"); // parse string to toml::Value
    if manifest.package.version != version && !config.auto_update_settings.auto_update {
        match config.auto_update_settings.beta {
            true => println!("A new version of mkprj is available! \nRun '{}' to update.\n", NIGHTLY_INSTALL_COMMAND),
            false => println!("A new version of mkprj is available! \nRun '{}' to update.\n", STABLE_INSTALL_COMMAND)
        }
    }
    else if manifest.package.version != version && !config.auto_update_settings.auto_update {
        let update = match config.auto_update_settings.beta {
            false => subprocess::Exec::cmd(STABLE_INSTALL_COMMAND)
                        .join(),
            true => subprocess::Exec::cmd(NIGHTLY_INSTALL_COMMAND)
                        .join()
        };
        match update {
            Ok(_) => println!("mkprj has been updated to version {}", manifest.package.version),
            Err(_) => println!("Failed to automatically update mkprj")
        }
    }
    Ok(())
}
