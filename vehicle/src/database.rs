use rand::seq::IndexedRandom;

const EV_DATABASE: &[(&str, f64, f64, f64)] = &[
    ("Skoda Elroq 85", 171.0, 77000.0, 120000.0),
    ("Mercedes-Benz CLA 250+", 150.0, 85000.0, 170000.0),
    ("Smart #5 Premium", 202.0, 94000.0, 230000.0),
    ("Kia EV3", 171.0, 78000.0, 105000.0),
    ("Volkswagen ID.7 Pro", 162.0, 77000.0, 125000.0),
    ("Mini Electric Cooper SE", 161.0, 28900.0, 44000.0),
    ("Tesla Model 3 RWD", 137.0, 57500.0, 108000.0),
    ("Tesla Model Y RWD", 164.0, 60000.0, 110000.0),
    ("BMW i4 eDrive35", 156.0, 70000.0, 120000.0),
    ("Hyundai IONIQ 6 Long Range 2WD", 149.0, 77000.0, 220000.0),
    ("Renault Megane E-Tech EV60", 158.0, 60000.0, 130000.0),
    ("Audi A6 Sportback e-tron", 161.0, 100000.0, 250000.0),
    ("Polestar 2 Long Range Single Motor", 166.0, 78000.0, 150000.0),
    ("Porsche Taycan", 166.0, 93000.0, 270000.0),
    ("BYD ATTO 3", 183.0, 60500.0, 71000.0),
    ("Fiat 500e Hatchback", 159.0, 42000.0, 85000.0),
    ("Volkswagen ID.3 Pro", 162.0, 77000.0, 125000.0),
    ("Lucid Air Touring", 159.0, 112000.0, 300000.0),
    ("Ford Capri Standard Range RWD", 163.0, 52000.0, 150000.0),
    ("BMW iX1 eDrive20", 166.0, 66000.0, 130000.0),
    ("Audi Q4 e-tron 50 quattro", 175.0, 82000.0, 135000.0),
    ("Nissan Ariya 87kWh", 180.0, 87000.0, 130000.0),
    ("Volvo EX30 Twin Motor Performance", 190.0, 69000.0, 150000.0),
    ("Genesis GV60 Sport Plus", 185.0, 77000.0, 240000.0),
    ("Honda e:Ny1", 172.0, 62000.0, 100000.0),
    ("Mazda MX-30", 190.0, 35500.0, 50000.0),
];

pub fn random_ev() -> (&'static str, f64, f64, f64) {
    let mut rng = rand::rng();
    *EV_DATABASE.choose(&mut rng).unwrap()
}
