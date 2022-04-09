
use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub ambient_color: Color,
    pub ambient_intensity: f64,
    pub diffuse_color: Color,
    pub diffuse_intensity: f64,
    pub specular_color: Color,
    pub specular_intensity: f64,

}
impl Material {
    pub fn new(
        ambient_color: Color, ambient_intensity: f64,
        diffuse_color: Color, diffuse_intensity: f64,
        specular_color: Color, specular_intensity: f64
    ) -> Material {
        Material{
            ambient_color, ambient_intensity,
            diffuse_color, diffuse_intensity,
            specular_color, specular_intensity
        }
    }
}
impl Default for Material {
    fn default() -> Material {
        let white = Color::new(255, 255, 255);
        Material {
            ambient_color: white,
            ambient_intensity: 1.0,
            diffuse_color: white,
            diffuse_intensity: 1.0,
            specular_color: white,
            specular_intensity: 1.0,
        }
    }
}


#[derive(Debug, Clone, Copy)]
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
