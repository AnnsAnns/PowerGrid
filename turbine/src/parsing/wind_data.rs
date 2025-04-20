use crate::meta_data::MetaDataType;

use super::{download_data_for, read_for};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WindData {
    #[serde(rename = "STATIONS_ID")]
    stations_id: usize,
    #[serde(rename = "MESS_DATUM")]
    mess_datum: String,
    #[serde(rename = "QN")]
    qn: usize,
    #[serde(rename = "FF_10")]
    ff_10: f64,
    #[serde(rename = "DD_10")]
    dd_10: f64,
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
            "{},{},{},{},{},{}",
            self.stations_id, self.mess_datum, self.qn, self.ff_10, self.dd_10, self.eor
        )
    }
}