
// use crate::materials;
use crate::objects::*;
use crate::hit::Hittables;
use crate::vector::Vector;


#[derive(Debug)]
pub struct Scene {
    pub hittable_objects: Vec<Hittables>,
    pub light: light::Light,
}

pub fn get_sample_scene(up: Vector) -> Scene {
    let red = materials::Color::new(222, 0, 0);
    let green = materials::Color::new(0, 200, 20);
    let mat_red = materials::Material{ambient_color: red, ..Default::default()};
    let mat_green = materials::Material{ambient_color: green, ..Default::default()};
    let sphere1 = sphere::Sphere::new(Vector::new(1.0, 1.0, 5.0), 2.0, mat_red);
    let sphere2 = sphere::Sphere::new(Vector::new(-2.0, -1.0, 6.0), 3.0, mat_red);
    let plane = plane::Plane::new(up, -2.0, mat_green);
    let light = light::Light::new(Vector::new(-1.0, 2.0, 1.0), 10.0, materials::Color::new(0,0,255));
    Scene{
        hittable_objects: vec![
            Hittables::Sphere(sphere1), 
            Hittables::Sphere(sphere2),
            Hittables::Plane(plane),
        ], 
        light
    }
}