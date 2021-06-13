use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};
use console::style;
use indicatif::HumanDuration;

use anyhow::Result;
use std::time::Instant;
use std::io::Write;
use std::fs;

use crate::config::{LOOKING_GLASS, SPARKLE, INIT_CONFIG};


pub fn create_file(path: &Path, content: &str) -> Result<()> {
    if let Some(p) = path.parent() {
        create_dir_all(p)?;
    }
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn root_path() -> Result<PathBuf> {
    let mut path = dirs::home_dir().unwrap();
    path.push(".call");
    Ok(path)
}


pub fn init() -> Result<()> {
    let path = Path::new(".");
    let started = Instant::now();

    let template_file = root_path()?.join("template.toml");
    let call_file = path.join("Call.yml");


    if template_file.exists() {
        println!(
            "{} {} Copy Call.yml file from template.toml",
            style(format!("[1/{}]", 1)).bold().dim(),
            LOOKING_GLASS
        );
        fs::copy(&template_file, &call_file)?;
    } else {
        println!(
            "{} {} Create Call.yml file...",
            style(format!("[1/{}]", 1)).bold().dim(),
            LOOKING_GLASS
        );

        let config = INIT_CONFIG.trim_start();

        create_file(&call_file, &config)?;
    }


    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
    Ok(())
}

