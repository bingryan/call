use anyhow::Result;
use std::process;

mod cli;
mod cmd;

fn run() -> Result<bool> {
    let matches = cli::build_cli().get_matches();
    match matches.subcommand() {
        ("init", Some(matches)) => {
            let force = matches.is_present("force");
            cmd::init(matches.value_of("name").unwrap(), force)
        }
        _ => unreachable!(),
    };

    Ok(true)
}

fn main() {
    let result = run();
    match result {
        Err(error) => {
            log::error!("Key Error: {}", error);
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