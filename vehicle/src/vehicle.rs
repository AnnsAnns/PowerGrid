use std::{char, f64::consts::PI};
use log::info;
use powercable::tickgen::INTERVAL_15_MINS;
use rand::Rng;
use serde::Serialize;

use crate::{battery::Battery, database::random_ev};

const INTERVAL_5_MINS: usize = INTERVAL_15_MINS / 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum VehicleStatus {
    RANDOM, // Vehicle is driving randomly
    Driving, // Vehicle is currently driving to a destination
    Charging, // Vehicle is currently charging
    Broken, // Vehicle is broken and cannot be used
}

#[derive(Debug, Serialize)]
pub struct Vehicle {
    name: String,
    model: String,
    status: VehicleStatus,
    location: (f64, f64), // (latitude, longitude)
    destination: (f64, f64), // (latitude, longitude)
    consumption: f64, // Wh/km
    speed: f64, // mps
    battery: Battery,
}

impl Vehicle {
    pub fn new(
        name: String,
        latitude: f64,
        longitude: f64,
    ) -> Self {
        let mut rng = rand::rng();
        let (model, consumption,capacity , max_charge) = random_ev();
        let battery = Battery::new(capacity, rng.random_range(0.4..1.0), max_charge);
        Vehicle {
            name: name,
            model: model.to_owned(),
            status: VehicleStatus::RANDOM,
            location: (latitude, longitude),
            destination: (latitude, longitude),
            consumption: consumption,
            speed: 50.0 / 3.6,
            battery: battery,
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

    pub fn distance_to(&self, latitude: f64, longitude: f64) -> f64 { // TODO: simplify
        let this_rad = (Vehicle::to_radians(self.location.0), Vehicle::to_radians(self.location.1));
        let other_rad = (Vehicle::to_radians(latitude), Vehicle::to_radians(longitude));

        let lat_diff = other_rad.0 - this_rad.0;
        let lon_diff = other_rad.1 - this_rad.1;

        let haversine_component = (lat_diff / 2.0).sin().powi(2) + this_rad.0.cos() * other_rad.0.cos() * (lon_diff / 2.0).sin().powi(2);
        let angular_distance = 2.0 * haversine_component.sqrt().atan2((1.0 - haversine_component).sqrt());

        let earth_radius_km = 6371.0;
        earth_radius_km * angular_distance
    }

    pub fn get_location(&self) -> (f64, f64) {
        self.location
    }

    pub fn get_destination(&self) -> (f64, f64) {
        self.destination
    }

    pub fn get_consumption(&self) -> f64 {
        self.consumption
    }

    pub fn get_speed_mps(&self) -> f64 {
        self.speed
    }

    pub fn get_speed_kph(&self) -> f64 {
        self.speed * 3.6
    }

    pub fn battery(&mut self) -> &mut Battery {
        &mut self.battery
    }

    pub fn battery_non_mut(&self) -> &Battery {
        &self.battery
    }

    pub fn set_destination(&mut self, latitude: f64, longitude: f64) {
        self.destination = (latitude, longitude);
    }

    pub fn set_speed_kph(&mut self, speed_kph: f64) {
        self.speed = speed_kph / 3.6;
    }

    pub fn get_longitude(&self) -> f64 {
        self.location.1
    }

    pub fn get_latitude(&self) -> f64 {
        self.location.0
    }

    pub fn drive(&mut self) {
        let soc = self.battery.state_of_charge();
        if soc <= 0.0 {
            self.speed = 0.0;
            return;
        }

        let distance_now = self.speed * INTERVAL_5_MINS as f64 / 1000.0; // m to km
        let consumption_now = self.consumption * self.speed_efficiency_factor();
        let charge_requested = distance_now * consumption_now;
        let charge_used = self.battery.remove_charge(charge_requested);
        let charge_factor = charge_requested / charge_used;

        let total_distance = self.distance_to(self.destination.0, self.destination.1) * charge_factor;
        if total_distance > 0.0 {
            let step_ratio = distance_now / total_distance;
            self.location.0 += step_ratio * (self.destination.0 - self.location.0);
            self.location.1 += step_ratio * (self.destination.1 - self.location.1);

            // do the rest for free :)
            if total_distance <= distance_now {
                self.location = self.destination;
            }
        }
    }

    pub fn charge(&mut self, amount: usize) {
        if self.status != VehicleStatus::Charging {
            self.battery.add_charge(amount as f64);
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