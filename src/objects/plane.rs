
use crate::hit;
use crate::ray;
use crate::vector;
use crate::vector::Vector;
use crate::materials;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub normal: Vector,
    offset: f64,
    pub material: materials::BaseMat,
}

impl Plane {
    pub fn new(normal: Vector, offset: f64, material: materials::BaseMat) -> Plane {
        Plane{normal: normal.normalise(), offset, material}
    }
}

impl hit::Hittable for Plane {
    fn intersect(&self, ray: ray::Ray) -> Option<hit::Hit> {
        let n_dot_dir = vector::dot(&self.normal, &ray.direction);
        let t = -(vector::dot(&self.normal, &ray.origin) + self.offset) / n_dot_dir;
        if (n_dot_dir == 0.0) || (t < 0.0) {
            None
        }
        else {
            Some(hit::Hit::new(t, ray.at(t), self.normal))
        }
    }
}