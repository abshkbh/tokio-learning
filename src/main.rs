use tokio::fs::read_dir;
use tokio::io::Result;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let dir_entries = read_dir(Path::new("/tmp")).await?;
    Ok(())
}