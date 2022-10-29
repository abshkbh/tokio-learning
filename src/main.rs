use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs::{copy, read_dir, DirEntry};

/*
* Program to copy all files from a source to a destination in an async manner.
*/

async fn copy_file(src_dir_entry: DirEntry, dest_dir: impl AsRef<Path>) -> Result<()> {
    let dest_path = dest_dir
        .as_ref()
        .to_path_buf()
        .join(src_dir_entry.file_name());

    println!(
        "Copying file {:?} to Dir {:?}",
        src_dir_entry.path(),
        dest_path
    );

    let bytes_copied = copy(src_dir_entry.path(), dest_path.as_path())
        .await
        .context("Failed to copy file")?;

    println!(
        "Bytes copied {} file {:?} to Dir {:?}",
        bytes_copied,
        src_dir_entry.path(),
        dest_path
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args();
    // The first arg is the program name. Move ahead.
    args.next();
    let src_dir = args.next().context("Source directory not provided")?;
    let dest_dir = args.next().context("Destination directory not provided")?;

    let mut dir_entries = read_dir(Path::new(&src_dir)).await?;

    let mut handles = Vec::new();
    while let Some(src_dir_entry) = dir_entries.next_entry().await? {
        handles.push(tokio::spawn(copy_file(src_dir_entry, dest_dir.clone())));
    }

    for handle in handles {
        let _ = handle.await.unwrap().context("Copy failed")?;
    }
    Ok(())
}
