use std::fs::{create_dir_all, File};
use std::path::Path;
use std::path::PathBuf;
use console::{style, Emoji};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};

use anyhow::Result;
use std::time::Instant;
use std::io::Write;


const CONFIG: &str = r#"[serve.config]
activate = "dev"

[serve.name.dev]
host = "192.168.2.17"
port = 22
authentication_type = "Openssh"
username = "rust"

[serve.name.stage]
host = "192.168.2.17"
port = 22
authentication_type = "Password"
username = "rust"
password = "123456"

[serve.name.prod]
host = "192.168.2.17"
port = 22
authentication_type = "KeyPair"
private_key_file = "~/.ssh/id_rsa"
pass_phrase = "123456"
"#;

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");


pub fn create_file(path: &Path, content: &str) -> Result<()> {
    if let Some(p) = path.parent() {
        create_dir_all(p)?;
    }
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}


pub fn init(name: &str, force: bool) -> Result<()> {
    let path = Path::new(name);
    let started = Instant::now();

    println!(
        "{} {}Create configure file...",
        style(format!("[1/{}]", 1)).bold().dim(),
        LOOKING_GLASS
    );

    let config = CONFIG
        .trim_start();

    // generate project data catalog
    create_file(&path.join("Call.toml"), &config)?;

    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
    Ok(())
}

