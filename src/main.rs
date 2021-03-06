#![allow(dead_code)]
// #![allow(unused_variables)]

mod hit;
mod objects;
mod ray;
mod vector;

use crate::objects::*;
use crate::hit::Hittable;
use vector::Vector;

const MAX_BOUNCES: i32 = 4;
const EPSILON: f64 = 0.0001;
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
    let fov = 90_f64.to_radians();
    let up = Vector::new(0.0, 1.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let target = Vector::new(0.0, 0.0, 1.0);
    let t = (target - eye).normalise();
    let right = vector::cross(&up, &t).normalise();

    // Vectors to next pixel
    let ratio = (image_width as f64) /(image_height as f64);
    let grid_width = 2.0*((fov/2.0).tan());
    let grid_height = grid_width / ratio;
    let dx = right * (grid_width / (image_width-1) as f64);
    let dy = -up * (grid_height / (image_height-1) as f64);
    let top_left = t - right*(grid_width/2.0) + up*(grid_height/2.0);

    // let scene = scene::get_sample_scene(up);
    let scene = scene::random_sphere_scene();

    // Shoot ray for each pixel
    let mut buffer: image::RgbImage = image::ImageBuffer::new(image_width, image_height);
    for (x, y, img_pixel) in buffer.enumerate_pixels_mut(){
        let pixel_vec = top_left + (dx*(x) as f64) + (dy*(y) as f64);
        let pixel_ray = ray::Ray::new(eye, pixel_vec);

        // Progress report every 10%
        if y % (image_height / 10) == 0 {
            print!("\r{} rows remaining ", image_height-y);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }

        let color = raytrace(&scene, pixel_ray, 0);

        *img_pixel = materials::Color::from_vector(color*255.0).to_img_rgb();
    }
    
    buffer.save("image.png").unwrap();
    println!("\nDone in: {:.2?}", timer_start.elapsed());
}


fn raytrace(scene: &scene::Scene, ray: ray::Ray, depth: i32) -> Vector {
    if depth > MAX_BOUNCES {
        Vector::new(0.0, 0.0, 0.0)
    }
    else {
        // painting in some fake sky background
        let t = 0.5*(ray.direction.y + 1.0);
        let mut color = Vector::new(1.0, 1.0, 1.0)*(1.0-t) + Vector::new(0.2, 0.5, 1.0)*t;

        let mut max_distance = f64::MAX;
        for object in &scene.hittable_objects {
            match object.intersect(ray) {
                None => {},
                Some(hit) => {
                    if hit.t < max_distance {
                        max_distance = hit.t;
                        let offset_hit_point = hit.point + hit.normal*crate::EPSILON;

                        // Color based on normals
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
                                let ac = mat.ambient_color.to_vector() / 255.0;
                                let ak = mat.ambient_intensity;
                                let a_part = ac * ak;
                                // Diffuseak
                                let dc = mat.diffuse_color.to_vector() / 255.0;
                                let dk = mat.diffuse_intensity;
                                let d_part = dc * dk * (vector::dot(&hit.normal, &light_dir));
                                // Speculardk
                                let sc = mat.specular_color.to_vector() / 255.0;
                                let sk = mat.specular_intensity;
                                let specular_falloff = 2;
                                let s_part = sc * sk * vector::dot(&refl_direction, &-ray.direction).powf(specular_falloff as f64);
                                color = a_part + d_part + s_part;
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
