struct Charger {
    latitude: f64,
    longitude: f64,
    name: String,
    client: rumqttc::AsyncClient,
    capacity: usize,
    current_charge: usize,
    charging_ports: usize,
    used_ports: usize,
}

impl Charger {
    pub fn new(
        latitude: f64,
        longitude: f64,
        client: rumqttc::AsyncClient,
        capacity: usize,
        charging_ports: usize,
        name: String,
    ) -> Self {
        Charger {
            latitude,
            longitude,
            client,
            capacity,
            current_charge: 0,
            charging_ports,
            used_ports: 0,
            name,
        }
    }
}