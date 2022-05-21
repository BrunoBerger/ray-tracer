#![allow(dead_code)]
// #![allow(unused_variables)]

mod hit;
mod util;
mod objects;
mod ray;
mod vector;

use crate::objects::*;
use crate::hit::Hittable;
use vector::Vector;

const MAX_BOUNCES: i32 = 4;
const EPSILON: f64 = 0.0001;


fn main() {
    let timer_start = std::time::Instant::now();

    // Camera setup
    let fov = 90_f64.to_radians();
    let up = Vector::new(0.0, 1.0, 0.0);
    let eye = Vector::new(0.0, 0.0, -1.0);
    let target = Vector::new(0.0, 0.0, 1.0);
    let t = (target - eye).normalise();
    let right = vector::cross(&up, &t).normalise();

    // Vectors to next pixel
    const IMAGE_WIDTH: u32 = 500;
    const IMAGE_HEIGHT: u32 = IMAGE_WIDTH;
    let grid_width = 2.0*((fov/2.0).tan());
    let grid_height = grid_width;
    let dx = right * (grid_width / (IMAGE_WIDTH-1) as f64);
    let dy = -up * (grid_height / (IMAGE_HEIGHT-1) as f64);
    let top_left = t - right*(grid_width/2.0) + up*(grid_height/2.0); //TODO: normalise ?

    // let scene = scene::get_sample_scene(up);
    let scene = scene::random_sphere_scene();

    // Shoot ray for each pixel
    let mut buffer: image::RgbImage = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, img_pixel) in buffer.enumerate_pixels_mut(){
        let pixel_vec = top_left + (dx*(x) as f64) + (dy*(y) as f64);
        let pixel_ray = ray::Ray::new(eye, pixel_vec);
        
        // if x == 250 && y == 125 {
        //     println!("Debug Pixel")
        // }

        let color = raytrace(&scene, pixel_ray, 0);

        *img_pixel = materials::Color::from_vector(color*255.0).to_img_rgb();
    }
    buffer.save("image.png").unwrap();

    let timer_elapsed = timer_start.elapsed();
    println!("Done in: {:.2?}", timer_elapsed);
}


fn raytrace(scene: &scene::Scene, ray: ray::Ray, depth: i32) -> Vector {
    let mut color = Vector::new(0.0, 0.0, 0.0);
    if depth > MAX_BOUNCES {
        Vector::new(0.0, 0.0, 0.0)
    }
    else { 
        // paint in some fake default-background
        // let t = 0.5*(ray.direction.y + 1.0);
        // color = Vector::new(1.0, 1.0, 1.0)*(1.0-t) + Vector::new(0.2, 0.5, 1.0)*t;

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
                        for s_object in &scene.hittable_objects {
                            if s_object != object { // 
                                match s_object.intersect(ray::Ray::new(offset_hit_point, light_dir)) {
                                    None => {},
                                    Some(s_hit) => {
                                        // let temp_t = hit.point.distance(&s_hit.point);
                                        if s_hit.t > 0.0 && s_hit.t < distance_to_light {
                                            shadow_color = shadow_color * 0.4; //TODO: what here
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        match object.material() {
                            materials::BaseMat::Lambertian(mat) => {
                                // Phong //TODO: better naming
                                // Ambient
                                let ca = mat.ambient_color.to_vector() / 255.0;
                                let ka = mat.ambient_intensity;
                                let a_part = ca * ka;
                                // Diffuse
                                let cd = mat.diffuse_color.to_vector() / 255.0;
                                let kd = mat.diffuse_intensity;
                                let d_part = cd * kd * (vector::dot(&hit.normal, &light_dir));
                                // Specular
                                let cs = mat.specular_color.to_vector() / 255.0;
                                let ks = mat.specular_intensity;
                                let specular_falloff = 2;
                                let s_part = cs * ks * vector::dot(&refl_direction, &-ray.direction).powf(specular_falloff as f64);
                                color = a_part + d_part + s_part;
                            }
                            materials::BaseMat::Metal(_mat) => {
                                let refl_ray = ray::Ray::new(offset_hit_point, refl_direction);
                                // let refl_color = Vector::new(0.0, 1.0, 0.0);
                                let refl_color = raytrace(&scene, refl_ray, depth+1);
                                color = refl_color;
                            }
                        }
                        
                        color = color.scale(shadow_color);
                    }
                }
            }
        }
        color
    }
}
