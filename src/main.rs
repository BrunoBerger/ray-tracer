#![allow(dead_code)]
// #![allow(unused_variables)]
// use std::fs::File;
// use std::io::BufWriter;
use std::io::prelude::*;
use rayon::prelude::*;

mod hit;
mod objects;
mod ray;
mod vector;
mod util;

use crate::colors;
use crate::objects::*;
use crate::hit::Hittable;
use vector::Vector;

const SAMPLES: i32 = 4;
const MAX_BOUNCES: i32 = 4;
const EPSILON: f32 = 0.0001;
const DEFAULT_RES: u32 = 500;


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
    println!("Creating {image_width}x{image_height} image.png");

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
        scene::random_sphere_scene();
        // scene::path_trace_demo_scene();

    println!("Setup done in: {:.2?}", timer_start.elapsed());
    let timer_raytrace = std::time::Instant::now();
    // Shoot ray for each pixel
    
    // let file = File::create("image.png").unwrap();
    // let mut encoder = png::Encoder::new(BufWriter::new(file), image_width, image_height);
    // encoder.set_color(png::ColorType::Rgb);
    // encoder.set_depth(png::BitDepth::Eight);
    // let mut writer = encoder.write_header().unwrap().into_stream_writer().unwrap();
    // for y in (0..image_height as usize).par_iter_mut() {
    //     // eprint!("{}/{}\n", image_height - 1 - y, image_height);
    //     for x in 0..image_width {
    //         let mut color = Vector::new(0.0, 0.0, 0.0);
    //         for _ in 0 .. SAMPLES {
    //             let pixel_vec = top_left + (dx*(x) as f32) + (dy*(y) as f32);
    //             let pixel_ray = ray::Ray::new(eye, pixel_vec);
    //             color += raytrace(&scene_def, pixel_ray, 0);
    //         }
    //         writer.write(&(color / SAMPLES).encode()).unwrap();
    //     }
    // }
    
    // New file writing from https://medium.com/@cfsamson/from-48s-to-5s-optimizing-a-350-line-pathtracer-in-rust-191ab4a1a412
    let filename = String::from("output-rust.ppm");
    println!(
        "Width: = {}, Height: = {}, Samples = {}",
        image_width, image_height, SAMPLES
    );
    println!("Writing data to {}", filename);
    let mut file = std::fs::File::create(filename).unwrap();
    write!(file, "P6 {} {} 255 ", image_width, image_height).unwrap();
    const BYTES_PER_PIXEL: usize = 3;
    let mut bytes = vec![0u8; image_height as usize * image_width as usize * BYTES_PER_PIXEL];
    bytes.par_chunks_mut(BYTES_PER_PIXEL)
        .into_par_iter()
        .enumerate()
        .for_each(|(idx, chunk)| {
            let y = (idx / image_width as usize) as f32;
            let x = (idx % image_height as usize) as f32;
            let mut color = colors::BLACK;
            for _ in 0..SAMPLES {
                let pixel_vec = top_left + (dx*(x) as f32) + (dy*(y) as f32);
                let pixel_ray = ray::Ray::new(eye, pixel_vec);
                color += raytrace(&scene_def, pixel_ray, 0);
            }
            color = color * 255.0;
            chunk[0] = color.x as u8;
            chunk[1] = color.y as u8;
            chunk[2] = color.z as u8;
        });

    file.write_all(&bytes).unwrap();

    println!("Raytracing done in: {:.2?}", timer_raytrace.elapsed());
    println!("Complete time: {:.2?}", timer_start.elapsed());
}


fn raytrace(scene: &scene::Scene, ray: ray::Ray, depth: i32) -> Vector {
    if depth > MAX_BOUNCES {
        colors::BLACK
    }
    else {
        // painting in some fake sky background
        let t = 0.5*(ray.direction.y + 1.0);
        let mut color = Vector::new(1.0, 1.0, 1.0)*(1.0-t) + Vector::new(0.2, 0.5, 1.0)*t;

        let mut max_distance = f32::MAX;
        for object in &scene.hittable_objects {
            match object.intersect(ray) {
                None => {},
                Some(hit) => {
                    if hit.t < max_distance {
                        max_distance = hit.t;
                        let offset_hit_point = hit.point + hit.normal*crate::EPSILON;

                        // Vector based on normals
                        // let n = hit.normal;
                        // color = (Vector::new(n.x+1.0, n.y+1.0, n.z+1.0))*0.5;

                        let light_dir = scene.light.position - hit.point;
                        let refl_direction = ray.direction - hit.normal * vector::dot(&ray.direction, &hit.normal) * 2.0;

                        // Shadow
                        let mut shadow_color = Vector::new(1.0, 1.0, 1.0);
                        let distance_to_light = hit.point.distance(&scene.light.position);
                        for shadow_object in &scene.hittable_objects {
                            if shadow_object != object {
                                match shadow_object.intersect(ray::Ray::new(offset_hit_point, light_dir)) {
                                    None => {},
                                    Some(s_hit) => {
                                        if s_hit.t > 0.0 && s_hit.t < distance_to_light {
                                            shadow_color = shadow_color * 0.4;
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        // Shading or Reflection
                        match object.material() {
                            materials::BaseMat::Lambertian(mat) => {
                                // Phong Shading
                                // Ambient
                                let ac = mat.ambient_color;
                                let ak = mat.ambient_intensity;
                                let a_part = ac * ak;
                                // Diffuseak
                                let dc = mat.diffuse_color;
                                let dk = mat.diffuse_intensity;
                                let d_part = dc * dk * (vector::dot(&hit.normal, &light_dir));
                                // Speculardk
                                let sc = mat.specular_color;
                                let sk = mat.specular_intensity;
                                let specular_falloff = 2_f32;
                                let s_part = sc * sk * vector::dot(&refl_direction, &-ray.direction).powf(specular_falloff);
                                color = a_part + d_part + s_part;


                                // Scattering
                                let mut indirect_color = Vector::new(0.0, 0.0, 0.0);
                                for _ in 0..crate::SAMPLES {
                                    let scatter_ray = ray::Ray::new(
                                        offset_hit_point,
                                        hit.normal + util::rndm_on_sphere()
                                    );
                                    indirect_color += raytrace(&scene, scatter_ray, depth+2)
                                }
                                indirect_color = indirect_color / crate::SAMPLES;

                                color.scale(indirect_color*0.2);
                            }
                            materials::BaseMat::Metal(_mat) => {
                                let refl_ray = ray::Ray::new(offset_hit_point, refl_direction);
                                let refl_color = raytrace(&scene, refl_ray, depth+1);
                                color = refl_color;
                            }
                        }
                        color = color.scale(shadow_color);
                    }
                }
            }
        }
        color // return from the top-else
    }
}
