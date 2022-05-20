
use crate::hit;
use crate::ray;
// use crate::vector;
use crate::vector::Vector;
use crate::materials;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    min: Vector,
    max: Vector,
    pub material: materials::Material,
}

impl Aabb {
    pub fn new(min: Vector, max: Vector, material: materials::Material) -> Aabb {
        Aabb{
            min: Vector::new(
                f64::min(min.x, max.x),
                f64::min(min.y, max.y),
                f64::min(min.z, max.z)
            ), 
            max: Vector::new(
                f64::max(min.x, max.x),
                f64::max(min.y, max.y),
                f64::max(min.z, max.z)
            ), 
            material
        }
    }
}

impl hit::Hittable for Aabb {
    // using: https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection
    fn intersect(&self, ray: ray::Ray) -> Option<hit::Hit> {
        let mut temp_swap_var;
        let mut tmin = (self.min.x - ray.origin.x) / ray.direction.x;
        let mut tmax = (self.max.x - ray.origin.x) / ray.direction.x;
        if  tmin > tmax {
            // std::mem::swap(&mut tmin, &mut tmax) // unsafe
            temp_swap_var = tmin;
            tmin = tmax;
            tmax = temp_swap_var;
        }

        let mut tymin = (self.min.y - ray.origin.y) / ray.direction.y;
        let mut tymax = (self.max.y - ray.origin.y) / ray.direction.y;
        if  tymin > tymax {
            // std::mem::swap(&mut tmin, &mut tmax) // unsafe
            temp_swap_var = tymin;
            tymin = tymax;
            tymax = temp_swap_var;
        }

        if (tmin > tymax) || (tymin > tmax) {
            return None
        }
        if tymin > tmin {
            tmin = tymax;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min.z - ray.origin.z) / ray.direction.z;
        let mut tzmax = (self.max.z - ray.origin.z) / ray.direction.z;
        if tzmin > tzmax {
            temp_swap_var = tzmin;
            tzmin = tzmax;
            tzmax = temp_swap_var;
        }
        
        if (tmin > tzmax) || (tzmin > tmax) {
            return None
        }
        if  tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }

        Some(hit::Hit::new(tmin, ray.at(tmin), Vector::new(1.0, 1.0, 1.0)))
        //TODO create swap fn
    }
}