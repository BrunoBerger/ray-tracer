#![allow(dead_code)]
// #![allow(unused_variables)]

mod sphere;
mod ray;
mod vector;

use image::{RgbImage, ImageBuffer, Rgb};
use vector::Vector;

fn main() {
    
    // Camera setup
    let fov = 90_f64.to_radians();
    let up = Vector::new(0.0, 1.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let target = Vector::new(0.0, 0.0, 1.0);
    let t = (target - eye).normalise();
    let right = vector::cross(&up, &t).normalise();
    
    // Vectors to next pixel
    const IMAGE_WIDTH: u32 = 200;
    const IMAGE_HEIGHT: u32 = 200;
    let grid_width = 2.0*(fov/2.0).tan();
    let grid_height = grid_width;
    let dx = right * (grid_width / (IMAGE_WIDTH-1) as f64);
    let dy = -up * (grid_height / (IMAGE_HEIGHT-1) as f64);
    let top_left = t - right*(grid_width/2.0) + up*(grid_height/2.0);

    // Creating scene
    let sphere1 = sphere::Sphere::new(Vector::new(1.0, 0.0, 4.0), 1.0);


    // Shoot ray for each pixel
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, img_pixel) in buffer.enumerate_pixels_mut(){
        let pixel_vec = top_left + (dx*(x) as f64) + (dy*(y) as f64);
        let pixel_ray = ray::Ray::new(eye, pixel_vec);

        let col;
        if sphere1.intersect(pixel_ray) {
            col = 100;
        }
        else {
            col = 0;
        }
        *img_pixel = Rgb([col as u8, 0 as u8, 0]);
    }   
    buffer.save("image.png").unwrap();
}
