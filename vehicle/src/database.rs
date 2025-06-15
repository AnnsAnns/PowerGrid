use rand::seq::IndexedRandom;

const EV_DATABASE: &[(&str, f64, f64, usize)] = &[
    ("Skoda Elroq 85", 17.1, 77.0, 120),
    ("Mercedes-Benz CLA 250+", 15.0, 85.0, 1700),
    ("Smart #5 Premium", 20.2, 94.0, 230),
    ("Kia EV3", 17.1, 78.0, 105),
    ("Volkswagen ID.7 Pro", 16.2, 77.0, 125),
    ("Mini Electric Cooper SE", 16.1, 28.0, 44),
    ("Tesla Model 3 RWD", 13.7, 57.0, 108),
    ("Tesla Model Y RWD", 16.4, 60.0, 110),
    ("BMW i4 eDrive35", 15.6, 70.0, 120),
    ("Hyundai IONIQ 6 Long Range 2WD", 14.9, 77.0, 220),
    ("Renault Megane E-Tech EV60", 15.8, 60.0, 130),
    ("Audi A6 Sportback e-tron", 16.1, 100.0, 250),
    ("Polestar 2 Long Range Single Motor", 16.6, 78.0, 150),
    ("Porsche Taycan", 16.6, 93.0, 270),
    ("BYD ATTO 3", 18.3, 60.0, 71),
    ("Fiat 500e Hatchback", 15.9, 42.0, 85),
    ("Volkswagen ID.3 Pro", 16.2, 77.0, 125),
    ("Lucid Air Touring", 15.9, 112.0, 300),
    ("Ford Capri Standard Range RWD", 16.3, 52.0, 150),
    ("BMW iX1 eDrive20", 16.6, 66.0, 130),
    ("Audi Q4 e-tron 50 quattro", 17.5, 82.0, 135),
    ("Nissan Ariya 87kWh", 18.0, 87.0, 130),
    ("Volvo EX30 Twin Motor Performance", 19.0, 69.0, 150),
    ("Genesis GV60 Sport Plus", 18.5, 77.0, 240),
    ("Honda e:Ny1", 17.2, 62.0, 100),
    ("Mazda MX-30", 19.0, 35.0, 50),
];

/// # Description
/// Returns a random electric vehicle from the database.
/// 
/// # Returns
/// A tuple containing:
/// - The name of the vehicle.
/// - The consumption of the vehicle in kWh/100km.
/// - The battery capacity of the vehicle in kWh.
/// - The maximum charge rate of the vehicle in kW.
pub fn random_ev() -> (&'static str, f64, f64, usize) {
    let mut rng = rand::rng();
    *EV_DATABASE.choose(&mut rng).unwrap()
}
