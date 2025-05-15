use chrono::NaiveTime;
use log::{debug, trace};
use tokio::{fs::File, io::{AsyncBufReadExt, BufReader}};

#[derive(Debug, Clone, Copy)]
pub enum ConsumerType {
    H,
    G,
    L,
}

impl ConsumerType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "H" => ConsumerType::H,
            "G" => ConsumerType::G,
            "L" => ConsumerType::L,
            _ => panic!("Unknown consumer type"),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ConsumerType::H => "H".to_string(),
            ConsumerType::G => "G".to_string(),
            ConsumerType::L => "L".to_string(),
        }
    }

    pub fn to_icon(&self) -> String {
        match self {
            ConsumerType::H => ":derelict_house_building:".to_string(),
            ConsumerType::G => ":convenience_store:".to_string(),
            ConsumerType::L => ":cow2:".to_string(),
        }
    }
}


pub struct Consumer {
    latitude: f64,
    longitude: f64,
    consumer_type: ConsumerType,
    current_consumption: usize,
    current_scale: usize,
}

impl Consumer {
    pub fn new(
        latitude: f64,
        longitude: f64,
        consumer_type: ConsumerType,
    ) -> Self {
        Consumer {
            latitude,
            longitude,
            consumer_type,
            current_consumption: 0,
            current_scale: 1,
        }
    }

    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    pub fn get_consumer_type(&self) -> ConsumerType {
        self.consumer_type.clone()
    }

    pub fn set_current_consumption(&mut self, consumption: usize) {
        self.current_consumption = consumption;
    }

    pub fn get_current_consumption(&self) -> usize {
        self.current_consumption
    }

    /**
     * Get the demand for the given time from the CSV file.
     * 
     * @param time The time to get the demand for.
     * @return The demand for the given time. Already rounded and scaled.
     *         Returns None if the time is not found in the CSV file.
     */
    pub async fn get_demand(&self, time: NaiveTime) -> Option<usize> {
        let file = File::open("../tmp/slp.csv").await.ok()?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await.ok()? {
            let columns: Vec<&str> = line.split(';').collect();
            trace!("Column: {:?}", columns);
            if let Some(record_time) = columns.get(0) {
                if let Ok(parsed_time) = NaiveTime::parse_from_str(record_time, "%H:%M") {
                    if parsed_time == time {
                        if let Some(value) = columns.get(self.consumer_type.clone() as usize + 1) {
                            debug!("Value: {:?}", value);
                            if let Ok(demand) = value.parse::<f32>() {
                                trace!("Parsed demand: {:?}", demand);
                                trace!("Current scale: {:?}", self.get_current_scale());
                                let result = (demand * self.get_current_scale() as f32).round() as usize;
                                debug!("Demand: {:?}", result);
                                return Some(result);
                            }
                        }
                    }
                }
            }
        }
        None
    }

    pub fn set_current_scale(&mut self, scale: usize) {
        self.current_scale = scale;
    }

    pub fn get_current_scale(&self) -> usize {
        self.current_scale
    }
}