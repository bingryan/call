#![allow(dead_code)]
use anyhow::Result;
use std::{env, process, fs};
use clap::{crate_authors, crate_description, crate_version, App, Arg};
use yaml_rust::YamlLoader;
use crate::config::CallConfig;

#[macro_use]
mod call_macro;
mod cmd;
mod config;



fn run() -> Result<bool> {
    let config_file = env::current_dir()?.join("Call.yml");

    let matches = App::new("call")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("command")
                .help("command to run.")
                .empty_values(false),
        )
        .get_matches();


    if let Some(command) = matches.value_of("command") {
        match command {
            _ if command == "i" => {
                cmd::init()?
            }
            _ => {
                let s = fs::read_to_string(config_file.as_path())?;
                let yml = YamlLoader::load_from_str(s.as_ref())?;
                let config = CallConfig::build(yml[0].to_owned())?;
                cmd::runner(command, &config)?
            }
        }
    }
    Ok(true)
}

fn main() {
    let result = run();
    match result {
        Err(error) => {
            log::error!("Call Error: {}", error);
            process::exit(1);
        }
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
    }
}