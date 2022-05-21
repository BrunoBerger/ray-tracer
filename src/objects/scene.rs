
use rand::Rng;
// use rand::seq::IteratorRandom;

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
    let blue = materials::Color::new(0, 255, 255);
    let mat_dif_red = materials::diffuse_from_color(red);
    let mat_dif_green = materials::diffuse_from_color(green);
    let mat_metal = materials::BaseMat::new_metal(blue);
   
    let plane_ground = plane::Plane::new(up, 2.0, mat_dif_green);
    let plane_wall = plane::Plane::new(Vector::new(1.0, 0.0, 0.0), 2.4, materials::diffuse_from_color(blue));
    let sphere1 = sphere::Sphere::new(Vector::new(-2.0, -1.1, 5.0), 2.0, mat_metal);
    let sphere2 = sphere::Sphere::new(Vector::new(2.0, -1.0, 4.0), 1.5, mat_metal);
    let sphere3 = sphere::Sphere::new(Vector::new(0.0, 2.0, 4.5), 1.0, mat_dif_red);
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

pub fn random_sphere_scene() -> Scene {
    let mut hittable_objects = Vec::new();
    let mut rng = rand::thread_rng();

    let grey = materials::Color::new(180, 180, 180);

    let mat_metal = materials::BaseMat::new_metal(grey);
    let mat_diff = materials::diffuse_from_color(grey);
    let mut mats = Vec::new();
    mats.push(mat_metal);

    let plane_ground = plane::Plane::new(Vector::new(0.0, 1.0, 0.0), 3.0, mat_diff);
    hittable_objects.push(Hittables::Plane(plane_ground));

    for _ in 0 .. 200 {
        let new_pos =Vector::new(
            rng.gen_range(-8.0 .. 8.0),
            rng.gen_range(-3.0 .. 6.0),
            rng.gen_range(3.0 .. 20.0)
        );
        let new_mat = materials::diffuse_from_color(materials::Color::new(
            rng.gen_range(0 .. 255),
            rng.gen_range(0 .. 255),
            rng.gen_range(0 .. 255)
        ));

        let mats = vec![new_mat, mat_metal];

        let new_sphere = Hittables::Sphere(sphere::Sphere::new(
            new_pos,
            rng.gen_range(0.2 .. 1.0),
            // mats.choose(&mut rng).unwrap()
            mats[ rng.gen_range(0 ..= 1) ]
        ));
        hittable_objects.push(new_sphere);
    }

    let light = light::Light::new(Vector::new(5.0, 5.0, 2.0), 1.0, materials::Color::new(0,0,255));
    Scene{
        hittable_objects,
        light,
    }
}
