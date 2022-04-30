const EARTHS_RADIUS_MILES: f64 = 3960.0;
const EARTHS_RADIUS_KILOMETERS: f64 = 6371.0;

#[derive(Clone)]
pub struct Point {
    latitude: f64,
    longitude: f64,
}

impl Point {
    pub fn distance(&self, b: Point, unit: Unit) -> f64 {
        distance(self.clone(), b, unit)
    }
}

pub enum Unit {
    Miles,
    Kilometers,
}

impl Unit {
    #[inline]
    fn radius(&self) -> f64 {
        use Unit::*;

        match self {
            Miles => EARTHS_RADIUS_MILES,
            Kilometers => EARTHS_RADIUS_KILOMETERS,
        }
    }
}

pub fn distance(a: Point, b: Point, unit: Unit) -> f64 {
    let lat_a: f64 = (a.latitude).to_radians();
    let lat_b: f64 = (b.latitude).to_radians();

    let lat_delta: f64 = (b.latitude - a.latitude).to_radians() / 2.0;
    let lon_delta: f64 = (b.longitude - a.longitude).to_radians() / 2.0;

    let a: f64 = lat_delta.sin().powf(2.0) + lon_delta.sin().powf(2.0) * lat_a.cos() * lat_b.cos();
    let c: f64 = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    unit.radius() * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miles() {
        let a = Point { latitude: 33.456789, longitude: -117.034576 };
        let b = Point { latitude: 32.652387, longitude: -114.098743 };

        let dist = distance(a, b, Unit::Miles);

        assert_eq!(178.91786761171463, dist);
    }

    #[test]
    fn test_zero() {
        let a = Point { latitude: 33.456789, longitude: -117.034576 };
        let b = Point { latitude: 33.456789, longitude: -117.034576 };

        let dist = distance(a, b, Unit::Miles);

        assert_eq!(0.0, dist);
    }

    #[test]
    fn test_kilometers() {
        let a = Point { latitude: 33.456789, longitude: -117.034576 };
        let b = Point { latitude: 32.652387, longitude: -114.098743 };

        let dist = distance(a, b, Unit::Kilometers);

        assert_eq!(287.84993296824086, dist);
    }
}
