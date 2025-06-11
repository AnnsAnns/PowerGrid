use log::{debug, trace};
use powercable::Position;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

#[derive(Debug, Clone, Copy)]
/**
 * ConsumerType represents the type of consumer.
 * It can be Household (H), Business (G), or Farmer (L).
 */
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

    pub fn to_detailed_string(&self) -> String {
        match self {
            ConsumerType::H => "Haushalt".to_string(),
            ConsumerType::G => "Gewerbe".to_string(),
            ConsumerType::L => "Landwirt".to_string(),
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

/**
 * Consumer represents a consumer in the system.
 * It has a position, type, current consumption, and a timeline of demand.
 * The timeline is loaded from a CSV file and contains the demand for each time step.
 */
pub struct Consumer {
    position: Position,
    consumer_type: ConsumerType,
    current_consumption: usize,
    scale: usize,
    timeline: Vec<f32>,
    current_pointer: usize,
}

impl Consumer {
    pub fn new(position: Position, consumer_type: ConsumerType) -> Self {
        Consumer {
            position,
            consumer_type,
            current_consumption: 0,
            scale: 1,
            timeline: Vec::new(),
            current_pointer: 0,
        }
    }

    pub fn get_latitude(&self) -> f64 {
        self.position.latitude
    }

    pub fn get_longitude(&self) -> f64 {
        self.position.longitude
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_consumer_type(&self) -> &ConsumerType {
        &self.consumer_type
    }

    pub fn set_current_consumption(&mut self, consumption: usize) {//TODO: consider scale here soo demand gets deleted
        self.current_consumption = consumption;
    }

    pub fn get_current_consumption(&self) -> usize {
        self.current_consumption
    }

    pub fn tick(&mut self) {
        self.current_pointer = (self.current_pointer + 1) % self.timeline.len();
    }
    
    /**
     * Parse the CSV file and load the demand timeline into memory.
     * This should be called at initialization.
     */
    pub async fn parse_csv(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open("../tmp/slp.csv").await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        self.timeline.clear();

        while let Some(line) = lines.next_line().await? {
            let columns: Vec<&str> = line.split(';').collect();
            trace!("Column: {:?}", columns);

            if let Some(value) = columns.get(self.consumer_type as usize + 1) {
                debug!("Value: {:?}", value);
                if let Ok(demand) = value.parse::<f32>() {
                    trace!("Parsed demand: {:?}", demand);
                    self.timeline.push(demand as f32);
                }
            }
        }

        Ok(())
    }

    /**
     * # Description
     * Returns the current demand based on the timeline and scale.
     * 
     * # Returns
     * The current demand as a usize.
     */
    pub fn get_demand(&self) -> usize {
        (self.timeline.get(self.current_pointer).unwrap().clone() * self.scale as f32)
            as usize
    }

    /**
     * # Description
     * Sets the scale of the consumer.
     * 
     * # Arguments
     * `scale`: The new scale to set.
     */
    pub fn set_scale(&mut self, scale: usize) {
        self.scale = scale;
    }
}
