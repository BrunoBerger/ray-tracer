
use crate::ray;
use crate::vector::Vector;
use crate::hit;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    p0: Vector,
    p1: Vector,
    p2: Vector,
}

impl Triangle {
    pub fn new(p0: Vector,p1: Vector,p2: Vector) -> Triangle {
        Triangle{p0, p1, p2}
    }
}

impl hit::Hittable for Triangle {
    fn intersect(&self, ray: ray::Ray) -> Option<hit::Hit> {
        unimplemented!();
    }
}
