
use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector
}

impl Ray {
    pub fn new (origin: Vector, direction: Vector) -> Ray {
        Ray{origin, direction: direction.normalise()}
    }

    pub fn at (&self, t: f32) -> Vector {
        self.origin + self.direction*t
    }
}


impl std::fmt::Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}+t*{}", self.origin, self.direction)
    }
}
