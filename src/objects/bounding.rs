
use crate::hit;
use crate::ray;
// use crate::vector;
use crate::vector::Vector;
// use crate::materials;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: Vector,
    pub max: Vector,
    // pub material: materials::BaseMat,
}

impl Aabb {
    pub fn new(min: Vector, max: Vector) -> Aabb {
        Aabb{
            min: Vector::new(
                f32::min(min.x, max.x),
                f32::min(min.y, max.y),
                f32::min(min.z, max.z)
            ), 
            max: Vector::new(
                f32::max(min.x, max.x),
                f32::max(min.y, max.y),
                f32::max(min.z, max.z)
            ), 
            // material
        }
    }
}
pub fn add_boxes(a: Aabb, b: Aabb) -> Aabb {
    let new_min = Vector::new(
        f32::min(a.min.x, b.min.x),
        f32::min(a.min.y, b.min.y),
        f32::min(a.min.z, b.min.z)
    );
    let new_max = Vector::new(
        f32::max(a.max.x, b.max.x),
        f32::max(a.max.y, b.max.y),
        f32::max(a.max.z, b.max.z)
    );
    Aabb::new(new_min, new_max)
}

// impl hit::Hittable for Aabb {
//     fn intersect(&self, ray: ray::Ray) -> Option<hit::Hit> {
//         // return None;

//         // Using: https://raytracing.github.io/books/RayTracingTheNextWeek.html#boundingvolumehierarchies/anoptimizedaabbhitmethod
//         let mut tmin = 5.0;
//         let mut tmax = 5.0;
//         let mut invDirAxis = 1.0 / ray.direction.x;
//         let mut t0 = (&self.min.x - ray.origin.x) * invDirAxis;
//         let mut t1 = (&self.max.x - ray.origin.x) * invDirAxis;
//         if invDirAxis < 0.0 {std::mem::swap(&mut t0, &mut t1)}
//         tmin = if t0 < tmin {t0} else {tmin};
//         tmax = if t1 < tmax {t1} else {tmax};
//         if tmax <= tmin {
//             return None
//         }

//         Some(hit::Hit::new(t0, ray.at(tmin), ray.direction))
//     }
// }

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

        Some(hit::Hit::new(tmin, ray.at(tmax), Vector::new(1.0, 1.0, 1.0), Box::new(*self)))
        //TODO create swap fn
    }
    fn bounding_box(&self) -> Aabb {
        *self
    }
}