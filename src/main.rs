use eyre::{Result, WrapErr};
use std::{fs, io, path::PathBuf};

fn main() -> Result<()> {
    let files = get_files()?;

    println!("{:?}", files);
    Ok(())
}

/// Returns the sorted list of files in `data/input/`.
fn get_files() -> Result<Vec<PathBuf>> {
    let mut files = fs::read_dir("data/input/")
        .wrap_err("Could not read input files")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .wrap_err("Could not map entries to paths")?;
    files.sort();

    Ok(files)
}
