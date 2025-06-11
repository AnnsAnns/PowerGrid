use std::f64::consts::PI;
use powercable::{tickgen::INTERVAL_15_MINS, Position};
use rand::Rng;

use crate::{battery::Battery, database::random_ev};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug)]
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
    consumption: f64,// kWh/km
    battery: Battery,
    port: Option<usize>,// Port number for charging, if applicable
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
            destination: location,// Initially, the destination is the same as the location
            consumption,
            battery,
            port: None,// Initially, the vehicle is not connected to any charging port
        }
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_model(&self) -> &String {
        return &self.model;
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

    pub fn set_port(&mut self, port: Option<usize>) {
        self.port = port;
    }

    pub fn get_port(&self) -> Option<usize> {
        self.port
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

    pub fn battery(&mut self) -> &mut Battery {
        &mut self.battery
    }

    pub fn battery_non_mut(&self) -> &Battery {
        &self.battery
    }

    pub fn set_destination(&mut self, destination: Position) {
        self.destination = destination;
    }

    pub fn get_longitude(&self) -> f64 {
        self.location.longitude
    }

    pub fn get_latitude(&self) -> f64 {
        self.location.latitude
    }

    pub fn drive(&mut self, speed_kmh: f64) {
        let soc = self.battery.get_soc();
        if soc <= 0.0 {
            return;
        }

        let distance_now = speed_kmh * (INTERVAL_15_MINS as f64 / 3600.0); // seconds to hours
        let efficiency_factor = Vehicle::speed_efficiency_factor(speed_kmh);
        let consumption_now = self.consumption * efficiency_factor;
        let charge_requested = distance_now * consumption_now;
        let charge_used = self.battery.remove_charge(charge_requested as usize) as f64;
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

    fn speed_efficiency_factor(speed_kmh: f64) -> f64 {
        let rolling_resistance = 0.0005; // approximate coefficient
        let aerodynamic_drag = 0.00003; // approximate drag factor
        1.0 + rolling_resistance * speed_kmh + aerodynamic_drag * speed_kmh.powi(2)
    }

    fn to_radians(deg: f64) -> f64 {
        deg * PI / 180.0
    }
}