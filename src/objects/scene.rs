
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
   
    let plane_ground = plane::Plane::new(up, 2.0, mat_green);
    let plane_wall = plane::Plane::new(Vector::new(1.0, 0.0, 0.0), 2.0, mat_green);
    let sphere1 = sphere::Sphere::new(Vector::new(-2.0, -1.1, 5.0), 1.0, mat_red);
    let sphere2 = sphere::Sphere::new(Vector::new(2.0, -1.0, 4.0), 1.0, mat_red);
    let sphere3 = sphere::Sphere::new(Vector::new(0.0, 2.0, 4.5), 1.0, mat_red);
    // let sphere4 = sphere::Sphere::new(sphere3.center, sphere3.center.distance(&sphere2.center), mat_red);

    // let triangle1 = triangle::Triangle::new(
    //     Vector::new(-1.0, -1.0, 3.0), //3
    //     Vector::new(-1.5, -3.0, 3.0), //2
    //     Vector::new(-2.0, -1.0, 4.0), //1
    //     materials::diffuse_from_color(red)
    // );
    // let triangle2 = triangle::Triangle::new(
    //     // Vector::new(0.0, 0.0, 3.0), //1
    //     sphere1.center,
    //     sphere2.center,
    //     sphere3.center,
    //     materials::diffuse_from_color(red)
    // );

    let light = light::Light::new(Vector::new(5.0, 5.0, 5.0), 1.0, materials::Color::new(0,0,255));
    // let light_pos_sphere = sphere::Sphere::new(light.position, 1.0, mat_red);
   
    Scene{
        hittable_objects: vec![
            Hittables::Sphere(sphere1), 
            Hittables::Sphere(sphere2),
            Hittables::Sphere(sphere3),
            // Hittables::Sphere(sphere4),
            // Hittables::Sphere(light_pos_sphere),
            Hittables::Plane(plane_ground),
            Hittables::Plane(plane_wall),
            // Hittables::Triangle(triangle1),
            // Hittables::Triangle(triangle2),
        ], 
        light
    }
}