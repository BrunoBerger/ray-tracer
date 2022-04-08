
use crate::vector::Vector;
use crate::materials;

#[derive(Debug, Clone, Copy)]
pub struct Light {
    position: Vector,
    intensity: f64,
    color: materials::Color,
}

impl Light {
    pub fn new(position: Vector, intensity: f64, color: materials::Color) -> Light {
        Light{position, intensity, color}
    }
}