use rand::Rng;

use crate::battery::Battery;

#[derive(Debug)]
pub struct Vehicle {
    name: String,
    location: (f64, f64), // (latitude, longitude)
    destination: (f64, f64), // (latitude, longitude)
    battery: Battery,
}

impl Vehicle {
    pub fn new(
        name: String,
        latitude: f64,
        longitude: f64,
        battery: Battery,
    ) -> Self {
        Vehicle {
            name: name,
            location: (latitude, longitude),
            destination: (latitude, longitude),
            battery: battery,
        }
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn distance_to(&self, latitude: f64, longitude: f64) -> f64 {
        let lat_diff = self.location.0 - latitude;
        let lon_diff = self.location.1 - longitude;
        (lat_diff * lat_diff + lon_diff * lon_diff).sqrt()
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

    pub fn drive(&mut self) {
        let soc = self.battery.state_of_charge();
        if soc <= 0.0 {
            return;
        }

        // simple placeholder implementation
        let mut rng = rand::rng();
        if self.location.0 < self.destination.0 {
            self.location.0 += rng.random_range(-0.002..0.02);
        }
        if self.location.0 > self.destination.0 {
            self.location.0 -= rng.random_range(-0.002..0.02);
        }
        if self.location.1 < self.destination.1 {
            self.location.1 += rng.random_range(-0.002..0.02);
        }
        if self.location.1 > self.destination.1 {
            self.location.1 -= rng.random_range(-0.002..0.02);
        }
        if self.is_close(0.02) {
            self.location = self.destination
        }

        // placeholder args
        self.battery.remove_charge(0.5, 5.0, 20.0);
    }

    fn is_close(&self, tolerance: f64) -> bool {
        (self.location.0 - self.destination.0).abs() < tolerance &&
        (self.location.1 - self.destination.1).abs() < tolerance
    }
}