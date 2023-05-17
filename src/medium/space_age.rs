#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        return Self { seconds: s as f64 };
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({d:?}) to the number of years on this planet for that duration"
        );
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        let earth_seconds = 31557600;
        let planet_seconds = 0.2408467 * earth_seconds as f64;
        let years:f64 = d.seconds / planet_seconds as f64;
        return years;
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        let earth_seconds = 31557600;
        let planet_seconds = 0.61519726 * earth_seconds as f64;
        let years:f64 = d.seconds / planet_seconds as f64;
        return years;
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        let earth_seconds = 31557600;
        let years:f64 = d.seconds / earth_seconds as f64;
        return years;
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        let earth_seconds = 31557600;
        let planet_seconds = 1.8808158 * earth_seconds as f64;
        let years:f64 = d.seconds / planet_seconds as f64;
        return years;
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        let earth_seconds = 31557600;
        let planet_seconds = 11.862615 * earth_seconds as f64;
        let years:f64 = d.seconds / planet_seconds as f64;
        return years;
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        let earth_seconds = 31557600;
        let planet_seconds = 29.447498 * earth_seconds as f64;
        let years:f64 = d.seconds / planet_seconds as f64;
        return years;
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        let earth_seconds = 31557600;
        let planet_seconds = 84.016846 * earth_seconds as f64;
        let years:f64 = d.seconds / planet_seconds as f64;
        return years;
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        let earth_seconds = 31557600;
        let planet_seconds = 164.79132 * earth_seconds as f64;
        let years:f64 = d.seconds / planet_seconds as f64;
        return years;
    }
}


// Needs to implement with macros this solution.
macro_rules! planet {
    ($n:ident, $p:expr) => {
        pub struct $n; impl Planet for $n { fn period() -> f64 { $p } }
    }
}