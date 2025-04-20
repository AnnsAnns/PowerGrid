use crate::ftp_access;

use super::{MetaDataType, MetaDataElement};

pub struct MetaDataWrapper {
    meta_data_type: MetaDataType,
    meta_data: Vec<MetaDataElement>,
}

impl MetaDataWrapper {
    pub async fn new(meta_data_type: MetaDataType) -> Result<Self, String> {
        match ftp_access::read_text_from_url(meta_data_type.to_url()).await {
            Ok(data) => {
                let lines: Vec<&str> = data.lines().collect();
                if lines.len() < 3 {
                    return Err("Insufficient data in metadata file".to_string());
                }

                let meta_data: Vec<MetaDataElement> = lines[2..]
                    .iter()
                    .filter_map(|line| {
                        let fields: Vec<&str> = line.split_whitespace().collect();
                        if fields.len() < 9 {
                            return None;
                        }
                        Some(MetaDataElement::new(
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
    ) -> Option<&MetaDataElement> {
        self.meta_data.iter().min_by(|a, b| {
            let dist_a = ((a.geo_breite - latitude).powi(2) + (a.geo_laenge - longitude).powi(2)).sqrt();
            let dist_b = ((b.geo_breite - latitude).powi(2) + (b.geo_laenge - longitude).powi(2)).sqrt();
            dist_a.partial_cmp(&dist_b).unwrap()
        })
    }
}
