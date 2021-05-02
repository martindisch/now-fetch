use eyre::eyre;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename = "tbody")]
pub struct Table {
    #[serde(rename = "tr")]
    rows: Vec<Row>,
}

#[derive(Debug, Deserialize)]
pub struct Row {
    #[serde(rename = "td")]
    data_cells: Option<DataCells>,
}

#[derive(Debug, Deserialize)]
pub struct DataCells(
    OptionalTextCell,
    AudioCell,
    TextCell,
    OptionalTextCell,
    TextCell,
    OptionalTextCell,
);

#[derive(Debug, Deserialize)]
pub struct OptionalTextCell {
    #[serde(rename = "$value")]
    body: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TextCell {
    #[serde(rename = "$value")]
    body: String,
}

#[derive(Debug, Deserialize)]
pub struct AudioCell {
    audio: Audio,
    a: String,
}

#[derive(Debug, Deserialize)]
pub struct Audio {
    src: Url,
}

#[derive(Debug)]
pub struct Expression {
    prefix: Option<String>,
    word: String,
    transcription: String,
    inflection: Option<String>,
    english: String,
    audio: Url,
}

impl From<DataCells> for Expression {
    fn from(data_cells: DataCells) -> Self {
        Self {
            prefix: data_cells.0.body,
            word: data_cells.1.a,
            transcription: data_cells.2.body,
            inflection: data_cells.3.body,
            english: data_cells.4.body,
            audio: data_cells.1.audio.src,
        }
    }
}

impl TryFrom<Table> for Vec<Expression> {
    type Error = eyre::Report;

    fn try_from(table: Table) -> Result<Self, Self::Error> {
        table
            .rows
            .into_iter()
            .skip(1)
            .map(|row| {
                row.data_cells.map(Expression::from).ok_or_else(|| {
                    eyre!("A table row did not contain the expected values")
                })
            })
            .collect()
    }
}

#[derive(Debug, Serialize)]
pub struct Flashcard {
    front: String,
    back: String,
    sound: String,
}

impl TryFrom<Expression> for Flashcard {
    type Error = eyre::Report;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        let has_prefix = expression.prefix.is_some();
        let back = format!(
            "<p>{}{}{}</p><p>{}</p><p>{}</p>",
            expression.prefix.unwrap_or_else(|| String::from("")),
            if has_prefix {
                String::from(" ")
            } else {
                String::from("")
            },
            expression.word,
            expression.transcription,
            expression.inflection.unwrap_or_else(|| String::from(""))
        );

        let sound_file = expression
            .audio
            .path_segments()
            .ok_or_else(|| eyre!("URL does not contain file"))?
            .last()
            .ok_or_else(|| eyre!("No last element in iterator"))?;

        Ok(Self {
            front: expression.english,
            back,
            sound: format!("[sound:{}]", sound_file),
        })
    }
}
