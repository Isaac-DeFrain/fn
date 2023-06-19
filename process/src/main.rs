use std::{process::Command, path::PathBuf};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let watch_dir = PathBuf::from(concat!(env!("HOME"), "/.mina-indexer/watch-blocks"));
    let mut command = Command::new("gsutil");
    command.arg("-m");
    command.arg("cp");
    command.arg("-n");
    command.arg(&format!("gs://mina_network_block_data/mainnet-42-*.json"));
    command.arg(&watch_dir.display().to_string());
    
    let mut cmd = command.spawn()?;
    match cmd.wait() {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::Error::from(e)),
    }
}
