use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Battery {
    capacity: usize, // in kWh
    level: usize, // in kWh
    max_charge: usize, // in kW
}

impl Battery {
    /**
     * Creates a new Battery instance.
     * 
     * # Arguments
     * `capacity`: The total capacity of the battery in kWh.
     * `soc`: The initial state of charge of the battery (0..1).
     * `max_charge`: The maximum charge rate of the battery in kW.
     */
    pub fn new(
        capacity: usize,
        soc: f64,
        max_charge: usize,
    ) -> Self {
        Battery {
            capacity,
            level: (capacity as f64 * soc) as usize,
            max_charge,
        }
    }

    /**
     * Returns the total capacity of the battery in kWh.
     */
    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    /**
     * Returns the current level of the battery in kWh.
     */
    pub fn get_level(&self) -> usize {
        self.level
    }

    /**
     * Returns the state of charge (SoC) of the battery as a percentage (0..1).
     */
    pub fn get_soc(&self) -> f64 {
        self.level as f64 / self.capacity as f64
    }

    /**
     * Returns the free capacity of the battery in kWh.
     */
    pub fn get_free_capacity(&self) -> usize {
        self.capacity as usize - self.level as usize
    }

    pub fn add_charge(&mut self, charge: usize) -> usize {
        // apply scaling
        let applied_charge = charge.min(self.max_charge);
        let charge_rate = applied_charge as f64 * self.charge_scaling();
        
        // consume energy
        let charge_efficiency = 0.9;
        let charge_efficiency = charge_efficiency;
        let energy_added = (charge_rate  * charge_efficiency) as usize;
        self.level = (self.level + energy_added).min(self.capacity);
        charge_rate as usize
    }
    
    pub fn remove_charge(&mut self, charge: usize) -> usize {
        let discharge_efficiency = 0.94;
        let energy_demand = (charge as f64 * discharge_efficiency) as usize;
        let energy_delivered = if self.level >= energy_demand {
            self.level -= energy_demand;
            energy_demand
        } else {
            let actual_energy = self.level as f64 * discharge_efficiency;
            self.level = 0;
            actual_energy as usize
        };
        energy_delivered
    }

    fn charge_scaling(&self) -> f64 {
        let soc = self.get_soc();

        // trickle charging (0..10%)
        if soc < 0.1 {
            0.1
        }
        // constant current (10..80%)
        else if soc < 0.8 {
            1.0
        }
        // constant voltage (80..100%)
        else {
            let linear_scale = (1.0 - soc) / (1.0 - 0.8);
            linear_scale.powf(1.5)
        }
    }
}