use eyre::{Result, WrapErr};
use std::{fs, io};

fn main() -> Result<()> {
    let mut files = fs::read_dir("data/input/")
        .wrap_err("Could not read input files")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .wrap_err("Could not map entries to paths")?;
    files.sort();

    println!("{:?}", files);
    Ok(())
}
