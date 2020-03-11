const SECONDS_IN_EARTH_YEAR: u64 = 31_557_600;

#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration {
            earth_years: seconds as f64 / SECONDS_IN_EARTH_YEAR as f64,
        }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD_IN_EARTH_YEARS: f64;

    fn years_during(d: &Duration) -> f64 {
        d.earth_years / Self::ORBITAL_PERIOD_IN_EARTH_YEARS
    }
}

// TODO: use macro
pub struct Mercury;

pub struct Venus;

pub struct Earth;

pub struct Mars;

pub struct Jupiter;

pub struct Saturn;

pub struct Uranus;

pub struct Neptune;

impl Planet for Mercury {
    const ORBITAL_PERIOD_IN_EARTH_YEARS: f64 = 0.240_846_7;
}

impl Planet for Venus {
    const ORBITAL_PERIOD_IN_EARTH_YEARS: f64 = 0.615_197_26;
}

impl Planet for Earth {
    const ORBITAL_PERIOD_IN_EARTH_YEARS: f64 = 1.;
}

impl Planet for Mars {
    const ORBITAL_PERIOD_IN_EARTH_YEARS: f64 = 1.880_815_8;
}

impl Planet for Jupiter {
    const ORBITAL_PERIOD_IN_EARTH_YEARS: f64 = 11.862_615;
}

impl Planet for Saturn {
    const ORBITAL_PERIOD_IN_EARTH_YEARS: f64 = 29.447_498;
}

impl Planet for Uranus {
    const ORBITAL_PERIOD_IN_EARTH_YEARS: f64 = 84.016_846;
}

impl Planet for Neptune {
    const ORBITAL_PERIOD_IN_EARTH_YEARS: f64 = 164.79132;
}
