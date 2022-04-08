#![allow(dead_code)]
// #![allow(unused_variables)]

mod light;
mod plane;
mod sphere;
mod materials;
mod hit;

mod ray;
mod vector;

use crate::hit::Hittable;
use image::{RgbImage, ImageBuffer, Rgb};
use vector::Vector;

const MAX_BOUNCES: i32 = 2;


struct Scene<T: hit::Hittable> {
    pub objects: Vec<T>
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
    const IMAGE_HEIGHT: u32 = 300;
    let grid_width = 2.0*((fov/2.0).tan());
    let grid_height = grid_width;
    let dx = right * (grid_width / (IMAGE_WIDTH-1) as f64);
    let dy = -up * (grid_height / (IMAGE_HEIGHT-1) as f64);
    let top_left = t - right*(grid_width/2.0) + up*(grid_height/2.0);

    // Creating scene
    let red = materials::Color::new(222, 0, 0);
    let white = materials::Color::new(255, 255, 255);
    let sphere1 = sphere::Sphere::new(Vector::new(1.0, 0.0, 3.0), 1.0, red);
    let sphere2 = sphere::Sphere::new(Vector::new(0.0, -155.0, 6.0), 150.0, red);
    let light = light::Light::new(Vector::new(0.0, 4.0, 2.0), 10.0, white);
    // let plane = plane::Plane::new(up, -4.0);
    let scene = vec![sphere1, sphere2];

    // Shoot ray for each pixel
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, img_pixel) in buffer.enumerate_pixels_mut(){
        let pixel_vec = top_left + (dx*(x) as f64) + (dy*(y) as f64);
        let pixel_ray = ray::Ray::new(eye, pixel_vec.normalise());

        let mut color = Vector::new(0.0, 0.0, 0.0);
        color = raytrace(&mut color, &scene, pixel_ray, 0);
        
        *img_pixel = materials::Color::from_vector(color).to_img_RGB();
    }   
    buffer.save("image.png").unwrap();
}



// TODO: create color struct
fn raytrace(color: &mut Vector, scene: &std::vec::Vec<sphere::Sphere>, ray: ray::Ray, mut depth: i32) -> Vector {
    if depth > MAX_BOUNCES {
        Vector::new(0.0, 0.0, 0.0)
    }
    else {
        let mut max_distance: f64 = f64::MAX;
        for object in scene {
            match object.intersect(ray) {
                None => {},
                Some(hit) => {
                    if hit.t < max_distance {
                        max_distance = hit.t;
                        depth += 1;

                        // Color based on normals
                        let n = hit.normal;
                        let tmp_c = (Vector::new(n.x+1.0, n.y+1.0, n.z+1.0))*0.5;
                        *color = tmp_c*255.0;
                    }
                }
            }
        }
        *color
    }
}