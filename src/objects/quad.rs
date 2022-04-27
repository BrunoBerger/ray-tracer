
use crate::hit;
use crate::ray;
// use crate::vector;
use crate::vector::Vector;
use crate::materials;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    min: Vector,
    max: Vector,
    pub material: materials::Material,
}

impl Aabb {
    pub fn new(min: Vector, max: Vector, material: materials::Material) -> Aabb {
        Aabb{min, max, material}
    }
}

impl hit::Hittable for Aabb {
    fn intersect(&self, _ray: ray::Ray) -> Option<hit::Hit> {
        unimplemented!();
    }
}