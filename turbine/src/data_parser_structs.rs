#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MetaData {
    pub stations_id: String,
    pub von_datum: String,
    pub bis_datum: String,
    pub stationshoehe: usize,
    pub geo_breite: f64,
    pub geo_laenge: f64,
    pub stationsname: String,
    pub bundesland: String,
    pub abgabe: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TurbineData {
    stations_id: usize,
    mess_datum: String,
    qn: usize,
    ff_10: f64,
    dd_10: f64,
    eor: String,
}

impl TurbineData {
    pub fn to_string(&self) -> String {
        format!(
            "{},{},{},{},{},{}",
            self.stations_id, self.mess_datum, self.qn, self.ff_10, self.dd_10, self.eor
        )
    }
}

/// Parses CSV data from the given path
/// Returning a parsed array of TurbineData
pub fn read_from_csv(csv_path: String) -> Result<Vec<TurbineData>, Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_path(csv_path)?;

    let mut records = Vec::new();
    for result in reader.deserialize() {
        let record: TurbineData = result?;
        records.push(record);
    }
    Ok(records)
}

impl MetaData {
    pub fn new(
        stations_id: String,
        von_datum: String,
        bis_datum: String,
        stationshoehe: usize,
        geo_breite: f64,
        geo_laenge: f64,
        stationsname: String,
        bundesland: String,
        abgabe: String,
    ) -> Self {
        MetaData {
            stations_id,
            von_datum,
            bis_datum,
            stationshoehe,
            geo_breite,
            geo_laenge,
            stationsname,
            bundesland,
            abgabe,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{}",
            self.stations_id,
            self.von_datum,
            self.bis_datum,
            self.stationshoehe,
            self.geo_breite,
            self.geo_laenge,
            self.stationsname,
            self.bundesland,
            self.abgabe
        )
    }
}