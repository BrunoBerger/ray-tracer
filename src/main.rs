#![allow(dead_code)]
#![allow(unused_variables)]

mod sphere;
mod plane;
mod triangle;
mod light;
mod materials;

mod math;
mod hit;
mod ray;
mod vector;

use crate::hit::Hittable;
use image::{RgbImage, ImageBuffer, Rgb};
use vector::Vector;

const MAX_BOUNCES: i32 = 2;


struct Scene<T: hit::Hittable> {
    pub objects: Vec<T>,
    pub light: light::Light,
}

fn main() {
    // Camera setup
    let fov = 90_f64.to_radians();
    let up = Vector::new(0.0, 1.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let target = Vector::new(0.0, 0.0, 1.0);
    let t = (target - eye).normalise();
    let right = vector::cross(&up, &t).normalise();
    
    // Vectors to next pixel
    const IMAGE_WIDTH: u32 = 300;
    const IMAGE_HEIGHT: u32 = IMAGE_WIDTH;
    let grid_width = 2.0*((fov/2.0).tan());
    let grid_height = grid_width;
    let dx = right * (grid_width / (IMAGE_WIDTH-1) as f64);
    let dy = -up * (grid_height / (IMAGE_HEIGHT-1) as f64);
    let top_left = t - right*(grid_width/2.0) + up*(grid_height/2.0); //TODO: normalise ?

    // Creating scene
    let red = materials::Color::new(222, 0, 0);
    let green = materials::Color::new(0, 200, 20);
    let mat_red = materials::Material{ambient_color: red, ..Default::default()};
    let mat_green = materials::Material{ambient_color: green, ..Default::default()};
    let sphere1 = sphere::Sphere::new(Vector::new(1.0, 0.0, 3.0), 2.0, mat_red);
    let sphere2 = sphere::Sphere::new(Vector::new(0.0, -151.0, 6.0), 150.0, mat_green);
    // let plane = plane::Plane::new(up, -4.0);
    let light = light::Light::new(Vector::new(-1.0, 2.0, 2.0), 10.0, materials::Color::new(0,0,255));

    // TODO: generic collection?
    let scene = Scene{objects: vec![sphere1, sphere2], light};

    // Shoot ray for each pixel
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, img_pixel) in buffer.enumerate_pixels_mut(){
        let pixel_vec = top_left + (dx*(x) as f64) + (dy*(y) as f64);
        let pixel_ray = ray::Ray::new(eye, pixel_vec.normalise());

        let mut color = Vector::new(0.0, 0.0, 0.0);
        color = raytrace(&mut color, &scene, pixel_ray, 0);
        
        *img_pixel = materials::Color::from_vector(color).to_img_rgb();
    }   
    buffer.save("image.png").unwrap();
}


fn raytrace(color: &mut Vector, scene: &Scene<sphere::Sphere>, ray: ray::Ray, mut depth: i32) -> Vector {
    if depth > MAX_BOUNCES {
        Vector::new(0.0, 0.0, 0.0)
    }
    else {
        let mut max_distance: f64 = f64::MAX;
        for object in &scene.objects {
            match object.intersect(ray) {
                None => {},
                Some(hit) => {
                    if hit.t < max_distance {
                        max_distance = hit.t;
                        depth += 1;

                        // Color based on normals
                        // let n = hit.normal;
                        // let tmp_c = (Vector::new(n.x+1.0, n.y+1.0, n.z+1.0))*0.5;
                        // *color = tmp_c*255.0;

                        // let refl_direction = ray.direction*2.0*vector::dot(&ray.direction, &hit.normal) - ray.direction;
                        // let refl_ray = ray::Ray::new(hit.point, refl_direction);
                        // let refl_color; //?
                        // let refl_color = raytrace(refl_color, &scene, refl_ray, depth);

                        // Phong //TODO: better naming
                        let ca = object.material.ambient_color.to_vector() / 255.0;
                        let ka = object.material.ambient_intensity;

                        let light_dir = (hit.point - scene.light.position).normalise();
                        let cd = object.material.diffuse_color.to_vector() / 255.0;
                        let kd = object.material.diffuse_intensity;

                        *color = ca*ka + cd*kd*(vector::dot(&hit.normal, &light_dir));
                        *color = *color * 255.0;
                    }
                }
            }
        }
        *color
    }
}