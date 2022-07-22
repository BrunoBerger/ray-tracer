
use crate::ray;
use crate::vector::Vector;
use crate::objects::*;

pub trait Hittable {
    fn intersect(&self, ray: ray::Ray) -> Option<Hit>;
    fn bounding_box(&self) -> bounding::Aabb;
    // fn get_center(&self) -> Vector;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hittables {
    Sphere(sphere::Sphere),
    Plane(plane::Plane),
    Triangle(triangle::Triangle),
    Aabb(bounding::Aabb)
}
// Possiblities if this does not work out:
// https://stackoverflow.com/questions/26378842/how-do-i-overcome-match-arms-with-incompatible-types-for-structs-implementing-sa
impl Hittables {
    pub fn material(&self) -> materials::BaseMat {
        match *self {
            Hittables::Sphere(obj) => obj.material,
            Hittables::Plane(obj) => obj.material,
            Hittables::Triangle(obj) => obj.material,
            Hittables::Aabb(_) => materials::BaseMat::new_metal(colors::BLACK)
        }
    }
    pub fn get_center(&self) -> Vector {
        match self {
            Hittables::Sphere(obj) => obj.center,
            Hittables::Plane(obj) => obj.normal * obj.offset,
            Hittables::Triangle(obj) => (obj.p0 + obj.p1 + obj.p2) / 3,
            Hittables::Aabb(obj) => obj.max - obj.min
        }
    }
    pub fn get_bounds(&self) -> bounding::Aabb {
        self.bounding_box()
    }
}
impl Hittable for Hittables {
    fn intersect(&self, ray: ray::Ray) -> Option<Hit> {
        match self {
            Hittables::Sphere(obj) => obj.intersect(ray),
            Hittables::Plane(obj) => obj.intersect(ray),
            Hittables::Triangle(obj) => obj.intersect(ray),
            Hittables::Aabb(obj) => obj.intersect(ray)
        }
    }
    fn bounding_box(&self) -> bounding::Aabb {
        match self {
            Hittables::Sphere(obj) => obj.bounding_box(),
            Hittables::Plane(obj) => obj.bounding_box(),
            Hittables::Triangle(obj) => obj.bounding_box(),
            Hittables::Aabb(obj) => obj.bounding_box()
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub t: f32,
    pub point: Vector,
    pub normal: Vector,
}
impl Hit {
    pub fn new (t: f32, point: Vector, normal: Vector) -> Hit {
        Hit{t, point, normal: normal.normalise()}
    }
}
impl std::fmt::Display for Hit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "P{}N{}", self.point, self.normal)
    }
}
