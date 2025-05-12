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

    pub fn get_location(&self) -> (f64, f64) {
        self.location
    }

    pub fn get_destination(&self) -> (f64, f64) {
        self.destination
    }

    pub fn get_battery(&mut self) -> &mut Battery {
        &mut self.battery
    }

    pub fn set_destination(&mut self, latitude: f64, longitude: f64) {
        self.destination = (latitude, longitude);
    }

}