use eyre::{eyre, Result, WrapErr};
use quick_xml::de;
use std::path::Path;
use std::{
    convert::TryInto,
    fs::{self, File},
    io,
    path::PathBuf,
};

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
        let output_path = get_csv_output_path(&file)
            .wrap_err("Could not build output path")?;
        write_csv(&expressions, &output_path)
            .wrap_err("Could not write CSV file")?;
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

/// Returns the path to a CSV file with the same name as `input` in the output
/// directory.
fn get_csv_output_path(input: &Path) -> Result<PathBuf> {
    let file_stem = input
        .file_stem()
        .ok_or_else(|| eyre!("Could not find file name"))?
        .to_str()
        .ok_or_else(|| eyre!("Could not convert filename to str"))?;

    Ok(PathBuf::from(format!("data/output/{}.csv", file_stem)))
}

/// Writes the expressions to a semicolon-separated CSV file at the given path.
fn write_csv(expressions: &[Expression], path: &Path) -> Result<()> {
    let file = File::create(path)
        .wrap_err("Could not create file in data/output/ directory")?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_writer(file);

    for expression in expressions {
        writer
            .serialize(expression)
            .wrap_err("Could not serialize expression")?;
    }

    writer.flush().wrap_err("Could not flush CSV file")?;

    Ok(())
}
