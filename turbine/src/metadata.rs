use crate::{data_parser_structs::MetaData, ftp_access};

const WIND_METADATA_URL: &str = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/10_minutes/wind/now/zehn_now_ff_Beschreibung_Stationen.txt";
const AIR_TEMP_METADATA_URL: &str = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/10_minutes/air_temperature/now/zehn_now_tu_Beschreibung_Stationen.txt";
const REQUEST_URL_TEMP: &str = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/10_minutes/air_temperature/now/10minutenwerte_TU_";
const REQUEST_URL_WIND: &str = "https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/10_minutes/wind/now/10minutenwerte_wind_";

#[derive(Debug, Clone)]
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

    pub fn to_string(&self) -> String {
        match self {
            MetaDataType::Wind => "Wind".to_string(),
            MetaDataType::AirTemperature => "AirTemperature".to_string(),
        }
    }

    pub fn to_access_url(&self) -> String {
        match self {
            MetaDataType::Wind => REQUEST_URL_WIND.to_string(),
            MetaDataType::AirTemperature => REQUEST_URL_TEMP.to_string(),
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

    /// Returns the metadata for a specific station ID
    /// Note, if the distance is too large, it may be useless but we still return it
    pub fn get_nearest_station(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> Option<&MetaData> {
        self.meta_data.iter().min_by(|a, b| {
            let dist_a = ((a.geo_breite - latitude).powi(2) + (a.geo_laenge - longitude).powi(2)).sqrt();
            let dist_b = ((b.geo_breite - latitude).powi(2) + (b.geo_laenge - longitude).powi(2)).sqrt();
            dist_a.partial_cmp(&dist_b).unwrap()
        })
    }
}
