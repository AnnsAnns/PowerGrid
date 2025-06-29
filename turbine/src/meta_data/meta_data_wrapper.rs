use crate::parsing::read_text_from_url;

use super::{MetaDataType, MetaDataElement};

pub struct MetaDataWrapper {
    #[allow(dead_code)] // Removing this would make debugging harder
    pub meta_data_type: MetaDataType,
    pub meta_data: Vec<MetaDataElement>,
}

impl MetaDataWrapper {
    pub async fn new(meta_data_type: MetaDataType) -> Result<Self, String> {
        match read_text_from_url(meta_data_type.to_url()).await {
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
                            fields[0].parse().ok()?,
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
    pub fn get_nearest_n_stations(
        &mut self,
        amount: usize,
        latitude: f64,
        longitude: f64,
    ) -> Option<Vec<&MetaDataElement>> {
        // Sort the metadata by distance to the given latitude and longitude
        self
            .meta_data
            .sort_by(|a, b| {
                //println!("Calculating distance for station: {}", a.stationsname);
                let dist_a = a.calculate_distance(latitude, longitude);
                let dist_b = b.calculate_distance(latitude, longitude);
                dist_a.partial_cmp(&dist_b).unwrap()
            });
        
        // println!("Station names sorted by distance:");
        // for station in &self.meta_data {
        //     println!("Station: {}", station.stationsname);
        //     println!("Distance: {}", station.calculate_distance(latitude, longitude));
        //     println!("---------------------");
        // }

        // Return the nearest station
        if self.meta_data.len() < amount {
            None
        } else {
            let nearest_stations: Vec<&MetaDataElement> = self
                .meta_data
                .iter()
                .take(amount)
                .collect();
            Some(nearest_stations)
        }
    }
}
