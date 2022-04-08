use crate::vector::Vector;


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


    pub fn to_img_RGB(&self) -> image::Rgb<u8> {
        image::Rgb([self.x, self.y, self.z])
    }
}
