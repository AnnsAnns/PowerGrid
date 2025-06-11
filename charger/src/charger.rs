use log::{debug, warn};
use powercable::{offer::structure::OFFER_PACKAGE_SIZE, Position};

#[derive(Debug, Clone)]
pub struct Charger {
    name: String,
    position: Position,
    rate: usize,// in kw/s
    capacity: usize,// in kWh
    reserved_charge: usize,// in kWh
    current_charge: usize,// in kWh
    charging_ports: usize,
    reserved_ports: usize,
}

impl Charger {
    /**
     * Creates a new Charger instance.
     * 
     * # Arguments
     * `name`: The name of the charger.
     * `position`: The geographical position of the charger.
     * `rate`: The charging rate of the charger in kW/s.
     * `capacity`: The total capacity of the charger in kWh.
     * `charging_ports`: The number of charging ports available on the charger.
     */
    pub fn new(
        name: String,
        position: Position,
        rate: usize,
        capacity: usize,
        charging_ports: usize,
    ) -> Self {
        Charger {
            name,
            position,
            rate,
            capacity,
            reserved_charge: 0,
            current_charge: 0,
            charging_ports,
            reserved_ports: 0,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_available_charge(&self) -> usize {
        // Calculate the available charge based on current charge and reserved charge
        if self.current_charge >= self.reserved_charge {
            self.current_charge - self.reserved_charge
        } else {
            0 // No available charge if reserved exceeds current
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

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_latitude(&self) -> f64 {
        self.position.latitude
    }

    pub fn get_longitude(&self) -> f64 {
        self.position.longitude
    }

    pub fn get_ports(&self) -> usize {
        self.charging_ports
    }

    pub fn get_free_ports(&self) -> usize {
        self.charging_ports - self.reserved_ports
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    pub fn get_current_charge(&self) -> usize {
        self.current_charge
    }

    pub fn get_charge_percentage(&self) -> f64 {
        self.current_charge as f64 / self.capacity as f64
    }

    pub fn get_current_price(&self) -> f64 {
        1.0 - self.get_charge_percentage()
    }

    /// Gets the amount of charge needed to fill the charger
    pub fn amount_of_needed_packages(&self) -> usize {
        // Calculate the number of packages needed to fill the charger
        let remaining_capacity = self.capacity - self.current_charge;

        if remaining_capacity == 0 {
            return 0; // No packages needed if already full
        }

        let packages_needed = (remaining_capacity as f64 / OFFER_PACKAGE_SIZE).ceil() as usize;
        packages_needed
    }

    /// Gets the price of the charger if it had a charge of `amount` added to it.
    /// This is used to progressively reduce the price of buy offers sent
    /// I swear this makes sense :P
    pub fn get_price_if_had_charge(&self, amount: usize) -> f64 {
        let mut price = 1.0 - ((self.current_charge + amount) as f64 / self.capacity as f64);
        // At a certain point we run into float weirdness when transforming
        if price < 0.1 {
            price = 0.1;
        }
        debug!("Charger {} would have price {} if it had {} charge added", self.name, price, amount);

        price
    }

    pub fn reserve_charge(&mut self, charge: usize) -> isize {
        // Reserve charge if available
        if self.get_available_charge() >= charge {
            self.reserved_charge += charge;
            charge as isize
        } else {
            debug!("Charger {} does not have enough available charge to reserve {}. Available: {}", self.name, charge, self.get_available_charge());
            0 // Not enough charge to reserve
        }
    }

    pub fn release_reserved_charge(&mut self, charge: usize) -> isize {
        // Release reserved charge
        if self.reserved_charge >= charge {
            self.reserved_charge -= charge;
            charge as isize
        } else {
            debug!("Charger {} does not have enough reserved charge to release {}. Reserved: {}", self.name, charge, self.reserved_charge);
            0 // Not enough reserved charge to release
        }
    }

    pub fn reserve_port(&mut self) -> bool {
        // Reserve a charging port if available
        if self.reserved_ports < self.charging_ports {
            self.reserved_ports += 1;
            true
        } else {
            debug!("Charger {} has no free ports to reserve", self.name);
            false // No free ports available
        }
    }

    pub fn release_port(&mut self) -> bool {
        // Release a charging port if used
        if self.reserved_ports > 0 {
            self.reserved_ports -= 1;
            true
        } else {
            debug!("Charger {} has no ports to release", self.name);
            false // No ports to release
        }
    }
}