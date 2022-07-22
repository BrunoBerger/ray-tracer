
use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BaseMat {
    Metal(Metal),
    Lambertian(Lambertian),
    
}
impl BaseMat {
    pub fn new_metal(albedo: Vector) -> BaseMat {
        BaseMat::Metal( Metal{albedo} )
    }
    pub fn new_lambertian(
        ambient_color: Vector, ambient_intensity: f32,
        diffuse_color: Vector, diffuse_intensity: f32,
        specular_color: Vector, specular_intensity: f32
    ) -> BaseMat {
        BaseMat::Lambertian( Lambertian{
            ambient_color, ambient_intensity,
            diffuse_color, diffuse_intensity,
            specular_color, specular_intensity
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metal {
    pub albedo: Vector,
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lambertian {
    pub ambient_color: Vector,
    pub ambient_intensity: f32,
    pub diffuse_color: Vector,
    pub diffuse_intensity: f32,
    pub specular_color: Vector,
    pub specular_intensity: f32,

}

pub fn diffuse_from_color(in_color: Vector) -> BaseMat {
    BaseMat::Lambertian( Lambertian {
            ambient_color: in_color,
            ambient_intensity: 0.2,
            diffuse_color: in_color,
            diffuse_intensity: 0.1,
            specular_color: in_color,
            specular_intensity: 0.1,
        }
    )
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color{r,g,b}
    }
    pub fn from_vector(vec: Vector) -> Color {
        Color{
            r: vec.x as u8,
            g: vec.y as u8,
            b: vec.z as u8
        }
    }
    pub fn to_vector(&self) -> Vector {
        Vector::new(self.r as f32, self.g as f32, self.b as f32)
    }
    pub fn to_img_rgb(&self) -> image::Rgb<u8> {
        image::Rgb([self.r, self.g, self.b])
    }
}
