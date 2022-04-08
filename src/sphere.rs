
use crate::hit;
use crate::ray;
use crate::vector;
use crate::vector::Vector;
use crate::materials;


#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Vector,
    radius: f64,
    pub color: materials::Color,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, color: materials::Color) -> Sphere {
        Sphere{center, radius, color}
    }

    pub fn normal(&self, point: Vector) -> Vector {
        // Check for validity ?
        (self.center - point).normalise()
    }
}

impl hit::Hittable for Sphere {
    fn intersect(&self, ray: ray::Ray) -> Option<hit::Hit> {
        // Build quadratic equasion: 0 = ax^2 + bx + c
        let orign_to_center = ray.origin - self.center;
        let a = 1.0; //if ray.dir not normalised: vector::dot(&ray.direction, &ray.direction
        let b = (vector::dot(&ray.direction, &orign_to_center)) * 2.0;
        let c = vector::dot(&orign_to_center, &orign_to_center) - self.radius*self.radius;

        let discriminant = b*b - 4.0*a*c;
        if discriminant > 0.0 {
            let t0 = (-b - discriminant.sqrt()) / 2.0;
            let t1 = (-b + discriminant.sqrt()) / 2.0;
            
            // TODO: redo this better
            let t;
            if (t0 < 0.0) & (t1 > 0.0) {
                t = t1;
            }
            else if (t1 > 0.0) & (t1 < 0.0) {
                t = t0;
            }
            else if  (t1 > 0.0) & (t1 > 0.0) {
                t = t0.min(t1);
            }
            else {
                return None
            }          
            // println!("{}{}{}", t0, t1, t);
            let point = ray.at(t);
            let normal = self.normal(point);
            Some(hit::Hit::new(t, point, normal))
        }
        else {
            None
        } 
    }
}

