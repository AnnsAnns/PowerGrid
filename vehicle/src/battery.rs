use serde::Serialize;

pub const CHARGE_EFFICIENCY: f64 = 0.9;// 90% charge efficiency
pub const DISCHARGE_EFFICIENCY: f64 = 0.94;// 94% discharge efficiency

/// # Description
/// The `Battery` struct represents an electric vehicle's battery.
/// 
/// # Fields
/// - `max_capacity`: The maximum capacity of the battery in kWh.
/// - `level`: The current level of charge in the battery in kWh.
/// - `max_charge_rate`: The maximum charge rate of the battery in kW. !! 150 kW means 150 kWh can be added in one hour. !!
#[derive(Clone, Debug, Serialize)]
pub struct Battery {
    max_capacity: f64,
    level: f64,
    max_charge_rate: usize,
}

impl Battery {
    /// # Description
    /// Creates a new Battery instance.
    /// 
    /// # Arguments
    /// - `max_capacity`: The maximum capacity of the battery in kWh.
    /// - `soc`: The state of charge of the battery as a fraction (0.0 to 1.0).
    /// - `max_charge_rate`: The maximum charge rate of the battery in kW.
    /// 
    /// # Returns
    /// A new Battery instance with the specified parameters.
    pub fn new(
        max_capacity: f64,
        soc: f64,
        max_charge_rate: usize,
    ) -> Self {
        Battery {
            max_capacity,
            level: max_capacity * soc,
            max_charge_rate,
        }
    }

    /// # Returns
    /// The total capacity of the battery in kWh.
    pub fn get_max_capacity(&self) -> f64 {
        self.max_capacity
    }

    /// # Returns
    /// The current level of charge in the battery in kWh.
    pub fn get_level(&self) -> f64 {
        self.level
    }

    /// # Returns
    /// The maximum charge rate of the battery in kW.
    pub fn get_max_charge_rate(&self) -> usize {
        self.max_charge_rate
    }

    /// # Returns
    /// The state of charge (SoC) of the battery(0.0 to 1.0).
    pub fn get_soc(&self) -> f64 {
        self.get_level()/ self.get_max_capacity()
    }

    /// # Returns
    /// The current state of charge (SoC) of the battery as a percentage (0 to 100).
    pub fn get_soc_percentage(&self) -> f64 {
        self.get_soc() * 100.0
    }

    /// # Returns
    /// The amount of free capacity in the battery in kWh.
    pub fn get_free_capacity(&self) -> f64 {
        self.get_max_capacity() - self.get_level()
    }

    /// Calculates the maximum amount of charge that can be added to the battery.
    /// # Arguments
    /// `charge`: An optional parameter that specifies the amount of charge to be added.
    /// If `None`, the maximum charge rate of the battery is used.
    pub fn max_addable_charge(&self, charge: Option<usize>) -> usize {
        // apply scaling
        let charge = charge.unwrap_or(self.max_charge_rate);
        let applied_charge = charge.min(self.get_free_capacity() as usize);
        let charge_rate = applied_charge as f64 * self.charge_scaling();

        // calculate energy that could be added
        let energy_added = charge_rate * CHARGE_EFFICIENCY;
        (energy_added.max(1.0) as usize).min(self.get_free_capacity() as usize)
    }

    pub fn add_charge(&mut self, charge: usize) -> usize {
        // apply scaling
        let energy_added = self.max_addable_charge(Some(charge));

        self.level = (self.level + energy_added as f64).min(self.get_max_capacity());
        energy_added
    }
    
    pub fn remove_charge(&mut self, charge: f64) -> f64 {
        let energy_demand = charge * DISCHARGE_EFFICIENCY;
        
        if self.level >= energy_demand {
            self.level -= energy_demand;
            energy_demand
        } else {
            let actual_energy = self.level * DISCHARGE_EFFICIENCY;
            self.level = 0.0;
            if actual_energy > 0.0 {
                actual_energy
            } else {
                1.0
            }
        }
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