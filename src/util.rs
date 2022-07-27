
use crate::vector::Vector;
use rand_distr::{UnitSphere, Distribution};

pub fn rndm_on_sphere() -> Vector {
    let v: [f32; 3] = UnitSphere.sample(&mut rand::thread_rng());
    Vector::new(v[0], v[1], v[2])
}