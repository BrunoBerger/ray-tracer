use crate::ray;
use crate::vector;
use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    normal: Vector,
    offset: f64
}

impl Plane {
    pub fn new(normal: Vector, offset: f64) -> Plane {
    Plane{normal, offset}
    }

    pub fn intersect(&self, ray: ray::Ray) -> bool {
        let n_dot_dir = vector::dot(&self.normal, &ray.direction);
        let t = (vector::dot(&self.normal, &ray.origin) + self.offset) / n_dot_dir;
        if (n_dot_dir == 0.0) & (t < 0.0) {
            false
        }
        else {
            true
        }
    }
}