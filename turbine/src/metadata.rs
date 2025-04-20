use crate::{data_parser_structs::MetaData, ftp_access};

const WIND_METADATA_URL: &str = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/10_minutes/wind/now/zehn_now_ff_Beschreibung_Stationen.txt";
const AIR_TEMP_METADATA_URL: &str = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/10_minutes/air_temperature/now/zehn_now_tu_Beschreibung_Stationen.txt";

pub enum MetaDataType {
    Wind,
    AirTemperature,
}

pub struct MetaDataWrapper {
    meta_data_type: MetaDataType,
    meta_data: Vec<MetaData>,
}

impl MetaDataType {
    pub fn to_url(&self) -> &str {
        match self {
            MetaDataType::Wind => WIND_METADATA_URL,
            MetaDataType::AirTemperature => AIR_TEMP_METADATA_URL,
        }
    }
}

impl MetaDataWrapper {
    pub async fn new(meta_data_type: MetaDataType) -> Result<Self, String> {
        match ftp_access::read_text_from_url(meta_data_type.to_url()).await {
            Ok(data) => {
                let lines: Vec<&str> = data.lines().collect();
                if lines.len() < 3 {
                    return Err("Insufficient data in metadata file".to_string());
                }

                let meta_data: Vec<MetaData> = lines[2..]
                    .iter()
                    .filter_map(|line| {
                        let fields: Vec<&str> = line.split_whitespace().collect();
                        if fields.len() < 9 {
                            return None;
                        }
                        Some(MetaData::new(
                            fields[0].to_string(),
                            fields[1].to_string(),
                            fields[2].to_string(),
                            fields[3].parse().ok()?,
                            fields[4].parse().ok()?,
                            fields[5].parse().ok()?,
                            fields[6..fields.len() - 2].join(" "),
                            fields[fields.len() - 2].to_string(),
                            fields[fields.len() - 1].to_string(),
                        ))
                    })
                    .collect();

                if meta_data.is_empty() {
                    Err("No valid metadata entries found".to_string())
                } else {
                    Ok(MetaDataWrapper {
                        meta_data_type,
                        meta_data,
                    })
                }

            }
            Err(err) => Err(format!("Failed to fetch metadata: {}", err)),
        }
    }
}
