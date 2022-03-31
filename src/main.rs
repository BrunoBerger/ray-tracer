
mod ray;
mod vector;

use image::{RgbImage, ImageBuffer, Rgb};
use vector::Vector;

fn main() {
    // let o = vector::Vector{x:0.0, y:0.0, z:0.0};
    let o = Vector::new(0 as f64, f64::from(0), 0.0);
    let c = Vector::new(1.0, 0.0, 0.0);
    println!("{o}{c}");


    let dist = o.distance(&c);
    let len = c.length();
    println!("{dist} {len}");
    let dot = vector::dot(&o, &c);
    println!("dot:{dot}");

    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        *pixel = Rgb([x as u8, y as u8, 0]);
    }
    buffer.save("image.png").unwrap();
}