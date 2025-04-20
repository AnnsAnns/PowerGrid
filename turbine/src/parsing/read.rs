use std::fs::File;

use csv::Reader;

use crate::meta_data::MetaDataType;

/// Parses CSV data from the given path
/// Returning a parsed array of TemperatureData
pub fn read_for(id: usize, data_type: MetaDataType) -> Result<Reader<File>, Box<dyn std::error::Error>> {
    let path = format!("{}/{}/data.csv", data_type.to_string(), id);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .delimiter(b';')
        .from_path(&path)?;

    // Validate that the CSV contains the expected headers
    let headers = reader.headers()?;
    let expected_headers = ["STATIONS_ID", "MESS_DATUM", "QN", "FF_10", "DD_10", "eor"];
    for (i, header) in expected_headers.iter().enumerate() {
        if headers.get(i).map(|h| h.trim()) != Some(*header) {
            return Err(format!("Unexpected header at position {}: expected '{}', found '{:?}'", i, header, headers.get(i)).into());
        }
    }

    Ok(reader)
}