use rand::seq::IndexedRandom;

const EV_DATABASE: &[(&str, f64, usize, usize)] = &[
    ("Skoda Elroq 85", 0.171, 77, 120),
    ("Mercedes-Benz CLA 250+", 0.15, 85, 1700),
    ("Smart #5 Premium", 0.202, 94, 230),
    ("Kia EV3", 0.171, 78, 105),
    ("Volkswagen ID.7 Pro", 0.162, 77, 125),
    ("Mini Electric Cooper SE", 0.161, 28, 44),
    ("Tesla Model 3 RWD", 0.137, 57, 108),
    ("Tesla Model Y RWD", 0.164, 60, 110),
    ("BMW i4 eDrive35", 0.156, 70, 120),
    ("Hyundai IONIQ 6 Long Range 2WD", 0.149, 77, 220),
    ("Renault Megane E-Tech EV60", 0.158, 60, 130),
    ("Audi A6 Sportback e-tron", 0.161, 100, 250),
    ("Polestar 2 Long Range Single Motor", 0.166, 78, 150),
    ("Porsche Taycan", 0.166, 93, 270),
    ("BYD ATTO 3", 0.183, 60, 71),
    ("Fiat 500e Hatchback", 0.159, 42, 85),
    ("Volkswagen ID.3 Pro", 0.162, 77, 125),
    ("Lucid Air Touring", 0.159, 112, 300),
    ("Ford Capri Standard Range RWD", 0.163, 52, 150),
    ("BMW iX1 eDrive20", 0.166, 66, 130),
    ("Audi Q4 e-tron 50 quattro", 0.175, 82, 135),
    ("Nissan Ariya 87kWh", 0.18, 87, 130),
    ("Volvo EX30 Twin Motor Performance", 0.19, 69, 150),
    ("Genesis GV60 Sport Plus", 0.185, 77, 240),
    ("Honda e:Ny1", 0.172, 62, 100),
    ("Mazda MX-30", 0.19, 35, 50),
];

/**
 * Returns a random electric vehicle (EV) from the database.
 * 
 * # Returns
 * A tuple containing the model name, consumption (kWh/100 km), battery capacity (kWh), and maximum charge rate (kW).
 */
pub fn random_ev() -> (&'static str, f64, usize, usize) {
    let mut rng = rand::rng();
    *EV_DATABASE.choose(&mut rng).unwrap()
}
