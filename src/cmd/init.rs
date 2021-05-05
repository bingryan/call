use std::fs::{create_dir_all, File};
use std::path::Path;
use console::style;
use indicatif::HumanDuration;

use anyhow::Result;
use std::time::Instant;
use std::io::Write;

use crate::config::{LOOKING_GLASS, SPARKLE, INIT_CONFIG};


pub fn create_file(path: &Path, content: &str) -> Result<()> {
    if let Some(p) = path.parent() {
        create_dir_all(p)?;
    }
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}


pub fn init() -> Result<()> {
    let path = Path::new(".");
    let started = Instant::now();

    println!(
        "{} {} Create Call.yml file...",
        style(format!("[1/{}]", 1)).bold().dim(),
        LOOKING_GLASS
    );

    let config = INIT_CONFIG
        .trim_start();

    // generate project data catalog
    create_file(&path.join("Call.yml"), &config)?;

    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
    Ok(())
}

