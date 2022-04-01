
use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    origin: Vector,
    radius: f64
}

impl Sphere {
    pub fn new(origin: Vector, radius: f64) -> Sphere {
        Sphere{origin, radius}
    }
}