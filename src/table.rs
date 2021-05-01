use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "tbody")]
pub struct Table {
    #[serde(rename = "tr")]
    rows: Vec<Row>,
}

#[derive(Debug, Deserialize)]
pub struct Row {
    #[serde(rename = "td")]
    data_cells:
        Option<(TextCell, AudioCell, TextCell, TextCell, TextCell, TextCell)>,
}

#[derive(Debug, Deserialize)]
pub struct TextCell {
    #[serde(rename = "$value")]
    body: Option<Vec<String>>,
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
