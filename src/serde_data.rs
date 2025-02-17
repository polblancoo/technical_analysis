use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub fecha: String,
    #[serde(rename = "ultimo", deserialize_with = "deserialize_floats")]
    pub ultimo: f64,
    #[serde(rename = "apertura", deserialize_with = "deserialize_floats")]
    pub apertura: f64,
    #[serde(rename = "maximo", deserialize_with = "deserialize_floats")]
    pub maximo: f64,
    #[serde(rename = "minimo", deserialize_with = "deserialize_floats")]
    pub minimo: f64,
    #[serde(rename = "vol.")]
    pub volumen: String,
    #[serde(rename = "% var.")]
    pub var: String
}

// Función para deserializar flotantes con formato de coma
fn deserialize_floats<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: String = Deserialize::deserialize(deserializer)?;
    value
        .replace(',', "")
        .parse()
        .map_err(serde::de::Error::custom)
}

pub fn load_data_from_csv(file_path: &str) -> Result<Vec<Data>, Box<dyn std::error::Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;
    let mut data = Vec::new();

    for result in rdr.deserialize() {
        let record: Data = result?;
        data.push(record);
    }

    Ok(data)
}