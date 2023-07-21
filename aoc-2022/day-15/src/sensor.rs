use std::{fmt::Display, ops::Range};

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Sensor {
    pub point: Point,
    pub beacon: Point,
    pub bounds: Bounds,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Bounds {
    top: Point,
    left: Point,
    right: Point,
    bottom: Point,
}

impl Bounds {
    pub fn range_y(&self, y: i32) -> Option<Range<i32>> {
        None
    }
}

impl IntoIterator for Bounds {
    type Item = Point;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        vec![self.top, self.left, self.right, self.bottom].into_iter()
    }
}

impl Sensor {
    pub fn distance(&self) -> i32 {
        self.point.distance(self.beacon)
    }
    // p is reachable from the sensor if its distance from the sensor
    // is less than or equal to the distance from the sensor to the beacon.
    pub fn can_reach(&self, p: Point) -> bool {
        let p_dist = self.point.distance(p);
        p_dist <= self.distance()
    }
    pub fn new(point: Point, beacon: Point) -> Self {
        let distance = point.distance(beacon);
        let bounds = Bounds {
            top: Point(point.0, point.1 - distance),
            left: Point(point.0 - distance, point.1),
            right: Point(point.0 + distance, point.1),
            bottom: Point(point.0, point.1 + distance),
        };
        Self {
            point,
            beacon,
            bounds,
        }
    }
}

impl Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sensor(point:{}\tbeacon:{}\tdistance:{})",
            self.point,
            self.beacon,
            self.distance()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reachable() {
        let map_point = Point(25, 22);
        let sensor = Sensor::new(Point(2, 18), Point(-2, 15));
        assert_eq!(sensor.distance(), 7);
        println!("Sensor distance: {}", sensor.point.distance(map_point));
        println!("Beacon distance: {}", sensor.beacon.distance(map_point));
        assert!(!sensor.can_reach(map_point))
    }
}
