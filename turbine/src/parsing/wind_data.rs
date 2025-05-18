use crate::meta_data::MetaDataType;

use super::{download_data_for, read_for};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WindData {
    #[serde(rename = "STATIONS_ID")]
    pub stations_id: usize,
    #[serde(rename = "MESS_DATUM")]
    pub date: String,
    #[serde(rename = "QN")]
    pub quality_level: usize,
    #[serde(rename = "FF_10")]
    pub wind_strength: f64,
    #[serde(rename = "DD_10")]
    pub wind_direction: f64,
    #[serde(rename = "eor")]
    pub eor: String,
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
}