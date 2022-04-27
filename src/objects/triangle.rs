
use crate::ray;
use crate::hit;
use crate::objects::*;
use crate::vector::*;

use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    // points always counterclockwise
    p0: Vector,
    p1: Vector,
    p2: Vector,
    plane: plane::Plane,
    pub material: materials::Material,
}
impl Triangle {
    pub fn calculate_plane(p0: Vector, p1: Vector, p2: Vector, mat: materials::Material) -> plane::Plane {
        let normal = -cross( &(p1-p0), &(p2-p0) );
        let offset = normal.x*p0.x + normal.y*p0.y + normal.z*p0.z;
        plane::Plane::new(normal, offset, mat)
    }
    pub fn new(p0: Vector, p1: Vector, p2: Vector, material: materials::Material) -> Triangle {
        let plane = Triangle::calculate_plane(p0, p1, p2, material);
        Triangle{p0, p1, p2, plane, material}
    }
}


impl hit::Hittable for Triangle {
    fn intersect(&self, ray: ray::Ray) -> Option<hit::Hit> {
        // Implementing: https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/barycentric-coordinates
        // 1. Find intersection with plane of triangle
        match self.plane.intersect(ray) {
            None => None,
            Some(hit) => {
                // 2. Check if point in triangle
                let n = self.plane.normal;
                let mut c;

                let edge0 = self.p1 -self.p0;
                let vp0 = hit.point - self.p0;
                c = cross(&edge0, &vp0);
                if dot(&n, &c) < 0.0 {
                    return None
                }

                let edge1 = self.p2 -self.p1;
                let vp1 = hit.point - self.p1;
                c = cross(&edge1, &vp1);
                if dot(&n, &c) < 0.0 {
                    return None
                }
                
                let edge2 = self.p0 -self.p2;
                let vp2 = hit.point - self.p2;
                c = cross(&edge2, &vp2);
                if dot(&n, &c) < 0.0 {
                    return None
                }
                
                Some(hit::Hit::new(hit.t, hit.point, n))
            }
        }
    }  
}
