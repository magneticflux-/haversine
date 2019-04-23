extern crate noisy_float;

use noisy_float::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Location {
    pub latitude: R64,
    pub longitude: R64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Units {
    Miles,
    Kilometers,
}

pub fn distance(start: Location, end: Location, units: Units) -> R64 {
    /// Values from Moritz, H. Journal of Geodesy (2000) 74: 128. https://doi.org/10.1007/s001900050278
    const KILOMETERS: f64 = 6371.0087714;
    const MILES: f64 = 3958.76131603933;
    let r: R64;

    match units {
        Units::Miles => r = r64(MILES),
        Units::Kilometers => r = r64(KILOMETERS)
    }

    let d_lat: R64 = (end.latitude - start.latitude).to_radians();
    let d_lon: R64 = (end.longitude - start.longitude).to_radians();
    let lat1: R64 = (start.latitude).to_radians();
    let lat2: R64 = (end.latitude).to_radians();

    let a: R64 = ((d_lat / r64(2.0)).sin()) * ((d_lat / r64(2.0)).sin()) + ((d_lon / r64(2.0)).sin()) * ((d_lon / r64(2.0)).sin()) * (lat1.cos()) * (lat2.cos());
    let c: R64 = r64(2.0) * ((a.sqrt()).atan2((r64(1.0) - a).sqrt()));

    r * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn haversine_distance_in_miles() {
        assert_eq!(r64(0.3412300584989182), distance(Location { latitude: r64(38.898556), longitude: r64(-77.037852) }, Location { latitude: r64(38.897147), longitude: r64(-77.043934) }, Units::Miles));
    }

    #[test]
    fn haversine_distance_in_kilometers() {
        assert_eq!(r64(0.549156547264883), distance(Location { latitude: r64(38.898556), longitude: r64(-77.037852) }, Location { latitude: r64(38.897147), longitude: r64(-77.043934) }, Units::Kilometers));
    }
}
