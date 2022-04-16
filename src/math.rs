
use crate::vector::Vector;

pub fn rndm_on_unit_sphere() -> Vector {
    use rand_distr::{UnitSphere, Distribution};
    let v: [f64; 3] = UnitSphere.sample(&mut rand::thread_rng());
    Vector::new(v[0], v[1], v[2])
}

pub fn phong_lighting() -> Vector {
    unimplemented!();
}