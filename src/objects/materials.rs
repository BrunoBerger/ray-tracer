
use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BaseMat {
    Metal(Metal),
    Lambertian(Lambertian),
    
}
impl BaseMat {
    pub fn new_metal(albedo: Color) -> BaseMat {
        BaseMat::Metal( Metal{albedo} )
    }
    pub fn new_lambertian(
        ambient_color: Color, ambient_intensity: f64,
        diffuse_color: Color, diffuse_intensity: f64,
        specular_color: Color, specular_intensity: f64
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
    pub albedo: Color,
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lambertian {
    pub ambient_color: Color,
    pub ambient_intensity: f64,
    pub diffuse_color: Color,
    pub diffuse_intensity: f64,
    pub specular_color: Color,
    pub specular_intensity: f64,

}

pub fn diffuse_from_color(in_color: Color) -> BaseMat {
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
    pub x: u8,
    pub y: u8,
    pub z: u8
}
impl Color {
    pub fn new(x: u8, y: u8, z: u8) -> Color {
        Color{x,y,z}
    }
    pub fn from_vector(vec: Vector) -> Color {
        Color{
            x: vec.x as u8,
            y: vec.y as u8,
            z: vec.z as u8
        }
    }
    pub fn to_vector(&self) -> Vector {
        Vector::new(self.x as f64, self.y as f64, self.z as f64)
    }
    pub fn to_img_rgb(&self) -> image::Rgb<u8> {
        image::Rgb([self.x, self.y, self.z])
    }
}
