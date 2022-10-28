use std::io;
use std::path::Path;
use tokio::fs::read_dir;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut dir_entries = read_dir(Path::new("/tmp/foo")).await?;

    while let Some(dir_entry) = dir_entries.next_entry().await? {
        println!("{:?}", dir_entry.path())
    }
    Ok(())
}
