use rand::seq::IndexedRandom;


pub fn return_random_location() -> (f64, f64, String) {
    let locations = [
        (53.48966185257319, 10.212405605043587, "Bergedorfer Schloss"),
        (53.55614780911228, 10.022301666081713, "HAW Hamburg"),
        (53.87515713546287, 8.683569769499961, "Cuxhaven"),
        (53.886347242224446, 10.768899992578282, "Lübeck"),
        (53.2571460875733, 9.954602811594505, "Lüneburger Heide"),
        (53.563000628504305, 9.860752036089025, "Botanischer Garten"),
        (53.532942231992436, 9.829231537981025, "Airbus"),
        (53.563307985354, 10.00449269152269, "Außenalster")
    ];
    return locations
        .choose(&mut rand::rng())
        .map(|&(lat, lon, name)| (lat, lon, name.to_string()))
        .unwrap_or((53.55614780911228, 10.022301666081713, "HAW Hamburg".to_string()));
}