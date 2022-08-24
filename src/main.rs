#![allow(dead_code)]
// #![allow(unused_variables)]

use std::fs::File;
use std::io::BufWriter;

use rayon::prelude::*;

mod objects;
mod hit;
mod ray;
mod raytrace;
mod util;
mod vector;
mod encoding;

use crate::objects::*;
use vector::Vector;

const SAMPLES: i32 = 1;
const MAX_BOUNCES: i32 = 2;
const EPSILON: f32 = 0.0001;
const DEFAULT_RES: u32 = 500;
const BYTES_PER_PIXEL: usize = 3;

fn main() {
    let timer_start = std::time::Instant::now();

    // Check for args, otherwise fall back to default
    let image_width: u32;
    let image_height: u32;
    match std::env::args().nth(1) {
        Some(arg) => image_width = arg.parse::<u32>().unwrap(),
        None => image_width = DEFAULT_RES
    }
    match std::env::args().nth(2) {
        Some(arg) => image_height = arg.parse::<u32>().unwrap(),
        None => image_height = image_width
    }
    println!("Creating {image_width}x{image_height}px image.png, with {SAMPLES} sample(s) & {MAX_BOUNCES} bounces");

    // Camera setup
    let fov = 90_f32.to_radians();
    let up = Vector::new(0.0, 1.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let target = Vector::new(0.0, 0.0, 1.0);
    let t = (target - eye).normalise();
    let right = vector::cross(&up, &t).normalise();

    // Vectors to next pixel
    let ratio = (image_width as f32) /(image_height as f32);
    let grid_width = 2.0*((fov/2.0).tan());
    let grid_height = grid_width / ratio;
    let dx = right * (grid_width / (image_width-1) as f32);
    let dy = -up * (grid_height / (image_height-1) as f32);
    let top_left = t - right*(grid_width/2.0) + up*(grid_height/2.0);

    // let scene_def_for_tree = scene::get_bounding_sample_scene();
    // let scene_tree = tree::generate_tree(scene_def_for_tree);

    let scene_def =
        // scene::get_bounding_sample_scene();
        // scene::get_object_sample_scene(up);
        // scene::random_sphere_scene();
        scene::consistent_sphere_scene();
        // scene::path_trace_demo_scene();

    // setting up png-file and vector for pixel data
    const BYTES_PER_PIXEL: usize = 3;
    let file = File::create("image.png").unwrap();
    let mut encoder = png::Encoder::new(BufWriter::new(file), image_width, image_height);
    encoder.set_color(png::ColorType::Rgb);
    let mut writer = encoder.write_header().unwrap();
    let mut pixel_data = vec![0u8; image_height as usize * image_width as usize * BYTES_PER_PIXEL];

    println!("Setup done in: {:.2?}", timer_start.elapsed());
    let timer_raytrace = std::time::Instant::now();

    pixel_data.par_chunks_mut(BYTES_PER_PIXEL)
        .into_par_iter()
        .enumerate()
        .for_each(|(idx, chunk)| {
            // TODO fix for non-square aspect ratio
            let y = (idx / image_width as usize) as f32;
            let x = (idx % image_height as usize) as f32;
            // println!("{:#?} x:{x} y:{y}", rayon::current_thread_index().unwrap());
            let mut color = colors::BLACK;
            for _ in 0..SAMPLES {
                let pixel_vec = top_left + (dx*(x) as f32) + (dy*(y) as f32);
                let pixel_ray = ray::Ray::new(eye, pixel_vec);
                color += raytrace::raytrace(&scene_def, pixel_ray, 0);
            }
            color = color * 255.0 / SAMPLES;
            chunk[0] = color.x as u8;
            chunk[1] = color.y as u8;
            chunk[2] = color.z as u8;
        });
    
    println!("Raytracing done in: {:.2?}", timer_raytrace.elapsed());

    let timer_encode = std::time::Instant::now();
    writer.write_image_data(&pixel_data).unwrap();
    
    encoding::qoi_encode(
        pixel_data, 
        encoding::QoiDesc{
            width: image_width,
            height: image_height,
            channels: 3,
            colorspace: 255 
        });
    println!("Encoding done in: {:.2?}", timer_encode.elapsed());
}
