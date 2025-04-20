#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct MetaDataElement {
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

impl MetaDataElement {
    pub fn calculate_distance(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> f64 {
        let lat_diff = self.geo_breite - latitude;
        let lon_diff = self.geo_laenge - longitude;
        // println!(
        //     "{}: Latitude difference: {}, ({}-{}) \n Longitude difference: {} ({}-{})",
        //     self.stationsname,
        //     lat_diff,
        //     self.geo_breite,
        //     latitude,
        //     lon_diff,
        //     self.geo_laenge,
        //     longitude
        // );
        (lat_diff.powi(2) + lon_diff.powi(2)).sqrt()
    }

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
        MetaDataElement {
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
            "ID: {}, Height: {}, Breite: {}, Länge: {}, Name: {}, Bundesland: {}",
            self.stations_id,
            self.stationshoehe,
            self.geo_breite,
            self.geo_laenge,
            self.stationsname,
            self.bundesland
        )
    }
}