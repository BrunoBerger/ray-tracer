
use crate::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vector,
    pub intensity: f32,
    pub color: Vector,
}

impl Light {
    pub fn new(position: Vector, intensity: f32, color: Vector) -> Light {
        Light{position, intensity, color}
    }
}