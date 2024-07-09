use std::{iter::Skip, process::Command};

use crate::types::Config;

pub fn handler(command: String, mut args: Skip<std::env::Args>, config: Config) {
    let project_name = command;
    std::fs::create_dir(&project_name).expect("Failed to create project directory");
    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = Command::new("cmd");
        c.arg("/C");
        c
    } else {
        let mut c = Command::new("sh");
        c.arg("-c");
        c
    };
    match args.len() {
        0 => {
            cmd.current_dir(project_name.as_str());
            cmd.arg("git init && touch .gitignore");
            cmd.output().expect("Failed to run git init");
        },
        1 => { // if an argument is given
            match args.nth(0).unwrap().as_str() {
                "--node" | "--deno" | "--js" | "--ts" => {
                    cmd.current_dir(project_name.as_str());
                    let npm_init = match config.node_command.as_str() {
                        "npm" => cmd.arg("npm init -y").output(),
                        "deno" => cmd.arg("deno init").output(),
                        "yarn" => cmd.arg("yarn init -y").output(),
                        "bun" => cmd.arg("bun init").output(),
                        _ => cmd.arg(format!("{} init", config.node_command)).output(),
                    }; 
                    println!("{:#?}", npm_init.expect("Failed to run init command").stdout);
                },
                "--py" => {
                    cmd.current_dir(project_name.as_str());
                    let pipenv_init = cmd.arg(format!("pipenv install")).output();
                    println!("{:#?}", pipenv_init.expect("Failed to run pipenv install").stdout);
                },
                "--rust" => {
                    std::fs::remove_dir(project_name.as_str()).expect("Directory not empty or not found");
                    let cargo_init = cmd.arg("cargo new ".to_string() + project_name.as_str()).output();
                    println!("{:#?}", cargo_init.expect("Failed to run cargo init").stdout);
                },
                "--go" => {
                    cmd.current_dir(project_name.as_str());
                    let go_init = cmd.arg("go mod init ".to_string() + project_name.as_str()).output();
                    println!("{:#?}", go_init.expect("Failed to run go mod init").stdout);
                },
                "--cpp" => {
                    cmd.current_dir(project_name.as_str());
                    let cmake_init = cmd.arg("cmake .").output();
                    println!("{:#?}", cmake_init.expect("Failed to run cmake .").stdout);
                },
                "--c" => {
                    cmd.current_dir(project_name.as_str());
                    let make_init = cmd.arg("make").output();
                    println!("{:#?}", make_init.expect("Failed to run make").stdout);
                },
                "--java" => {
                    cmd.current_dir(project_name.as_str());
                    let gradle_init = cmd.arg("gradle init").output();
                    println!("{:#?}", gradle_init.expect("Failed to run gradle init").stdout);
                },
                "--tsc" => {
                    cmd.current_dir(project_name.as_str());
                    let tsc_init = cmd.arg("tsc --init").output();
                    println!("{:#?}", tsc_init.expect("Failed to run tsc init").stdout);
                },
                "--cs" => {
                    cmd.current_dir(project_name.as_str());
                    let dotnet_init = cmd.arg("dotnet new console").output();
                    println!("{:#?}", dotnet_init.expect("Failed to run dotnet new").stdout);
                },
                "--swift" => {
                    cmd.current_dir(project_name.as_str());
                    let swift_init = cmd.arg("swift package init --type executable").output();
                    println!("{:#?}", swift_init.expect("Failed to run swift package init").stdout);
                },
                "--rb" => {
                    cmd.current_dir(project_name.as_str());
                    let rb_init = cmd.arg("bundle init").output();
                    println!("{:#?}", rb_init.expect("Failed to run bundle init").stdout);
                },
                "--php" => {
                    cmd.current_dir(project_name.as_str());
                    let composer_init = cmd.arg("composer init").output();
                    println!("{:#?}", composer_init.expect("Failed to run composer init").stdout);
                },
                "--lua" => {
                    cmd.current_dir(project_name.as_str());
                    let lua_init = cmd.arg("lua init").output();
                    println!("{:#?}", lua_init.expect("Failed to run lua init").stdout);
                },
                "--perl" => {
                    cmd.current_dir(project_name.as_str());
                    let perl_init = cmd.arg("perl init").output();
                    println!("{:#?}", perl_init.expect("Failed to run perl init").stdout);
                },
                "--haskell" => {
                    cmd.current_dir(project_name.as_str());
                    let stack_init = cmd.arg("stack init").output();
                    println!("{:#?}", stack_init.expect("Failed to run stack init").stdout);
                },
                "--erlang" => {
                    cmd.current_dir(project_name.as_str());
                    let rebar3_init = cmd.arg("rebar3 new app ".to_string() + project_name.as_str()).output();
                    println!("{:#?}", rebar3_init.expect("Failed to run rebar3 new app").stdout);
                },
                "--elixir" => {
                    cmd.current_dir(project_name.as_str());
                    let mix_init = cmd.arg("mix new ".to_string() + project_name.as_str()).output();
                    println!("{:#?}", mix_init.expect("Failed to run mix new").stdout);
                },
                "--crystal" => {
                    cmd.current_dir(project_name.as_str());
                    let crystal_init = cmd.arg("crystal init app ".to_string() + project_name.as_str()).output();
                    println!("{:#?}", crystal_init.expect("Failed to run crystal init app").stdout);
                },
                "--dart" => {
                    cmd.current_dir(project_name.as_str());
                    let dart_init = cmd.arg("dart create ".to_string() + project_name.as_str()).output();
                    println!("{:#?}", dart_init.expect("Failed to run dart create").stdout);
                },
                "--kotlin" => {
                    cmd.current_dir(project_name.as_str());
                    let kotlin_init = cmd.arg("kotlin init ".to_string() + project_name.as_str()).output();
                    println!("{:#?}", kotlin_init.expect("Failed to run kotlin init").stdout);
                },
                "--scala" => {
                    cmd.current_dir(project_name.as_str());
                    let sbt_init = cmd.arg("sbt new ".to_string() + project_name.as_str()).output();
                    println!("{:#?}", sbt_init.expect("Failed to run sbt new").stdout);
                },
                "--clojure" => {
                    cmd.current_dir(project_name.as_str());
                    let lein_init = cmd.arg("lein new ".to_string() + project_name.as_str()).output();
                    println!("{:#?}", lein_init.expect("Failed to run lein new").stdout);
                },
                "--groovy" => {
                    cmd.current_dir(project_name.as_str());
                    let gradle_init = cmd.arg("gradle init").output();
                    println!("{:#?}", gradle_init.expect("Failed to run gradle init").stdout);
                },
                "--r" => {
                    cmd.current_dir(project_name.as_str());
                    let r_init = cmd.arg("Rscript init").output();
                    println!("{:#?}", r_init.expect("Failed to run Rscript init").stdout);
                },
                "--julia" => {
                    cmd.current_dir(project_name.as_str());
                    let julia_init = cmd.arg("julia init").output();
                    println!("{:#?}", julia_init.expect("Failed to run julia init").stdout);
                }
                _ => {
                    println!("Invalid argument");
                },
            }
        },
        _ => {
            println!("Invalid number of arguments");
        },
    }
    subprocess::Exec::cmd(config.editor)
        .arg("./".to_owned() + project_name.as_str())
        .join()
        .expect("Failed to open editor");
}