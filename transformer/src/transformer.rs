pub struct Transformer {
    total_consumption: f64,
    total_power: f64,
    current_consumption: f64,
    current_power: f64,
}

impl Transformer {
    pub fn new() -> Self {
        Transformer {
            total_consumption: 0.0,
            total_power: 0.0,
            current_consumption: 0.0,
            current_power: 0.0,
        }
    }

    pub fn add_consumption(&mut self, consumption: f64) {
        self.current_consumption += consumption;
        self.total_consumption += consumption;
    }

    pub fn add_power(&mut self, power: f64) {
        self.current_power += power;
        self.total_power += power;
    }

    pub fn get_current_consumption(&self) -> f64 {
        self.current_consumption
    }

    pub fn get_current_power(&self) -> f64 {
        self.current_power
    }

    pub fn get_total_consumption(&self) -> f64 {
        self.total_consumption
    }

    pub fn get_total_power(&self) -> f64 {
        self.total_power
    }

    pub fn reset(&mut self) {
        self.current_consumption = 0.0;
        self.current_power = 0.0;
    }

    pub fn get_difference(&self) -> f64 {
        self.current_power - self.current_consumption
    }
}