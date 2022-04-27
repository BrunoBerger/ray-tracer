
use crate::hit;
use crate::ray;
// use crate::vector;
use crate::vector::Vector;
use crate::materials;

#[derive(Debug, Clone, Copy)]
pub struct Quad {
    min: Vector,
    max: Vector,
    pub material: materials::Material,
}

impl Quad {
    pub fn new(min: Vector, max: Vector, material: materials::Material) -> Quad {
        Quad{min, max, material}
    }
}

impl hit::Hittable for Quad {
    fn intersect(&self, ray: ray::Ray) -> Option<hit::Hit> {
        unimplemented!();
    }
}