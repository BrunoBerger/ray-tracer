
use crate::ray;
use crate::vector;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: vector::Vector,
    radius: f64
}

impl Sphere {
    pub fn new(center: vector::Vector, radius: f64) -> Sphere {
        Sphere{center, radius}
    }

    pub fn intersect(&self, ray: ray::Ray) -> bool {
        
        // Build quadratic equasion: 0 = ax^2 + bx + c
        let orign_to_center = self.center - ray.origin;
        let a = vector::dot(&ray.direction, &ray.direction);
        let b = (vector::dot(&ray.direction, &orign_to_center)) * 2.0;
        let c = vector::dot(&orign_to_center, &orign_to_center) - self.radius.sqrt();

        let discriminant = b*b - 4.0*a*c;
    
        if discriminant > 0.0 {
            true
        }
        else {
            false
        } 
    }
}

