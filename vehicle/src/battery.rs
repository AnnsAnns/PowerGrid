use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Battery {
    capacity: f64, // in Wh
    level: f64, // in Wh
    max_charge: f64, // in W
}

impl Battery {
    pub fn new(
        capacity: f64,
        soc: f64, // State of Charge (0..1)
        max_charge: f64,
    ) -> Self {
        Battery {
            capacity,
            level: capacity * soc,
            max_charge,
        }
    }

    pub fn state_of_charge(&self) -> f64 {
        self.level / self.capacity
    }

    pub fn get_free_capacity(&self) -> usize {
        self.capacity as usize - self.level as usize
    }

    pub fn add_charge(&mut self, charge: f64) -> f64 {
        // apply scaling
        let applied_charge = charge.min(self.max_charge);
        let charge_rate = applied_charge * self.charge_scaling();
        
        // consume energy
        let charge_efficiency = 0.9;
        let charge_efficiency = charge_efficiency;
        let energy_drawn = charge_rate;
        let energy_added = energy_drawn * charge_efficiency;
        self.level = (self.level + energy_added).min(self.capacity);
        energy_drawn
    }
    
    pub fn remove_charge(&mut self, charge: f64) -> f64 {
        let discharge_efficiency = 0.94;
        let energy_demand = charge * discharge_efficiency;
        let energy_delivered = if self.level >= energy_demand {
            self.level -= energy_demand;
            energy_demand
        } else {
            let actual_energy = self.level * discharge_efficiency;
            self.level = 0.0;
            actual_energy
        };
        energy_delivered
    }

    fn charge_scaling(&self) -> f64 {
        let soc = self.state_of_charge();

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