#![allow(dead_code)]
#[macro_use]
extern crate serde_derive;

use anyhow::Result;
use clap::{crate_authors, crate_description, crate_version, App, Arg};
use std::{env, fs, process};
use yaml_rust::YamlLoader;

use crate::config::{CallConfig, CallSystemConfig};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};

#[macro_use]
mod call_macro;
mod cmd;
mod config;

fn root_path() -> Result<PathBuf> {
	let mut path = dirs::home_dir().unwrap();
	path.push(".call");
	Ok(path)
}

pub fn create_file(path: &Path, content: &str) -> Result<()> {
	if let Some(p) = path.parent() {
		create_dir_all(p)?;
	}
	let mut file = File::create(&path)?;
	file.write_all(content.as_bytes())?;
	Ok(())
}

pub fn config_file(call_config_root: &PathBuf) -> Result<(PathBuf, PathBuf)> {
	match call_config_root.exists() {
		true => {
			let settings = CallSystemConfig::build(call_config_root).unwrap();
			let template_file = root_path()?.join(settings.template);

			let call_file = env::current_dir()?.join(settings.call_config_path).join("Call.yml");
			Ok((template_file, call_file))
		}
		false => {
			let template_file = root_path()?.join("template.toml");
			let call_file = env::current_dir()?.join("Call.yml");
			Ok((template_file, call_file))
		}
	}
}

fn run() -> Result<bool> {
	let matches = App::new("call")
		.version(crate_version!())
		.author(crate_authors!())
		.about(crate_description!())
		.arg(Arg::with_name("command").help("command to run.").empty_values(false))
		.get_matches();

	if let Some(command) = matches.value_of("command") {
		match command {
			_ if command == "i" => cmd::init()?,
			_ => {
				let call_config_root = root_path()?.join("config.toml");
				let (_template_file, call_file) = config_file(&call_config_root)?;
				let s = fs::read_to_string(call_file.as_path())?;
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
