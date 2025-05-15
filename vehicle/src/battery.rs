#[derive(Debug)]
pub struct Battery {
    capacity: f64, // in kWh
    level: f64, // in kWh
    temperature: f64, // in °C
    cooling_rate: f64,
    max_charge: f64, // in kW
    max_discharge: f64, // in kW
    charge_efficiency: f64,
    discharge_efficiency: f64,
}

impl Battery {
    pub fn new(
        capacity: f64,
        soc: f64, // State of Charge (0..1)
        temperature: f64,
        cooling_rate: f64,
        max_charge: f64,
        max_discharge: f64,
        charge_efficiency: f64,
        discharge_efficiency: f64,
    ) -> Self {
        Battery {
            capacity,
            level: capacity * soc,
            temperature,
            cooling_rate,
            max_charge,
            max_discharge,
            charge_efficiency,
            discharge_efficiency,
        }
    }

    pub fn state_of_charge(&self) -> f64 {
        self.level / self.capacity
    }

    pub fn add_charge(&mut self, charge: f64, tick_time: f64, ambient_temperature: f64) -> f64 {
        // apply scaling
        let applied_charge = charge.min(self.max_charge);
        let charge_rate = applied_charge * self.charge_scaling() * self.temperature_factor();
        
        // consume energy
        let energy_drawn = charge_rate * tick_time;
        let charge_factor = self.temperature_factor();
        let charge_efficiency = self.charge_efficiency * charge_factor;
        let energy_added = energy_drawn * charge_efficiency;
        self.level = (self.level + energy_added).min(self.capacity);

        // simulate heating
        let internal_heating = energy_drawn.powf(1.1) * 0.6;
        self.update_temperature(ambient_temperature, internal_heating);

        energy_drawn
    }
    
    pub fn remove_charge(&mut self, charge: f64, tick_time: f64, ambient_temperature: f64) -> f64 {
        // apply temperature factor
        let discharge_factor = self.temperature_factor();
        let applied_charge = (charge.min(self.max_discharge)) * discharge_factor;
    
        // energy demand
        let desired_energy = applied_charge * tick_time;
        let energy_output = desired_energy * self.discharge_efficiency;
    
        // energy delivered
        let energy_delivered = if self.level >= energy_output {
            self.level -= energy_output;
            energy_output
        } else {
            let actual_energy = self.level * self.discharge_efficiency;
            self.level = 0.0;
            actual_energy
        };
    
        // simulate heating
        let internal_heating = (applied_charge.powf(1.05) + desired_energy.powf(1.1)) * 0.5;
        self.update_temperature(ambient_temperature, internal_heating);
    
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

    fn temperature_factor(&self) -> f64 {
        let optimal = 25.0;
        let spread = 10.0;
        f64::exp(-((self.temperature - optimal) / spread).powi(2))
    }

    fn update_temperature(&mut self, ambient_temperature: f64, internal_heating: f64) {
        self.temperature += internal_heating;
        let cooling_rate = 0.05 + (self.temperature - ambient_temperature).abs() * 0.002;
        self.temperature += (ambient_temperature - self.temperature) * cooling_rate;
    }
}