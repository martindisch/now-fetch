use eyre::eyre;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

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
    src: String,
}

#[derive(Debug, Serialize)]
pub struct Expression {
    prefix: Option<String>,
    word: String,
    transcription: String,
    inflection: Option<String>,
    english: String,
    audio: String,
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
