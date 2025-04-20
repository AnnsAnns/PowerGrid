#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WindData {
    stations_id: usize,
    mess_datum: String,
    qn: usize,
    ff_10: f64,
    dd_10: f64,
    eor: String,
}

impl WindData {
    pub fn to_string(&self) -> String {
        format!(
            "{},{},{},{},{},{}",
            self.stations_id, self.mess_datum, self.qn, self.ff_10, self.dd_10, self.eor
        )
    }
}

/// Parses CSV data from the given path
/// Returning a parsed array of WindData
pub fn read_from_csv(csv_path: String) -> Result<Vec<WindData>, Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_path(csv_path)?;

    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: WindData = result?;
        records.push(record);
    }
    Ok(records)
}