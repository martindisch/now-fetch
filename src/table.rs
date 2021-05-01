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
    data_cells: Option<Vec<Cell>>,
}

#[derive(Debug, Deserialize)]
pub struct Cell {
    #[serde(rename = "$value")]
    body: Option<Vec<String>>,
}
