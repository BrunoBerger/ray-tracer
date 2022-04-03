
mod ray;
mod vector;

use image::{RgbImage, ImageBuffer, Rgb};
use vector::Vector;

fn main() {
    
    // Camera setup
    let fov = 90_f64.to_radians();
    let up = Vector::new(0.0, 1.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -2.0);
    let target = Vector::new(1.0, 0.0, 3.0);
    let t = (target - eye).normalise();
    let right = vector::cross(&up, &t).normalise();

    // Viewport sizes
    let hx = 2.0*(fov/2.0).tan();
    let hy = hx;

    // Vectors to next pixel
    const IMAGE_WIDTH: u32 = 100;
    const IMAGE_HEIGHT: u32 = 100;
    let dx = right * (hx / (IMAGE_WIDTH-1) as f64);
    let dy = -up * (hy / (IMAGE_HEIGHT-1) as f64);
    let top_Left = t - right*(hx/2.0) + up*(hy/2.0);


    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, img_pixel) in buffer.enumerate_pixels_mut(){
        let pixel_vec = top_Left + (dx*(x) as f64) + (dy*(y) as f64);
        let pixel_ray = ray::Ray::new(eye, pixel_vec);
        *img_pixel = Rgb([x as u8, y as u8, 0]);
    }
    buffer.save("image.png").unwrap();
}
