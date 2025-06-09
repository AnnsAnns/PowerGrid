use std::{char, f64::consts::PI};
use log::info;
use powercable::tickgen::INTERVAL_15_MINS;
use rand::Rng;

use crate::{battery::Battery, database::random_ev};

const TRAVELED: f64 = 12.5; // equals 50 km/h cause one tick is 15 minutes

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleStatus {
    RANDOM, // Vehicle is driving randomly
    Driving, // Vehicle is currently driving to a destination
    Charging, // Vehicle is currently charging
    Broken, // Vehicle is broken and cannot be used
}

#[derive(Debug)]
pub struct Vehicle {
    name: String,
    model: String,
    status: VehicleStatus,
    location: (f64, f64), // (latitude, longitude)
    destination: (f64, f64), // (latitude, longitude)
    consumption: f64, // Wh/km
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
            battery: battery,
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

    pub fn battery(&mut self) -> &mut Battery {
        &mut self.battery
    }

    pub fn battery_non_mut(&self) -> &Battery {
        &self.battery
    }

    pub fn set_destination(&mut self, latitude: f64, longitude: f64) {
        self.destination = (latitude, longitude);
    }

    pub fn get_longitude(&self) -> f64 {
        self.location.1
    }

    pub fn get_latitude(&self) -> f64 {
        self.location.0
    }

    pub fn drive(&mut self, speed_kmh: f64) {
        let soc = self.battery.state_of_charge();
        if soc <= 0.0 {
            return;
        }

        let distance_now = speed_kmh * (INTERVAL_15_MINS as f64 / 3600.0); // seconds to hours
        let efficiency_factor = Vehicle::speed_efficiency_factor(speed_kmh);
        let consumption_now = self.consumption * efficiency_factor;
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

    fn speed_efficiency_factor(speed_kmh: f64) -> f64 {
        let rolling_resistance = 0.0005; // approximate coefficient
        let aerodynamic_drag = 0.00003; // approximate drag factor
        1.0 + rolling_resistance * speed_kmh + aerodynamic_drag * speed_kmh.powi(2)
    }

    pub fn charge(&mut self, amount: usize) {
        if self.status != VehicleStatus::Charging {
            self.battery.add_charge(amount as f64);
        }
    }

    fn to_radians(deg: f64) -> f64 {
        deg * PI / 180.0
    }
}