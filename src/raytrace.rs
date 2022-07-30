
use crate::colors;
use crate::hit::Hittable;
use crate::objects::*;
use crate::ray;
use crate::util;
use crate::vector;
use crate::vector::Vector;

pub fn raytrace(scene: &scene::Scene, ray: ray::Ray, depth: i32) -> Vector {
    if depth > crate::MAX_BOUNCES {
        colors::BLACK
    }
    else {
        // painting in some fake sky background
        let t = 0.5*(ray.direction.y + 1.0);
        let mut color = colors::WHITE*(1.0-t) + Vector::new(0.2, 0.5, 1.0)*t;

        let mut max_distance = f32::MAX;
        for object in &scene.hittable_objects {
            match object.intersect(ray) {
                None => {},
                Some(hit) => {
                    if hit.t < max_distance {
                        max_distance = hit.t;
                        let offset_hit_point = hit.point + hit.normal*crate::EPSILON;

                        // Debug Option: Coloring based on normals
                        // let n = hit.normal;
                        // color = (Vector::new(n.x+1.0, n.y+1.0, n.z+1.0))*0.5;

                        let light_dir = scene.light.position - hit.point;
                        let refl_direction = ray.direction - hit.normal * vector::dot(&ray.direction, &hit.normal) * 2.0;

                        // Shadow
                        let mut shadow_color = colors::WHITE;
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
                                // Diffuse
                                let dc = mat.diffuse_color;
                                let dk = mat.diffuse_intensity;
                                let d_part = dc * dk * (vector::dot(&hit.normal, &light_dir));
                                // Specular
                                let sc = mat.specular_color;
                                let sk = mat.specular_intensity;
                                let specular_falloff = 2_f32;
                                let s_part = sc * sk * vector::dot(&refl_direction, &-ray.direction).powf(specular_falloff);
                                color = a_part + d_part + s_part;

                                // Scattering
                                // let mut indirect_color = colors::BLACK;
                                // for _ in 0..crate::SAMPLES {
                                //     let scatter_ray = ray::Ray::new(
                                //         offset_hit_point,
                                //         hit.normal + util::rndm_on_sphere()
                                //     );
                                //     indirect_color += raytrace(&scene, scatter_ray, depth +1)
                                // }
                                // indirect_color = indirect_color / crate::SAMPLES;
                                // color = color.scale(indirect_color);
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
