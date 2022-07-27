
use crate::ray;
use crate::hit;
use crate::objects::*;
use crate::vector::*;

use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    // points always counterclockwise
    pub p0: Vector,
    pub p1: Vector,
    pub p2: Vector,
    plane: plane::Plane,
    pub material: materials::BaseMat,
}
impl Triangle {
    pub fn calculate_plane(p0: Vector, p1: Vector, p2: Vector, mat: materials::BaseMat) -> plane::Plane {
        let normal = cross( &(p1-p0), &(p2-p1) ).normalise();
        let offset = normal.x*p0.x + normal.y*p0.y + normal.z*p0.z;
        plane::Plane::new(normal, offset, mat)
    }
    pub fn new(p0: Vector, p1: Vector, p2: Vector, material: materials::BaseMat) -> Triangle {
        let plane = Triangle::calculate_plane(p0, p1, p2, material);
        Triangle{p0, p1, p2, plane, material}
    }
}

impl hit::Hittable for Triangle {
    fn intersect(&self, ray: ray::Ray) -> Option<hit::Hit> {
        // compute plane's normal
        let p0p1 = self.p1 - self.p0;
        let p0p2 = self.p2 -self.p0;
        let n = cross(&p0p1, &p0p2);
        let _denom = dot(&n, &n);

        // Step 1: Find ray-plane intersection
        let n_dot_ray_direction = dot(&n, &ray.direction);
        if n_dot_ray_direction.abs() < crate::EPSILON {
            return None
        }
        let d = dot(&n, &self.p0);
        let t = (dot(&n, &ray.origin) + d) / n_dot_ray_direction;
        if t < 0.0 {
            return None
        }
        let hit_point = ray.origin + ray.direction * t;

        // Step 2: inside-outside test
        let mut c; // vector perpendicular to triangle's plane 

        let edge0 = self.p1 - self.p0;
        let vp0 = hit_point - self.p0;
        c = cross(&edge0, &vp0);
        if dot(&n, &c) < 0.0 {
            return None
        }

        let edge1 = self.p2 - self.p1;
        let vp1 = hit_point - self.p1;
        c = cross(&edge1, &vp1);
        let u = dot(&n, &c);
        if u < 0.0 {
            return None
        }

        let edge2 = self.p0 - self.p2;
        let vp2 = hit_point - self.p2;
        c = cross(&edge2, &vp2);
        let v = dot(&n, &c);
        if v < 0.0 {
            return None
        }

        // u /= denom;
        // v /= denom;

        Some(hit::Hit::new(t, hit_point, n))
    }
    fn bounding_box(&self) -> bounding::Aabb {
        unimplemented!();
    }
}
