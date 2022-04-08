
use crate::hit;
use crate::ray;
use crate::vector::Vector;

pub trait Hittable {
    fn intersect(&self, ray: ray::Ray) -> Option<hit::Hit>;
}


#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub t: f64,
    pub point: Vector,
    pub normal: Vector,
}

impl Hit {
    pub fn new (t: f64, point: Vector, normal: Vector) -> Hit {
        Hit{t, point, normal}
    }
}


impl std::fmt::Display for Hit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "P{}N{}", self.point, self.normal)
    }
}
