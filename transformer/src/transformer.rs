pub struct Transformer {
    total_consumption: f64,
    total_power: f64,
    current_consumer_consumption: f64,
    current_charger_consumption: f64,
    current_power: f64,
    battery: f64,
    total_vehicle_detour: f64,
}

impl Transformer {
    pub fn new() -> Self {
        Transformer {
            total_consumption: 0.0,
            total_power: 0.0,
            current_consumer_consumption: 0.0,
            current_charger_consumption: 0.0,
            current_power: 0.0,
            battery: 100.0, // Start with a full battery
            total_vehicle_detour: 0.0,
        }
    }

    pub fn add_consumer_consumption(&mut self, consumption: f64) {
        self.current_consumer_consumption += consumption;
        self.total_consumption += consumption;
    }

    pub fn add_charger_consumption(&mut self, consumption: f64) {
        self.current_charger_consumption += consumption;
        self.total_consumption += consumption;
    }

    pub fn add_power(&mut self, power: f64) {
        self.current_power += power;
        self.total_power += power;
    }

    pub fn add_vehicle_detour(&mut self, detour: f64) {
        self.total_vehicle_detour += detour;
    }

    pub fn get_current_consumer_consumption(&self) -> f64 {
        self.current_consumer_consumption
    }

    pub fn get_current_charger_consumption(&self) -> f64 {
        self.current_charger_consumption
    }

    pub fn get_total_current_consumption(&self) -> f64 {
        self.current_consumer_consumption + self.current_charger_consumption
    }

    pub fn get_current_power(&self) -> f64 {
        self.current_power
    }

    pub fn reset(&mut self) {
        self.current_consumer_consumption = 0.0;
        self.current_charger_consumption = 0.0;
        self.current_power = 0.0;
        self.battery = 0.0;
    }

    pub fn get_difference(&self) -> f64 {
        self.current_power - self.get_total_current_consumption()
    }

    pub fn add_battery(&mut self, amount: f64) {
        self.battery += amount;
    }

    pub fn get_battery(&self) -> f64 {
        self.battery
    }

    pub fn get_total_vehicle_detour(&self) -> f64 {
        self.total_vehicle_detour
    }
}