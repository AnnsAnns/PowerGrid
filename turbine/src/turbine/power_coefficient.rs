#[derive(Debug)]
pub struct WindPowerCoefficient {
    pub wind_speed: f64, // Wind speed in m/s
    pub power_coefficient: f64, // Coefficient (Cp)
    pub power_estimation: f64, // Estimated power output in Watts
}

pub fn find_closest_coefficient_for_wind(wind: f64) -> f64 {
    let coefficients = get_wind_power_coefficients_e101();
    if wind <= coefficients.first().unwrap().wind_speed {
        return coefficients.first().unwrap().power_coefficient;
    }
    if wind >= coefficients.last().unwrap().wind_speed {
        return coefficients.last().unwrap().power_coefficient;
    }

    for window in coefficients.windows(2) {
        if let [a, b] = window {
            if wind >= a.wind_speed && wind <= b.wind_speed {
                let t = (wind - a.wind_speed) / (b.wind_speed - a.wind_speed);
                return a.power_coefficient + t * (b.power_coefficient - a.power_coefficient);
            }
        }
    }

    0.0 // Default value if no interpolation is possible
}

pub fn get_wind_power_coefficients_e101() -> Vec<WindPowerCoefficient> {
    vec![
        WindPowerCoefficient { wind_speed: 1.0, power_coefficient: 0.000, power_estimation: 0.0 },
        WindPowerCoefficient { wind_speed: 2.0, power_coefficient: 0.076, power_estimation: 3.0 },
        WindPowerCoefficient { wind_speed: 3.0, power_coefficient: 0.279, power_estimation: 37.0 },
        WindPowerCoefficient { wind_speed: 4.0, power_coefficient: 0.376, power_estimation: 118.0 },
        WindPowerCoefficient { wind_speed: 5.0, power_coefficient: 0.421, power_estimation: 258.0 },
        WindPowerCoefficient { wind_speed: 6.0, power_coefficient: 0.452, power_estimation: 479.0 },
        WindPowerCoefficient { wind_speed: 7.0, power_coefficient: 0.469, power_estimation: 790.0 },
        WindPowerCoefficient { wind_speed: 8.0, power_coefficient: 0.478, power_estimation: 1200.0 },
        WindPowerCoefficient { wind_speed: 9.0, power_coefficient: 0.478, power_estimation: 1710.0 },
        WindPowerCoefficient { wind_speed: 10.0, power_coefficient: 0.477, power_estimation: 2340.0 },
        WindPowerCoefficient { wind_speed: 11.0, power_coefficient: 0.439, power_estimation: 2867.0 },
        WindPowerCoefficient { wind_speed: 12.0, power_coefficient: 0.358, power_estimation: 3034.0 },
        WindPowerCoefficient { wind_speed: 13.0, power_coefficient: 0.283, power_estimation: 3050.0 },
        WindPowerCoefficient { wind_speed: 14.0, power_coefficient: 0.227, power_estimation: 3050.0 },
        WindPowerCoefficient { wind_speed: 15.0, power_coefficient: 0.184, power_estimation: 3050.0 },
        WindPowerCoefficient { wind_speed: 16.0, power_coefficient: 0.152, power_estimation: 3050.0 },
        WindPowerCoefficient { wind_speed: 17.0, power_coefficient: 0.127, power_estimation: 3050.0 },
        WindPowerCoefficient { wind_speed: 18.0, power_coefficient: 0.107, power_estimation: 3050.0 },
        WindPowerCoefficient { wind_speed: 19.0, power_coefficient: 0.091, power_estimation: 3050.0 },
        WindPowerCoefficient { wind_speed: 20.0, power_coefficient: 0.078, power_estimation: 3050.0 },
        WindPowerCoefficient { wind_speed: 21.0, power_coefficient: 0.067, power_estimation: 3050.0 },
        WindPowerCoefficient { wind_speed: 22.0, power_coefficient: 0.058, power_estimation: 3050.0 },
        WindPowerCoefficient { wind_speed: 23.0, power_coefficient: 0.051, power_estimation: 3050.0 },
        WindPowerCoefficient { wind_speed: 24.0, power_coefficient: 0.045, power_estimation: 3050.0 },
        WindPowerCoefficient { wind_speed: 25.0, power_coefficient: 0.040, power_estimation: 3050.0 },
        WindPowerCoefficient { wind_speed: 34.0, power_coefficient: 0.0, power_estimation: 0.0 },
    ]
}
