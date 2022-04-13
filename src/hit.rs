
use crate::ray;
use crate::vector::Vector;
use crate::objects::*;

pub trait Hittable {
    fn intersect(&self, ray: ray::Ray) -> Option<Hit>;
}

#[derive(Debug, Clone, Copy)]
pub enum Hittables {
    Sphere(sphere::Sphere),
    Plane(plane::Plane),
}
impl Hittables {
    pub fn material(&self) -> materials::Material {
        match *self {
            Hittables::Sphere(obj) => obj.material,
            Hittables::Plane(obj) => obj.material,
        }
    }
}
impl Hittable for Hittables {
    fn intersect(&self, ray: ray::Ray) -> Option<Hit> {
        match self {
            Hittables::Sphere(obj) => obj.intersect(ray),
            Hittables::Plane(obj) => obj.intersect(ray),
        }
    }
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
