use std::f64::consts::PI;
use powercable::{tickgen::INTERVAL_15_MINS, Position};
use rand::Rng;
use serde::Serialize;

use crate::{battery::Battery, database::random_ev};

const INTERVAL_5_MINS: usize = INTERVAL_15_MINS / 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
/**
 * VehicleStatus represents the current status of a vehicle.
 * It can be one of the following:
 * - Randp,: Vehicle is driving randomly
 * - SearchingForCharger: Vehicle is searching for a charging station
 * - Charging: Vehicle is currently charging
 * - Broken: Vehicle is broken and cannot be used
 */
pub enum VehicleStatus {
    RANDOM,
    SearchingForCharger,
    Charging,
    Broken,
}

#[derive(Debug, Serialize)]
/**
 * Vehicle represents an electric vehicle in the simulation.
 * It contains information about the vehicle's name, model, status, location, destination,
 * consumption, battery, and port number if charging.
 */
pub struct Vehicle {
    name: String,
    model: String,
    status: VehicleStatus,
    location: Position,
    destination: Position,
    consumption: f64, // kWh/100 km
    speed: f64, // mps
    battery: Battery,
}

impl Vehicle {
    /**
     * Creates a new Vehicle instance with a random model, consumption, and battery.
     * * # Arguments
     * `name`: The name of the vehicle.
     * `location`: The geographical position of the vehicle.
     * # Returns
     * A new Vehicle instance with the specified name and location, and random model, consumption, and battery.
     */
    pub fn new(
        name: String,
        location: Position,
    ) -> Self {
        let mut rng = rand::rng();
        let (model, consumption,capacity , max_charge) = random_ev();
        let battery = Battery::new(capacity, rng.random_range(0.4..1.0), max_charge);
        Vehicle {
            name,
            model: model.to_owned(),
            status: VehicleStatus::RANDOM,
            location,
            destination: location, // Initially, the destination is the same as the location
            consumption,
            speed: 50.0 / 3.6,
            battery,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_model(&self) -> &String {
        &self.model
    }

    pub fn get_status(&self) -> &VehicleStatus {
        &self.status
    }

    pub fn set_status(&mut self, status: VehicleStatus) {
        self.status = status;
    }

    pub fn get_consumption(&self) -> f64 {
        self.consumption
    }
  
    pub fn distance_to(&self, latitude: f64, longitude: f64) -> f64 { // TODO: simplify
        let this_rad = (Vehicle::to_radians(self.location.latitude), Vehicle::to_radians(self.location.longitude));
        let other_rad = (Vehicle::to_radians(latitude), Vehicle::to_radians(longitude));

        let lat_diff = other_rad.0 - this_rad.0;
        let lon_diff = other_rad.1 - this_rad.1;

        let haversine_component = (lat_diff / 2.0).sin().powi(2) + this_rad.0.cos() * other_rad.0.cos() * (lon_diff / 2.0).sin().powi(2);
        let angular_distance = 2.0 * haversine_component.sqrt().atan2((1.0 - haversine_component).sqrt());

        let earth_radius_km = 6371.0;
        earth_radius_km * angular_distance
    }

    pub fn get_location(&self) -> Position {
        self.location
    }

    pub fn get_destination(&self) -> Position {
        self.destination
    }

    pub fn set_destination(&mut self, destination: Position) {
        self.destination = destination;
    }

    pub fn get_speed_mps(&self) -> f64 {
        self.speed
    }

    pub fn get_speed_kph(&self) -> f64 {
        self.speed * 3.6
    }

    pub fn set_speed_kph(&mut self, speed_kph: f64) {
        self.speed = speed_kph / 3.6;
    }

    pub fn battery(&mut self) -> &mut Battery {
        &mut self.battery
    }

    pub fn battery_non_mut(&self) -> &Battery {
        &self.battery
    }

    pub fn get_longitude(&self) -> f64 {
        self.location.longitude
    }

    pub fn get_latitude(&self) -> f64 {
        self.location.latitude
    }

    pub fn drive(&mut self) {
        let soc = self.battery.get_soc();
        if soc <= 0.0 || self.status == VehicleStatus::Charging {
            self.speed = 0.0;
            return;
        }

        let distance_now = self.speed * INTERVAL_5_MINS as f64 / 1000.0; // m to km
        let consumption_now = self.consumption * self.speed_efficiency_factor();
        let charge_requested = distance_now * consumption_now;
        let charge_used = self.battery.remove_charge(charge_requested);
        let charge_factor = charge_requested / charge_used;

        let total_distance = self.distance_to(self.destination.latitude, self.destination.longitude) * charge_factor;
        if total_distance > 0.0 {
            let step_ratio = distance_now / total_distance;
            self.location.latitude += step_ratio * (self.destination.latitude - self.location.latitude);
            self.location.longitude += step_ratio * (self.destination.longitude - self.location.longitude);

            // do the rest for free :)
            if total_distance <= distance_now {
                self.location = self.destination;
            }
        }
    }

    fn speed_efficiency_factor(&self) -> f64 {
        let rolling_resistance = 0.0005; // approximate coefficient
        let aerodynamic_drag = 0.00003; // approximate drag factor
        1.0 + rolling_resistance * self.speed + aerodynamic_drag * self.speed.powi(2)
    }

    fn to_radians(deg: f64) -> f64 {
        deg * PI / 180.0
    }
}