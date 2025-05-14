use chrono::NaiveTime;
use tokio::{fs::File, io::{AsyncBufReadExt, BufReader}};

#[derive(Debug, Clone, Copy)]
pub enum ConsumerType {
    H0,
    G0,
    G1,
    G2,
    G3,
    G4,
    G5
}

impl ConsumerType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "H0" => ConsumerType::H0,
            "G0" => ConsumerType::G0,
            "G1" => ConsumerType::G1,
            "G2" => ConsumerType::G2,
            "G3" => ConsumerType::G3,
            "G4" => ConsumerType::G4,
            "G5" => ConsumerType::G5,
            _ => panic!("Unknown consumer type"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ConsumerType::H0 => "H0".to_string(),
            ConsumerType::G0 => "G0".to_string(),
            ConsumerType::G1 => "G1".to_string(),
            ConsumerType::G2 => "G2".to_string(),
            ConsumerType::G3 => "G3".to_string(),
            ConsumerType::G4 => "G4".to_string(),
            ConsumerType::G5 => "G5".to_string(),
        }
    }

    pub fn to_icon(&self) -> String {
        match self {
            ConsumerType::H0 => ":derelict_house_building:".to_string(),
            ConsumerType::G0 => ":convenience_store:".to_string(),
            ConsumerType::G1 => ":post_office:".to_string(),
            ConsumerType::G2 => ":weight_lifter:".to_string(),
            ConsumerType::G3 => ":factory:".to_string(),
            ConsumerType::G4 => ":barber:".to_string(),
            ConsumerType::G5 => ":croissant:".to_string(),
        }
    }
}


pub struct Consumer {
    latitude: f64,
    longitude: f64,
    name: String,
    consumer_type: ConsumerType,
    current_consumption: f32,
    current_scale: u8,
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
            consumer_type,
            current_consumption: 0.0,
            current_scale: 1,
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

    pub fn set_current_consumption(&mut self, consumption: f32) {
        self.current_consumption = consumption;
    }

    pub fn get_current_consumption(&self) -> f32 {
        self.current_consumption
    }

    pub async fn get_demand(&mut self, time: NaiveTime) -> Option<f32> {
        let file = File::open("../tmp/slp.csv").await.ok()?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await.ok()? {
            let columns: Vec<&str> = line.split(',').collect();
            if let Some(record_time) = columns.get(0) {
                if let Ok(parsed_time) = NaiveTime::parse_from_str(record_time, "%H:%M:%S") {
                    if parsed_time == time {
                        if let Some(value) = columns.get(self.consumer_type.clone() as usize + 1) {
                            if let Ok(demand) = value.parse::<f32>() {
                                return Some(demand); // Wert direkt zurückgeben
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn set_current_scale(&mut self, scale: u8) {
        self.current_scale = scale;
    }

    pub fn get_current_scale(&self) -> u8 {
        self.current_scale
    }
}