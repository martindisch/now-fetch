use eyre::{eyre, Result, WrapErr};
use quick_xml::de;
use reqwest::blocking::Client;
use std::path::Path;
use std::{
    convert::{TryFrom, TryInto},
    fs::{self, File},
    io,
    path::PathBuf,
};

mod table;

use table::{Expression, Flashcard, Table};

fn main() -> Result<()> {
    let files = get_files()
        .wrap_err("Could not read files in data/input/ directory")?;
    create_output_directories()
        .wrap_err("Could not create output directories")?;

    for file in files {
        println!("Working on {:?}", file);
        let content =
            get_cleaned_content(&file).wrap_err("Could not read file")?;
        let parsed_table: Table =
            de::from_str(&content).wrap_err("Could not deserialize HTML")?;

        let expressions: Vec<Expression> = parsed_table
            .try_into()
            .wrap_err("Could not convert HTML table")?;
        download_files(&expressions)
            .wrap_err("Could not download audio files")?;
        let flashcards = expressions
            .into_iter()
            .map(Flashcard::try_from)
            .collect::<Result<Vec<_>>>()
            .wrap_err("Could not convert expression to flashcard")?;

        let output_path = get_csv_output_path(&file)
            .wrap_err("Could not build output path")?;
        write_csv(&flashcards, &output_path)
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

/// Creates `data/output/media` if it doesn't exist.
fn create_output_directories() -> Result<()> {
    fs::create_dir_all("data/output/media")
        .wrap_err("Could not create data/output/media/")?;
    Ok(())
}

/// Reads the file and removes no-break spaces (deserializer can't handle
/// them).
fn get_cleaned_content(file: &Path) -> Result<String> {
    let original_content =
        fs::read_to_string(file).wrap_err("Could not read file")?;
    let cleaned_content = str::replace(&original_content, "&nbsp;", "");

    Ok(cleaned_content)
}

/// Downloads the audio files for the expressions into `data/output/media`.
fn download_files(expressions: &[Expression]) -> Result<()> {
    let client = Client::new();

    for expression in expressions {
        let mut response = client
            .get(expression.audio.clone())
            .send()
            .wrap_err("Could not download audio file")?;

        let sound_file = expression
            .audio
            .path_segments()
            .ok_or_else(|| eyre!("URL does not contain file"))?
            .last()
            .ok_or_else(|| eyre!("No last element in iterator"))?;
        let mut output =
            File::create(format!("data/output/media/{}", sound_file))
                .wrap_err("Could not create file for audio")?;

        io::copy(&mut response, &mut output)
            .wrap_err("Could not copy content to file")?;
    }

    Ok(())
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

/// Writes the flashcards to a semicolon-separated CSV file at the given path.
fn write_csv(flashcards: &[Flashcard], path: &Path) -> Result<()> {
    let file = File::create(path)
        .wrap_err("Could not create file in data/output/ directory")?;
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_writer(file);

    for flashcard in flashcards {
        writer
            .serialize(flashcard)
            .wrap_err("Could not serialize flashcard")?;
    }

    writer.flush().wrap_err("Could not flush CSV file")?;

    Ok(())
}
