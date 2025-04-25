struct Charger {
    client: rumqttc::AsyncClient,
    capacity: usize,
    current_charge: usize,
    charging_ports: usize,
    used_ports: usize,
}