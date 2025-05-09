use log::info;

pub struct Charger {
    latitude: f64,
    longitude: f64,
    name: String,
    rate: usize,
    capacity: usize,
    current_charge: usize,
    charging_ports: usize,
    used_ports: usize,
}

impl Charger {
    pub fn new(
        latitude: f64,
        longitude: f64,
        capacity: usize,
        rate: usize,
        charging_ports: usize,
        name: String,
    ) -> Self {
        Charger {
            latitude,
            longitude,
            capacity,
            rate,
            current_charge: 0,
            charging_ports,
            used_ports: 0,
            name,
        }
    }

    pub fn add_charge(&mut self, charge: usize) -> isize {
        // Take into account charging rate
        let actual_charge = std::cmp::min(charge, self.rate);
        
        if self.current_charge + actual_charge <= self.capacity {
            self.current_charge += actual_charge;
            actual_charge as isize
        } else {
            info!("Charger {} is full. Current charge: {}, Attempted to add: {}", self.name, self.current_charge, actual_charge);
            let remaining_capacity = self.capacity - self.current_charge;
            self.current_charge = self.capacity;
            remaining_capacity as isize
        }
    }

    pub fn remove_charge(&mut self, charge: usize) -> isize {
        // Take into account discharge rate
        let actual_charge = std::cmp::min(charge, self.rate);
        
        if self.current_charge >= actual_charge {
            self.current_charge -= actual_charge;
            actual_charge as isize
        } else {
            info!("Charger {} is empty. Current charge: {}, Attempted to remove: {}", self.name, self.current_charge, actual_charge);
            let remaining_charge = self.current_charge;
            self.current_charge = 0;
            remaining_charge as isize
        }
    }

    pub fn get_charge_percentage(&self) -> f64 {
        (self.current_charge as f64 / self.capacity as f64) * 100.0
    }
}