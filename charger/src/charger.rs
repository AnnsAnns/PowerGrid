use log::{debug, info};
use powercable::offer::structure::OFFER_PACKAGE_SIZE;

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
            debug!("Charger {} is full. Current charge: {}, Attempted to add: {}", self.name, self.current_charge, actual_charge);
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
            debug!("Charger {} is empty. Current charge: {}, Attempted to remove: {}", self.name, self.current_charge, actual_charge);
            let remaining_charge = self.current_charge;
            self.current_charge = 0;
            remaining_charge as isize
        }
    }

    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    pub fn get_current_charge(&self) -> usize {
        self.current_charge
    }

    pub fn get_charge_percentage(&self) -> f64 {
        (self.current_charge as f64 / self.capacity as f64)
    }

    pub fn get_current_price(&self) -> f64 {
        1.0 - self.get_charge_percentage()
    }

    /// Gets the amount of charge needed to fill the charger
    pub fn amount_of_needed_packages(&self) -> usize {
        // Calculate the number of packages needed to fill the charger
        let remaining_capacity = self.capacity - self.current_charge;
        let packages_needed = (remaining_capacity as f64 / OFFER_PACKAGE_SIZE).ceil() as usize;
        packages_needed
    }

    /// Gets the price of the charger if it had a charge of `amount` added to it.
    /// This is used to progressively reduce the price of buy offers sent
    /// I swear this makes sense :P
    pub fn get_price_if_had_charge(&self, amount: usize) -> f64 {
        1.0 - ((self.current_charge + amount) as f64 / self.capacity as f64)
    }
}