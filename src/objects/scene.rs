
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::objects::*;
use crate::hit::Hittables;
use crate::vector::Vector;

#[derive(Debug)]
pub struct Scene {
    pub hittable_objects: Vec<Hittables>,
    pub light: light::Light,
}

pub fn get_bounding_sample_scene() -> Scene {
    let mat_dif_red = materials::diffuse_from_color(colors::RED);

    let mut hittable_objects = Vec::new();
    let size = 3;
    let dist_to_cam = 3;
    for x in -size .. size {
        for y in -size .. size {
            for z in dist_to_cam .. size*2 + dist_to_cam {
                let s = sphere::Sphere::new(Vector::new(x as f32, y as f32, z as f32), 0.3, mat_dif_red);
                hittable_objects.push(Hittables::Sphere(s));
            }
        }
    }

    let light = light::Light::new(Vector::new(5.0, 5.0, 2.0), 1.0, Vector::new_from_u8(0,0,255));
    Scene{hittable_objects, light}
}

pub fn get_object_sample_scene(up: Vector) -> Scene {
    let red = Vector::new_from_u8(222, 0, 0);
    let green = Vector::new_from_u8(0, 200, 20);
    let blue = Vector::new_from_u8(0, 255, 255);
    let mat_dif_red = materials::diffuse_from_color(red);
    let mat_dif_green = materials::diffuse_from_color(green);
    let mat_metal = materials::BaseMat::new_metal(blue);
   
    let plane_ground = plane::Plane::new(up, 2.0, mat_dif_green);
    let plane_wall = plane::Plane::new(Vector::new(1.0, 0.0, 0.0), 2.4, materials::diffuse_from_color(blue));
    let sphere1 = sphere::Sphere::new(Vector::new(-2.0, -1.1, 5.0), 2.0, mat_metal);
    let sphere2 = sphere::Sphere::new(Vector::new(2.0, -1.0, 4.0), 1.5, mat_metal);
    let sphere3 = sphere::Sphere::new(Vector::new(0.0, 2.0, 4.5), 1.0, mat_dif_red);
    // let sphere4 = sphere::Sphere::new(sphere3.center, sphere3.center.distance(&sphere2.center), mat_red);
    let box1 = bounding::Aabb::new(
        Vector::new(2.0, -1.0, 4.0), 
        Vector::new(0.0, 2.0, 4.5)
    );
    let box2 = bounding::Aabb::new(
        Vector::new(-0.5, -1.5, 2.0), 
        Vector::new(0.5, -1.0, 2.5)
    );

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

    let light = light::Light::new(Vector::new(5.0, 5.0, 5.0), 1.0, Vector::new_from_u8(0,0,255));
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
            Hittables::Aabb(box1),
            Hittables::Aabb(box2),
        ], 
        light
    }
}

pub fn random_sphere_scene() -> Scene {
    base_sphere_scene(rand::thread_rng())
}
pub fn consistent_sphere_scene() -> Scene {
    base_sphere_scene(ChaCha8Rng::seed_from_u64(2))
}
fn base_sphere_scene<T>(mut rng: T) -> Scene
where T: rand::Rng
{
    let mut hittable_objects = Vec::new();
    // let mut rng = rand::thread_rng();
    // let mut rng = ChaCha8Rng::seed_from_u64(2);

    let grey = Vector::new_from_u8(180, 180, 180);

    let mat_metal = materials::BaseMat::new_metal(grey);
    let mat_diff = materials::diffuse_from_color(grey);
    let mut mats = Vec::new();
    mats.push(mat_metal);

    let plane_ground = plane::Plane::new(Vector::new(0.0, 1.0, 0.0), 4.0, mat_diff);
    hittable_objects.push(Hittables::Plane(plane_ground));

    for _ in 0 .. 200 {
        let new_pos = Vector::new(
            rng.gen_range(-8.0 .. 8.0),
            rng.gen_range(-4.0 .. 6.0),
            rng.gen_range(3.0 .. 20.0)
        );
        let new_mat = materials::diffuse_from_color(Vector::new_from_u8(
            rng.gen_range(0 ..= 255),
            rng.gen_range(0 ..= 255),
            rng.gen_range(0 ..= 255)
        ));

        let mats = vec![new_mat, mat_metal];

        let new_sphere = Hittables::Sphere(sphere::Sphere::new(
            new_pos,
            rng.gen_range(0.2 .. 1.0),
            mats[ rng.gen_range(0 ..= 1) ]
        ));
        hittable_objects.push(new_sphere);
    }

    let light = light::Light::new(Vector::new(5.0, 5.0, 2.0), 1.0, Vector::new_from_u8(0,0,255));
    Scene{hittable_objects, light}
}

pub fn path_trace_demo_scene() -> Scene {
    let mut hittable_objects = Vec::new();

    let white = Vector::new_from_u8(255, 255, 255);
    let red = Vector::new_from_u8(255, 0, 0);
    let green = Vector::new_from_u8(0, 255, 0);
    let gray = Vector::new_from_u8(180, 180, 180);

    let diff_white = materials::diffuse_from_color(white);
    let diff_red = materials::diffuse_from_color(red);
    let diff_green = materials::diffuse_from_color(green);
    let diff_gray = materials::diffuse_from_color(gray);
    // let metal_mat = materials::BaseMat::new_metal(white);

    let ground_plane = plane::Plane::new(Vector::new(0.0, 1.0, 0.0), 4.0, diff_white);
    let celing_plane = plane::Plane::new(Vector::new(0.0, -1.0, 0.0), 4.0, diff_white);
    let back_wall = plane::Plane::new(Vector::new(0.0, 0.0, -1.0), 8.0, diff_white);
    let left_wall = plane::Plane::new(Vector::new(1.0, 0.0, 0.0), 4.0, diff_red);
    let right_wall = plane::Plane::new(Vector::new(-1.0, 0.0, 0.0), 4.0, diff_green);
    
    // let metal_sphere = sphere::Sphere::new(Vector::new(1.2, -3.5, 5.0), 1.0, metal_mat);
    let diff_spehre = sphere::Sphere::new(Vector::new(-1.2, -4.0, 5.0), 1.0, diff_gray);

    hittable_objects.push( Hittables::Plane(ground_plane) );
    hittable_objects.push( Hittables::Plane(celing_plane) );
    hittable_objects.push( Hittables::Plane(back_wall) );
    hittable_objects.push( Hittables::Plane(left_wall) );
    hittable_objects.push( Hittables::Plane(right_wall) );
    // hittable_objects.push( Hittables::Sphere(metal_sphere) );
    hittable_objects.push( Hittables::Sphere(diff_spehre) );

    let light = light::Light::new(Vector::new(3.0, 3.0, 2.0), 1.0, Vector::new_from_u8(0,0,255));
    Scene{hittable_objects, light}
}