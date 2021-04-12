use anyhow::Result;
use std::process;
use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg, SubCommand};


mod cmd;
mod config;


fn run() -> Result<bool> {
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
                cmd::init();
            },
            _ => {
                cmd::runner(command);
            },
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