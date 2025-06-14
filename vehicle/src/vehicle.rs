use log::debug;
use powercable::{tickgen::PHASE_AS_HOUR, Position};
use rand::Rng;
use serde::Serialize;

use crate::{battery::Battery, database::random_ev};

const ROLLING_RESISTANCE: f64 = 0.0005; // approximate coefficient
const AERODYNAMIC_DRAG: f64 = 0.00003; // approximate drag factor

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
/// # Description
/// The `VehicleStatus` enum represents the different states a vehicle can be in.
/// 
/// # Variants
/// - `RANDOM`: The vehicle is in a random state.
/// - `SearchingForCharger`: The vehicle is looking for a charger.
/// - `Charging`: The vehicle is currently charging.
/// - `Broken`: The vehicle is broken and cannot be used.
pub enum VehicleStatus {
    RANDOM,
    WAITING,
    SearchingForCharger,
    Charging,
    Broken,
}

#[derive(Debug, Serialize)]
/// # Description
/// The `Vehicle` struct represents an electric vehicle in our simulation.
/// It can drive and charge on a `charger::Charger`.
/// 
/// # Fields
/// - `name`: The name of the vehicle.
/// - `model`: The model of the vehicle.
/// - `status`: The current status of the vehicle.
/// - `location`: The current geographical position of the vehicle.
/// - `destination`: The destination position of the vehicle.
/// - `consumption`: The consumption of the vehicle in kWh per 100 km.
/// - `scale`: A scale factor for the vehicle's consumption, default is 1.0.
/// - `speed`: The speed of the vehicle in km/h, default is 50 km/h.
pub struct Vehicle {
    name: String,
    model: String,
    status: VehicleStatus,
    location: Position,
    destination: Position,
    consumption: f64,
    scale: f64,
    speed: usize,
    battery: Battery,
}

impl Vehicle {
    /// # Description
    /// Creates a new `Vehicle` instance.
    /// 
    /// # Arguments
    /// - `name`: The name of the vehicle.
    /// - `location`: The initial geographical position of the vehicle.
    /// 
    /// # Returns
    /// A new `Vehicle` instance with the specified `name` and `location`, and a random `model`, `consumption`, and `battery`.
    pub fn new(
        name: String,
        location: Position,
    ) -> Self {
        let mut rng = rand::rng();
        let (model, consumption, capacity, max_charge) = random_ev();
        let battery = Battery::new(capacity, rng.random_range(0.4..1.0), max_charge);
        Vehicle {
            name,
            model: model.to_owned(),
            status: VehicleStatus::RANDOM,
            location,
            destination: location,// Initially, the destination is the same as the location
            consumption,
            scale: 1.0,
            speed: 50,
            battery,
        }
    }

    /// # Returns
    /// The name of the vehicle as a `String`.
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// # Returns
    /// The model of the vehicle as a `String`.
    pub fn get_model(&self) -> String {
        self.model.clone()
    }

    /// # Sets
    /// The status of the vehicle.
    /// 
    /// # Arguments
    /// - `status`: The new status to set for the vehicle.
    pub fn set_status(&mut self, status: VehicleStatus) {
        self.status = status;
    }

    /// # Returns
    /// The current status of the vehicle as a `VehicleStatus`.
    pub fn get_status(&self) -> VehicleStatus {
        self.status
    }

    /// # Description
    /// Returns the consumption of the vehicle in kWh per 100 km.
    /// The consumption is scaled by the `scale` factor, which can be adjusted.
    /// 
    /// # Returns
    /// The scaled consumption value (kWh/100km).
    pub fn get_consumption(&self) -> f64 {
        self.consumption * self.scale
    }

    /// # Returns
    /// The speed efficiency factor of the vehicle, which is a function of its speed.
    /// The higher the speed, the more energy is consumed due to rolling resistance and aerodynamic drag.
    fn speed_efficiency_factor(&self) -> f64 {
        1.0 + ROLLING_RESISTANCE * self.speed as f64 + AERODYNAMIC_DRAG * (self.speed as f64).powi(2)
    }

    /// # Returns
    /// The current consumption of the vehicle in kWh/100km, adjusted for the vehicle's speed.
    pub fn get_current_consumption(&self) -> f64 {
        self.get_consumption() * self.speed_efficiency_factor()
    }

    /// # Description
    /// Returns the range of the vehicle in kilometers based on its battery capacity and consumption.
    /// 
    /// # Returns
    /// The range of the vehicle in kilometers.
    pub fn get_range(&self) -> f64 {
        self.battery.get_level() / (self.get_consumption() / 100.0)// kWh / kWh/km = km
    }

    /// # Sets
    /// The scale factor for the vehicle's consumption.
    /// 
    /// # Arguments
    /// - `scale`: The new scale factor to set.
    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    /// # Returns
    /// The location of the vehicle as a `Position`.
    pub fn get_location(&self) -> Position {
        self.location
    }

    /// # Sets
    /// The destination of the vehicle.
    pub fn set_destination(&mut self, destination: Position) {
        self.destination = destination;
    }

    /// # Returns
    /// The destination of the vehicle as a `Position`.
    pub fn get_destination(&self) -> Position {
        self.destination
    }    

    /// # Sets
    /// The speed of the vehicle in km/h.
    pub fn set_speed(&mut self, speed: usize) {
        self.speed = speed;
    }

    /// # Returns
    /// The speed of the vehicle in km/h.
    pub fn get_speed(&self) -> usize {
        self.speed
    }

    /// # Returns
    /// The battery of the vehicle as a mutable reference.
    pub fn battery(&mut self) -> &mut Battery {
        &mut self.battery
    }

    /// # Returns
    /// The battery of the vehicle as a non-mutable reference.
    pub fn battery_non_mut(&self) -> &Battery {
        &self.battery
    }

    /// # Returns
    /// The distance from the vehicle's current location to another position.
    pub fn distance_to(&self, other:Position) -> f64 {
        self.location.distance_to(other)
    }

    /// # Description
    pub fn drive(&mut self) {
        let wanted_distance = self.get_speed() as f64 * PHASE_AS_HOUR;// km/h * h = km
        let wanted_energy = (self.get_current_consumption() / 100.0) * wanted_distance;// kWh/km * km = kWh
        let used_energy = self.battery.remove_charge(wanted_energy);
        debug!("Wanded distance: {}, wanted energy: {}, used energy: {}", 
            wanted_distance, wanted_energy, used_energy);

        let charge_factor = wanted_energy / used_energy;

        let total_distance = self.distance_to(self.get_destination()) * charge_factor;
        debug!("Total distance: {}", total_distance);
        if total_distance > 0.0 {
            let step_ratio = wanted_distance/ total_distance;
            self.location.latitude += step_ratio * (self.destination.latitude - self.location.latitude);
            self.location.longitude += step_ratio * (self.destination.longitude - self.location.longitude);

            // do the rest for free :)
            if total_distance <= wanted_distance {
                self.location = self.destination;
            }
        }
        
    }
}