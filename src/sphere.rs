
use crate::hit;
use crate::ray;
use crate::vector;
use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Vector,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vector, radius: f64) -> Sphere {
        Sphere{center, radius}
    }

    pub fn normal(&self, point: Vector) -> Vector {
        // Check for validity ?
        (self.center - point).normalise()
    }

    pub fn intersect(&self, ray: ray::Ray) -> Option<hit::Hit> {
        // Build quadratic equasion: 0 = ax^2 + bx + c
        let orign_to_center = ray.origin - self.center;
        let a = 1.0; //if ray.dir not normalised: vector::dot(&ray.direction, &ray.direction
        let b = (vector::dot(&ray.direction, &orign_to_center)) * 2.0;
        let c = vector::dot(&orign_to_center, &orign_to_center) - self.radius*self.radius;

        let discriminant = b*b - 4.0*a*c;
        if discriminant > 0.0 {
            let t0 = (-b - discriminant.sqrt()) / 2.0;
            let t1 = (-b + discriminant.sqrt()) / 2.0;
            let t = t0.min(t1);
            
            let point = ray.at(t);
            let normal = self.normal(point);
            Some(hit::Hit::new(t, point, normal))
        }
        else {
            None
        } 
    }
}

