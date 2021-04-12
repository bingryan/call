use anyhow::Result;

pub fn runner(command: &str) -> Result<()> {
    println!("call run: {:?}", command);
    Ok(())
}

