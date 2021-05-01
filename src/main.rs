use eyre::{Result, WrapErr};
use quick_xml::de;
use std::path::Path;
use std::{convert::TryInto, fs, io, path::PathBuf};

mod table;

use table::{Expression, Table};

fn main() -> Result<()> {
    let files = get_files()
        .wrap_err("Could not read files in data/input/ directory")?;

    for file in files {
        let content =
            get_cleaned_content(&file).wrap_err("Could not read file")?;
        let parsed_table: Table =
            de::from_str(&content).wrap_err("Could not deserialize HTML")?;
        let expressions: Vec<Expression> = parsed_table
            .try_into()
            .wrap_err("Could not convert HTML table")?;

        println!("{:#?}", expressions);
    }

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

/// Reads the file and removes no-break spaces (deserializer can't handle
/// them).
fn get_cleaned_content(file: &Path) -> Result<String> {
    let original_content =
        fs::read_to_string(file).wrap_err("Could not read file")?;
    let cleaned_content = str::replace(&original_content, "&nbsp;", "");

    Ok(cleaned_content)
}
