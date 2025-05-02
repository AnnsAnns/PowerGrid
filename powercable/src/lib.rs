use rand::Rng;

pub fn generate_latitude_longitude_within_germany() -> (f64, f64) {
    let mut rng = rand::rng();
    let latitude = rng.random_range(47.2701..55.0581);
    let longitude = rng.random_range(5.8663..15.0419);
    (latitude, longitude)
}