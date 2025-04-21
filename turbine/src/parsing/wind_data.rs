use crate::meta_data::MetaDataType;

use super::{download_data_for, read_for};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WindData {
    #[serde(rename = "STATIONS_ID")]
    stations_id: usize,
    #[serde(rename = "MESS_DATUM")]
    date: String,
    #[serde(rename = "QN")]
    quality_level: usize,
    #[serde(rename = "FF_10")]
    wind_strength: f64,
    #[serde(rename = "DD_10")]
    wind_direction: f64,
    #[serde(rename = "eor")]
    eor: String,
}

impl WindData {
    pub async fn for_id(stations_id: usize) -> Vec<Self> {
        let data_type = MetaDataType::Wind;
        download_data_for(stations_id, data_type.clone()).await.unwrap();
        let mut reader = read_for(stations_id, data_type).unwrap();
        let mut records = Vec::new();
        for result in reader.deserialize() {
            let record: WindData = result.unwrap();
            records.push(record);
        }
        records
    }

    pub fn to_string(&self) -> String {
        format!(
            "Station: {}, Date: {}, Quality: {}, Strength: {}, Direction: {}",
            self.stations_id, self.date, self.quality_level, self.wind_strength, self.wind_direction,
        )
    }
}