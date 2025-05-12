#[derive(Debug, Clone)]
pub enum ConsumerType {
    G0,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    L0,
    L1,
    L2,
    H0,
    H0DYN,
}

impl ConsumerType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "G0" => ConsumerType::G0,
            "G1" => ConsumerType::G1,
            "G2" => ConsumerType::G2,
            "G3" => ConsumerType::G3,
            "G4" => ConsumerType::G4,
            "G5" => ConsumerType::G5,
            "G6" => ConsumerType::G6,
            "G7" => ConsumerType::G7,
            "L0" => ConsumerType::L0,
            "L1" => ConsumerType::L1,
            "L2" => ConsumerType::L2,
            "H0" => ConsumerType::H0,
            "H0DYN" => ConsumerType::H0DYN,
            _ => panic!("Unknown consumer type"),
        }
    }
}


pub struct Consumer {
    latitude: f64,
    longitude: f64,
    name: String,
    consumer_type: ConsumerType,
}

impl Consumer {
    pub fn new(
        latitude: f64,
        longitude: f64,
        name: String,
        consumer_type: ConsumerType,
    ) -> Self {
        Consumer {
            latitude,
            longitude,
            name,
            consumer_type: ConsumerType::G0,
        }
    }

    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_consumer_type(&self) -> ConsumerType {
        self.consumer_type.clone()
    }

    pub fn amount_of_needed_packages(&self) -> usize {
        0
    }

    pub fn get_price_if_had_charge(&self, _amount: usize) -> f64 {
        0.0
    }
}