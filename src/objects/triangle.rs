
use crate::ray;
use crate::hit;
use crate::vector::Vector;
use crate::materials;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    p0: Vector,
    p1: Vector,
    p2: Vector,
    pub material: materials::Material,
}

impl Triangle {
    pub fn new(p0: Vector,p1: Vector,p2: Vector, material: materials::Material) -> Triangle {
        Triangle{p0, p1, p2, material}
    }
}

impl hit::Hittable for Triangle {
    fn intersect(&self, ray: ray::Ray) -> Option<hit::Hit> {
        unimplemented!();
    }
}
