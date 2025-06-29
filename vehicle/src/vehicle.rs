use tracing::debug;
use powercable::{tickgen::PHASE_AS_HOUR, Position, RANDOM_SEED};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Serialize, Deserialize};

use crate::{battery::Battery, database::random_ev};

/// Rolling resistance coefficient is used to calculate consumption based on speed
const ROLLING_RESISTANCE: f64 = 0.0005;
/// Aerodynamic drag coefficient is used to calculate consumption based on speed
const AERODYNAMIC_DRAG: f64 = 0.00003;

/// # Description
/// The `VehicleStatus` enum represents the different states a vehicle can be in.
/// 
/// # Variants
/// - `Random`: The vehicle is in a random state.
/// - `Waiting`: The vehicle is waiting.
/// - `SearchingForCharger`: The vehicle is looking for a charger.
/// - `Charging`: The vehicle is currently charging.
/// - `Broken`: The vehicle is broken and cannot be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VehicleStatus {
    Random,
    Parked,
    SearchingForCharger,
    Charging,
    Broken,
}

/// # Description
/// The `VehicleAlgorithm` enum defines the different algorithms that can be used to determine the vehicle's behavior when searching for a charger.
/// 
/// # Variants
/// - `Best`: The vehicle will choose the best charger, based on cheapest overall cost.
/// - `Random`: The vehicle will choose a charger randomly.
/// - `Closest`: The vehicle will choose the closest charger.
/// - `Cheapest`: The vehicle will choose the cheapest charger, based on price per kWh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VehicleAlgorithm {
    Best,
    Random,
    Closest,
    Cheapest,
}

/// # Description
/// The `Deadline` struct represents a deadline for a vehicle to reach a certain level of charge by a certain tick.
/// 
/// # Fields
/// - `ticks_remaining`: The tick at which the deadline is set. Counts down, each tick represents 5 minutes.
/// - `target_soc`: The state of charge that the vehicle must reach by the deadline.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VehicleDeadline {
    pub ticks_remaining: i64,
    pub target_soc: f64,
}

/// # Description
/// The `Vehicle` struct represents an electric vehicle in our simulation.
/// It can drive and charge on a `charger::Charger`.
/// 
/// # Fields
/// - `name`: The name of the vehicle.
/// - `model`: The model of the vehicle.
/// - `status`: The current status of the vehicle, default is `VehicleStatus::Random`.
/// - `location`: The current geographical position of the vehicle.
/// - `next_stop`: The next destination position of the vehicle (for charging).
/// - `destination`: The final destination position of the vehicle.
/// - `consumption`: The consumption of the vehicle in kWh per 100 km.
/// - `scale`: A scale factor for the vehicle's consumption, default is 1.0.
/// - `speed`: The speed of the vehicle in km/h.
/// - `battery`: The battery of the vehicle, which contains information about its capacity, current charge level, and maximum charge rate.
/// - `algorithm`: The algorithm used by the vehicle to determine its behavior when searching for a charger.
#[derive(Clone, Debug, Serialize)]
pub struct Vehicle {
    name: String,
    model: String,
    status: VehicleStatus,
    location: Position,
    next_stop: Position,
    destination: Position,
    consumption: f64,
    scale: f64,
    speed: usize,
    battery: Battery,
    algorithm: VehicleAlgorithm,
    deadline: VehicleDeadline,
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
    /// A new `Vehicle` instance with the specified `name` and `location`, and a random `model`, `consumption`, `battery`.
    pub fn new(
        name: String,
        location: Position,
        seed: u64,
    ) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let (model, consumption, capacity, max_charge) = random_ev(seed);
        let battery = Battery::new(capacity, rng.random_range(0.4..1.0), max_charge);
        Vehicle {
            name,
            model: model.to_owned(),
            status: VehicleStatus::Parked,
            location,
            next_stop: location,
            destination: location,// Initially, the destination is the same as the location
            consumption,
            scale: 1.0,
            speed: 0,
            battery,
            algorithm: VehicleAlgorithm::Best,
            deadline: VehicleDeadline { ticks_remaining: 12 * 24, target_soc: 0.8 }
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

    /// # Returns
    /// The location of the vehicle as a `Position`.
    pub fn get_location(&self) -> Position {
        self.location
    }

    /// # Sets
    /// The next-stop-destination of the vehicle.
    pub fn set_next_stop(&mut self, next_stop: Position) {
        self.next_stop = next_stop;
    }

    /// # Returns
    /// The next-stop-destination of the vehicle as a `Position`.
    pub fn get_next_stop(&self) -> Position {
        self.next_stop
    } 

    /// # Sets
    /// The destination of the vehicle.
    pub fn set_destination(&mut self, destination: Position) {
        self.next_stop = destination;
        self.destination = destination;
    }

    /// # Returns
    /// The destination of the vehicle as a `Position`.
    pub fn get_destination(&self) -> Position {
        self.destination
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

    /// # Sets
    /// The algorithm used by the vehicle to determine its behavior when searching for a charger.
    pub fn set_algorithm(&mut self, algorithm: VehicleAlgorithm) {
        self.algorithm = algorithm;
    }

    /// # Returns
    /// The algorithm used by the vehicle to determine its behavior when searching for a charger.
    pub fn get_algorithm(&self) -> VehicleAlgorithm {
        self.algorithm
    }

    /// # Sets
    /// The deadline to which a the battery must be charged.
    pub fn set_deadline(&mut self, deadline: VehicleDeadline) {
        self.deadline = deadline;
    }

    /// # Returns
    /// The deadline to which a the battery must be charged.
    pub fn get_deadline(&self) -> VehicleDeadline {
        self.deadline
    }

    /// # Description
    pub fn drive(&mut self) {
        self.deadline.ticks_remaining -= 1;

        // check status and set speed
        if self.status == VehicleStatus::Parked
        || self.status == VehicleStatus::Charging
        || self.status == VehicleStatus::Broken {
            self.speed = 0;
            return;
        } else {
            match self.battery.get_soc() {
                0.0..0.2 => self.speed = 30,
                0.2..0.5 => self.speed = 60,
                _ => self.speed = 90,
            }
        }

        // compute energy demand
        let wanted_distance = self.get_speed() as f64 * PHASE_AS_HOUR;// km/h * h = km
        let wanted_energy = (self.get_current_consumption() / 100.0) * wanted_distance;// kWh/km * km = kWh
        let used_energy = self.battery.remove_charge(wanted_energy);
        debug!("Wanded distance: {}, wanted energy: {}, used energy: {}", 
            wanted_distance, wanted_energy, used_energy);

        let charge_factor = wanted_energy / used_energy;

        // drive
        let total_distance = self.distance_to(self.get_next_stop()) * charge_factor;
        debug!("Total distance: {}", total_distance);
        if total_distance > 0.0 {
            let step_ratio = wanted_distance/ total_distance;
            self.location.latitude += step_ratio * (self.next_stop.latitude - self.location.latitude);
            self.location.longitude += step_ratio * (self.next_stop.longitude - self.location.longitude);

            // do the rest for free :)
            if total_distance <= wanted_distance {
                self.location = self.next_stop;
            }
        }
    }
}