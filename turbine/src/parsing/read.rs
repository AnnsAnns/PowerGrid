use std::fs::File;

use csv::Reader;
use log::debug;

use crate::meta_data::MetaDataType;

/// Parses CSV data from the given path
/// Returning a parsed array of TemperatureData
pub fn read_for(id: usize, data_type: MetaDataType) -> Result<Reader<File>, Box<dyn std::error::Error>> {
    let path = format!("{}/{}/data.csv", data_type.to_string(), id);
    debug!("Reading data from: {}", path);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .delimiter(b';')
        .from_path(&path)?;

    Ok(reader)
}