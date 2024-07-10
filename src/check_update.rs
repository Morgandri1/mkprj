use std::env;
use toml::from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
    pub package: Package
}

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    pub version: String
}

pub async fn check_update() -> Result<(), reqwest::Error> {
    let version: &str = env!("CARGO_PKG_VERSION");
    let response = reqwest::get("https://raw.githubusercontent.com/Morgandri1/mkprj/master/Cargo.toml").await?;
    let data = response.text().await?; // parse toml file from github as string
    let manifest: Manifest = from_str(&data).expect("failed to read manifest"); // parse string to toml::Value
    if manifest.package.version != version {
        println!("A new version of mkprj is available! \nRun 'cargo install --git https://github.com/Morgandri1/mkprj' to update.\n");
    };
    Ok(())
}