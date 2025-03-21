// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    /// s, measured in seconds: {s}
    fn from(s: u64) -> Self {
        Self { seconds: s as f64 }
    }
}

impl From<f64> for Duration {
    fn from(s: f64) -> Self {
        Self { seconds: s }
    }
}

pub trait Planet {
    fn orbital_duration() -> Duration;

    /// convert a duration ({d:?}) to the number of years on this planet for that duration
    fn years_during(d: &Duration) -> f64 {
        d.seconds / Self::orbital_duration().seconds
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
    fn orbital_duration() -> Duration {
        Duration::from(31557600.0 * 0.2408467)
    }
}
impl Planet for Venus {
    fn orbital_duration() -> Duration {
        Duration::from(31557600.0 * 0.61519726)
    }
}
impl Planet for Earth {
    fn orbital_duration() -> Duration {
        Duration::from(31557600.0)
    }
}
impl Planet for Mars {
    fn orbital_duration() -> Duration {
        Duration::from(31557600.0 * 1.8808158)
    }
}
impl Planet for Jupiter {
    fn orbital_duration() -> Duration {
        Duration::from(31557600.0 * 11.862615)
    }
}
impl Planet for Saturn {
    fn orbital_duration() -> Duration {
        Duration::from(31557600.0 * 29.447498)
    }
}
impl Planet for Uranus {
    fn orbital_duration() -> Duration {
        Duration::from(31557600.0 * 84.016846)
    }
}
impl Planet for Neptune {
    fn orbital_duration() -> Duration {
        Duration::from(31557600.0 * 164.79132)
    }
}
