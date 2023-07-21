use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Sensor {
    pub point: Point,
    pub beacon: Point,
}

pub struct Beacon {
    sensor: Sensor,
}

impl Sensor {
    fn distance(&self) -> i32 {
        self.point.distance(self.beacon)
    }
    pub fn distance_to(&self, p: Point) -> i32 {
        self.point.distance(p)
    }
    // p is reachable from the sensor if its distance from the sensor
    // is less than or equal to the distance from the sensor to the beacon.
    pub fn reachable(&self, p: Point) -> bool {
        let p_dist = self.point.distance(p);
        let b_dist = self.beacon.distance(p);
        p_dist <= b_dist
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
